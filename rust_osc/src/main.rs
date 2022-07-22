extern crate getopts;
use getopts::Options;
use btleplug::api::CharPropFlags;
use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral, WriteType, ScanFilter,
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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_toio_connected(toio_connected: i32){
    if toio_connected == 1 {println!("1 peripheral connected");}
    else {println!("{} peripherals connected", toio_connected);}
}

fn return_toio_id(name: &str) -> &str{
    //skeleton function
    return name;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut toio_connected = 0;
    let mut verbose = false;

    //read command line arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "port", "set receiving port", "PORT_NUMBER");
    opts.optopt("r", "remote", "set remote port", "IP:PORT_NUMBER");
    opts.optopt("i", "host_id", "set host id number", "ID_NUMBER");
    opts.optopt("n", "names", "connect to those cubes only (toio Core Cube-AAA, toio Core Cube-BBB,...)", "AAA,BBB,CCC");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "print more connection details");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}",f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }
    if matches.opt_present("v") {
        verbose = true;
    }

    let port_number = matches.opt_str("p").unwrap_or("3334".to_string());
    let listening_address = format!("0.0.0.0:{}",port_number);

    //this will be filled with the names to connect to *if needed*
    let possible_names = match matches.opt_str("n") {
        Some(names) => {
            let mut res = Vec::new();
            for part in names.split(",") {
                if part.len() == 3 {
                    res.push(format!("toio Core Cube-{}", return_toio_id(part)));
                } else {
                    panic!("Error while reading the names");
                }
            }
            Some(res)
        },
        None => {None},
    };

    //the ID/addresses of the cubes
    //let addresses: Arc<Mutex<Vec<BDAddr>>> = Arc::new(Mutex::new(Vec::new()));
    let uuids: Arc<Mutex<Vec<btleplug::platform::PeripheralId>>> = Arc::new(Mutex::new(Vec::new()));
    let senders: Arc<Mutex<HashMap<btleplug::platform::PeripheralId, mpsc::Sender<OscMessage>>>> =
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
    //let addresses2 = addresses.clone();
    let uuids2 = uuids.clone();
    let senders2 = senders.clone();

    tokio::spawn(async move {
        while let Ok((len, addr)) = r.recv_from(&mut buf).await {
            println!("{:?} bytes received from {:?}", len, addr);
            let packet = rosc::decoder::decode(&buf[..len]).unwrap();
            match packet {
                OscPacket::Message(msg) => {
                    if msg.args.len() > 0 {
                        let mut marg = 0;
                        if let OscType::Int(i) = msg.args[0] {
                            marg = i;
                        }
                        println!("Got a message for {}", marg);

                        // find the address
                        let maybe_uuid = {
                            let storage_uuid = uuids2.lock().unwrap();
                            if storage_uuid.len() > marg as usize {
                                Some(storage_uuid[marg as usize].clone())
                            } else {
                                None
                            }
                        };
                        if let Some(uuid) = maybe_uuid {
                            //try to get the channel and not breaking everything
                            let sender = {
                                let sends = senders2.lock().unwrap();
                                sends.get(&uuid).map(|p| p.clone())
                                //we drop the lock here because we *clone*
                            };
                            if let Some(channel) = sender {
                                println!("Sending to...");
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
    println!("Found Manager: {:?}", manager);

    // get the first bluetooth adapter
    // connect to the adapter
    let adapters = manager.adapters().await.unwrap();
    println!("Found Adapter: ");
    for adapter in adapters.iter() {
        println!("Found Adapter: {}", adapter.adapter_info().await?);
    }

    let central = adapters.into_iter().nth(0).unwrap();

    //get the events from the central
    let mut events = central.events().await?;

    // start scanning for devices
    central.start_scan(ScanFilter{ services: vec![TOIO_SERVICE_UUID]}).await?;

    if verbose {
        println!("Scanning for BTLE events on {:?}...",central);
    }
    else {
        println!("Scanning for BTLE events...");
    }
    

    //Scan all the time
    while let Some(event) = events.next().await {
        //println!("event... {:?}", event);
        match event {
            CentralEvent::DeviceDiscovered(bd_addr) => {
                let peripheral = central.peripheral(&bd_addr).await.unwrap();

                let properties = peripheral.properties().await?.unwrap();
                let local_name = properties.local_name.unwrap_or("".to_string());
                println!("Seeing peripheral with name: {}", local_name);

                
                let services = properties.services;
                let should_connect = if let Some(names)=&possible_names {
                    names.contains(&local_name)
                } else {
                    services.contains(&TOIO_SERVICE_UUID)
                };

                //if (services.contains(&TOIO_SERVICE_UUID)) || possible_names.contains(&local_name)  {
                if should_connect {
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
                    let peripheral_id = peripheral.id();
                    //println!("Peripheral ID is {:?}", peripheral_id);

                    //find the id for this cube
                    //let mut addr = addresses.lock().unwrap();
                    let mut storage_uuid = uuids.lock().unwrap();

                    //do we already know that address?
                    let id = if let Some(index) = storage_uuid.iter().position(|a| a == &peripheral_id) {
                        index
                    } else {
                        //no, add it to the list
                        storage_uuid.push(peripheral_id.clone());
                        storage_uuid.len() - 1
                    };
                    //get rid of the mutex lock
                    drop(storage_uuid);
                    //println!("Connecting to Cube {} address is {}", id, address);

                    //creating the channels
                    let (tx, mut rx) = mpsc::channel::<OscMessage>(1_000);
                    //saving one end to allow to receive OSC
                    let mut sends = senders.lock().unwrap();
                    sends.insert(peripheral_id, tx);
                    drop(sends);

                    let id2 = id;
                    let p2 = peripheral.clone();
                    tokio::spawn(async move {
                        while let Some(message) = rx.recv().await {
                            println!("Received {:?} for cube {}", message, id2);
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
                                            service_uuid: TOIO_SERVICE_UUID,
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
                                            service_uuid: TOIO_SERVICE_UUID,
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
                                            service_uuid: TOIO_SERVICE_UUID,
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
                                            service_uuid: TOIO_SERVICE_UUID,
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
                                        service_uuid: TOIO_SERVICE_UUID,
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
                    if verbose {
                        println!("Peripheral {} connected: {}", address, is_connected);
                        println!("Discovering peripheral characteristics...");
                    }
                    let chars = peripheral.characteristics();

                    for characteristic in chars.into_iter() {
                        if verbose {
                            println!("Checking {:?}", characteristic);
                        }
                        if characteristic.uuid == POSITION_CHARACTERISTIC_UUID
                            && characteristic.properties.contains(CharPropFlags::NOTIFY)
                        {
                            if verbose {
                                println!(
                                    "Subscribing to position characteristic {:?}",
                                    characteristic.uuid
                                );
                            }
                            peripheral.subscribe(&characteristic).await?;
                        } 
                        if characteristic.uuid == BUTTON_CHARACTERISTIC_UUID
                            && characteristic.properties.contains(CharPropFlags::NOTIFY)
                        {
                            if verbose {
                                println!(
                                    "Subscribing to button characteristic {:?}",
                                    characteristic.uuid
                                );
                            }
                            peripheral.subscribe(&characteristic).await?;
                        } 
                        if characteristic.uuid == MOTION_CHARACTERISTIC_UUID
                            && characteristic.properties.contains(CharPropFlags::NOTIFY)
                        {
                            if verbose {
                                println!(
                                    "Subscribing to motion characteristic {:?}",
                                    characteristic.uuid
                                );
                            }
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
                    toio_connected += 1;
                    print_toio_connected(toio_connected);
                }
            }
            CentralEvent::DeviceDisconnected(bd_addr) => {
                println!("DeviceDisconnected: {:?}", bd_addr);
                toio_connected -= 1;
                print_toio_connected(toio_connected);
            }
            _ => {}
        }
    }

    Ok(())
}