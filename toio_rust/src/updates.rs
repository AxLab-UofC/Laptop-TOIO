use crate::{constants::*, debugPrint};
use btleplug::api::ValueNotification;
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

/// An enum to list out all possible updates to receive from a toio
#[allow(dead_code)]
#[derive(Debug)]
pub enum Update {
    Position {
        x_center: u16,
        y_center: u16,
        theta: u16,
        x_sensor: u16,
        y_sensor: u16,
    },
    Standard {
        standard: u32,
        theta: u16,
    },
    PositionMissed,
    StandardMissed,
    MotorTargetResponse {
        control: u8,
        response: u8,
    },
    MultiTargetResponse {
        control: u8,
        response: u8,
    },
    MotorSpeed {
        left_speed: u8,
        right_speed: u8,
    },
    Motion {
        horizontal: u8,
        collision: u8,
        double_tap: u8,
        posture: u8,
        shake: u8,
    },
    PostureEuler {
        roll: i16,
        pitch: i16,
        yaw: i16,
    },
    PostureQuaternion {
        w: f32,
        x: f32,
        y: f32,
        z: f32,
    },
    PostureHighPrecisionEuler {
        roll: f32,
        pitch: f32,
        yaw: f32,
    },
    Magnetic {
        state: u8,
        strength: u8,
        forcex: i8,
        forcey: i8,
        forcez: i8,
    },
    Button {
        pressed: bool,
    },
    Battery {
        level: u8,
    },
}

pub struct Updates {
    receiver: Receiver<Update>,
}

impl Updates {
    pub fn new(receiver: Receiver<Update>) -> Updates {
        return Updates { receiver };
    }

    pub async fn next(&mut self) -> Option<Update> {
        return self.receiver.recv().await;
    }
}

pub fn parse_update(notification: ValueNotification) -> Option<Update> {
    let vals = notification.value;
    match notification.uuid {
        POSITION => match vals[0] {
            0x01 => Some(Update::Position {
                x_center: vals[1] as u16 | (vals[2] as u16) << 8,
                y_center: vals[3] as u16 | (vals[4] as u16) << 8,
                theta: vals[5] as u16 | (vals[6] as u16) << 8,
                x_sensor: vals[7] as u16 | (vals[8] as u16) << 8,
                y_sensor: vals[9] as u16 | (vals[10] as u16) << 8,
            }),
            0x02 => Some(Update::Standard {
                standard: vals[1] as u32
                    | (vals[2] as u32) << 8
                    | (vals[3] as u32) << 16
                    | (vals[4] as u32) << 24,
                theta: vals[5] as u16 | (vals[6] as u16) << 8,
            }),
            0x03 => Some(Update::PositionMissed),
            0x04 => Some(Update::StandardMissed),
            _ => {
                debugPrint!(
                    "Unknown {} Update: {:?}",
                    get_uuid_characteristic(notification.uuid),
                    vals
                );
                None
            }
        },
        MOTOR => match vals[0] {
            0x83 => Some(Update::MotorTargetResponse {
                control: vals[1],
                response: vals[2],
            }),
            0x84 => Some(Update::MultiTargetResponse {
                control: vals[1],
                response: vals[2],
            }),
            0xe0 => Some(Update::MotorSpeed {
                left_speed: vals[1],
                right_speed: vals[2],
            }),
            _ => {
                debugPrint!(
                    "Unknown {} Update: {:?}",
                    get_uuid_characteristic(notification.uuid),
                    vals
                );
                None
            }
        },
        MOTION => match vals[0] {
            0x01 => Some(Update::Motion {
                horizontal: vals[1],
                collision: vals[2],
                double_tap: vals[3],
                posture: vals[4],
                shake: vals[5],
            }),
            0x02 => Some(Update::Magnetic {
                state: *vals.get(1)?,
                strength: *vals.get(2)?,
                forcex: *vals.get(3)? as i8,
                forcey: *vals.get(4)? as i8,
                forcez: *vals.get(5)? as i8,
            }),
            0x03 => match vals.get(1)? {
                0x01 => Some(Update::PostureEuler {
                    roll: read_i16(&vals, 2)?,
                    pitch: read_i16(&vals, 4)?,
                    yaw: read_i16(&vals, 6)?,
                }),
                0x02 => Some(Update::PostureQuaternion {
                    w: read_f32(&vals, 2)?,
                    x: read_f32(&vals, 6)?,
                    y: read_f32(&vals, 10)?,
                    z: read_f32(&vals, 14)?,
                }),
                0x03 => Some(Update::PostureHighPrecisionEuler {
                    roll: read_f32(&vals, 2)?,
                    pitch: read_f32(&vals, 6)?,
                    yaw: read_f32(&vals, 10)?,
                }),
                _ => None,
            },
            _ => {
                debugPrint!(
                    "Unknown {} Update: {:?}",
                    get_uuid_characteristic(notification.uuid),
                    vals
                );
                None
            }
        },
        BATTERY => Some(Update::Battery { level: vals[0] }),
        BUTTON => Some(Update::Button {
            pressed: vals[1] == 0x80,
        }),
        _ => {
            debugPrint!(
                "Unknown {} Update: {:?}",
                get_uuid_characteristic(notification.uuid),
                vals
            );
            None
        }
    }
}

pub fn get_uuid_characteristic(uuid: Uuid) -> String {
    match uuid {
        SERVICE => "Service",
        POSITION => "Position",
        MOTOR => "Motor",
        LIGHT => "Light",
        SOUND => "Sound",
        MOTION => "Motion",
        BUTTON => "Button",
        BATTERY => "Battery",
        CONFIG => "Config",
        _ => "",
    }
    .to_owned()
}

fn read_i16(vals: &[u8], i: usize) -> Option<i16> {
    Some(i16::from_le_bytes(vals.get(i..i + 2)?.try_into().ok()?))
}

fn read_f32(vals: &[u8], i: usize) -> Option<f32> {
    Some(f32::from_le_bytes(vals.get(i..i + 4)?.try_into().ok()?))
}
