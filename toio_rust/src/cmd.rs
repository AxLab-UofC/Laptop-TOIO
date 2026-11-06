use crate::constants::*;
use btleplug::api::{CharPropFlags, WriteType};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TargetCommand {
    pub x_target: u16,
    pub y_target: u16,
    pub theta_target: u16,
}

#[derive(Clone, Debug)]
pub struct LedCommand {
    pub duration: u8,
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

#[derive(Clone, Debug)]
pub struct MidiCommand {
    pub duration: u8,
    pub note: u8,
    pub volume: u8,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Command {
    // Request Commands
    MotionRequest,
    MagneticRequest,
    PostureRequest {
        format: u8,
    },

    // Motor Commands
    MotorControl {
        left_direction: u8,
        left_speed: u8,
        right_direction: u8,
        right_speed: u8,
    },
    MotorDuration {
        left_direction: u8,
        left_speed: u8,
        right_direction: u8,
        right_speed: u8,
        duration: u8,
    },
    MotorTarget {
        control: u8,
        timeout: u8,
        move_type: u8,
        max_speed: u8,
        speed_change: u8,
        x_target: u16,
        y_target: u16,
        theta_target: u16,
    },
    MultiTarget {
        control: u8,
        timeout: u8,
        move_type: u8,
        max_speed: u8,
        speed_change: u8,
        op_add: u8,
        targets: Vec<TargetCommand>,
    },
    MotorAcceleration {
        velocity: u8,
        acceleration: u8,
        rotational_velocity: u16,
        rotational_direction: u8,
        direction: u8,
        priority: u8,
        duration: u8,
    },

    // Light Commands
    LedOff,
    Led {
        duration: u8,
        red: u8,
        green: u8,
        blue: u8,
    },
    MultiLed {
        repetitions: u8,
        lights: Vec<LedCommand>,
    },

    // Sound Commands
    SoundOff,
    Sound {
        sound_effect: u8,
        volume: u8,
    },
    Midi {
        repetitions: u8,
        notes: Vec<MidiCommand>,
    },
}

pub struct CommandData {
    pub uuid: Uuid,
    pub cmd: Vec<u8>,
    pub response_flag: CharPropFlags,
    pub response_type: WriteType,
}

fn parse_target_command(vals: Vec<TargetCommand>) -> Vec<u8> {
    let mut cmd = vec![];

    for target in vals.iter() {
        cmd.push((target.x_target & 0x00FF) as u8);
        cmd.push(((target.x_target & 0xFF00) >> 8) as u8);
        cmd.push((target.y_target & 0x00FF) as u8);
        cmd.push(((target.y_target & 0xFF00) >> 8) as u8);
        cmd.push((target.theta_target & 0x00FF) as u8);
        cmd.push(((target.theta_target & 0xFF00) >> 8) as u8);
    }

    return cmd;
}

fn parse_led_command(repetitions: u8, vals: Vec<LedCommand>) -> Vec<u8> {
    let mut cmd = vec![0x04, repetitions, vals.len() as u8];

    for led in vals.iter() {
        cmd.push(led.duration);
        cmd.push(0x01);
        cmd.push(0x01);
        cmd.push(led.red);
        cmd.push(led.green);
        cmd.push(led.blue);
    }

    return cmd;
}

fn parse_midi_command(repetitions: u8, vals: Vec<MidiCommand>) -> Vec<u8> {
    let mut cmd = vec![0x03, repetitions, vals.len() as u8];

    for note in vals.iter() {
        cmd.push(note.duration);
        cmd.push(note.note);
        cmd.push(note.volume);
    }

    return cmd;
}

fn build_command_vec(command: Command) -> Vec<u8> {
    match command {
        Command::MotionRequest => vec![0x81],
        Command::MagneticRequest => vec![0x82],
        Command::PostureRequest { format } => vec![0x83, format],
        Command::MotorControl {
            left_direction,
            left_speed,
            right_direction,
            right_speed,
        } => vec![
            0x01,
            0x01,
            left_direction,
            left_speed,
            0x02,
            right_direction,
            right_speed,
        ],
        Command::MotorDuration {
            left_direction,
            left_speed,
            right_direction,
            right_speed,
            duration,
        } => vec![
            0x02,
            0x01,
            left_direction,
            left_speed,
            0x02,
            right_direction,
            right_speed,
            duration,
        ],
        Command::MotorTarget {
            control,
            timeout,
            move_type,
            max_speed,
            speed_change,
            x_target,
            y_target,
            theta_target,
        } => vec![
            0x03,
            control,
            timeout,
            move_type,
            max_speed,
            speed_change,
            0x00,
            (x_target & 0x00FF) as u8,
            ((x_target & 0xFF00) >> 8) as u8,
            (y_target & 0x00FF) as u8,
            ((y_target & 0xFF00) >> 8) as u8,
            (theta_target & 0x00FF) as u8,
            ((theta_target & 0xFF00) >> 8) as u8,
        ],
        Command::MultiTarget {
            control,
            timeout,
            move_type,
            max_speed,
            speed_change,
            op_add,
            targets,
        } => {
            let mut cmd = vec![
                0x04,
                control,
                timeout,
                move_type,
                max_speed,
                speed_change,
                0x00,
                op_add,
            ];
            cmd.append(&mut parse_target_command(targets));
            cmd
        }
        Command::MotorAcceleration {
            velocity,
            acceleration,
            rotational_velocity,
            rotational_direction,
            direction,
            priority,
            duration,
        } => vec![
            0x05,
            velocity,
            acceleration,
            (rotational_velocity & 0x00FF) as u8,
            ((rotational_velocity & 0xFF00) >> 8) as u8,
            rotational_direction,
            direction,
            priority,
            duration,
        ],
        Command::LedOff => vec![0x01],
        Command::Led {
            duration,
            red,
            green,
            blue,
        } => vec![0x03, duration, 0x01, 0x01, red, green, blue],
        Command::MultiLed {
            repetitions,
            lights,
        } => parse_led_command(repetitions, lights),
        Command::SoundOff => vec![0x01],
        Command::Sound {
            sound_effect,
            volume,
        } => vec![0x02, sound_effect, volume],
        Command::Midi { repetitions, notes } => parse_midi_command(repetitions, notes),
    }
}

pub fn build_command_data(command: Command) -> CommandData {
    let uuid = match command {
        Command::MotionRequest | Command::MagneticRequest | Command::PostureRequest { .. } => {
            MOTION
        }
        Command::MotorControl { .. }
        | Command::MotorDuration { .. }
        | Command::MotorTarget { .. }
        | Command::MultiTarget { .. }
        | Command::MotorAcceleration { .. } => MOTOR,
        Command::LedOff | Command::Led { .. } | Command::MultiLed { .. } => LIGHT,
        Command::SoundOff | Command::Sound { .. } | Command::Midi { .. } => SOUND,
    };

    let cmd = build_command_vec(command);

    let (response_flag, response_type) = match uuid {
        LIGHT | SOUND | MOTION => (CharPropFlags::WRITE, WriteType::WithResponse),
        _ => (
            CharPropFlags::WRITE_WITHOUT_RESPONSE,
            WriteType::WithoutResponse,
        ),
    };

    return CommandData {
        uuid,
        cmd,
        response_flag,
        response_type,
    };
}
