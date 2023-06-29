use crate::bridge::{Bridge, Message};
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct BridgeManager {
    to_cubes: Sender<Message>,
    from_cubes: Receiver<Message>,
    cubelist: Vec<String>,
    //to_ui: Sender<Message>,
    senders_to_bridge: Arc<Mutex<HashMap<String, Sender<Message>>>>,
    bridges: Vec<(String, usize)>,
}

impl BridgeManager {
    pub fn new(to_cubes: Sender<Message>, from_cubes: Receiver<Message>, cubelist: Vec<String>/*, to_ui: Sender<Message>*/) -> BridgeManager {
        BridgeManager {
            to_cubes,
            from_cubes,
            cubelist,
            //to_ui,
            senders_to_bridge: Arc::new(Mutex::new(HashMap::new())),
            bridges: Vec::with_capacity(32),
        }
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind("0.0.0.0:11111").unwrap();
        info!("Server listening on port 11111");

        let (to_manager, from_bridge) = unbounded();
        {
            let senders_to_bridge = self.senders_to_bridge.clone();
            thread::spawn(move || {
                for stearm in listener.incoming() {
                    match stearm {
                        Ok(stream) => {
                            let to_manager = to_manager.clone();
                            let (to_bridge, from_manager) = unbounded();
                            let address = match stream.peer_addr().unwrap() {
                                SocketAddr::V4(addr) => addr.ip().to_string(),
                                SocketAddr::V6(addr) => addr.ip().to_string(),
                            };
                            info!("Bridge {} connected", address);
                            senders_to_bridge.lock().unwrap().insert(address, to_bridge);
                            thread::spawn(move || {
                                Bridge::new(to_manager, from_manager, stream).start();
                            });
                        }
                        Err(e) => {
                            error!("Error: {}", e);
                        }
                    }
                }
            });
        }

        loop {
            select! {
                recv(from_bridge) -> message => self.process_message(&message.unwrap()),
                recv(self.from_cubes) -> message => self.process_message(&message.unwrap()),
            }
        }
    }

    fn process_message(&mut self, message: &Message) {
        match message {
            Message::NewCubeFound(cube_address) => {
                //println!("Want to send to {}", cube_address);
                trace!("{:?}", self.bridges);
                if let Some((bridge_address, _slots)) = self.bridges.pop() {
                    if self.cubelist.contains(cube_address){
                        self.send_message(&bridge_address, message);
                    }
                }
            }

            Message::Available(ip_address, slots, _) => {
                if *slots > 0 {
                    match self.bridges.iter().position(|(addr, _)| addr == ip_address) {
                        Some(index) => self.bridges[index].1 = *slots,
                        None => self.bridges.push((ip_address.clone(), *slots)),
                    }
                    self.bridges.sort_by(|a, b| a.1.cmp(&b.1));
                    // debug!("{:?}", self.bridges);
                }
                //self.to_ui.try_send(message.clone()).unwrap();
            }

            m @ Message::Connected(_, _) | m @ Message::Disconnected(_, _) | m @ Message::IDInfo(_, _) | m @ Message::BatteryInfo(_, _) => {
                self.to_cubes.try_send(m.clone()).unwrap()
            }

            Message::SetLamp(_, bridge_address, _, _, _)
            | Message::SetDirection(_, bridge_address, _)
            | Message::MoveToTarget(_, bridge_address, _, _, _, _)
            | Message::StopMotor(_, bridge_address) 
            | Message::MotorControl(_, bridge_address, _, _, _) => {
                self.send_message(bridge_address, message);
            }

            _ => (),
        }
    }

    fn send_message(&mut self, bridge_address: &String, message: &Message) {
        if let Some(sender) = self.senders_to_bridge.lock().unwrap().get(bridge_address) {
            sender.try_send(message.clone()).unwrap();
        }
    }
}
