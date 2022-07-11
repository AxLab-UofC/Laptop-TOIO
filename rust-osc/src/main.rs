extern crate getopts;
use getopts::Options;
use btleplug::api::CharPropFlags;
use btleplug::api::{
    BDAddr, Central, CentralEvent, Characteristic, Manager as _, Peripheral, WriteType,
};

use btleplug::platform::Manager;
use futures::stream::StreamExt;
use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, net::SocketAddr};
use tokio::time;
use tokio::{net::UdpSocket, sync::mpsc};
use uuid::Uuid;

//Update TOIO_NUM to the number of TOIO you are connecting to, and 
//TOIO_ALLOWED to the IDs of the TOIO you want to connect to
const TOIO_NUM: usize = 1;
const TOIO_ALLOWED: [i32; TOIO_NUM] = 
    [31];

//#[macro_use]
extern crate log;

//characteristic of interest
const TOIO_SERVICE_UUID: Uuid = Uuid::from_u128(0x10B20100_5B3B_4571_9508_CF3EFCD7BBAE);
const POSITION_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x10B20101_5B3B_4571_9508_CF3EFCD7BBAE);
const MOTOR_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x10B20102_5B3B_4571_9508_CF3EFCD7BBAE);
const BUTTON_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x10B20107_5B3B_4571_9508_CF3EFCD7BBAE);
const LIGHT_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x10B20103_5B3B_4571_9508_CF3EFCD7BBAE);
const MOTION_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x10B20106_5B3B_4571_9508_CF3EFCD7BBAE);
const SOUND_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x10B20104_5B3B_4571_9508_CF3EFCD7BBAE);

//The list of bluetooth adresses of the TOIO Bots currently known.
//update to add bots
const BLTADDR: [&str; 129] = 
["BLE Address",  //TOIO Num
"0",  // #1
"66:35:33:35:30:64",  // #2
"38:62:66:39:30:32",  // #3
"62:62:64:38:37:64",  // #4
"38:37:33:31:34:61",  // #5
"66:64:65:63:66:37",  // #6
"38:34:36:39:62:61",  // #7
"61:34:38:64:37:39",  // #8
"0",  // #9
"0",  // #10
"0",  // #11
"38:30:65:36:35:34",  // #12
"36:32:66:39:32:65",  // #13
"0",  // #14
"65:33:36:65:64:65",  // #15
"0",  // #16
"0",  // #17
"65:62:64:38:62:63",  // #18
"36:66:65:33:31:62",  // #19
"36:64:34:39:62:66",  // #20
"30:33:65:63:31:35",  // #21
"35:65:30:63:37:37",  // #22
"39:31:61:61:64:32",  // #23
"61:36:32:61:38:64",  // #24
"0",  // #25
"35:39:61:61:64:31",  // #26
"66:36:39:64:64:62",  // #27
"64:39:35:32:32:61",  // #28
"0",  // #29
"0",  // #30
"66:61:39:61:37:61",  // #31
"61:66:65:35:37:33",  // #32
"62:33:61:36:38:39",  // #33
"63:31:37:39:33:36",  // #34
"36:63:32:37:31:64",  // #35
"0",  // #36
"0",  // #37
"0",  // #38
"0",  // #39
"0",  // #40
"34:65:61:37:33:61",  // #41
"63:37:39:32:66:64",  // #42
"39:63:36:38:66:61",  // #43
"65:33:33:64:35:31",  // #44
"33:30:62:30:66:35",  // #45
"66:62:37:62:63:63",  // #46
"33:63:32:61:34:64",  // #47
"0",  // #48
"33:63:37:65:33:30",  // #49
"0",  // #50
"0",  // #51
"0",  // #52
"0",  // #53
"0",  // #54
"0",  // #55
"0",  // #56
"0",  // #57
"0",  // #58
"0",  // #59
"0",  // #60
"0",  // #61
"0",  // #62
"0",  // #63
"0",  // #64
"0",  // #65
"0",  // #66
"0",  // #67
"0",  // #68
"0",  // #69
"0",  // #70
"0",  // #71
"0",  // #72
"0",  // #73
"0",  // #74
"0",  // #75
"0",  // #76
"0",  // #77
"0",  // #78
"33:37:32:38:32:30",  // #79
"34:34:32:34:38:65",  // #80
"65:30:37:66:66:61",  // #81
"33:61:62:30:35:66",  // #82
"66:31:65:66:63:64",  // #83
"65:34:63:36:31:35",  // #84
"33:33:62:33:65:33",  // #85
"38:66:33:63:63:32",  // #86
"37:63:35:37:37:30",  // #87
"39:32:39:30:65:62",  // #88
"63:32:32:38:30:32",  // #89
"33:31:36:36:31:65",  // #90
"33:66:35:61:61:64",  // #91
"35:63:33:33:34:32",  // #92
"36:34:35:38:35:32",  // #93
"39:66:33:35:64:32",  // #94
"32:30:34:65:39:35",  // #95
"38:32:62:34:66:64",  // #96
"64:66:30:66:62:32",  // #97
"62:62:64:30:39:63",  // #98
"38:34:39:66:65:66",  // #99
"35:62:39:66:65:61",  // #100
"34:30:37:39:66:65",  // #101
"33:39:30:32:37:34",  // #102
"38:35:61:37:32:32",  // #103
"61:36:63:36:30:31",  // #104
"65:39:66:31:35:33",  // #105
"65:33:32:31:34:38",  // #106
"38:32:34:39:66:37",  // #107
"37:63:39:62:33:33",  // #108
"64:62:38:32:64:31",  // #109
"30:36:66:30:61:31",  // #110
"36:61:65:38:62:62",  // #111
"33:34:34:31:37:32",  // #112
"31:64:31:61:39:33",  // #113
"37:65:63:38:64:61",  // #114
"33:31:39:32:30:33",  // #115
"61:32:36:66:32:33",  // #116
"36:38:35:63:62:63",  // #117
"34:31:31:61:37:61",  // #118
"62:31:36:63:31:61",  // #119
"64:37:65:38:39:38",  // #120
"30:35:63:35:38:31",  // #121
"38:37:31:35:33:37",  // #122
"32:66:33:39:38:30",  // #123
"33:66:34:36:38:65",  // #124
"36:32:32:33:66:38",  // #125
"66:61:32:30:32:31",  // #126
"63:63:34:37:31:30",  // #127
"64:31:36:34:63:62",  // #128
];

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_toio_connected(toio_connected: i32){
    if toio_connected == 1 {println!("1 peripheral connected");}
    else {println!("{} peripherals connected", toio_connected);}
}

