use std::error::Error;
use std::sync::Arc;
use std::time::SystemTime;
use std::vec;

use futures::future::Either;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use futures::stream::StreamExt;

use uuid::Uuid;

use btleplug::{
    api::{
        Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral, ScanFilter,
        ValueNotification, WriteType,
    },
    platform,
    platform::{Adapter, Manager},
};

pub const SERVICE: Uuid = Uuid::from_u128(0x10B20100_5B3B_4571_9508_CF3EFCD7BBAE);
pub const POSITION: Uuid = Uuid::from_u128(0x10B20101_5B3B_4571_9508_CF3EFCD7BBAE);
pub const MOTOR: Uuid = Uuid::from_u128(0x10B20102_5B3B_4571_9508_CF3EFCD7BBAE);
pub const LIGHT: Uuid = Uuid::from_u128(0x10B20103_5B3B_4571_9508_CF3EFCD7BBAE);
pub const SOUND: Uuid = Uuid::from_u128(0x10B20104_5B3B_4571_9508_CF3EFCD7BBAE);
pub const MOTION: Uuid = Uuid::from_u128(0x10B20106_5B3B_4571_9508_CF3EFCD7BBAE);
pub const BUTTON: Uuid = Uuid::from_u128(0x10B20107_5B3B_4571_9508_CF3EFCD7BBAE);
pub const BATTERY: Uuid = Uuid::from_u128(0x10B20108_5B3B_4571_9508_CF3EFCD7BBAE);
pub const CONFIG: Uuid = Uuid::from_u128(0x10B201FF_5B3B_4571_9508_CF3EFCD7BBAE);

const IDARR: [&str; 193] = [
    "Individual ID", //TOIO Num
    "0",             // #1
    "j1c",           // #2
    "r81",           // #3
    "26E",           // #4
    "76t",           // #5
    "broken",        // #6
    "k5k",           // #7
    "h41",           // #8
    "0",             // #9
    "0",             // #10
    "0",             // #11
    "Q3A",           // #12
    "03a",           // #13
    "0",             // #14
    "K0m",           // #15
    "0",             // #16
    "0",             // #17
    "p8B",           // #18
    "91B",           // #19
    "p75",           // #20
    "G1E",           // #21
    "k2L",           // #22
    "b5p",           // #23
    "J6C",           // #24
    "a1K",           // #25
    "b8T",           // #26
    "b6A",           // #27
    "01c",           // #28
    "0",             // #29
    "0",             // #30
    "E2N",           // #31
    "G7t",           // #32
    "L6T",           // #33
    "C0E",           // #34
    "t79",           // #35
    "J6k",           // #36
    "d6f",           // #37
    "0",             // #38
    "M75",           // #39
    "310",           // #40
    "M5p",           // #41
    "A4a",           // #42
    "M9J",           // #43
    "i01",           // #44
    "T5m",           // #45
    "j1G",           // #46
    "40G",           // #47
    "L6n",           // #48
    "a3F",           // #49
    "J8d",           // #50
    "227",           // #51
    "k4i",           // #52
    "J68",           // #53
    "90J",           // #54
    "k96",           // #55
    "0",             // #56
    "0",             // #57
    "0",             // #58
    "0",             // #59
    "0",             // #60
    "0",             // #61
    "0",             // #62
    "0",             // #63
    "0",             // #64
    "0",             // #65
    "0",             // #66
    "0",             // #67
    "0",             // #68
    "0",             // #69
    "0",             // #70
    "0",             // #71
    "0",             // #72
    "0",             // #73
    "0",             // #74
    "0",             // #75
    "0",             // #76
    "0",             // #77
    "0",             // #78
    "E7c",           // #79
    "P1B",           // #80
    "F2B",           // #81
    "L1H",           // #82
    "D5i",           // #83
    "m4Q",           // #84
    "m1k",           // #85
    "r52",           // #86
    "k89",           // #87
    "D2K",           // #88
    "65r",           // #89
    "f3K",           // #90
    "13c",           // #91
    "e1a",           // #92
    "0",             // #93
    "e6e",           // #94
    "07F",           // #95
    "m8k",           // #96
    "79H",           // #97
    "0",             // #98
    "i1M",           // #99
    "R3C",           // #100
    "D98",           // #101
    "m86",           // #102
    "a66",           // #103
    "0",             // #104
    "E8T",           // #105
    "J8n",           // #106
    "N0b",           // #107
    "586",           // #108
    "p50",           // #109
    "c9k",           // #110
    "N0N",           // #111
    "0",             // #112
    "B1m",           // #113
    "h7E",           // #114
    "c05",           // #115
    "K20",           // #116
    "32D",           // #117
    "F19",           // #118
    "r4d",           // #119
    "D2F",           // #120
    "D0m",           // #121
    "m6B",           // #122
    "M0j",           // #123
    "Q8G",           // #124
    "A1t",           // #125
    "p7J",           // #126
    "t0H",           // #127
    "M5i",           // #128
    "j1L",           // #129
    "e7i",           // #130
    "T1E",           // #131
    "85i",           // #132
    "71H",           // #133
    "20H",           // #134
    "T9n",           // #135
    "58B",           // #136
    "J4R",           // #137
    "93N",           // #138
    "t0F",           // #139
    "M7G",           // #140
    "r4P",           // #141
    "i1d",           // #142
    "a22",           // #143
    "M39",           // #144
    "C23",           // #145
    "816",           // #146
    "E0M",           // #147
    "T4b",           // #148
    "L1L",           // #149
    "i5m",           // #150
    "P2R",           // #151
    "t77",           // #152
    "A5E",           // #153
    "88e",           // #154
    "k1b",           // #155
    "m04",           // #156
    "41b",           // #157
    "B4k",           // #158
    "J1M",           // #159
    "H4M",           // #160
    "C1D",           // #161
    "12K",           // #162
    "822",           // #163
    "E1T",           // #164
    "Q4H",           // #165
    "k4d",           // #166
    "k4J",           // #167
    "L70",           // #168
    "31f",           // #169
    "G1P",           // #170
    "34e",           // #171
    "939",           // #172
    "24F",           // #173
    "43r",           // #174
    "M81",           // #175
    "01E",           // #176
    "A0N",           // #177
    "65f",           // #178
    "Q6p",           // #179
    "93R",           // #180
    "r0i",           // #181
    "A35",           // #182
    "P40",           // #183
    "G9R",           // #184
    "c7C",           // #185
    "P17",           // #186
    "76f",           // #187
    "99p",           // #188
    "96E",           // #189
    "p3E",           // #190
    "h6t",           // #191
    "n2L",           // #192
];

