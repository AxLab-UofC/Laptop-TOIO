macro_rules! or_na {
    ($pat:pat = $expr:expr => $body:expr) => {
        if let $pat = $expr {
            $body
        } else {
            "N/A".to_string()
        }
    };
}

mod cmd;
mod constants;
mod osc;
mod terminal;
mod toio;
mod updates;

use clap::Parser;
use futures::future::join_all;
use futures::future::Either::{Left, Right};
use osc::*;
use std::error::Error;
use std::process;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use terminal::*;
use toio::*;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::RwLock;
use tokio::time;

use crate::updates::Update;

#[derive(Parser)]
#[command(name = "toio")]
struct Args {
    /// Set receiving port
    #[arg(short, long)]
    port: Option<usize>,

    /// Set remote port
    #[arg(short, long)]
    remote: Option<usize>,

    /// Show terminal stream instead of terminal UI
    #[arg(short, long)]
    terminal_stream: bool,

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

    // create scanner and array of toios
    let scanner = match args.axlab_id.clone() {
        Some(filter) => {
            if args.terminal_stream {
                if args.ordered {
                    println!("Running Ordered Search for Toios: {:?}", filter.clone());
                } else {
                    println!("Running Unordered Search for Toios: {:?}", filter.clone());
                }
            }
            ToioScanner::new_with_filter(args.ordered, filter.clone()).await?
        }
        None => {
            if !args.search {
                println!("You must provide IDs when not in search mode");
                process::exit(0);
            }

            if args.terminal_stream {
                println!("Running Unfiltered Search for Toios");
            }
            ToioScanner::new().await?
        }
    };

    let mut toios = scanner.search().await?;
    let connected: Arc<RwLock<Vec<Arc<RwLock<Toio>>>>> = Arc::new(RwLock::new(vec![]));

    // server and client address
    let host_addr = format!("127.0.0.1:{}", args.port.unwrap_or(3334));
    let to_addr = format!("127.0.0.1:{}", args.remote.unwrap_or(3333)).parse()?;
    if args.terminal_stream {
        println!(
            "Listening on port {} and sending to port {}",
            host_addr, to_addr
        )
    }

    // open socket and create buffer
    let socket = Arc::new(UdpSocket::bind(&host_addr).await?);
    let mut buf = [0u8; rosc::decoder::MTU];

    // whenever a message is received through OSC, forward to toio
    let sock = socket.clone();
    let connected_clone = connected.clone();
    tokio::spawn(async move {
        while let Ok((size, _)) = sock.recv_from(&mut buf).await {
            if let Ok((_, packet)) = rosc::decoder::decode_udp(&buf[..size]) {
                match handle_packet(packet) {
                    CommandResult::Success { toio_num, cmd } => {
                        if args.terminal_stream {
                            println!("[{:?}] Command Sent: {:?}", SystemTime::now(), cmd);
                        }

                        // find toio from connected
                        let toio = connected_clone.read().await.get(toio_num).cloned();
                        if let Some(toio) = toio {
                            let toio = toio.read().await;

                            // Update last command timestamp
                            toio.update_last_command();

                            // add command to toio command queue
                            if let Err(err) = toio.queue_command(cmd) {
                                if args.terminal_stream {
                                    let reason = match err {
                                        TrySendError::Full(_) => "Command Queue Full",
                                        TrySendError::Closed(_) => "Toio Disconnected",
                                    };
                                    eprintln!(
                                        "[{:?}] Command Dropped for toio {}: {}",
                                        SystemTime::now(),
                                        toio_num,
                                        reason
                                    );
                                }
                            }
                        }
                    }
                    CommandResult::Error {
                        toio_num,
                        name,
                        vals,
                    } => {
                        if args.terminal_stream {
                            println!(
                                "[{:?}] Error Parsing Command for toio {}: {} {:?}",
                                SystemTime::now(),
                                toio_num,
                                name,
                                vals
                            );
                        }
                    }
                    CommandResult::NoMatch {
                        toio_num,
                        name,
                        vals,
                    } => {
                        if args.terminal_stream {
                            println!(
                                "[{:?}] Command not found for toio {}: {} {:?}",
                                SystemTime::now(),
                                toio_num,
                                name,
                                vals
                            );
                        }
                    }
                    CommandResult::None => {}
                }
            }
        }
    });

    // whenever we connect to a toio, add it to the list
    let connected_clone = connected.clone();
    tokio::spawn(async move {
        while let Some(peripheral_update) = toios.next().await {
            match peripheral_update {
                Left(toio_peripheral) => {
                    // clone socket
                    let sock = socket.clone();

                    // listen for updates from toio
                    let mut updates = toio_peripheral.updates().await.unwrap();

                    // create instance of Toio to record toio info
                    let toio = Toio::new(toio_peripheral);
                    if args.terminal_stream {
                        println!("Toio Connected: {}", toio.id);
                    }

                    let state = toio.state.clone();

                    // request permission to write to list of connected toios
                    let mut connected_write = connected_clone.write().await;
                    let id = connected_write.len();

                    // start process to listen for messages from toio
                    let toio_channel = tokio::spawn({
                        async move {
                            loop {
                                match updates.next().await {
                                    Some(update) => {
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

                                        send_packet(
                                            &sock,
                                            to_addr,
                                            id,
                                            update,
                                            args.terminal_stream,
                                        );
                                    }
                                    None => break, // Stream ended, break out
                                }
                            }
                        }
                    });

                    let toio = Arc::new(RwLock::new(toio));
                    {
                        let mut toio_write = toio.write().await;
                        toio_write.add_channel(toio_channel);
                    }
                    connected_write.push(toio);
                }
                Right(peripheral_id) => {
                    // request permission to write to list of connected toios
                    let connected_write = connected_clone.write().await;

                    // Collect all peripheral_ids with their indices
                    let ids = join_all(connected_write.iter().map(|x| async {
                        let toio = x.read().await;
                        toio.toio.peripheral_id.clone()
                    }))
                    .await;

                    let toio_id = ids.iter().position(|id| *id == peripheral_id);

                    // Find the index of the matching peripheral_id
                    if let Some(idx) = toio_id {
                        let mut toio = connected_write[idx].write().await;
                        toio.disconnect();
                        if args.terminal_stream {
                            println!("Toio Disconnected: {}", toio.id);
                        }
                    };
                }
            }
        }
    });

    // start terminal UI process
    let mut terminal: ToioUI = None;
    if !args.terminal_stream {
        terminal = setup_terminal()?;
    }

    // update UI from all of the toios
    let connected_clone = connected.clone();
    let mut interval = time::interval(Duration::from_millis(50)); // 20 FPS
    loop {
        interval.tick().await;

        let connected_read = connected_clone.read().await;

        // get info from all of the toios
        let toio_info = join_all(connected_read.iter().map(create_toio_info)).await;

        // update UI
        if let Some(ref mut toio_ui) = terminal {
            toio_ui.draw(create_toio_ui(toio_info, args.axlab_id.clone()))?;
        }

        // exit terminal if "Q" key is pressed
        if handle_events()? {
            exit_terminal()?;
            process::exit(0);
        }
    }
}
