use crate::bridge::{IDInfo, Message};
use crossbeam_channel::{Receiver, Sender};
use nalgebra::{Isometry2, Point2, Vector2};
use ncollide2d::pipeline::{CollisionGroups, CollisionObjectSlabHandle, GeometricQueryType};
use ncollide2d::query::Ray;
use ncollide2d::shape::{Cuboid, ShapeHandle};
use ncollide2d::world::CollisionWorld;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use std::net::UdpSocket;
use rosc::{OscMessage, OscPacket, OscType};
use rosc::encoder;
use crate::cubelist::return_toio_id;
use crate::cubelist::return_toio_mac;
use std::convert::TryInto;

pub const DOTS_PER_METER: f64 = 411.0 / 0.560;
pub const CUBE_SIZE: f64 = 0.0318 * DOTS_PER_METER; // 31.8mm
pub const HIT_LEN: f64 = 0.020 * DOTS_PER_METER; // 20mm

// #01
// const MAT_MIN: [f32; 2] = [98.0, 142.0];
// const MAT_MAX: [f32; 2] = [402.0, 358.0];
// #05-12
// const MAT_MIN: [f32; 2] = [340.0, 35.0];
// const MAT_MAX: [f32; 2] = [949.0, 898.0];
// #01-12
const MAT_MIN: [f32; 2] = [34.0, 35.0];
const MAT_MAX: [f32; 2] = [949.0, 898.0];
const MAT_SIZE: [f32; 2] = [MAT_MAX[0] - MAT_MIN[0], MAT_MAX[1] - MAT_MIN[1]];
const MAT_CENTER: [f32; 2] = [(MAT_MIN[0] + MAT_MAX[0]) / 2.0, (MAT_MIN[1] + MAT_MAX[1]) / 2.0];
const MIN_RADIUS: f32 = 50.0;
const MAX_RADIUS: f32 = MAT_SIZE[0] / 2.0;

#[derive(Debug)]
pub struct Cube {
    position: Vector2<f32>,
    rotation: f32,
    address: String,
    bridge: String,
    to_bridge: Sender<Message>,
    battery: u8,
    going_around: bool,
    // last_message_time: Instant,
    last_move_time: Instant,
    handle: CollisionObjectSlabHandle,
    hit: bool,
}

impl Cube {
    pub fn set_lamp(&self, red: u8, green: u8, blue: u8) {
        self.to_bridge
            .send(Message::SetLamp(self.address.clone(), self.bridge.clone(), red, green, blue))
            .unwrap();
    }

    pub fn set_battery(&mut self, value: u8, force: bool) {
        if value == self.battery && !force {
            return;
        }
        self.battery = value;
        let (r, g, b) = match self.battery {
            0..=10 => (255, 0, 0),
            11..=20 => (255, 0, 0),
            21..=50 => (254, 176, 25),
            _ => (0, 255, 0),
        };
        self.to_bridge
            .send(Message::SetLamp(self.address.clone(), self.bridge.clone(), r, g, b))
            .unwrap();
    }

    pub fn set_direction(&mut self, angle: u16) {
        self.going_around = false;
        self.to_bridge
            .send(Message::SetDirection(self.address.clone(), self.bridge.clone(), angle))
            .unwrap();
    }

    pub fn look_center(&mut self) {
        let x = MAT_CENTER[0] - self.position.x;
        let y = MAT_CENTER[1] - self.position.y;
        let a = y.atan2(x).to_degrees();
        self.set_direction((a + 270.0) as u16);
    }

    pub fn stop_motor(&self) {
        self.to_bridge
            .send(Message::StopMotor(self.address.clone(), self.bridge.clone()))
            .unwrap();
    }

    pub fn start_go_around(&mut self) {
        self.going_around = true;
        self.last_move_time = Instant::now();
    }

    pub fn stop_go_around(&mut self) {
        self.going_around = false;
        self.stop_motor();
    }