fn return_toio_id(addr: BDAddr) -> i32 {
    if !((&BLTADDR).into_iter().any(|v| v.to_string() == addr.to_string())) {
        return 0;
    }

    let toio_id = BLTADDR
    .iter()
    .position(|&x| x == addr.to_string())
    .unwrap();

    return toio_id as i32;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut toio_connected = 0;
    //read command line arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    //allow input (Not nessasary)
    let mut opts = Options::new();
    opts.optopt("p", "port", "set receiving port", "PORT_NUMBER");
    opts.optopt("r", "remote", "set remote port", "IP:PORT_NUMBER");
    opts.optopt("i", "host_id", "set host id number", "ID_NUMBER");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}",f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    //listen from port
    let port_number = matches.opt_str("p").unwrap_or("3334".to_string());
    let listening_address = format!("0.0.0.0:{}",port_number);

    //the ID/addresses of the cubes
    let addresses: Arc<Mutex<Vec<BDAddr>>> = Arc::new(Mutex::new(Vec::new()));
    let senders: Arc<Mutex<HashMap<BDAddr, mpsc::Sender<OscMessage>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    //OSC listening on port 3334
    let sock = UdpSocket::bind((listening_address).parse::<SocketAddr>().unwrap()).await?;
    println!("OSC listening on port {}", port_number);

    let r = Arc::new(sock);
    let s = r.clone();
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    //Where to send packets
    let remote = matches.opt_str("r").unwrap_or("127.0.0.1:3333".to_string());
    let remote_read = remote.parse::<SocketAddr>();
    let remote_addr = if remote_read.is_ok() {
        remote_read.unwrap()
    } else {
        eprintln!("Remote address {} is wrongly formatted, use IP:PORT (127.0.0.1:3333)", remote);
        return Ok(());
    };

    let host_id = matches.opt_str("i").unwrap_or("0".to_string()).parse::<i32>().unwrap_or(0);
    println!("Sending messages to {} prefixed by {}", remote_addr, host_id);


    //Send OSC
    tokio::spawn(async move {
        //just one channel
        while let Some((bytes, addr)) = rx.recv().await {
            s.send_to(&bytes, &addr).await.unwrap();
        }
    });

    //Receive OSC
    let mut buf = [0; 1024];
    let addresses2 = addresses.clone();
    let senders2 = senders.clone();

    tokio::spawn(async move {
        while let Ok((len, addr)) = r.recv_from(&mut buf).await {
            //println!("{:?} bytes received from {:?}", len, addr);
            let packet = rosc::decoder::decode(&buf[..len]).unwrap();
            match packet {
                OscPacket::Message(msg) => {
                    if msg.args.len() > 0 {
                        let mut marg = 0;
                        if let OscType::Int(i) = msg.args[0] {
                            marg = i;
                        }
                        //println!("Got a message for {}", marg);

                        // find the address
                        let maybe_address = {
                            let addr = addresses2.lock().unwrap();
                            if addr.len() > marg as usize {
                                Some(addr[marg as usize])
                            } else {
                                None
                            }
                        };
                        if let Some(address) = maybe_address {
                            //try to get the channel and not breaking everything
                            let sender = {
                                let sends = senders2.lock().unwrap();
                                sends.get(&address).map(|p| p.clone())
                                //we drop the lock here because we *clone*
                            };
                            if let Some(channel) = sender {
                                //println!("Sending to...");
                                channel.send(msg).await.unwrap();
                            }
                        }
                    }
                }
                OscPacket::Bundle(bundle) => {
                    println!("OSC Bundle: {:?}", bundle);
                }
            }
        }
    });

    let manager = Manager::new().await?;

    // get the first bluetooth adapter
    // connect to the adapter
    let adapters = manager.adapters().await.unwrap();
    let central = adapters.into_iter().nth(0).unwrap();

    //get the events from the central
    let mut events = central.events().await?;

    // start scanning for devices
    central.start_scan().await?;

    println!("Scanning for BTLE events...");

    //Scan all the time
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(bd_addr) => {
                let peripheral = central.peripheral(bd_addr).await.unwrap();
                
                let properties = peripheral.properties().await?;
                let services = properties.unwrap().services;

                if services.contains(&TOIO_SERVICE_UUID) {
                    let toio_id = return_toio_id(bd_addr);
                    println!("TOIO {} found: {}", toio_id, bd_addr);

                    if !(TOIO_ALLOWED.contains(&toio_id)) {
                        continue;
                    }

                    //we kave a toio cube!
                    let tx3 = tx.clone();
                    let is_connected = peripheral.is_connected().await?;
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            eprintln!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    time::sleep(Duration::from_millis(200)).await;
                    let properties = peripheral.properties().await?;
                    let address = properties.unwrap().address;

                    //find the id for this cube
                    let mut addr = addresses.lock().unwrap();

                    //do we already know that address?
                    let id = if let Some(index) = addr.iter().position(|&a| a == address) {
                        index
                    } else {
                        //no, add it to the list
                        addr.push(address);
                        addr.len() - 1
                    };
                    //get rid of the mutex lock
                    drop(addr);

                    //creating the channels
                    let (tx, mut rx) = mpsc::channel::<OscMessage>(1_000);
                    //saving one end to allow to receive OSC
                    let mut sends = senders.lock().unwrap();
                    sends.insert(address, tx);
                    drop(sends);

                    let id2 = id;
                    let p2 = peripheral.clone();
                    tokio::spawn(async move {
                        while let Some(message) = rx.recv().await {
                            //println!("Received {:?} for cube {}", message, id2);
                            match message.addr.as_ref() {
                                "/motor" => {
                                    if message.args.len() == 4 {
                                        //we should have 4 args
                                        let mut marg = [0; 4];
                                        for k in 0..4 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }
                                        let leftdirection = if marg[1] < 0 { 0x02 } else { 0x01 };
                                        let rightdirection = if marg[2] < 0 { 0x02 } else { 0x01 };

                                        let characteristic = Characteristic {
                                            uuid: MOTOR_CHARACTERISTIC_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x02,                 //motor
                                            0x01,                 //left
                                            leftdirection,        //direction
                                            marg[1].abs() as u8,  //speed
                                            0x02,                 //right
                                            rightdirection,       //direction
                                            marg[2].abs() as u8,  //speed
                                            (marg[3] / 10) as u8, //length
                                        ];
                                        p2.write(&characteristic, &cmd, WriteType::WithoutResponse)
                                            .await
                                            .unwrap();
                                    } else {
                                        //error
                                    }
                                }
                                "/led" => {
                                    if message.args.len() == 5 {
                                        //we should have 5 args
                                        let mut marg = [0; 5];
                                        for k in 0..5 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }
                                        let characteristic = Characteristic {
                                            uuid: LIGHT_CHARACTERISTIC_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x03,                 //light
                                            (marg[1] / 10) as u8, //length
                                            0x01,                 //led
                                            0x01,                 //reserved
                                            marg[2].abs() as u8,  //red
                                            marg[3].abs() as u8,  //green
                                            marg[4].abs() as u8,  //blue
                                        ];
                                        println!("{:?}", cmd);
                                        p2.write(&characteristic, &cmd, WriteType::WithResponse)
                                            .await
                                            .unwrap();
                                    } else {
                                        //error
                                    }
                                }
                                "/sound" => {
                                    if message.args.len() == 3 {
                                        //we should have 3 args
                                        let mut marg = [0; 3];
                                        for k in 0..3 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }
                                        let characteristic = Characteristic {
                                            uuid: SOUND_CHARACTERISTIC_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x02,          //sound
                                            marg[1] as u8, //sound effect ID
                                            marg[2] as u8  //volume
                                        ];
                                        println!("{:?}", cmd);
                                        p2.write(&characteristic, &cmd, WriteType::WithResponse)
                                            .await
                                            .unwrap();
                                    } else {
                                        //error
                                    }
                                }
                                "/midi" => {
                                    if message.args.len() == 4 {
                                        //we should have 5 args
                                        let mut marg = [0; 4];
                                        for k in 0..4 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }
                                        let characteristic = Characteristic {
                                            uuid: SOUND_CHARACTERISTIC_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x03,                 //light
                                            0x01,                 //repetitions
                                            0x01,                 //operations
                                            marg[1].abs() as u8,  //duration
                                            marg[2].abs() as u8,  //MIDI note number
                                            marg[3].abs() as u8,  //volume
                                        ];
                                        println!("{:?}", cmd);
                                        p2.write(&characteristic, &cmd, WriteType::WithResponse)
                                            .await
                                            .unwrap();
                                    } else {
                                        //error
                                    }
                                }
                                "/motion" => {
                                    let characteristic = Characteristic {
                                        uuid: MOTION_CHARACTERISTIC_UUID,
                                        properties: CharPropFlags::WRITE,
                                    };
                                    let cmd = vec![
                                        0x81,
                                    ];
                                    //println!("{:?}", cmd);
                                    p2.write(&characteristic, &cmd, WriteType::WithResponse)
                                        .await
                                        .unwrap();
                                }
                                _ => {}
                            }
                        }
                    });

                    let is_connected = peripheral.is_connected().await?;
                    println!("Peripheral {} connected: {}", address, is_connected);

                    println!("Discovering peripheral characteristics...");
                    let chars = peripheral.discover_characteristics().await?;
                    for characteristic in chars.into_iter() {
                        println!("Checking {:?}", characteristic);
                        if characteristic.uuid == POSITION_CHARACTERISTIC_UUID
                            && characteristic.properties.contains(CharPropFlags::NOTIFY) {
                            println!(
                                "Subscribing to position characteristic {:?}",
                                characteristic.uuid
                            );
                            peripheral.subscribe(&characteristic).await?;
                        } 
                        if characteristic.uuid == BUTTON_CHARACTERISTIC_UUID
                            && characteristic.properties.contains(CharPropFlags::NOTIFY) {
                            println!(
                                "Subscribing to button characteristic {:?}",
                                characteristic.uuid
                            );
                            peripheral.subscribe(&characteristic).await?;
                        } 
                        if characteristic.uuid == MOTION_CHARACTERISTIC_UUID
                            && characteristic.properties.contains(CharPropFlags::NOTIFY) {
                            println!(
                                "Subscribing to motion characteristic {:?}",
                                characteristic.uuid
                            );
                            peripheral.subscribe(&characteristic).await?;
                        }
                    }

                    //after scanning all chars and subscribing
                    //we can expect to get notifications as a stream
                    //TODO figure a way to end the task on disconnect
                    let mut notification_stream = peripheral.notifications().await.unwrap();
                    tokio::spawn(async move {
                        while let Some(data) = notification_stream.next().await {
                            match data.uuid {
                                POSITION_CHARACTERISTIC_UUID => {
                                    //data is
                                    // data[0] is 1 for read, 3 for off
                                    // data[1] data[2] is x
                                    // data[3] data[4] is y
                                    // data[5] data[6] is angle
                                    if data.value[0] == 1 {
                                        let x = (data.value[2] as u32) << 8 | data.value[1] as u32;
                                        let y = (data.value[4] as u32) << 8 | data.value[3] as u32;
                                        let angle =
                                            (data.value[6] as u32) << 8 | data.value[5] as u32;
                                        let realx =
                                            (data.value[8] as u32) << 8 | data.value[7] as u32;
                                        let realy =
                                            (data.value[10] as u32) << 8 | data.value[9] as u32;
                                        //println!( "Received data from cube {}: {},{} {}", id, x, y, angle );
                                        let msg =
                                            encoder::encode(&OscPacket::Message(OscMessage {
                                                addr: "/position".to_string(),
                                                args: vec![
                                                    OscType::Int(host_id),
                                                    OscType::Int(id as i32),
                                                    OscType::Int(x as i32),
                                                    OscType::Int(y as i32),
                                                    OscType::Int(angle as i32),
                                                    OscType::Int(realx as i32),
                                                    OscType::Int(realy as i32),
                                                ],
                                            }))
                                            .unwrap();

                                        tx3.send((msg, remote_addr)).await.unwrap();
                                    }
                                },
                                BUTTON_CHARACTERISTIC_UUID => {
                                    let button = data.value[1];
                                    let msg = encoder::encode(&OscPacket::Message(OscMessage {
                                        addr: "/button".to_string(),
                                        args: vec![
                                            OscType::Int(host_id),
                                            OscType::Int(id as i32),
                                            OscType::Int(button as i32),
                                        ],
                                    }))
                                    .unwrap();

                                    tx3.send((msg, remote_addr)).await.unwrap();
                                },
                                MOTION_CHARACTERISTIC_UUID => {
                                    let flatness = data.value[1];
                                    let hit = data.value[2];
                                    let double_tap = data.value[3];
                                    let face_up = data.value[4];
                                    let shake_level = data.value[5];

                                    let msg = encoder::encode(&OscPacket::Message(OscMessage {
                                        addr: "/motion".to_string(),
                                        args: vec![
                                            OscType::Int(host_id),
                                            OscType::Int(id as i32),
                                            OscType::Int(flatness as i32),
                                            OscType::Int(hit as i32),
                                            OscType::Int(double_tap as i32),
                                            OscType::Int(face_up as i32),
                                            OscType::Int(shake_level as i32),
                                        ],
                                    }))
                                    .unwrap();

                                    tx3.send((msg, remote_addr)).await.unwrap();
                                },
                                _ => {}
                            }
                        }
                    });
                    println!("TOIO {} connected", toio_id);

                    toio_connected += 1;
                    print_toio_connected(toio_connected);
                }
            }
            CentralEvent::DeviceDisconnected(bd_addr) => {
                let toio_id = return_toio_id(bd_addr);
                println!("TOIO {} disconnected", toio_id);

                toio_connected -= 1;
                print_toio_connected(toio_connected);
            }
            _ => {}
        }
    }

    Ok(())
}