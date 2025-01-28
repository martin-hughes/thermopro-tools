use btleplug::api::{CharPropFlags, Characteristic, Peripheral as _, ValueNotification};
use btleplug::platform::Peripheral;
use futures::Stream;
use std::error::Error;
use std::pin::Pin;
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
