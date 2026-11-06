mod cmd;
mod constants;
mod macros;
mod osc;
mod terminal;
mod toio;
mod updates;

use crate::{osc::*, terminal::*, toio::*, updates::*};
use clap::Parser;
use futures::future::join_all;
use futures::future::Either::{Left, Right};
use std::time::{Duration, SystemTime};
use std::{error::Error, process, sync::atomic::Ordering, sync::Arc};
use tokio::sync::{mpsc::error::TrySendError, RwLock};
use tokio::{net::UdpSocket, time};

type ToioLock = Arc<RwLock<Vec<Arc<RwLock<Toio>>>>>;

#[derive(Parser)]
#[command(name = "toio")]
struct Args {
    /// Set receiving port
    #[arg(short, long)]
    port: Option<usize>,

    /// Set remote port
    #[arg(short, long)]
    remote: Option<usize>,

    /// Set remote IP
    #[arg(short, long)]
    ip: Option<String>,

    /// Show debug terminal stream instead of terminal UI
    #[arg(short, long)]
    debug: bool,

    /// Use unfiltered search
    #[arg(short, long)]
    search: bool,

    /// Use ordered search
    #[arg(short, long)]
    ordered: bool,

    /// Filter toios by IDs (comma-separated list e.g. 1,2,3)
    #[arg(short, long, value_delimiter = ',')]
    axlab_id: Option<Vec<usize>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    macros::DEBUG.store(args.debug, Ordering::Relaxed);

    // create scanner and array of toios
    let scanner = match args.axlab_id.clone() {
        Some(filter) => {
            if args.search {
                println!("You provide a filter and be in search mode");
                process::exit(0);
            }

            debugPrint!(
                "Running {} Search for Toios: {:?}",
                if args.ordered { "Ordered" } else { "Unordered" },
                filter
            );
            ToioScanner::new_filtered(args.ordered, filter.clone()).await?
        }
        None => {
            if !args.search {
                println!("You must provide IDs when not in search mode");
                process::exit(0);
            }

            if args.ordered {
                println!("You cannot be in ordered mode when not in search mode");
                process::exit(0);
            }

            debugPrint!("Running Unfiltered Search for Toios");
            ToioScanner::new_search().await?
        }
    };

    // parse server and client address from args
    let host_addr = format!("0.0.0.0:{}", args.port.unwrap_or(3334));
    let to_addr = format!(
        "{}:{}",
        args.ip.unwrap_or("127.0.0.1".to_string()),
        args.remote.unwrap_or(3333)
    )
    .parse()?;

    debugPrint!("Listening at {} and sending to {}", host_addr, to_addr);

    // start scanning for toios
    let mut toios = scanner.search().await?;
    let toio_lock: ToioLock = Arc::new(RwLock::new(vec![]));

    // open socket and create buffer
    let socket = Arc::new(UdpSocket::bind(&host_addr).await?);
    let mut buf = [0u8; rosc::decoder::MTU];

    // listen for commands over OSC and forward to toio
    let socket_clone = socket.clone();
    let toio_lock_clone = toio_lock.clone();
    tokio::spawn(async move {
        while let Ok((size, _)) = socket_clone.recv_from(&mut buf).await {
            if let Ok((_, packet)) = rosc::decoder::decode_udp(&buf[..size]) {
                match handle_packet(packet) {
                    CommandResult::Success { toio_num, cmd } => {
                        debugPrint!("Command Sent: {:?}", cmd);

                        // find toio from connected
                        let toio = toio_lock_clone.read().await.get(toio_num).cloned();
                        if let Some(toio) = toio {
                            let toio = toio.read().await;

                            // Update last command timestamp
                            toio.update_last_command();

                            // add command to toio command queue
                            if let Err(err) = toio.queue_command(cmd) {
                                let reason = match err {
                                    TrySendError::Full(_) => "Command Queue Full",
                                    TrySendError::Closed(_) => "Toio Disconnected",
                                };
                                debugPrint!("Command Dropped for toio {}: {}", toio_num, reason);
                            }
                        }
                    }
                    CommandResult::Error {
                        toio_num,
                        name,
                        vals,
                    } => {
                        debugPrint!(
                            "Error Parsing Command for toio {}: {} {:?}",
                            toio_num,
                            name,
                            vals
                        );
                    }
                    CommandResult::NoMatch {
                        toio_num,
                        name,
                        vals,
                    } => {
                        debugPrint!(
                            "Command not found for toio {}: {} {:?}",
                            toio_num,
                            name,
                            vals
                        );
                    }
                    CommandResult::Empty { name } => {
                        debugPrint!("Received Empty Command: {}", name);
                    }
                    CommandResult::None => {}
                }
            }
        }
    });