/// Format for a target to plug into the MotorTarget variant
/// of the Command enum.
#[derive(Clone, Debug)]
pub struct TargetCommand {
    pub x_target: u16,
    pub y_target: u16,
    pub theta_target: u16,
}

/// Format for a RGB color to plug into the MultiLed variant
/// of the Command enum.
#[derive(Clone, Debug)]
pub struct LedCommand {
    pub duration: u8,
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

/// Format for a MIDI note to plug into the Midi variant
/// of the Command enum.
#[derive(Clone, Debug)]
pub struct MidiCommand {
    pub duration: u8,
    pub note: u8,
    pub volume: u8,
}

/// An enum to list out all possible commands to send to a toio
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Command {
    //Request Commands
    MotionRequest,
    MagneticRequest,
    PostureRequest {
        format: u8,
    },

    //Motor Commands
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

    //Light Commands
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

    //Sound Commands
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
        roll: u16,
        pitch: u16,
        yaw: u16,
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

pub struct ToioScanner {
    central: Adapter,
    ordered: bool,
    filter: Option<Vec<String>>,
}

pub struct ToioReceiver {
    receiver: Receiver<Either<ToioPeripheral, platform::PeripheralId>>,
}

pub struct ToioPeripheral {
    pub name: String,
    peripheral: platform::Peripheral,
    pub peripheral_id: platform::PeripheralId,
}

// Consolidated state struct to reduce lock overhead
#[derive(Clone)]
pub struct ToioState {
    pub battery: Option<u8>,
    pub last_update: Option<SystemTime>,
    pub last_command: Option<SystemTime>,
}

impl ToioState {
    fn new() -> Self {
        ToioState {
            battery: None,
            last_update: None,
            last_command: None,
        }
    }
}

pub struct Toio {
    pub toio: ToioPeripheral,
    pub name: String,
    pub id: String,
    pub connected: bool,
    pub channel: Option<JoinHandle<()>>,
    pub state: Arc<RwLock<ToioState>>,
}

impl Updates {
    fn new(receiver: Receiver<Update>) -> Updates {
        return Updates { receiver };
    }

    pub async fn next(&mut self) -> Option<Update> {
        return self.receiver.recv().await;
    }
}

impl ToioScanner {
    pub async fn new() -> Result<ToioScanner, Box<dyn Error>> {
        let manager = Manager::new().await?;

        // get the first bluetooth adapter
        let central = manager
            .adapters()
            .await
            .unwrap()
            .into_iter()
            .nth(0)
            .unwrap();

        Ok(ToioScanner {
            central,
            filter: None,
            ordered: false,
        })
    }

    pub async fn new_with_filter(
        ordered: bool,
        filter: Vec<usize>,
    ) -> Result<ToioScanner, Box<dyn Error>> {
        let manager = Manager::new().await?;

        // get the first bluetooth adapter
        let central = manager
            .adapters()
            .await
            .unwrap()
            .into_iter()
            .nth(0)
            .unwrap();

        let toio_filter = filter.iter().map(|x| IDARR[*x].to_string()).collect();

        Ok(ToioScanner {
            central,
            filter: Some(toio_filter),
            ordered,
        })
    }

    pub async fn search(&self) -> Result<ToioReceiver, Box<dyn Error>> {
        let central = self.central.clone();
        let mut events = central.events().await?;

        // start scanning for devices
        central
            .start_scan(ScanFilter {
                services: vec![SERVICE, CONFIG],
            })
            .await?;

        let (tx, rx) = mpsc::channel(32);

        //Discovery Async Task
        let mut toio_filter = self.filter.clone();
        let ordered = self.ordered;
        tokio::spawn(async move {
            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceDiscovered(id) => {
                        let peripheral = central.peripheral(&id).await.unwrap();

                        if ordered {
                            // if filtered order, update filter on successful connection
                            if let Some(ref mut new_filter) = toio_filter.clone() {
                                if !new_filter.is_empty() {
                                    let curr_toio = new_filter.remove(0);

                                    let connect_success = Self::try_connect(
                                        peripheral,
                                        &tx,
                                        Some(vec![curr_toio.clone()]),
                                    )
                                    .await;

                                    if connect_success {
                                        toio_filter = Some(new_filter.to_vec());
                                    }
                                }
                            }
                        } else {
                            Self::try_connect(peripheral, &tx, toio_filter.clone()).await;
                        }
                    }
                    CentralEvent::ServicesAdvertisement { id, services: _ } => {
                        let peripheral = central.peripheral(&id).await.unwrap();

                        if ordered {
                            // if filtered order, update filter on successful connection
                            if let Some(ref mut new_filter) = toio_filter.clone() {
                                if !new_filter.is_empty() {
                                    let curr_toio = new_filter.remove(0);

                                    let connect_success = Self::try_connect(
                                        peripheral,
                                        &tx,
                                        Some(vec![curr_toio.clone()]),
                                    )
                                    .await;

                                    if connect_success {
                                        toio_filter = Some(new_filter.to_vec());
                                    }
                                }
                            }
                        } else {
                            Self::try_connect(peripheral, &tx, toio_filter.clone()).await;
                        }
                    }
                    CentralEvent::DeviceDisconnected(id) => {
                        Self::set_disconnected(id, &tx).await;
                    }
                    _ => {}
                }
            }
        });

        return Ok(ToioReceiver::new(rx));
    }

