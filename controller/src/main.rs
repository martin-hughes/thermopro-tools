use futures::StreamExt;
mod checksum;
mod command;
mod commands;
mod convert_commands;
mod convert_notifications;
mod device;
mod generate_commands;
mod notification;
mod peripheral;
mod receive_notifications;
mod transfer;
mod transfer_log;
mod ui;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::convert_commands::convert_commands;
use crate::convert_notifications::convert_notifications;
use crate::device::Device;
use crate::peripheral::{
    get_write_characteristic, has_required_characteristics, is_relevant_name,
    subscribe_to_notifications,
};
use crate::receive_notifications::receive_notifications;

use crate::device::DeviceState::Connected;
use crate::device::TempMode;
use crate::generate_commands::Commander;
use crate::transfer_log::{TransferLog, Transfer};
use crate::ui::draw_ui;
use crate::ui::ui_state::{UiCommands, UiState};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use bytes::Bytes;
use crossterm::event::EventStream;
use std::error::Error;
use std::io;
use std::io::Write;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time;
use tokio::time::timeout;
use crate::command::Command::Connect;
use crate::notification::Notification;

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

    let mut notifications = subscribe_to_notifications(&device).await?.unwrap();
    let device_writer = get_write_characteristic(&device).await?;

    // The gist of all these channels and async tasks is to form two simple pipelines:
    // Bluetooth peripheral sends Bytes,
    // Bytes are converted to Notification,
    // `receive_notifications` updates the state.
    //
    // `commander` receives requests (usually via the UI), and sends out Commands as needed,
    // Commands are converted to Bytes,
    // Bytes sent back to peripheral.

    const CHANNEL_SIZE: usize = 10;
    let (notification_tx, notification_rx) = channel(CHANNEL_SIZE);
    let (controller_in_tx, controller_in_rx) = channel(CHANNEL_SIZE);
    let (controller_out_tx, controller_out_rx) = channel(CHANNEL_SIZE);
    let (command_tx, mut command_rx) = channel(CHANNEL_SIZE);
    let (update_tx, mut update_rx) = channel(CHANNEL_SIZE);

    let mut tasks = JoinSet::new();
    let controller_device = Device::new();
    let transfers = TransferLog::new();
    let ui_state = Arc::new(Mutex::new(UiState::default()));

    let _ = tasks.spawn(convert_notifications(notification_rx, controller_in_tx));
    let _ = tasks.spawn(receive_notifications(
        controller_device.clone(),
        controller_in_rx,
        update_tx,
    ));
    let _ = tasks.spawn(convert_commands(controller_out_rx, command_tx));

    // Receiving from the bluetooth peripheral
    let transfers_a = transfers.clone();
    let _ = tasks.spawn(async move {
        while let Some(data) = timeout(Duration::from_secs(4), notifications.next())
            .await
            .unwrap()
        {
            let d: Bytes = data.value.into();
            let r: Result<Notification, _> = d.clone().try_into();
            if let Ok(rt) = r {
                transfers_a.push_transfer(Transfer::Notification(rt.clone()));
                if (notification_tx.send(d).await).is_err() {
                    return;
                };
            }
        }
    });

    // Sending to the bluetooth peripheral
    let transfers_b = transfers.clone();
    let _ = tasks.spawn(async move {
        while let Some(data) = command_rx.recv().await {
            transfers_b.push_transfer(Transfer::Command(Connect));
            if device
                .write(
                    &device_writer,
                    data.iter().as_slice(),
                    WriteType::WithoutResponse,
                )
                .await
                .is_err()
            {
                return;
            };
        }
    });

    let commander = Commander::new(controller_out_tx);
    commander.startup().await;

    let keyboard_ui_state = ui_state.clone();
    let keyboard_device = controller_device.clone();
    let _ = tasks.spawn(async move {
        let mut reader = EventStream::new();
        loop {
            let maybe_event = reader.next().await;
            match maybe_event {
                None => return,
                Some(Ok(event)) => {
                    if let Some(cmd) = keyboard_ui_state.lock().await.handle_event(event) {
                        match cmd {
                            UiCommands::Quit => return,
                            UiCommands::ToggleCelsius => {
                                if let Connected(dcs) = keyboard_device.get_state() {
                                    let new_mode = if dcs.celsius {
                                        TempMode::Fahrenheit
                                    } else {
                                        TempMode::Celsius
                                    };
                                    commander.set_temp_mode(new_mode).await;
                                }
                            }
                        }
                    }
                }
                Some(Err(_)) => return,
            }
        }
    });

    // Draw a UI and update as needed
    let _ = tasks.spawn(async move {
        let mut terminal = ratatui::init();
        let d = controller_device;
        let start = Instant::now();

        while (update_rx.recv().await).is_some() {
            draw_ui(
                &mut terminal,
                d.get_state(),
                transfers.get_transfers(),
                start.elapsed().as_millis() % 1500 > 750,
            );
        }
    });

    // If any task fails, bail out.
    tasks.join_next().await;
    ratatui::restore();

    // TODO: Probably always sending `Ok` isn't the best response...
    Ok(())
}
