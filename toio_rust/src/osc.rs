use crate::cmd::*;
use crate::updates::Update;
use crossterm::event::{self, Event, KeyCode};
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::io::{self};
use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::net::UdpSocket;

pub enum CommandResult {
    Success {
        toio_num: usize,
        cmd: Command,
    },
    Error {
        toio_num: usize,
        name: String,
        vals: Vec<i32>,
    },
    NoMatch {
        toio_num: usize,
        name: String,
        vals: Vec<i32>,
    },
    None,
}

type CmdValidator = fn(&[i32]) -> bool; // validate that an OSC message is a valid command
type CmdBuilder = fn(&[i32]) -> Command; // build a command from a valid OSC message

// builds a vector array of OSC i32
macro_rules! OscInts {
    ($($val:expr),* $(,)?) => {
        vec![$(OscType::Int($val as i32)),*]
    };
}

fn exact<const N: usize>(vals: &[i32]) -> bool {
    vals.len() == N
}

fn chunked<const MIN: usize, const SIZE: usize>(vals: &[i32]) -> bool {
    vals.len() >= MIN && (vals.len() - MIN) % SIZE == 0
}

pub fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn handle_packet(packet: OscPacket) -> CommandResult {
    match packet {
        OscPacket::Message(msg) => {
            let vals: Vec<i32> = msg
                .args
                .iter()
                .filter_map(|val| match val {
                    OscType::Int(i) => Some(*i),
                    _ => None,
                })
                .collect();
            let toio_num = vals[0] as usize;

            // look up the command's expected arity and how to build it
            let (match_cmd, build_cmd): (CmdValidator, CmdBuilder) = match msg.addr.as_ref() {
                "/motorBasic" => (exact::<5>, |vals| Command::MotorControl {
                    left_direction: vals[1] as u8,
                    left_speed: vals[2].abs() as u8,
                    right_direction: vals[3] as u8,
                    right_speed: vals[4].abs() as u8,
                }),
                "/motorDuration" => (exact::<6>, |vals| Command::MotorDuration {
                    left_direction: vals[1] as u8,
                    left_speed: vals[2].abs() as u8,
                    right_direction: vals[3] as u8,
                    right_speed: vals[4].abs() as u8,
                    duration: vals[5] as u8,
                }),
                "/motorTarget" => (exact::<9>, |vals| Command::MotorTarget {
                    control: vals[1] as u8,
                    timeout: vals[2] as u8,
                    move_type: vals[3] as u8,
                    max_speed: vals[4] as u8,
                    speed_change: vals[5] as u8,
                    x_target: vals[6] as u16,
                    y_target: vals[7] as u16,
                    theta_target: vals[8] as u16,
                }),
                "/motorAcceleration" => (exact::<8>, |vals| Command::MotorAcceleration {
                    velocity: vals[1] as u8,
                    acceleration: vals[2] as u8,
                    rotational_velocity: vals[3] as u16,
                    rotational_direction: vals[4] as u8,
                    direction: vals[5] as u8,
                    priority: vals[6] as u8,
                    duration: vals[7] as u8,
                }),
                "/multiTarget" => (chunked::<9, 3>, |vals| Command::MultiTarget {
                    control: vals[1] as u8,
                    timeout: vals[2] as u8,
                    move_type: vals[3] as u8,
                    max_speed: vals[4] as u8,
                    speed_change: vals[5] as u8,
                    op_add: 1,
                    targets: vals[6..]
                        .chunks(3)
                        .map(|target| TargetCommand {
                            x_target: target[0] as u16,
                            y_target: target[1] as u16,
                            theta_target: target[2] as u16,
                        })
                        .collect(),
                }),
                "/led" => (chunked::<6, 4>, |vals| Command::MultiLed {
                    repetitions: vals[1] as u8,
                    lights: vals[2..]
                        .chunks(4)
                        .map(|light| LedCommand {
                            duration: light[0] as u8,
                            red: light[1] as u8,
                            green: light[2] as u8,
                            blue: light[3] as u8,
                        })
                        .collect(),
                }),
                "/sound" => (exact::<3>, |vals| Command::Sound {
                    sound_effect: vals[1] as u8,
                    volume: vals[2] as u8,
                }),
                "/midi" => (chunked::<5, 3>, |vals| Command::Midi {
                    repetitions: vals[1] as u8,
                    notes: vals[2..]
                        .chunks(3)
                        .map(|note| MidiCommand {
                            duration: note[0] as u8,
                            note: note[1] as u8,
                            volume: note[2] as u8,
                        })
                        .collect(),
                }),
                "/motion" => (exact::<1>, |_| Command::MotionRequest),
                "/magnetic" => (exact::<1>, |_| Command::MagneticRequest),
                "/postureEuler" => (exact::<1>, |_| Command::PostureRequest { format: 0x01 }),
                "/postureQuaternion" => (exact::<1>, |_| Command::PostureRequest { format: 0x02 }),
                _ => {
                    return CommandResult::NoMatch {
                        toio_num,
                        name: msg.addr,
                        vals,
                    }
                }
            };

            if match_cmd(&vals) {
                CommandResult::Success {
                    toio_num,
                    cmd: build_cmd(&vals),
                }
            } else {
                CommandResult::Error {
                    toio_num,
                    name: msg.addr,
                    vals,
                }
            }
        }
        _ => CommandResult::None,
    }
}