    async fn try_connect(
        peripheral: platform::Peripheral,
        tx: &Sender<Either<ToioPeripheral, platform::PeripheralId>>,
        filter: Option<Vec<String>>,
    ) -> bool {
        if let Some(properties) = peripheral.properties().await.unwrap() {
            let fullname = properties.local_name.unwrap_or("".to_string());
            if peripheral.is_connected().await.unwrap() || !fullname.contains(&"toio") {
                return false;
            }

            let name: Vec<&str> = fullname.split('-').collect();
            let toio_name = name.last().unwrap_or(&&" ").to_string();
            if let Some(filter_list) = filter {
                if !filter_list.contains(&toio_name) {
                    return false;
                }
            }

            let toio_peripheral = ToioPeripheral::new(toio_name, peripheral);
            if !toio_peripheral.connect().await {
                return false;
            }

            tx.send(Either::Left(toio_peripheral)).await.unwrap();

            return true;
        }

        return false;
    }

    async fn set_disconnected(
        peripheral_id: platform::PeripheralId,
        tx: &Sender<Either<ToioPeripheral, platform::PeripheralId>>,
    ) {
        tx.send(Either::Right(peripheral_id)).await.unwrap();
    }
}

impl ToioReceiver {
    fn new(receiver: Receiver<Either<ToioPeripheral, platform::PeripheralId>>) -> ToioReceiver {
        return ToioReceiver { receiver };
    }

