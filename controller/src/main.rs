mod thermopro;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::thermopro::{has_required_characteristics, is_relevant_name, run_for_device};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::io::Write;
use std::io;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time;

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
        print!(".");
        io::stdout().flush().expect("Can't flush stdout");
    }
}

async fn check_peripheral(peripheral: &Peripheral) -> bool {
    let properties = peripheral.properties().await.unwrap();
    let is_connected = peripheral.is_connected().await.unwrap();
    let local_name = properties
        .unwrap()
        .local_name
        .unwrap_or(String::from("(peripheral name unknown)"));
    info!(
        "Peripheral {:?} is connected: {:?}",
        &local_name, is_connected
    );
    // Check if it's the peripheral we want.
    if is_relevant_name(local_name.as_str()) {
        info!("Found matching peripheral {:?}...", &local_name);
        if !is_connected {
            // Connect if we aren't already connected.
            if let Err(err) = peripheral.connect().await {
                warn!("Error connecting to peripheral, skipping: {}", err);
                return false;
            }
        }
        let is_connected = peripheral.is_connected().await.unwrap();
        info!(
            "Now connected ({:?}) to peripheral {:?}.",
            is_connected, &local_name
        );
        is_connected && has_required_characteristics(&peripheral).await
    } else {
        info!("Skipping unknown peripheral {:?}", peripheral);
        false
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        error!("No Bluetooth adapters found");
    }

    let device = find_device(adapter_list).await;
    run_for_device(device).await?;

    Ok(())
}
