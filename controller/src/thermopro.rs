use btleplug::api::{CharPropFlags, Characteristic, Peripheral, WriteType};
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;

const THERMOPRO_NAME: &str = "Thermopro";

/// UUID of the characteristic for which we should subscribe to notifications.
const WRITE_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x1086fff1_3343_4817_8bb2_b32206336ce8);
const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x1086fff2_3343_4817_8bb2_b32206336ce8);

pub fn is_relevant(name: &str) -> bool {
    name == THERMOPRO_NAME
}

fn is_notify_characteristic(characteristic: &Characteristic) -> bool {
    characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
        && characteristic.properties.contains(CharPropFlags::NOTIFY)
}

fn is_write_characteristic(characteristic: &Characteristic) -> bool {
    characteristic.uuid == WRITE_CHARACTERISTIC_UUID
        && characteristic.properties.contains(CharPropFlags::WRITE)
}

pub async fn run_for_device<P: Peripheral>(device: &P) -> Result<(), Box<dyn Error>> {
    println!("Discover peripheral services...");
    device.discover_services().await?;
    let characteristics = device.characteristics();
    let notify_characteristic = characteristics
        .iter()
        .find(|c| c.uuid == NOTIFY_CHARACTERISTIC_UUID)
        .unwrap();
    let write_characteristic = characteristics
        .iter()
        .find(|c| c.uuid == WRITE_CHARACTERISTIC_UUID)
        .unwrap();

    println!("Checking characteristic {:?}", notify_characteristic);
    // Subscribe to notifications from the characteristic with the selected
    // UUID.
    if !is_notify_characteristic(notify_characteristic)
        || !is_write_characteristic(write_characteristic)
    {
        return Err("Bad characteristics".into());
    }

    println!(
        "Subscribing to characteristic {:?}",
        notify_characteristic.uuid
    );
    device.subscribe(&notify_characteristic).await?;
    const CMD: [u8; 12] = [0x01, 0x09, 0x70, 0x32, 0xe2, 0xc1, 0x79, 0x9d, 0xb4, 0xd1, 0xc7, 0xb1];

    device
        .write(write_characteristic, &CMD, WriteType::WithoutResponse)
        .await?;

    // Print the first 4 notifications received.
    let mut notification_stream = device.notifications().await?.take(4);
    // Process while the BLE connection is not broken or stopped.
    while let Some(data) = timeout(Duration::from_secs(4), notification_stream.next())
        .await
        .unwrap()
    {
        println!("Received data from [{:?}]: {:?}", data.uuid, data.value);
    }

    println!("Disconnecting from peripheral...");
    device.disconnect().await.unwrap();

    Ok(())
}