    // listen for toio connections and disconnections
    let toio_lock_clone = toio_lock.clone();
    tokio::spawn(async move {
        while let Some(peripheral_update) = toios.next().await {
            match peripheral_update {
                Left(toio_peripheral) => {
                    // clone updates and toio_lock
                    let mut updates = toio_peripheral.updates().await.unwrap();
                    let mut toio_lock_write = toio_lock_clone.write().await;

                    // find if id was previously connected to
                    let existing_id = join_all(
                        toio_lock_write
                            .iter()
                            .map(|x| async { x.read().await.toio.peripheral_id.clone() }),
                    )
                    .await
                    .iter()
                    .position(|id| *id == toio_peripheral.peripheral_id);
                    let id = existing_id.unwrap_or(toio_lock_write.len());

                    // create instance of toio state and start listening for messages from toio
                    let state = Arc::new(RwLock::new(ToioState::new()));
                    let update_channel = tokio::spawn({
                        let state = state.clone();
                        let socket = socket.clone();
                        async move {
                            while let Some(update) = updates.next().await {
                                {
                                    let mut state_write = state.write().await;
                                    state_write.last_update = Some(SystemTime::now());

                                    // Update state based on update type
                                    match update {
                                        Update::Battery { level } => {
                                            state_write.battery = Some(level);
                                        }
                                        Update::Position {
                                            x_center, y_center, ..
                                        } => {
                                            state_write.curr_loc = Some(ToioLocation {
                                                x: x_center,
                                                y: y_center,
                                            });
                                        }
                                        _ => {}
                                    }
                                }

                                // forward message over OSC
                                send_packet(&socket, to_addr, id, update);
                            }
                        }
                    });

                    // connect or reconnect to a toio
                    if existing_id.is_some() {
                        let mut toio = toio_lock_write[id].write().await;
                        debugPrint!("Toio Reconnected: {}", toio.id);
                        toio.reconnect(toio_peripheral, update_channel, state);
                    } else {
                        let toio = Toio::connect(toio_peripheral, update_channel, state);
                        debugPrint!("Toio Connected: {}", toio.id);
                        toio_lock_write.push(Arc::new(RwLock::new(toio)));
                    }
                }
                Right(peripheral_id) => {
                    // request permission to write to list of connected toios
                    let toio_lock_write = toio_lock_clone.write().await;

                    // Collect all peripheral_ids with their indices
                    let toio_id = join_all(
                        toio_lock_write
                            .iter()
                            .map(|x| async { x.read().await.toio.peripheral_id.clone() }),
                    )
                    .await
                    .iter()
                    .position(|id| *id == peripheral_id);

                    // Find the index of the matching peripheral_id
                    if let Some(idx) = toio_id {
                        let mut toio = toio_lock_write[idx].write().await;
                        toio.disconnect();
                        debugPrint!("Toio Disconnected: {}", toio.id);
                    };
                }
            }
        }
    });

    // start terminal UI process
    let mut terminal: ToioUI = None;
    if !args.debug {
        terminal = setup_terminal()?;
    }

    // update UI from all of the toios
    let toio_lock_clone = toio_lock.clone();
    let mut interval = time::interval(Duration::from_millis(50)); // 20 FPS
    loop {
        interval.tick().await;

        // read toio info and draw UI
        let connected_read = toio_lock_clone.read().await;
        let toio_info = join_all(connected_read.iter().map(create_toio_info)).await;
        if let Some(ref mut toio_ui) = terminal {
            toio_ui.draw(create_toio_ui(
                toio_info,
                args.axlab_id.clone(),
                host_addr.clone(),
                to_addr.to_string(),
            ))?;
        }

        // exit terminal if "Q" key is pressed
        if handle_events()? {
            exit_terminal()?;
            process::exit(0);
        }
    }
}
