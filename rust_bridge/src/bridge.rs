use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crossbeam_channel::{Receiver, Sender};
use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::{thread, time};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BridgeMode {
    Bridge,
    Scanner,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum IDInfo {
    PositionID(u16, u16, u16),
    StandardID(u32, u16),
    PositionIDMissed,
    StandardIDMissed,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewCubeFound(String),
    Available(String, usize, String),
    Connected(String, String),
    Disconnected(String, String),
    IDInfo(String, IDInfo),
    BatteryInfo(String, u8),
    MoveToPos(String, u16, u16, u16, u16),
    MotorControl(String, String, i32, i32, u8),
    ShowBatteryInfoAll,
    SetLampAll(u8, u8, u8),
    SetLamp(String, String, u8, u8, u8),
    StopMotor(String, String),
    SetDirectionAll(u16),
    SetDirection(String, String, u16),
    LookCenterAll,
    StartGoAround,
    StopAll,
    MoveToTarget(String, String, u16, u16, u16, u16),
    HitStateChanged(String, bool),
    Unknown,
}

#[derive(Debug)]
pub struct Bridge {
    address: String,
    mode: BridgeMode,
    to_manager: Sender<Message>,
    from_manager: Receiver<Message>,
    stream: TcpStream,
}

impl Bridge {
    pub fn new(to_manager: Sender<Message>, from_manager: Receiver<Message>, stream: TcpStream) -> Bridge {
        let address = match stream.peer_addr().unwrap() {
            SocketAddr::V4(addr) => addr.ip().to_string(),
            SocketAddr::V6(addr) => addr.ip().to_string(),
        };
        Bridge {
            address,
            mode: BridgeMode::Unknown,
            to_manager,
            from_manager,
            stream,
        }
    }

    pub fn start(&mut self) {
        let mut data = [0 as u8; 4096];
        self.stream.set_nonblocking(true).unwrap();
        while match self.stream.read(&mut data) {
            Ok(size) => {
                if let Err(err) = self.process_message(&data[0..size]) {
                    error!("An error has occurred while processing received data: {}", err);
                };
                true
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if let Some(message) = self.from_manager.try_recv().ok() {
                    self.send_message(message);
                } else {
                    thread::sleep(time::Duration::from_millis(10));
                }
                true
            }
            Err(_) => {
                error!("An error occurred, terminating connection with {}", self.address);
                self.stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }

    fn process_message(&mut self, data: &[u8]) -> Result<(), String> {
        let mut reader = BufferReader::new(data);
        while reader.left() > 0 {
            let (address, command, payload) = {
                let topic = reader.read_string()?;
                let (address, command) = match topic.find('/') {
                    Some(_) => {
                        let v: Vec<&str> = topic.split('/').collect();
                        (v[0].to_string(), v[1].to_string())
                    }
                    _ => ("".to_string(), topic),
                };
                let payload_len = reader.read_u8()? as usize;
                let payload = reader.read_array(payload_len)?;
                trace!("buffer left = {}", data.len() - reader.p);
                (address, command, payload)
            };
            trace!("address={:?}, command={:?}, payload={:?}", address, command, payload);
            let message = match command.as_str() {
                "newcube" => {
                    self.mode = BridgeMode::Scanner;
                    let address = String::from_utf8(payload).unwrap();
                    info!("Found Cube {}", address);
                    Message::NewCubeFound(address)
                }

                "available" => {
                    if self.mode == BridgeMode::Unknown {
                        info!("{} is bridge!", self.address);
                    }
                    self.mode = BridgeMode::Bridge;
                    let slots = payload[0] as usize;
                    info!("Available slots of {} is {}", self.address, slots);
                    Message::Available(self.address.clone(), slots, address)
                }

                "connected" => {
                    info!("Cube {} connected", address);
                    Message::Connected(self.address.clone(), address)
                }

                "disconnected" => {
                    info!("Cube {} disconnected", address);
                    Message::Disconnected(self.address.clone(), address)
                }

                "position" => match payload[0] {
                    1 => {
                        let mut p = &payload[1..];
                        let x = p.read_u16::<LittleEndian>().unwrap();
                        let y = p.read_u16::<LittleEndian>().unwrap();
                        let a = p.read_u16::<LittleEndian>().unwrap();
                        let id = IDInfo::PositionID(x, y, a);
                        Message::IDInfo(address, id)
                    }
                    2 => {
                        let mut p = &payload[1..];
                        let value = p.read_u32::<LittleEndian>().unwrap();
                        let a = p.read_u16::<LittleEndian>().unwrap();
                        let id = IDInfo::StandardID(value, a);
                        Message::IDInfo(address, id)
                    }
                    3 => Message::IDInfo(address, IDInfo::PositionIDMissed),
                    4 => Message::IDInfo(address, IDInfo::StandardIDMissed),
                    _ => Message::Unknown,
                },

                "battery" => Message::BatteryInfo(address, payload[0]),

                &_ => Message::Unknown,
            };
            // trace!("message={:?}", message);
            self.to_manager.send(message).unwrap();
        }
        Ok(())
    }

    fn send_message(&mut self, message: Message) {
        match message {
            Message::NewCubeFound(cube_address) => {
                let mut buffer = Vec::<u8>::with_capacity(64);
                let topic = "/newcube";
                buffer.push(topic.len() as u8);
                buffer.extend(topic.as_bytes());
                buffer.push(cube_address.len() as u8);
                buffer.extend(cube_address.as_bytes());
                trace!("NewCubeFound: len={:?}, buffer={:?}", buffer.len(), buffer);
                self.stream.write(&buffer).unwrap();
            }

            Message::SetLamp(cube_address, _bridge_address, r, g, b) => {
                let mut buffer = Vec::<u8>::with_capacity(64);
                let topic = cube_address + "/lamp";
                buffer.push(topic.len() as u8);
                buffer.extend(topic.as_bytes());
                let payload = vec![0x03, 0x00, 0x01, 0x01, r, g, b];
                buffer.push(payload.len() as u8);
                buffer.extend(payload);
                trace!("SetLamp: len={:?}, buffer={:?}", buffer.len(), buffer);
                self.stream.write(&buffer).unwrap();
            }

            Message::SetDirection(cube_address, _, angle) => {
                let mut buffer = Vec::<u8>::with_capacity(64);
                let topic = cube_address + "/motor";
                buffer.push(topic.len() as u8);
                buffer.extend(topic.as_bytes());
                let mut payload = vec![0x03, 0x00, 5, 0, 100, 0, 0x00];
                payload.write_u16::<LittleEndian>(0xffff).unwrap();
                payload.write_u16::<LittleEndian>(0xffff).unwrap();
                payload.write_u16::<LittleEndian>(angle).unwrap();
                buffer.push(payload.len() as u8);
                buffer.extend(payload);
                trace!("SetDirection: len={:?}, buffer={:?}", buffer.len(), buffer);
                self.stream.write(&buffer).unwrap();
            }

            Message::MoveToTarget(cube_address, _, x, y, time, speed) => {
                let mut buffer = Vec::<u8>::with_capacity(64);
                let topic = cube_address + "/motor";
                buffer.push(topic.len() as u8);
                buffer.extend(topic.as_bytes());
                let mut payload = vec![0x03, 0x00];
                payload.write_u16::<LittleEndian>(time).unwrap();
                payload.write_u16::<LittleEndian>(0x00).unwrap();
                payload.write_u16::<LittleEndian>(speed).unwrap();
                payload.write_u16::<LittleEndian>(0x00).unwrap();
                payload.write_u16::<LittleEndian>(0x00).unwrap();
                payload.write_u16::<LittleEndian>(x).unwrap();
                payload.write_u16::<LittleEndian>(y).unwrap();
                payload.write_u16::<LittleEndian>(0x05 << 13).unwrap();
                buffer.push(payload.len() as u8);
                buffer.extend(payload);
                trace!("SetDirection: len={:?}, buffer={:?}", buffer.len(), buffer);
                self.stream.write(&buffer).unwrap();
            }
            
            Message::MotorControl(cube_address, _, left, right, duration) => {
                trace!("Motor control");
                let mut buffer = Vec::<u8>::with_capacity(64);
                let topic = cube_address + "/motor";
                buffer.push(topic.len() as u8);
                buffer.extend(topic.as_bytes());
                let leftdirection = if left < 0 { 0x02 } else { 0x01 };
                let rightdirection = if right < 0 { 0x02 } else { 0x01 };
                let payload = vec![0x02, 0x01, leftdirection, left as u8, 0x02, rightdirection, right as u8, duration / 10];
                buffer.push(payload.len() as u8);
                buffer.extend(payload);
                self.stream.write(&buffer).unwrap();
            }

            Message::StopMotor(cube_address, _) => {
                let mut buffer = Vec::<u8>::with_capacity(64);
                let topic = cube_address + "/motor";
                buffer.push(topic.len() as u8);
                buffer.extend(topic.as_bytes());
                let payload = vec![0x01, 0x01, 0x01, 0, 0x02, 0x01, 0];
                buffer.push(payload.len() as u8);
                buffer.extend(payload);
                trace!("StopMotor: len={:?}, buffer={:?}", buffer.len(), buffer);
                self.stream.write(&buffer).unwrap();
            }

            _ => (),
        }
    }
}

struct BufferReader {
    data: Vec<u8>,
    p: usize,
}

impl BufferReader {
    fn new(data: &[u8]) -> BufferReader {
        BufferReader {
            data: data.to_vec(),
            p: 0usize,
        }
    }

    fn left(&self) -> usize {
        self.data.len() - self.p
    }

    fn available(&self, size: usize) -> Result<&Self, String> {
        if self.p + size <= self.data.len() {
            Ok(self)
        } else {
            Err(format!("No more data available. left={}, requested={}", self.left(), size))
        }
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        let value = self.available(1)?.data[self.p];
        self.p += 1;
        Ok(value)
    }

    fn read_array(&mut self, size: usize) -> Result<Vec<u8>, String> {
        let array = self.available(size)?.data[self.p..(self.p + size)].to_vec();
        self.p += size;
        Ok(array)
    }

    fn read_string(&mut self) -> Result<String, String> {
        let len = self.read_u8()? as usize;
        let data = self.read_array(len)?;
        String::from_utf8(data).map_err(|_| "std::string::FromUtf8Erro".to_string())
    }
}