    fn send_next_move(&mut self) {
        const SPACING: f32 = 33.0;
        let p = Vector2::new(MAT_CENTER[0] - self.position.x, MAT_CENTER[1] - self.position.y);
        let r = nalgebra::clamp(p.norm(), MIN_RADIUS, MAX_RADIUS);
        let radius = ((r - MIN_RADIUS) / SPACING).round() * SPACING + MIN_RADIUS;
        let start_angle = p.y.atan2(p.x);
        const DISTANCE: f32 = 50.0; // TODO: calculate distance from speed
        let c = 2.0 * radius * std::f32::consts::PI;
        let end_angle = start_angle + DISTANCE / c * std::f32::consts::TAU;
        let target_x = -end_angle.cos() * radius + MAT_CENTER[0];
        let target_y = -end_angle.sin() * radius + MAT_CENTER[1];
        // trace!(
        //     "radius={:?}, C={}, start_angle={:?}, end_angle={:?}, x={:?}, y={:?}",
        //     radius,
        //     c,
        //     start_angle,
        //     end_angle,
        //     target_x,
        //     target_y,
        // );
        /*self.to_bridge
            .try_send(Message::MoveToTarget(
                self.address.clone(),
                self.bridge.clone(),
                target_x as u16,
                target_y as u16,
            ))
            .unwrap();*/
    }

    pub fn move_to_pos(&mut self, target_x: u16, target_y: u16, time: u16, speed: u16){
        self.to_bridge
            .try_send(Message::MoveToTarget(
                self.address.clone(),
                self.bridge.clone(),
                target_x as u16,
                target_y as u16,
                time as u16, 
                speed as u16,
            ))
            .unwrap();
    }

    pub fn motor_control(&mut self, left: i32, right: i32, duration: u8){
        self.to_bridge
            .try_send(Message::MotorControl(
                self.address.clone(),
                self.bridge.clone(),
                left as i32,
                right as i32,
                duration as u8,
            ))
            .unwrap();
    }

    pub fn tick(&mut self) {
        if self.going_around && !self.hit {
            let now = Instant::now();
            if (now - self.last_move_time).as_millis() >= 500 {
                self.send_next_move();
                self.last_move_time = now;
            }
        }
    }
}

pub struct CubeManager {
    cubes: HashMap<String, Cube>,
    to_bridge: Sender<Message>,
    from_bridge: Receiver<Message>,
    to_ui: Sender<Message>,
    world: CollisionWorld<f32, ()>,
    collision_group: CollisionGroups,
    going_around: bool,
    socket: Result<UdpSocket, std::io::Error>
}

pub struct CubeReceiver {
    sender: Sender<Message>,
    socket: Result<UdpSocket, std::io::Error>,
}

