use crate::peripheral::btleplug::{BtleplugReceiver, BtleplugWriter};
use btleplug::api::{
    Central, CharPropFlags, Characteristic, Manager as _, Peripheral as _, ScanFilter,
    ValueNotification,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::Stream;
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::pin::Pin;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time;
use uuid::Uuid;

fn log_err_and_ret(s: &'static str) -> &'static str {
    error!("{}", s);
    s
}

pub async fn get_device() -> Result<(BtleplugReceiver, BtleplugWriter), Box<dyn Error>> {
    let Ok(manager) = Manager::new().await else {
        return Err(log_err_and_ret("No adapters found").into());
    };
    let Ok(adapter_list) = manager.adapters().await else {
        return Err(log_err_and_ret("No adapters found").into());
    };
    if adapter_list.is_empty() {
        return Err(log_err_and_ret("No Bluetooth adapters found").into());
    }

    let device = find_device(adapter_list)
        .await
        .ok_or("Device find failed")?;
    let notifications = subscribe_to_notifications(&device).await?;
    let device_writer = get_write_characteristic(&device).await?;

    let reader = BtleplugReceiver::new(notifications?);
    let writer = BtleplugWriter::new(device, device_writer);

    Ok((reader, writer))
}

// Returning None here implies an actual error has occurred, as we will wait forever to find a device.
async fn find_device(adapter_list: Vec<Adapter>) -> Option<Peripheral> {
    let mut tasks = JoinSet::new();

    // TODO: There's a bug here... we start scanning on all adapters, but if a scan *fails* (as opposed to just not
    // finding a device) then it will return - which will trigger `tasks.join_next` to return a None result, and we'll
    // assume the whole device scan has failed.

    info!("Starting scan...");
    for adapter in adapter_list.iter() {
        let _ = tasks.spawn(find_device_from_adapter(adapter.clone()));
    }
    debug!("All adapter scan tasks spawned");

    match tasks.join_next().await {
        Some(Ok(Some(p))) => {
            info!("Found device");
            Some(p)
        }
        _ => {
            warn!("Failed to find device");
            None
        }
    }
}

async fn find_device_from_adapter(adapter: Adapter) -> Option<Peripheral> {
    let adapter_name = adapter
        .adapter_info()
        .await
        .unwrap_or("<Unknown>".to_string());
    debug!("Scanning on adapter {:?}", adapter_name);

    if adapter.start_scan(ScanFilter::default()).await.is_err() {
        warn!(
            "Can't scan adapter \"{:?}\" for connected devices...",
            adapter
        );
        return None;
    }

    loop {
        // TODO: Is it necessary to sleep at the beginning here?
        // I can't remember if there was a stability issue from not sleeping.
        time::sleep(Duration::from_secs(2)).await;
        let Ok(peripherals) = adapter.peripherals().await else {
            warn!(
                "Couldn't get peripherals list from adapter \"{:?}\"",
                adapter_name
            );
            return None;
        };

        // `peripherals` is all peripheral devices in range at this time.
        for peripheral in peripherals.iter() {
            if check_peripheral(peripheral).await {
                debug!("Found acceptable device on adapter {:?}", adapter_name);
                return Some(peripheral.clone());
            }
        }
    }
}

async fn check_peripheral(peripheral: &Peripheral) -> bool {
    let Ok(Some(properties)) = peripheral.properties().await else {
        warn!(
            "Could not retrieve properties from BLE device \"{:?}\"",
            peripheral
        );
        return false;
    };

    // If there's an error here, I think it's OK to assume the device isn't connected. We attempt to connect one
    // more time, lower down.
    let is_connected = peripheral.is_connected().await.unwrap_or(false);

    let local_name = properties
        .local_name
        .unwrap_or(String::from("(peripheral name unknown)"));
    debug!(
        "Checking peripheral {:?}. Connected? {:?}",
        &local_name, is_connected
    );
    // Check if it's the peripheral we want.
    if is_relevant_name(local_name.as_str()) {
        debug!("Peripheral name {:?} matches...", &local_name);
        if !is_connected {
            trace!("Not connected, attempting to connect");
            // Connect if we aren't already connected.
            if let Err(err) = peripheral.connect().await {
                warn!("Error connecting to peripheral, skipping: {}", err);
                return false;
            }
        }

        let is_connected = peripheral.is_connected().await.unwrap_or(false);

        if is_connected {
            trace!("Connected, checking characteristics");
            has_required_characteristics(peripheral).await
        } else {
            warn!("Peripheral was connected, now is not - skipping.");
            false
        }
    } else {
        debug!("Skipping unknown peripheral {:?}", peripheral);
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
    trace!("Discover peripheral services...");
    if device.discover_services().await.is_err() {
        warn!("Failed to discover peripheral services");
        return false;
    }

    let characteristics = device.characteristics();
    let Some(notify_characteristic) = characteristics
        .iter()
        .find(|c| c.uuid == NOTIFY_CHARACTERISTIC_UUID)
    else {
        debug!("Did not find appropriate notify characteristic");
        return false;
    };
    let Some(write_characteristic) = characteristics
        .iter()
        .find(|c| c.uuid == WRITE_CHARACTERISTIC_UUID)
    else {
        debug!("Did not find appropriate write characteristic");
        return false;
    };

    let r = is_notify_characteristic(notify_characteristic)
        && is_write_characteristic(write_characteristic);
    if !r {
        debug!("Characteristics did not have appropriate flags set");
    }
    r
}

fn is_notify_characteristic(characteristic: &Characteristic) -> bool {
    trace!("Checking notify characteristic: {:?}", characteristic);
    characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID
        && characteristic.properties.contains(CharPropFlags::NOTIFY)
}

fn is_write_characteristic(characteristic: &Characteristic) -> bool {
    trace!("Checking write characteristic: {:?}", characteristic);
    characteristic.uuid == WRITE_CHARACTERISTIC_UUID
        && characteristic.properties.contains(CharPropFlags::WRITE)
}

type BtleNotificationStream =
    btleplug::Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>>;
pub async fn subscribe_to_notifications(
    device: &Peripheral,
) -> Result<BtleNotificationStream, Box<dyn Error>> {
    // TODO: `discover_services` should already have been done, is it necessary to repeat it?

    device.discover_services().await?;
    let characteristics = device.characteristics();
    let notify_characteristic = characteristics
        .iter()
        .find(|c| c.uuid == NOTIFY_CHARACTERISTIC_UUID)
        .ok_or("Notify characteristic not found")?;

    // TODO: We should already have checked the characteristic, is it necessary to repeat it?
    trace!("Checking characteristic {:?}", notify_characteristic);
    // Subscribe to notifications from the characteristic with the selected
    // UUID.
    if !is_notify_characteristic(notify_characteristic) {
        warn!("Notify characteristic is not valid");
        return Err("Bad characteristics".into());
    }

    info!(
        "Subscribing to notify characteristic {:?}",
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
        .ok_or("Write characteristic not found")?;

    if !is_write_characteristic(write_characteristic) {
        return Err("Bad characteristics".into());
    }

    Ok(write_characteristic.clone())
}
