mod osc;
mod toio;
mod ui;

use osc::*;
use toio::*;
use ui::*;

use std::error::Error;
use std::net::UdpSocket;
use std::process;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use clap::Parser;
use futures::future::join_all;
use futures::future::Either::{Left, Right};
use tokio::sync::RwLock;
use tokio::time;

#[derive(Parser)]
#[command(name = "toio")]
struct Args {
    /// Set receiving port
    #[arg(short, long)]
    port: Option<usize>,

    /// Set remote port
    #[arg(short, long)]
    remote: Option<usize>,

    /// Show terminal UI
    #[arg(short, long)]
    terminal: bool,

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
            if args.terminal {
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

            if args.terminal {
                println!("Running Unfiltered Search for Toios");
            }
            ToioScanner::new().await?
        }
    };

    let mut toios = scanner.search().await?;
    let connected: Arc<RwLock<Vec<Arc<RwLock<Toio>>>>> = Arc::new(RwLock::new(vec![]));

    // server and client address
    let host_addr = format!("0.0.0.0:{}", args.port.unwrap_or(3334));
    let to_addr = format!("0.0.0.0:{}", args.remote.unwrap_or(3333));
    if args.terminal {
        println!(
            "Listening on port {} and sending to port {}",
            host_addr, to_addr
        )
    }

    // open socket and create buffer
    let socket = Arc::new(UdpSocket::bind(&host_addr)?);
    let mut buf = [0u8; rosc::decoder::MTU];

    // whenever a message is received through OSC, forward to toio
    let sock = socket.clone();
    let connected_clone = connected.clone();
    tokio::spawn(async move {
        while let Ok(size) = sock.recv(&mut buf) {
            if let Ok((_, packet)) = rosc::decoder::decode_udp(&buf[..size]) {
                if let Some((toionum, cmd)) = handle_packet(packet, args.terminal) {
                    let connected_read = connected_clone.read().await;
                    if toionum < connected_read.len() {
                        let toio = connected_read[toionum].read().await;

                        // Update last command timestamp
                        toio.update_last_command();

                        // Forward command to toio
                        toio.toio.send_command(cmd).await;
                    }
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
                    if args.terminal {
                        println!("Toio Connected: {}", toio.id);
                    }

                    let state = toio.state.clone();

                    // request permission to write to list of connected toios
                    let mut connected_write = connected_clone.write().await;
                    let id = connected_write.len();

                    // start process to listen for messages from toio
                    let toio_channel = tokio::spawn({
                        let to_addr = to_addr.clone();
                        async move {
                            loop {
                                match updates.next().await {
                                    Some(update) => {
                                        // Update state based on update type
                                        {
                                            let mut state_write = state.write().await;
                                            if let Update::Battery { level } = update {
                                                state_write.battery = Some(level);
                                            }
                                            state_write.last_update = Some(SystemTime::now());
                                        }

                                        send_packet(&sock, &to_addr, id, update, args.terminal);
                                    }
                                    None => {
                                        break; // Stream ended, break out
                                    }
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
                        if args.terminal {
                            println!("Toio Disconnected: {}", toio.id);
                        }
                    };
                }
            }
        }
    });

    // start TUI process
    let mut terminal: ToioUI = None;
    if !args.terminal {
        terminal = setup_terminal()?;
    }

    // update UI from all of the toios
    let connected_clone = connected.clone();
    let mut interval = time::interval(Duration::from_millis(50)); // 20 FPS
    loop {
        interval.tick().await;

        let connected_read = connected_clone.read().await;

        // get info from all of the toios
        let toio_info = join_all(connected_read.iter().map(|toio_guard| async {
            let toio = toio_guard.read().await;
            let name = toio.name.clone();
            let id = toio.id.clone();

            // Read toio state
            let state = toio.state.read().await;
            let connected = toio.connected;

            // get battery level
            let battery_string = if let Some(level) = state.battery {
                format!("{}", level)
            } else {
                "N/A".to_string()
            };

            // get time of last update
            let last_update_string = if let Some(last) = state.last_update {
                if let Ok(time) = last.elapsed() {
                    format_elapsed_time(time)
                } else {
                    "N/A".to_string()
                }
            } else {
                "N/A".to_string()
            };

            // get time of last command
            let last_command_string = if let Some(last) = state.last_command {
                if let Ok(time) = last.elapsed() {
                    format_elapsed_time(time)
                } else {
                    "N/A".to_string()
                }
            } else {
                "N/A".to_string()
            };

            (
                name,
                id,
                battery_string,
                last_update_string,
                last_command_string,
                connected,
            )
        }))
        .await;

        // update UI
        if let Some(ref mut toio_ui) = terminal {
            toio_ui.draw(ui(toio_info, args.axlab_id.clone()))?;
        }

        // exit terminal if "Q" key is pressed
        if handle_events()? {
            exit_terminal()?;
            process::exit(0);
        }
    }
}

// Helper function to format elapsed time
fn format_elapsed_time(time: Duration) -> String {
    if time.as_millis() < 50 {
        "<50ms".to_string()
    } else if time.as_secs() < 1 {
        format!(">{}ms", time.as_millis() - (time.as_millis() % 100))
    } else {
        format!("{}s", time.as_secs())
    }
}