impl CubeReceiver {
    pub fn new(sender: Sender<Message>) -> CubeReceiver {
        CubeReceiver{
            sender,
            socket: UdpSocket::bind("127.0.0.1:3334"),
        }
    }
    pub fn start(&mut self){
        let mut recbuf = [0; 1024];
        loop{
            let (number_of_bytes, _arc_address) = self.socket.as_ref().unwrap().recv_from(&mut recbuf).unwrap();
            let packet = rosc::decoder::decode(&recbuf[..number_of_bytes]).unwrap();
            match packet{
                OscPacket::Message(msg) => {
                    match msg.addr.as_ref(){
                        "/motorbasic" => {
                            let mut marg = [0; 5];
                            for k in 0..5 {
                                if let OscType::Int(i) = message.args[k] {
                                    marg[k] = i;
                                }
                            }
                        }
                        "/motortarget" => {
                            let mut marg = [0; 6];
                            for k in 0..6{
                                if let OscType::Int(i) = msg.args[k]{
                                    marg[k] = i as u16;
                                }
                            }
                            self.sender.try_send(Message::MoveToPos(return_toio_mac(marg[0] as usize), marg[2], marg[3], marg[4], marg[5])).unwrap();
                        }
                        "/motor" => {
                            let mut marg = [0; 4];
                            for k in 0..4{
                                if let OscType::Int(i) = msg.args[k]{
                                    marg[k] = i as i32;
                                }
                            }
                            self.sender.try_send(Message::MotorControl(return_toio_mac(marg[0] as usize), "a".to_string(), marg[1], marg[2], marg[3] as u8)).unwrap();
                        }
                        _ => {}
                    }
                    //println!("{:?}", msg);
                }
                OscPacket::Bundle(bundle) => {
                    println!("OSC Bundle: {:?}", bundle);
                }
            }
        }
    }
}
impl CubeManager {
    pub fn new(to_bridge: Sender<Message>, from_bridge: Receiver<Message>, to_ui: Sender<Message>) -> CubeManager {
        CubeManager {
            cubes: HashMap::new(),
            to_bridge,
            from_bridge,
            to_ui,
            world: CollisionWorld::new(0.01),
            collision_group: CollisionGroups::new(),
            going_around: false,
            socket: UdpSocket::bind("127.0.0.1:3335"),
        }
    }
    pub fn start(&mut self) {
        loop {
            let start = Instant::now();
            while let Ok(message) = self.from_bridge.try_recv() { //Is this receiving both from the bridge and from the UI?
                self.process_message(&message);
            }
            const FPS: f64 = 100.0;
            const FRAME_TIME: f64 = 1.0 / FPS;
            
            for cube in self.cubes.values_mut() {
                cube.tick();
            }
            self.world.update();

            if self.going_around {
                //let now = Instant::now();
                for cube in self.cubes.values_mut() {
                    // if (now - cube.last_message_time).as_millis() >= 1000 {
                    //     if let Some(object) = self.world.get_mut(cube.handle) {
                    //         object.set_position(Isometry2::new(Vector2::new(0.0, 0.0), 0.0));
                    //     }
                    // }
                    if let Some(obj) = self.world.get_mut(cube.handle) {
                        let a = (CUBE_SIZE / 2.0) as f32;
                        let ray = Ray::new(Point2::new(0.0, a + 1.0), Vector2::new(0.0, 1.0)).transform_by(obj.position());
                        let hit = self
                            .world
                            .first_interference_with_ray(&ray, HIT_LEN as f32, &self.collision_group)
                            .is_some();
                        if hit != cube.hit {
                            cube.hit = hit;
                            if hit {
                                cube.stop_motor();
                            }
                            self.to_ui.try_send(Message::HitStateChanged(cube.address.clone(), hit)).unwrap();
                        }
                    }
                }
            } else {
                for cube in self.cubes.values_mut() {
                    cube.hit = false;
                }
            }
            let dt = (Instant::now() - start).as_secs_f64();
            if dt < FRAME_TIME {
                // trace!("FPS={}, FRAME_TIME={}, dt={}, sleep={}", FPS, FRAME_TIME, dt, FRAME_TIME - dt);
                thread::sleep(Duration::from_secs_f64(FRAME_TIME - dt));
            }
        }
    }

