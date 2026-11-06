use crate::{cmd::*, constants::*, debugPrint, or_na, updates::*};
use btleplug::{
    api::{
        Central, CentralEvent, CharPropFlags, Characteristic, Manager as _, Peripheral, ScanFilter,
    },
    platform,
    platform::{Adapter, Manager},
};
use futures::{future::Either, stream::StreamExt};
use std::{error::Error, sync::Arc, time::SystemTime, vec};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{error::TrySendError, Receiver, Sender};
use tokio::{sync::RwLock, task::JoinHandle};

// How many commands for a toio can be queued up at once.
const COMMAND_QUEUE_SIZE: usize = 32;

pub struct ToioScanner {
    central: Adapter,
    ordered: bool,
    filter: Option<Vec<String>>,
    connected: Vec<String>,
}

pub struct ToioReceiver {
    receiver: Receiver<Either<ToioPeripheral, platform::PeripheralId>>,
}

#[derive(Clone)]
pub struct ToioPeripheral {
    pub name: String,
    peripheral: platform::Peripheral,
    pub peripheral_id: platform::PeripheralId,
}

#[derive(Clone)]
pub struct ToioLocation {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone)]
pub struct ToioState {
    pub battery: Option<u8>,
    pub last_update: Option<SystemTime>,
    pub last_command: Option<SystemTime>,
    pub curr_loc: Option<ToioLocation>,
}

pub struct Toio {
    pub toio: ToioPeripheral,
    pub name: String,
    pub id: String,
    pub connected: bool,
    pub channel: Option<JoinHandle<()>>,
    pub state: Arc<RwLock<ToioState>>,
    command_queue: Option<Sender<Command>>,
    writer: Option<JoinHandle<()>>,
}

impl ToioState {
    pub fn new() -> Self {
        ToioState {
            battery: None,
            last_update: None,
            last_command: None,
            curr_loc: None,
        }
    }
}

