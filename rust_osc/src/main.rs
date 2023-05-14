extern crate getopts;
use btleplug::api::CharPropFlags;
use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral, ScanFilter, WriteType,
};
use getopts::Options;

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
const TOIO_SERVICE_UUID:            Uuid = Uuid::from_u128(0x10B20100_5B3B_4571_9508_CF3EFCD7BBAE);
const POSITION_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x10B20101_5B3B_4571_9508_CF3EFCD7BBAE);
const MOTOR_CHARACTERISTIC_UUID:    Uuid = Uuid::from_u128(0x10B20102_5B3B_4571_9508_CF3EFCD7BBAE);
const LIGHT_CHARACTERISTIC_UUID:    Uuid = Uuid::from_u128(0x10B20103_5B3B_4571_9508_CF3EFCD7BBAE);
const SOUND_CHARACTERISTIC_UUID:    Uuid = Uuid::from_u128(0x10B20104_5B3B_4571_9508_CF3EFCD7BBAE);
const MOTION_CHARACTERISTIC_UUID:   Uuid = Uuid::from_u128(0x10B20106_5B3B_4571_9508_CF3EFCD7BBAE);
const BUTTON_CHARACTERISTIC_UUID:   Uuid = Uuid::from_u128(0x10B20107_5B3B_4571_9508_CF3EFCD7BBAE);
const BATTERY_CHARACTERISTIC_UUID:  Uuid = Uuid::from_u128(0x10B20108_5B3B_4571_9508_CF3EFCD7BBAE);

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_toio_connected(toio_connected: i32){
    if toio_connected == 1 {println!("1 peripheral connected");}
    else {println!("{} peripherals connected", toio_connected);}
}