pub fn send_packet(
    socket: &UdpSocket,
    to_addr: SocketAddr,
    id: usize,
    update: Update,
    show_terminal: bool,
) {
    let vals: Option<(&str, Vec<OscType>)> = match update {
        Update::Position {
            x_center,
            y_center,
            theta,
            ..
        } => Some(("/position", OscInts![x_center, y_center, theta])),
        Update::Battery { level } => Some(("/battery", OscInts![level])),
        Update::Button { pressed } => {
            Some(("/button", OscInts![if pressed { 0x80 } else { 0x00 }]))
        }
        Update::Motion {
            horizontal,
            collision,
            double_tap,
            posture,
            shake,
        } => Some((
            "/motion",
            OscInts![horizontal, collision, double_tap, posture, shake],
        )),
        Update::MotorTargetResponse { control, response } => {
            Some(("/motorresponse", OscInts![control, response]))
        }
        Update::MultiTargetResponse { control, response } => {
            Some(("/motorresponse", OscInts![control, response]))
        }
        Update::Standard { standard, theta } => Some(("/standard", OscInts![standard, theta])),
        Update::PositionMissed => Some(("/positionMissed", vec![])),
        Update::StandardMissed => Some(("/standardMissed", vec![])),
        Update::MotorSpeed {
            left_speed,
            right_speed,
        } => Some(("/motorSpeed", OscInts![left_speed, right_speed])),
        Update::PostureEuler { roll, pitch, yaw } => {
            Some(("/postureEuler", OscInts![roll, pitch, yaw]))
        }
        Update::PostureQuaternion { w, x, y, z } => Some((
            "/postureQuaternion",
            vec![
                OscType::Float(w),
                OscType::Float(x),
                OscType::Float(y),
                OscType::Float(z),
            ],
        )),
        Update::Magnetic {
            state,
            strength,
            forcex,
            forcey,
            forcez,
        } => Some((
            "/magnetic",
            OscInts![state, strength, forcex, forcey, forcez],
        )),
        _ => None,
    };

    if let Some((addr, args)) = vals {
        if show_terminal {
            println!(
                "[{:?}] Update Received [{}]: {:?}",
                SystemTime::now(),
                addr,
                args
            )
        }

        let msg = encoder::encode(&OscPacket::Message(OscMessage {
            addr: addr.to_string(),
            args: std::iter::once(OscType::Int(id as i32))
                .chain(args)
                .collect(),
        }))
        .unwrap();

        let _ = socket.try_send_to(&msg, to_addr);
    }
}
