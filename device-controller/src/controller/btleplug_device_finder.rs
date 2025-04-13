use crate::peripheral::btleplug::{BtleplugReceiver, BtleplugWriter};
use btleplug::api::{
    Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter,
    ValueNotification,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::Stream;
use std::error::Error;
use std::pin::Pin;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time;
use uuid::Uuid;

pub async fn get_device() -> (BtleplugReceiver, BtleplugWriter) {
    let Ok(manager) = Manager::new().await else {
        panic!("No adapters found");
    };
    let Ok(adapter_list) = manager.adapters().await else {
        panic!("No adapters found");
    };
    if adapter_list.is_empty() {
        panic!("No Bluetooth adapters found");
    }

    let device = find_device(adapter_list).await;
    let notifications = subscribe_to_notifications(&device).await.unwrap();
    let device_writer = get_write_characteristic(&device).await.unwrap();

    let reader = BtleplugReceiver::new(notifications.unwrap());
    let writer = BtleplugWriter::new(device, device_writer);

    (reader, writer)
}
async fn find_device(adapter_list: Vec<Adapter>) -> Peripheral {
    let mut tasks = JoinSet::new();
    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        let _ = tasks.spawn(find_device_from_adapter(adapter.clone()));
    }

    let p = tasks.join_next().await.unwrap().unwrap();
    println!("Found device");
    p
}

async fn find_device_from_adapter(adapter: Adapter) -> Peripheral {
    adapter
        .start_scan(ScanFilter::default())
        .await
        .expect("Can't scan BLE adapter for connected devices...");

    loop {
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await.unwrap();

        // All peripheral devices in range.
        for peripheral in peripherals.iter() {
            if check_peripheral(peripheral).await {
                return peripheral.clone();
            }
        }
    }
}

async fn check_peripheral(peripheral: &Peripheral) -> bool {
    let properties = peripheral.properties().await.unwrap();
    let is_connected = peripheral.is_connected().await.unwrap();
    let local_name = properties
        .unwrap()
        .local_name
        .unwrap_or(String::from("(peripheral name unknown)"));
    // TODO: Logging
    /*info!(
        "Peripheral {:?} is connected: {:?}",
        &local_name, is_connected
    );*/
    // Check if it's the peripheral we want.
    if is_relevant_name(local_name.as_str()) {
        // TODO: Logging
        /*info!("Found matching peripheral {:?}...", &local_name);*/
        if !is_connected {
            // Connect if we aren't already connected.
            if let Err(_) = peripheral.connect().await {
                // TODO: Logging
                /*warn!("Error connecting to peripheral, skipping: {}", err);*/
                return false;
            }
        }
        let is_connected = peripheral.is_connected().await.unwrap();
        // TODO: Logging
        /*info!(
            "Now connected ({:?}) to peripheral {:?}.",
            is_connected, &local_name
        );*/
        is_connected && has_required_characteristics(&peripheral).await
    } else {
        // TODO: Logging
        //info!("Skipping unknown peripheral {:?}", peripheral);
        false
    }
}

const THERMOPRO_NAME: &str = "Thermopro";

/// UUID of the characteristic for which we should subscribe to notifications.
const WRITE_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x1086fff1_3343_4817_8bb2_b32206336ce8);
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x1086fff2_3343_4817_8bb2_b32206336ce8);

pub fn is_relevant_name(name: &str) -> bool {
    name == THERMOPRO_NAME
}

pub async fn has_required_characteristics(device: &Peripheral) -> bool {
    // TODO: Logging
    //info!("Discover peripheral services...");
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

    // TODO: Logging
    //info!("Checking characteristic {:?}", notify_characteristic);
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

    // TODO: Logging
    //info!("Checking characteristic {:?}", notify_characteristic);
    // Subscribe to notifications from the characteristic with the selected
    // UUID.
    if !is_notify_characteristic(notify_characteristic) {
        return Err("Bad characteristics".into());
    }
    // TODO: Logging
    /*info!(
        "Subscribing to characteristic {:?}",
        notify_characteristic.uuid
    );*/
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
