#[macro_use]
extern crate log;
extern crate phf;

mod bridge;
mod bridge_manager;
mod cube;

// use rosc::{OscMessage, OscPacket, OscType}
use rosc::{OscPacket, OscType};
mod cubelist;
use cubelist::return_toio_mac;
// use bridge::{IDInfo, Message};
use bridge_manager::BridgeManager;
use cube::CubeReceiver;
use cube::CubeManager;
// use crossbeam_channel::{unbounded, Receiver, Sender};
use crossbeam_channel::{unbounded};
use std::thread;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::{net::SocketAddr};
// use tokio::{net::UdpSocket, sync::mpsc};
use tokio::{net::UdpSocket};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut cubelist = vec![];
    let reader = BufReader::new(File::open("cubes.csv")?);
    for line in reader.lines(){
        for i in line?.to_string().split(","){
            let id = i.trim().parse::<usize>().unwrap();
            let mac = return_toio_mac(id);
            println!("{}", mac);
            cubelist.push(mac);
        }
    }

    let (to_cubes_sender, to_cubes_receiver) = unbounded(); // -> Cubes
    let (to_bridges_sender, to_bridges_receiver) = unbounded(); // -> Bridge
    let (to_ui_sender, to_ui_receiver) = unbounded(); // -> UI
    let receiver = Some(to_ui_receiver);
    {
        let to_cubes_sender = to_cubes_sender.clone();
        thread::spawn(move || {
            BridgeManager::new(to_cubes_sender, to_bridges_receiver, cubelist).start();
        });
    }
    {
        let to_ui_sender = to_ui_sender.clone();
        thread::spawn(move || {
            CubeManager::new(to_bridges_sender, to_cubes_receiver, to_ui_sender).start();
        });
    }
    {
        let to_cubes_sender = to_cubes_sender.clone();
        thread::spawn(move || {
            CubeReceiver::new(to_cubes_sender).start();
        });
    }
    let sender = Some(to_cubes_sender);

    // let port_number = "3334";
    // let listening_address = format!("0.0.0.0:{}", port_number);
    // //OSC listening on port 3334
    // let sock = UdpSocket::bind((listening_address).parse::<SocketAddr>().unwrap()).await?;
    // println!("OSC listening on port {}", port_number);
    // let r = Arc::new(sock);
    // let s = r.clone();
    // //Receive OSC
    // let mut buf = [0; 1024];
    // //OSC listening on port 3334
    // let sock = UdpSocket::bind((listening_address).parse::<SocketAddr>().unwrap()).await?;
    // println!("OSC listening on port {}", port_number);
    // tokio::spawn(async move {
    //     while let Ok((len, addr)) = r.recv_from(&mut buf).await {
    //         println!("{:?} bytes received from {:?}", len, addr);
    //         let packet = rosc::decoder::decode(&buf[..len]).unwrap();
    //         match packet {
    //             OscPacket::Message(msg) => {
    //                 if msg.args.len() > 0 {
    //                     let mut marg = 0;
    //                     if let OscType::Int(i) = msg.args[0] {
    //                         marg = i;
    //                     }
    //                     println!("Got a message for {}", marg);
    //                     // find the address
    //                     let maybe_uuid = {
    //                         let storage_uuid = uuids2.lock().unwrap();
    //                         if storage_uuid.len() > marg as usize {
    //                             Some(storage_uuid[marg as usize].clone())
    //                         } else {
    //                             None
    //                         }
    //                     };
    //                     if let Some(uuid) = maybe_uuid {
    //                         //try to get the channel and not breaking everything
    //                         let sender = {
    //                             let sends = senders2.lock().unwrap();
    //                             sends.get(&uuid).map(|p| p.clone())
    //                             //we drop the lock here because we *clone*
    //                         };
    //                         if let Some(channel) = sender {
    //                             println!("Sending to...");
    //                             channel.send(msg).await.unwrap();
    //                         }
    //                     }
    //                 }
    //             }
    //             OscPacket::Bundle(bundle) => {
    //                 println!("OSC Bundle: {:?}", bundle);
    //             }
    //         }
    //     }
    // });

    Ok(())
}