use btleplug::api::{CharPropFlags, Characteristic, Peripheral as _, ValueNotification};
use btleplug::platform::Peripheral;
use bytes::Bytes;
use futures::Stream;
use std::error::Error;
use std::pin::Pin;
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

const THERMOPRO_NAME: &str = "Thermopro";

/// UUID of the characteristic for which we should subscribe to notifications.
const WRITE_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x1086fff1_3343_4817_8bb2_b32206336ce8);
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x1086fff2_3343_4817_8bb2_b32206336ce8);

pub fn is_relevant_name(name: &str) -> bool {
    name == THERMOPRO_NAME
}

pub async fn has_required_characteristics(device: &Peripheral) -> bool {
    info!("Discover peripheral services...");
    if device.discover_services().await.is_err() {
        return false;
    }

    let characteristics = device.characteristics();
    let notify_characteristic = characteristics
        .iter()
        .find(|c| c.uuid == NOTIFY_CHARACTERISTIC_UUID)
        .unwrap();
    let write_characteristic = characteristics
        .iter()
        .find(|c| c.uuid == WRITE_CHARACTERISTIC_UUID)
        .unwrap();

    info!("Checking characteristic {:?}", notify_characteristic);
    // Subscribe to notifications from the characteristic with the selected
    // UUID.
    is_notify_characteristic(notify_characteristic) && is_write_characteristic(write_characteristic)
}

fn is_notify_characteristic(characteristic: &Characteristic) -> bool {
    characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
        && characteristic.properties.contains(CharPropFlags::NOTIFY)
}

fn is_write_characteristic(characteristic: &Characteristic) -> bool {
    characteristic.uuid == WRITE_CHARACTERISTIC_UUID
        && characteristic.properties.contains(CharPropFlags::WRITE)
}

const CMD: [u8; 12] = [
    0x01, 0x09, 0x70, 0x32, 0xe2, 0xc1, 0x79, 0x9d, 0xb4, 0xd1, 0xc7, 0xb1,
];

type BtleNotificationStream =
    btleplug::Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>>;
pub async fn subscribe_to_notifications(
    device: &Peripheral,
) -> Result<BtleNotificationStream, Box<dyn Error>> {
    device.discover_services().await?;
    let characteristics = device.characteristics();
    let notify_characteristic = characteristics
        .iter()
        .find(|c| c.uuid == NOTIFY_CHARACTERISTIC_UUID)
        .unwrap();

    info!("Checking characteristic {:?}", notify_characteristic);
    // Subscribe to notifications from the characteristic with the selected
    // UUID.
    if !is_notify_characteristic(notify_characteristic) {
        return Err("Bad characteristics".into());
    }

    info!(
        "Subscribing to characteristic {:?}",
        notify_characteristic.uuid
    );
    device.subscribe(notify_characteristic).await?;

    Ok(device.notifications().await)
}

pub async fn get_write_characteristic(
    device: &Peripheral,
) -> Result<Characteristic, Box<dyn Error>> {
    device.discover_services().await?;
    let characteristics = device.characteristics();
    let write_characteristic = characteristics
        .iter()
        .find(|c| c.uuid == WRITE_CHARACTERISTIC_UUID)
        .unwrap();

    if !is_write_characteristic(write_characteristic) {
        return Err("Bad characteristics".into());
    }

    Ok(write_characteristic.clone())
}

pub enum Command {
    Connect,
}

#[derive(Debug)]
pub enum Notification {
    ConnectResponse,
    Temperatures,
}

impl TryFrom<Bytes> for Notification {
    type Error = &'static str;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value[0] {
            0x01 => Ok(Notification::ConnectResponse),
            0x30 => Ok(Notification::Temperatures),
            _ => Err("Invalid notification type"),
        }
    }
}

impl TryFrom<Command> for Bytes {
    type Error = &'static str;
    fn try_from(value: Command) -> Result<Self, Self::Error> {
        match value {
            Command::Connect => Ok(Bytes::from_static(&CMD)),
        }
    }
}

pub async fn controller(mut incoming: Receiver<Notification>, outgoing: Sender<Command>) {
    outgoing.send(Command::Connect).await.unwrap();
    loop {
        let n = incoming.recv().await.unwrap();
        println!("Received notification: {:?}", n);
    }
}

pub async fn convert_notifications(mut incoming: Receiver<Bytes>, outgoing: Sender<Notification>) {
    loop {
        let n = incoming.recv().await.unwrap();
        outgoing
            .send(Notification::try_from(n).unwrap())
            .await
            .unwrap();
    }
}

pub async fn convert_commands(mut incoming: Receiver<Command>, outgoing: Sender<Bytes>) {
    loop {
        let cmd = incoming.recv().await.unwrap();
        outgoing.send(Bytes::try_from(cmd).unwrap()).await.unwrap();
    }
}