fn return_toio_id(name: &str) -> &str{
    //list of toio ids
    const IDARR : [&str; 141] = [
        "Individual ID",  //TOIO Num
        "0",  // #1
        "j1c",  // #2
        "r81",  // #3
        "26E",  // #4
        "76t",  // #5
        "broken",  // #6
        "k5k",  // #7
        "h41",  // #8
        "0",  // #9
        "0",  // #10
        "0",  // #11
        "Q3A",  // #12
        "03a",  // #13
        "0",  // #14
        "K0m",  // #15
        "0",  // #16
        "0",  // #17
        "p8B",  // #18
        "91B",  // #19
        "p75",  // #20
        "G1E",  // #21
        "k2L",  // #22
        "b5p",  // #23
        "J6C",  // #24
        "0",  // #25
        "b8T",  // #26
        "b6A",  // #27
        "01c",  // #28
        "0",  // #29
        "0",  // #30
        "E2N",  // #31
        "G7t",  // #32
        "L6T",  // #33
        "C0E",  // #34
        "t79",  // #35
        "0",  // #36
        "0",  // #37
        "0",  // #38
        "0",  // #39
        "0",  // #40
        "M5p",  // #41
        "A4a",  // #42
        "M9J",  // #43
        "0",  // #44
        "T5m",  // #45
        "j1G",  // #46
        "40G",  // #47
        "L6n",  // #48
        "a3F",  // #49
        "J8d",  // #50
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
        "E7c",  // #79
        "P1B",  // #80
        "F2B",  // #81
        "0",  // #82
        "D5i",  // #83
        "m4Q",  // #84
        "m1k",  // #85
        "0",  // #86
        "k89",  // #87
        "D2K",  // #88
        "65r",  // #89
        "f3K",  // #90
        "13c",  // #91
        "e1a",  // #92
        "0",  // #93
        "e6e",  // #94
        "07F",  // #95
        "G74",  // #96
        "79H",  // #97
        "0",  // #98
        "i1M",  // #99
        "R3C",  // #100
        "D98",  // #101
        "0",  // #102
        "a66",  // #103
        "0",  // #104
        "E8T",  // #105
        "J8n",  // #106
        "N0b",  // #107
        "586",  // #108
        "p50",  // #109
        "c9k",  // #110
        "0",  // #111
        "0",  // #112
        "B1m",  // #113
        "h7E",  // #114
        "c05",  // #115
        "K20",  // #116
        "32D",  // #117
        "F19",  // #118
        "r4d",  // #119
        "D2F",  // #120
        "D0m",  // #121
        "m6B",  // #122
        "M0j",  // #123
        "Q8G",  // #124
        "0",  // #125
        "p7J",  // #126
        "t0H",  // #127
        "M5i",  // #128
        "j1L",  // #129
        "e7i",  // #130
        "T1E",  // #131
        "85i",  // #132
        "71H",  // #133
        "20H",  // #134
        "T9n",  // #135
        "58B",  // #136
        "J4R",  // #137
        "93N",  // #138
        "t0F",  // #139
        "M7G",  // #140
    ];
    match name.parse::<i32>() {
        Ok(n) => {
            let name = IDARR[n as usize];
            return name;
        },
        Err(_e) => {
            panic!("Error while reading the names");
        }
      }
}                

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut toio_connected = 0;
    let mut verbose = false;
    let mut search = false;
    let mut ordered = false;
    let mut ordernum = 0;

    //read command line arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "port", "set receiving port", "PORT_NUMBER");
    opts.optopt("r", "remote", "set remote port", "IP:PORT_NUMBER");
    opts.optopt("i", "host_id", "set host id number", "ID_NUMBER");
    opts.optopt("n", "names", "connect to those cubes only", "AAA,BBB,CCC");
    opts.optopt("a", "axlab_id", "connect to those cubes only (#1, #2, #3)", "1,2,3");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "print more connection details");
    opts.optflag("s", "search", "search for toios without specifying IDs");
    opts.optflag("o", "ordered", "search for toios in order of IDs");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}",f.to_string())}
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }
    if matches.opt_present("v") {
        verbose = true;
    }
    if matches.opt_present("s") {
        search = true;
    }

    let port_number = matches.opt_str("p").unwrap_or("3334".to_string());
    let listening_address = format!("0.0.0.0:{}", port_number);

    //this will be filled with the names to connect to *if needed*
    let possible_names = {
        let mut res = Vec::new();
        match matches.opt_str("n") {
            Some(names) => {
                for part in names.split(",") {
                    if part.len() == 3 {
                        res.push(format!("toio Core Cube-{}", part));
                    } else {
                        panic!("Error while reading the names");
                    }
                }
            },
            None => {}
        }
        match matches.opt_str("a") {
            Some(names) => {
                for part in names.split(",") {
                    res.push(format!("toio Core Cube-{}", return_toio_id(part)));
                }
            },
            None => {}
        }

        if !search && res.len() == 0 {
            panic!("Toio IDs must be specified unless in search mode");
        }

        if res.len() > 0 {
            ordernum = res.len();
            Some(res)
        } else {
            None
        }
    };

    if matches.opt_present("o") {
        ordered = true;
        ordernum = 1;
    }

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
        eprintln!(
            "Remote address {} is wrongly formatted, use IP:PORT (127.0.0.1:3333)",
            remote
        );
        return Ok(());
    };

    let host_id = matches
        .opt_str("i")
        .unwrap_or("0".to_string())
        .parse::<i32>()
        .unwrap_or(0);
    println!(
        "Sending messages to {} prefixed by {}",
        remote_addr, host_id
    );

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
    central
        .start_scan(ScanFilter {
            services: vec![TOIO_SERVICE_UUID],
        })
        .await?;

    if verbose {
        println!("Scanning for BTLE events on {:?}...",central);
    }
    else {
        println!("Scanning for BTLE events...");
    }  

    //Scan all the time
    while let Some(event) = events.next().await {
        //println!("event... {:?}", event);
        let mut device_candidate: Option<btleplug::platform::PeripheralId> = None;
        //Sometimes we miss the discovered event, so react on the DeviceUpdated as well
        match event {
            CentralEvent::DeviceUpdated(bd_addr) => {
                device_candidate = Some(bd_addr);
            }
            CentralEvent::DeviceDiscovered(bd_addr) => {
                device_candidate = Some(bd_addr);
            }
            CentralEvent::DeviceDisconnected(bd_addr) => {
                println!("DeviceDisconnected: {:?}", bd_addr);
                toio_connected -= 1;
                print_toio_connected(toio_connected);
            }
            _ => {}
        }

        
        //
        if let Some(bd_addr) = device_candidate {
            let peripheral = central.peripheral(&bd_addr).await.unwrap();

            let properties = peripheral.properties().await?.unwrap();
            let local_name = properties.local_name.unwrap_or("".to_string());

            let services = properties.services;
            let should_connect = if let Some(names) = &possible_names {
                if ordernum > names.len() {
                    ordernum -= 1;
                }
                names[0 ..ordernum].contains(&local_name)
            } else {
                services.contains(&TOIO_SERVICE_UUID)
            };


            //if (services.contains(&TOIO_SERVICE_UUID)) || possible_names.contains(&local_name)  {
            if should_connect {
                if ordered {
                    ordernum += 1;
                }
                //we kave a toio cube!
                let tx3 = tx.clone();
                if !(peripheral.is_connected().await?) {
                    println!("Device Connected: {}", local_name);
                    toio_connected  += 1;
                    print_toio_connected(toio_connected);

                    // Connect if we aren't already connected.
                    if let Err(err) = peripheral.connect().await {
                        eprintln!("Error connecting to peripheral, skipping: {}", err);
                        continue;
                    }
                    time::sleep(Duration::from_millis(200)).await;
                    // We should be connected now
                    let peripheral_id = peripheral.id();
                    if verbose {
                        println!("connecting with Peripheral ID: {:?}", peripheral_id);
                    }

                    //find the id for this cube
                    //let mut addr = addresses.lock().unwrap();
                    let mut storage_uuid = uuids.lock().unwrap();

                    //do we already know that address?
                    let id = if let Some(index) =
                        storage_uuid.iter().position(|a| a == &peripheral_id)
                    {
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
                                "/motorbasic" => {
                                    if message.args.len() == 5 {
                                        //we should have 5 args
                                        println!("Message received");
                                        let mut marg = [0; 5];
                                        for k in 0..5 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }

                                        let characteristic = Characteristic {
                                            uuid: MOTOR_CHARACTERISTIC_UUID,
                                            service_uuid: TOIO_SERVICE_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x01,                 //motor
                                            0x01,                 //left
                                            marg[1].abs() as u8,  //forwards or backwards
                                            marg[2].abs() as u8,  //speed
                                            0x02,                 //right
                                            marg[3].abs() as u8,  //forwards or backwards
                                            marg[4].abs() as u8,  //speed
                                        ];
                                        p2.write(&characteristic, &cmd, WriteType::WithoutResponse)
                                            .await
                                            .unwrap();
                                    } else {
                                        //error
                                    }
                                }
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
                                "/motortarget" => {
                                    if message.args.len() == 5 {
                                        //we should have 5 args
                                        let mut marg = [0; 5];
                                        for k in 0..5 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }
                                        let characteristic = Characteristic {
                                            uuid: MOTOR_CHARACTERISTIC_UUID,
                                            service_uuid: TOIO_SERVICE_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x03,                               //motor target
                                            0x00,                               //control distinction value
                                            0x05,                               //timeout period
                                            marg[1].abs() as u8,                //movement type
                                            0x50,                               //maximum motor speed
                                            0x00,                               //motor speed changes
                                            0x00,                               //reserved
                                            (marg[2].abs() & 0x00FF) as u8,     //x value of target
                                            ((marg[2].abs() & 0xFF00) >> 8) as u8, 
                                            (marg[3].abs() & 0x00FF) as u8,     //y value of target
                                            ((marg[3].abs() & 0xFF00) >> 8) as u8, 
                                            (marg[4].abs() & 0x00FF) as u8,     //θ value of target
                                            ((marg[4].abs() & 0xFF00) >> 8) as u8, 
                                        ];
                                        p2.write(&characteristic, &cmd, WriteType::WithoutResponse)
                                            .await
                                            .unwrap();
                                    } else if message.args.len() == 9 {
                                        //we should have 9 args
                                        let mut marg = [0; 9];
                                        for k in 0..9 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }
                                        let characteristic = Characteristic {
                                            uuid: MOTOR_CHARACTERISTIC_UUID,
                                            service_uuid: TOIO_SERVICE_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x03,                   //motor target
                                            marg[1].abs() as u8,    //control distinction value
                                            marg[2].abs() as u8,    //timeout period
                                            marg[3].abs() as u8,    //movement type
                                            marg[4].abs() as u8,    //maximum motor speed
                                            marg[5].abs() as u8,    //motor speed changes
                                            0x00,                   //reserved
                                            (marg[6].abs() & 0x00FF) as u8,     //x value of target
                                            ((marg[6].abs() & 0xFF00) >> 8) as u8, 
                                            (marg[7].abs() & 0x00FF) as u8,     //y value of target
                                            ((marg[7].abs() & 0xFF00) >> 8) as u8, 
                                            (marg[8].abs() & 0x00FF) as u8,     //θ value of target
                                            ((marg[8].abs() & 0xFF00) >> 8) as u8, 
                                        ];
                                        p2.write(&characteristic, &cmd, WriteType::WithoutResponse)
                                            .await
                                            .unwrap();
                                    } else {
                                        //error
                                    }
                                }
                                "/motoracceleration" => {
                                    if message.args.len() == 8 {
                                        //we should have 4 args
                                        let mut marg = [0; 8];
                                        for k in 0..8 {
                                            if let OscType::Int(i) = message.args[k] {
                                                marg[k] = i;
                                            }
                                        }
                                        let characteristic = Characteristic {
                                            uuid: MOTOR_CHARACTERISTIC_UUID,
                                            service_uuid: TOIO_SERVICE_UUID,
                                            properties: CharPropFlags::WRITE_WITHOUT_RESPONSE,
                                        };
                                        let cmd = vec![
                                            0x05,                               //motor control with acceleration
                                            marg[1].abs() as u8,                //translational speed
                                            marg[2].abs() as u8,                //acceleration
                                            (marg[3].abs() >> 8) as u8,         //rotational velocity
                                            (marg[3].abs() & 0x00FF) as u8, 
                                            marg[4].abs() as u8,                //rotational direction
                                            marg[5].abs() as u8,                //direction
                                            marg[6].abs() as u8,                //Priority designation
                                            marg[7].abs() as u8,                //duration
                                        ];
                                        p2.write(&characteristic, &cmd, WriteType::WithoutResponse)
                                            .await
                                            .unwrap();
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
                                "/magnetic" => {
                                    let characteristic = Characteristic {
                                        uuid: MOTION_CHARACTERISTIC_UUID,
                                        service_uuid: TOIO_SERVICE_UUID,
                                        properties: CharPropFlags::WRITE,
                                    };
                                    let cmd = vec![
                                        0x82
                                    ];
                                    //println!("{:?}", cmd);
                                    p2.write(&characteristic, &cmd, WriteType::WithResponse)
                                        .await
                                        .unwrap();
                                }
                                "/postureeuler" => {
                                    let characteristic = Characteristic {
                                        uuid: MOTION_CHARACTERISTIC_UUID,
                                        service_uuid: TOIO_SERVICE_UUID,
                                        properties: CharPropFlags::WRITE,
                                    };
                                    let cmd = vec![
                                        0x83,
                                        0x01
                                    ];
                                    //println!("{:?}", cmd);
                                    p2.write(&characteristic, &cmd, WriteType::WithResponse)
                                        .await
                                        .unwrap();
                                }
                                "/posturequaternion" => {
                                    let characteristic = Characteristic {
                                        uuid: MOTION_CHARACTERISTIC_UUID,
                                        service_uuid: TOIO_SERVICE_UUID,
                                        properties: CharPropFlags::WRITE,
                                    };
                                    let cmd = vec![
                                        0x83,
                                        0x02
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

                    //let is_connected = peripheral.is_connected().await?;
                    //println!("Peripheral {} connected: {}", address, is_connected);

                    //println!("Discovering peripheral characteristics...");
                    peripheral.discover_services().await.unwrap();
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
                        if characteristic.uuid == BATTERY_CHARACTERISTIC_UUID
                        && characteristic.properties.contains(CharPropFlags::NOTIFY)
                        {
                            if verbose {
                                println!(
                                    "Subscribing to battery characteristic {:?}",
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
                                }
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
                                }
                                MOTION_CHARACTERISTIC_UUID => {
                                    if data.value[0] == 0x01 {
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
                                    } else if data.value[0] == 0x02 {
                                        let state = data.value[1];
                                        let strength = data.value[2];
                                        let forcex = data.value[3];
                                        let forcey = data.value[4];
                                        let forcez = data.value[5];

                                        let msg = encoder::encode(&OscPacket::Message(OscMessage {
                                            addr: "/magnetic".to_string(),
                                            args: vec![
                                                OscType::Int(host_id),
                                                OscType::Int(id as i32),
                                                OscType::Int(state as i32),
                                                OscType::Int(strength as i32),
                                                OscType::Int(forcex as i32),
                                                OscType::Int(forcey as i32),
                                                OscType::Int(forcez as i32) 
                                            ],
                                        }))
                                        .unwrap();
    
                                        tx3.send((msg, remote_addr)).await.unwrap();
                                    } else if data.value[0] == 0x03 {
                                        if data.value[1] == 0x01 {
                                            let roll = data.value[2];
                                            let pitch = data.value[2];
                                            let yaw = data.value[3];

                                            let msg = encoder::encode(&OscPacket::Message(OscMessage {
                                                addr: "/postureeuler".to_string(),
                                                args: vec![
                                                    OscType::Int(host_id),
                                                    OscType::Int(id as i32),
                                                    OscType::Int(roll as i32),
                                                    OscType::Int(pitch as i32),
                                                    OscType::Int(yaw as i32)
                                                ],
                                            }))
                                            .unwrap();
        
                                            tx3.send((msg, remote_addr)).await.unwrap();
                                        } else {
                                            let w = data.value[2];
                                            let x = data.value[2];
                                            let y = data.value[3];
                                            let z = data.value[3];

                                            let msg = encoder::encode(&OscPacket::Message(OscMessage {
                                                addr: "/posturequaternion".to_string(),
                                                args: vec![
                                                    OscType::Int(host_id),
                                                    OscType::Int(id as i32),
                                                    OscType::Int(w as i32),
                                                    OscType::Int(x as i32),
                                                    OscType::Int(y as i32),
                                                    OscType::Int(z as i32)
                                                ],
                                            }))
                                            .unwrap();
        
                                            tx3.send((msg, remote_addr)).await.unwrap();
                                        }
                                    }
                                }
                                BATTERY_CHARACTERISTIC_UUID => {
                                    let battery = data.value[0];
                                    let msg = encoder::encode(&OscPacket::Message(OscMessage {
                                        addr: "/battery".to_string(),
                                        args: vec![
                                            OscType::Int(host_id),
                                            OscType::Int(id as i32),
                                            OscType::Int(battery as i32),
                                        ],
                                    }))
                                    .unwrap();

                                    tx3.send((msg, remote_addr)).await.unwrap();
                                }
                                _ => {}
                            }
                        }
                    });
                }
            } else {
                // println!("Device Found: {}", local_name);
            }
        }
    }

    Ok(())
}