    pub async fn next(&mut self) -> Option<Either<ToioPeripheral, platform::PeripheralId>> {
        return self.receiver.recv().await;
    }
}

impl ToioPeripheral {
    pub fn new(name: String, peripheral: platform::Peripheral) -> ToioPeripheral {
        ToioPeripheral {
            name,
            peripheral_id: peripheral.id(),
            peripheral,
        }
    }

    pub async fn connect(&self) -> bool {
        if let Err(_) = self.peripheral.connect().await {
            return false;
        } else if let Err(_) = self.peripheral.discover_services().await {
            return false;
        }

        for characteristic in self.peripheral.characteristics().into_iter() {
            if !characteristic.properties.contains(CharPropFlags::NOTIFY) {
                continue;
            }

            if let Err(err) = self.peripheral.subscribe(&characteristic).await {
                eprintln!("Error connecting to characteristic, skipping: {}", err);
                continue;
            }
        }

        return true;
    }

    pub async fn updates(&self) -> Result<Updates, Box<dyn Error>> {
        let (tx, rx) = mpsc::channel(32);

        let mut notification_stream = self.peripheral.notifications().await?;
        tokio::spawn(async move {
            while let Some(event) = notification_stream.next().await {
                if let Some(update) = ToioPeripheral::get_update(event) {
                    if tx.send(update).await.is_err() {
                        // Channel closed, exit gracefully
                        break;
                    }
                }
            }
        });

        return Ok(Updates::new(rx));
    }

    fn get_update(notification: ValueNotification) -> Option<Update> {
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
                    println!(
                        "Unkown {} Update: {:?}",
                        uuid_to_string(notification.uuid),
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
                    println!(
                        "Unkown {} Update: {:?}",
                        uuid_to_string(notification.uuid),
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
                _ => {
                    println!(
                        "Unkown {} Update: {:?}",
                        uuid_to_string(notification.uuid),
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
                println!(
                    "Unkown {} Update: {:?}",
                    uuid_to_string(notification.uuid),
                    vals
                );
                None
            }
        }
    }

    pub async fn send_command(&self, command: Command) {
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

        let (response_flag, response_type) = match uuid {
            LIGHT | SOUND => (CharPropFlags::WRITE, WriteType::WithResponse),
            _ => (
                CharPropFlags::WRITE_WITHOUT_RESPONSE,
                WriteType::WithoutResponse,
            ),
        };

        let cmd: Vec<u8> = match command {
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
        };

        self.write(uuid, cmd, response_flag, response_type).await;
    }

    pub async fn write(
        &self,
        uuid: Uuid,
        cmd: Vec<u8>,
        response_flag: CharPropFlags,
        response_type: WriteType,
    ) {
        let characteristic = Characteristic {
            uuid: uuid,
            service_uuid: SERVICE,
            properties: response_flag,
        };

        let _ = self
            .peripheral
            .write(&characteristic, &cmd, response_type)
            .await;
    }
}

impl Toio {
    pub fn new(toio: ToioPeripheral) -> Toio {
        return Toio {
            name: toio.name.clone(),
            id: if let Some(id) = return_toio_id(&toio.name) {
                format!("{}", id)
            } else {
                "N/A".to_owned()
            },
            connected: true,
            channel: None,
            state: Arc::new(RwLock::new(ToioState::new())),
            toio,
        };
    }

    pub fn add_channel(&mut self, channel: JoinHandle<()>) {
        self.channel = Some(channel);
    }

    pub fn disconnect(&mut self) {
        if let Some(channel) = &self.channel {
            channel.abort();
        }
        self.connected = false;
    }

    pub fn update_last_command(&self) {
        let state = self.state.clone();
        tokio::spawn(async move {
            let mut state_write = state.write().await;
            state_write.last_command = Some(SystemTime::now());
        });
    }
}

/// matches UUIDs of toios a string of their corresponding service
pub fn uuid_to_string(uuid: Uuid) -> String {
    return match uuid {
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
    .to_owned();
}

// parse a multi-targeting command
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

// parse a led command
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

// parse a MIDI command
fn parse_midi_command(repetitions: u8, vals: Vec<MidiCommand>) -> Vec<u8> {
    let mut cmd = vec![0x03, repetitions, vals.len() as u8];

    for note in vals.iter() {
        cmd.push(note.duration);
        cmd.push(note.note);
        cmd.push(note.volume);
    }

    return cmd;
}

// return the toio ID that corresponds to a specific toio
fn return_toio_id(name: &str) -> Option<usize> {
    return IDARR.iter().position(|&r| r == name);
}
