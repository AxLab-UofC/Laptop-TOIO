use std::io::{self};
use std::net::UdpSocket;
use std::time::SystemTime;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

use crossterm::event::{self, Event, KeyCode};

use crate::toio::*;

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

pub fn handle_packet(packet: OscPacket, show_terminal: bool) -> Option<(usize, Command)> {
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

            // extract command
            let cmd: Option<Command> = match msg.addr.as_ref() {
                "/motorbasic" => Some(Command::MotorControl {
                    left_direction: vals[1] as u8,
                    left_speed: vals[2] as u8,
                    right_direction: vals[3] as u8,
                    right_speed: vals[4] as u8,
                }),
                "/motorduration" => Some(Command::MotorDuration {
                    left_direction: vals[1] as u8,
                    left_speed: vals[2] as u8,
                    right_direction: vals[3] as u8,
                    right_speed: vals[4] as u8,
                    duration: vals[5] as u8,
                }),
                "/motortarget" => Some(Command::MotorTarget {
                    control: vals[1] as u8,
                    timeout: vals[2] as u8,
                    move_type: vals[3] as u8,
                    max_speed: vals[4] as u8,
                    speed_change: vals[5] as u8,
                    x_target: vals[6] as u16,
                    y_target: vals[7] as u16,
                    theta_target: vals[8] as u16,
                }),
                "/motoracceleration" => Some(Command::MotorAcceleration {
                    velocity: vals[1] as u8,
                    acceleration: vals[2] as u8,
                    rotational_velocity: vals[3] as u16,
                    rotational_direction: vals[4] as u8,
                    direction: vals[5] as u8,
                    priority: vals[6] as u8,
                    duration: vals[7] as u8,
                }),
                "/multitarget" => Some(Command::MultiTarget {
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
                "/led" => Some(Command::MultiLed {
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
                "/sound" => Some(Command::Sound {
                    sound_effect: vals[1] as u8,
                    volume: vals[2] as u8,
                }),
                "/midi" => Some(Command::Midi {
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
                _ => None,
            };

            if show_terminal {
                println!("[{:?}] Command Sent: {:?}", SystemTime::now(), cmd);
            }

            // Return pair of (toioID, cmd)
            cmd.map(|cmd| (vals[0] as usize, cmd))
        }
        _ => None,
    }
}

pub fn send_packet(
    socket: &UdpSocket,
    to_addr: &str,
    id: usize,
    update: Update,
    show_terminal: bool,
) {
    let vals: Option<(&str, Vec<i32>)> = match update {
        Update::Position {
            x_center,
            y_center,
            theta,
            ..
        } => Some((
            "/position",
            vec![x_center as i32, y_center as i32, theta as i32],
        )),
        Update::Battery { level } => Some(("/battery", vec![level as i32])),
        Update::Button { pressed } => Some(("/button", vec![if pressed { 0x80 } else { 0x00 }])),
        Update::Motion {
            horizontal,
            collision,
            double_tap,
            posture,
            shake,
        } => Some((
            "/motion",
            vec![
                horizontal as i32,
                collision as i32,
                double_tap as i32,
                posture as i32,
                shake as i32,
            ],
        )),
        Update::MotorTargetResponse { control, response } => {
            Some(("/motorresponse", vec![control as i32, response as i32]))
        }
        Update::MultiTargetResponse { control, response } => {
            Some(("/motorresponse", vec![control as i32, response as i32]))
        }
        Update::Standard { standard, theta } => {
            Some(("/standard", vec![standard as i32, theta as i32]))
        }
        Update::PositionMissed => Some(("/positionMissed", vec![])),
        Update::StandardMissed => Some(("/standardMissed", vec![])),
        Update::MotorSpeed {
            left_speed,
            right_speed,
        } => Some(("/motorSpeed", vec![left_speed as i32, right_speed as i32])),
        Update::PostureEuler { roll, pitch, yaw } => {
            Some(("/postureEuler", vec![roll as i32, pitch as i32, yaw as i32]))
        }
        Update::PostureQuaternion { w, x, y, z } => Some((
            "/PostureQuaternion",
            vec![w as i32, x as i32, y as i32, z as i32],
        )),
        Update::Magnetic {
            state,
            strength,
            forcex,
            forcey,
            forcez,
        } => Some((
            "/magnetic",
            vec![
                state as i32,
                strength as i32,
                forcex as i32,
                forcey as i32,
                forcez as i32,
            ],
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
            args: vec![id as i32]
                .iter()
                .chain(args.iter())
                .map(|x| OscType::Int(*x))
                .collect(),
        }))
        .unwrap();

        let _ = socket.send_to(&msg, to_addr);
    }
}