impl ToioScanner {
    pub async fn new_search() -> Result<ToioScanner, Box<dyn Error>> {
        let central = Manager::new()
            .await?
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
            connected: vec![],
        })
    }

    pub async fn new_filtered(
        ordered: bool,
        filter: Vec<usize>,
    ) -> Result<ToioScanner, Box<dyn Error>> {
        let central = Manager::new()
            .await?
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
            connected: vec![],
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

        // Discovery Async Task
        let mut toio_filter = self.filter.clone();
        let mut connected = self.connected.clone();
        let ordered = self.ordered;
        tokio::spawn(async move {
            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceDiscovered(id)
                    | CentralEvent::ServicesAdvertisement { id, services: _ } => {
                        let peripheral = central.peripheral(&id).await.unwrap();

                        if ordered {
                            if let Some(ref mut new_filter) = toio_filter.clone() {
                                let filter = if new_filter.len() > 0 {
                                    [connected.clone(), vec![new_filter[0].clone()]].concat()
                                } else {
                                    connected.clone()
                                };

                                let connect_success =
                                    Self::try_connect(peripheral, &tx, Some(filter)).await;

                                if let Some(toio_name) = connect_success {
                                    // if filtered order, update filter on successful connection
                                    let update_filter = !new_filter.is_empty()
                                        && toio_name == new_filter[0].to_string();

                                    if update_filter {
                                        new_filter.remove(0);
                                    }

                                    toio_filter = Some(new_filter.to_vec());
                                    connected.push(toio_name);
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
    ) -> Option<String> {
        if let Some(properties) = peripheral.properties().await.unwrap() {
            let full_name = properties.local_name.unwrap_or("".to_string());
            if peripheral.is_connected().await.unwrap() || !full_name.contains(&"toio") {
                return None;
            }

            let toio_name = get_toio_name(&full_name);
            if let Some(filter_list) = filter {
                if !filter_list.contains(&toio_name) {
                    return None;
                }
            }

            let toio_peripheral = ToioPeripheral::new(toio_name.clone(), peripheral);
            if !toio_peripheral.connect().await {
                return None;
            }

            tx.send(Either::Left(toio_peripheral)).await.unwrap();
            return Some(toio_name);
        }

        return None;
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
                debugPrint!("Error subscribing to characteristic, skipping: {}", err);
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
                if let Some(update) = parse_update(event) {
                    if tx.send(update).await.is_err() {
                        // Channel closed, exit gracefully
                        break;
                    }
                }
            }
        });

        return Ok(Updates::new(rx));
    }

    pub async fn send_command(&self, command: Command) {
        self.write(build_command_data(command)).await;
    }

    pub async fn write(&self, data: CommandData) {
        let characteristic = Characteristic {
            uuid: data.uuid,
            service_uuid: SERVICE,
            properties: data.response_flag,
        };

        let _ = self
            .peripheral
            .write(&characteristic, &data.cmd, data.response_type)
            .await;
    }
}

impl Toio {
    pub fn connect(
        toio: ToioPeripheral,
        channel: JoinHandle<()>,
        state: Arc<RwLock<ToioState>>,
    ) -> Toio {
        // create bluetooth writer task with queue per toio
        let (commands, writer) = Self::spawn_writer(toio.clone());

        // parse toio id
        let id = or_na!(Some(id) = get_toio_id(&toio.name) => format!("{}", id));

        return Toio {
            name: toio.name.clone(),
            id: id,
            connected: true,
            channel: Some(channel),
            state,
            command_queue: Some(commands),
            writer: Some(writer),
            toio,
        };
    }

    pub fn queue_command(&self, command: Command) -> Result<(), TrySendError<Command>> {
        match &self.command_queue {
            Some(commands) => commands.try_send(command),
            None => Err(TrySendError::Closed(command)),
        }
    }

    pub fn update_last_command(&self) {
        let state = self.state.clone();
        tokio::spawn(async move {
            let mut state_write = state.write().await;
            state_write.last_command = Some(SystemTime::now());
        });
    }

    pub fn disconnect(&mut self) {
        self.stop_tasks();
        self.connected = false;
    }

    pub fn reconnect(
        &mut self,
        toio: ToioPeripheral,
        channel: JoinHandle<()>,
        state: Arc<RwLock<ToioState>>,
    ) {
        self.stop_tasks();

        let (commands, writer) = Self::spawn_writer(toio.clone());
        self.toio = toio;
        self.state = state;
        self.command_queue = Some(commands);
        self.writer = Some(writer);
        self.channel = Some(channel);
        self.connected = true;
    }

    fn stop_tasks(&mut self) {
        if let Some(channel) = self.channel.take() {
            channel.abort();
        }
        if let Some(writer) = self.writer.take() {
            writer.abort();
        }
        self.command_queue = None;
    }

    fn spawn_writer(peripheral: ToioPeripheral) -> (Sender<Command>, JoinHandle<()>) {
        let (commands, mut queue) = mpsc::channel::<Command>(COMMAND_QUEUE_SIZE);
        let writer = tokio::spawn(async move {
            while let Some(command) = queue.recv().await {
                peripheral.send_command(command).await;
            }
        });
        return (commands, writer);
    }
}

pub fn get_toio_name(name: &str) -> String {
    let name = IDARR
        .iter()
        .find(|r| toio_name_variants(r).iter().any(|x| x == name));

    or_na!(Some(toio_name) = name => format!("{}", toio_name))
}

fn toio_name_variants(id: &str) -> Vec<String> {
    vec![
        format!("toio Core Cube-{}", id), // old firmware
        format!("toio-{} (AxLab-KN:0A.001)", id),
        format!("toio-{} (AxLab-KN", id),
        format!("toio-{}", id), // new firmware
        format!("toio-{} (toio Core Cube)", id),
    ]
}

// return the toio ID that corresponds to a specific toio
fn get_toio_id(name: &str) -> Option<usize> {
    return IDARR.iter().position(|&r| r == name);
}