    fn process_message(&mut self, message: &Message) {
        match message {
            Message::Connected(bridge_address, cube_address) => {
                self.add_new(bridge_address.clone(), cube_address.clone());
                self.to_ui.try_send(message.clone()).unwrap();
            }

            Message::Disconnected(_bridge_address, cube_address) => {
                if let Some(cube) = self.cubes.remove(cube_address) {
                    self.world.remove(&[cube.handle]);
                }
                self.to_ui.try_send(message.clone()).unwrap();
            }

            Message::BatteryInfo(cube_address, value) => {
                self.cubes.entry(cube_address.clone()).and_modify(|cube| cube.set_battery(*value, false));
            }

            Message::IDInfo(cube_address, id_info) => {
                match id_info {
                    IDInfo::PositionID(x, y, a) => match self.cubes.entry(cube_address.to_string()) {
                        Occupied(mut entry) => {
                            //println!("{}", return_toio_id(cube_address));
                            let mut cube = entry.get_mut();
                            cube.position = Vector2::new(*x as f32, *y as f32);
                            cube.rotation = *a as f32;
                            // cube.last_message_time = Instant::now();
                            if let Some(object) = self.world.get_mut(cube.handle) {
                                object.set_position(Isometry2::new(cube.position, (cube.rotation + 270.0).to_radians()));
                            }
                            let msg =
                                encoder::encode(&OscPacket::Message(OscMessage {
                                    addr: "/position".to_string(),
                                    args: vec![
                                        OscType::Int(0 as i32),
                                        OscType::Int(return_toio_id(cube_address)),
                                        OscType::Int(*x as i32),
                                        OscType::Int(*y as i32),
                                        OscType::Int(*a as i32),
                                    ],
                                }))
                                .unwrap();
                            self.socket.as_ref().unwrap().send_to(&msg, "127.0.0.1:3333").unwrap();
                        }
                        Vacant(_) => (),
                    },
                    IDInfo::PositionIDMissed => match self.cubes.entry(cube_address.to_string()) {
                        Occupied(mut entry) => {
                            let mut cube = entry.get_mut();
                            cube.position = Vector2::new(0.0, 0.0);
                            if let Some(object) = self.world.get_mut(cube.handle) {
                                object.set_position(Isometry2::new(cube.position, 0.0));
                            }
                        }
                        Vacant(_) => (),
                    },
                    _ => (),
                }
                self.to_ui.try_send(message.clone()).unwrap();
            }

            Message::ShowBatteryInfoAll => {
                for cube in self.cubes.values_mut() {
                    cube.set_battery(cube.battery, true);
                }
            }
            Message::SetLampAll(r, g, b) => {
                for cube in self.cubes.values() {
                    cube.set_lamp(*r, *g, *b);
                }
            }

            Message::SetDirectionAll(angle) => {
                self.going_around = false;
                for cube in self.cubes.values_mut() {
                    cube.set_direction(*angle);
                }
            }

            Message::MoveToPos(cube_address, target_x, target_y, time, speed) => {
                //println!("{:?}", self.cubes);
                if self.cubes.contains_key(&cube_address.to_string()){
                    self.cubes.get_mut(&cube_address.to_string()).unwrap().move_to_pos(*target_x, *target_y, *time, *speed);
                }
                else{
                    println!("Cube not connected!");
                }
            }

            Message::MotorControl(cube_address, _bridge, left, right, duration) => {
                self.cubes.get_mut(&cube_address.to_string()).unwrap().motor_control(*left, *right, *duration);
            }

            Message::LookCenterAll => {
                self.going_around = false;
                for cube in self.cubes.values_mut() {
                    cube.look_center();
                }
            }

            Message::StartGoAround => {
                println!("start go around");
                self.going_around = true;
                for cube in self.cubes.values_mut() {
                    cube.start_go_around();
                }
            }

            Message::StopAll => {
                println!("stop go around");
                self.going_around = false;
                for cube in self.cubes.values_mut() {
                    cube.stop_go_around();
                }
            }

            e @ _ => {
                error!("Unhandled message: {:?}", e);
            }
        }
    }

    pub fn add_new(&mut self, bridge_address: String, cube_address: String) {
        trace!("add_new: bridge_address={:?}, cube_address={:?}", bridge_address, cube_address);
        match self.cubes.entry(cube_address.clone()) {
            Occupied(mut entry) => {
                let cube = entry.get_mut();
                cube.bridge = bridge_address;
            }
            Vacant(entry) => {
                let size = (CUBE_SIZE / 2.0) as f32;
                let (handle, _) = self.world.add(
                    Isometry2::new(Vector2::new(0.0, 0.0), 0.0),
                    ShapeHandle::new(Cuboid::new(Vector2::new(size, size))),
                    self.collision_group,
                    GeometricQueryType::Contacts(0.0, 0.0),
                    (),
                );
                entry.insert(Cube {
                    position: Vector2::new(0.0, 0.0),
                    rotation: 0.0,
                    address: cube_address,
                    bridge: bridge_address,
                    to_bridge: self.to_bridge.clone(),
                    battery: 0,
                    going_around: self.going_around,
                    // last_message_time: Instant::now(),
                    last_move_time: Instant::now(),
                    handle,
                    hit: false,
                });
            }
        }
    }
}
