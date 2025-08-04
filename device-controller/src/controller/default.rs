#[cfg(feature = "dummy_device")]
use crate::controller::dummy_device_finder::get_device;

#[cfg(not(feature = "dummy_device"))]
use crate::controller::btleplug_device_finder::get_device;
use crate::controller::command_request::CommandRequest;
use crate::model::device::{TP25State, TemperatureMode};
use crate::model::probe::ProbeIdx::*;
use crate::model::probe::{AlarmState, AlarmThreshold, ProbeIdx};
use crate::peripheral::command::{
    build_alarm_ack_cmd, build_custom_cmd, build_report_profile_cmd, build_set_profile_cmd,
    build_set_temp_mode_command, build_startup_command, Command,
};
use crate::peripheral::interface::{TP25Receiver, TP25Writer};
use crate::peripheral::notification::{Decoded, Notification, ProbeProfileData, TemperatureData};
use crate::peripheral::transfer::Transfer;
use log::{debug, info};
use std::sync::Arc;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

pub struct Controller {}

type ProtectedDeviceState = Arc<Mutex<TP25State>>;

impl Controller {
    pub async fn run(
        state_update_tx: Sender<TP25State>,
        transfer_tx: Sender<Transfer>,
        command_request_rx: Receiver<CommandRequest>,
    ) {
        info!("Starting Controller");
        let device_state = TP25State {
            connected: false,
            ..TP25State::default()
        };
        let protected_device_state = Arc::new(Mutex::new(device_state));
        let saved_cmd_rqst_rx = Arc::new(Mutex::new(command_request_rx));

        loop {
            if state_update_tx
                .send(get_device_state(&protected_device_state).await)
                .await
                .is_err()
            {
                debug!("Controller stopping as unable to send state update");
                return;
            }

            let Ok((peripheral_rx, peripheral_tx)) = get_device().await else {
                // `get_device` only errors for unrecoverable errors such as no Bluetooth adapters.
                // If it merely can't find a decice, it keeps waiting. Therefore an error return
                // means there's no point continuing.
                debug!("Device search failed, so controller exiting");
                return;
            };
            {
                protected_device_state.lock().await.connected = true;
            }

            Self::handle_one_connection(
                peripheral_rx,
                peripheral_tx,
                &protected_device_state,
                &state_update_tx,
                &transfer_tx,
                saved_cmd_rqst_rx.clone(),
            )
            .await;
            {
                protected_device_state.lock().await.connected = false;
            }
        }
    }

    async fn handle_one_connection(
        mut peripheral_rx: impl TP25Receiver + 'static,
        peripheral_tx: impl TP25Writer + 'static + Sync,
        protected_device_state: &ProtectedDeviceState,
        state_update_tx: &Sender<TP25State>,
        transfer_tx: &Sender<Transfer>,
        command_request_rx: Arc<Mutex<Receiver<CommandRequest>>>,
    ) {
        let protected_device_state = protected_device_state.clone();
        let protected_device_state_b = protected_device_state.clone();
        let transfer_tx = transfer_tx.clone();
        let transfer_tx_b = transfer_tx.clone();
        let state_update_tx = state_update_tx.clone();

        let receiver_task = tokio::spawn(async move {
            loop {
                let Some(n) = peripheral_rx.get_notification().await else {
                    // If we stop receiving notifications then just exit.
                    // TODO: Update the screen to say disconnected, try to reconnect, etc.
                    return;
                };
                let device_state = &mut protected_device_state.lock().await;
                if transfer_tx
                    .send(Transfer::Notification(n.clone()))
                    .await
                    .is_err()
                {
                    return;
                }
                handle_notification(n, &state_update_tx, device_state).await;
            }
        });

        //let mut command_request_rx = command_request_rx.
        let ui_task = tokio::spawn(async move {
            // TODO: The next line doesn't really sit well here, conceptually.
            if send_startup_cmd(&peripheral_tx, &transfer_tx_b)
                .await
                .is_err()
            {
                return;
            };

            let mut command_request_rx = command_request_rx.lock().await;

            loop {
                let Some(r) = command_request_rx.recv().await else {
                    return;
                };
                if handle_command_request(
                    r,
                    &peripheral_tx,
                    &transfer_tx_b,
                    get_device_state(&protected_device_state_b).await,
                )
                .await
                .is_err()
                {
                    return;
                }
            }
        });

        select! { _ = receiver_task => {}, _ = ui_task => {}};
    }
}
async fn handle_notification(
    notification: Notification,
    ui_cmd_tx: &Sender<TP25State>,
    device_state: &mut TP25State,
) {
    update_model_from_notification(&notification, device_state);
    send_state_update(ui_cmd_tx, device_state.clone()).await;
}

fn update_model_from_notification(
    notification: &Notification,
    device_state: &mut TP25State,
) -> bool {
    match &notification.decoded {
        Decoded::Unknown => false,
        Decoded::Startup => false,
        Decoded::SetTempMode => false,
        Decoded::ReportProbeProfile(profile_data) => {
            handle_probe_profile(profile_data, device_state);
            true
        }
        Decoded::Temperatures(temps) => {
            handle_temps(temps, device_state);
            true
        }
        Decoded::SetProbeProfile => false,
        Decoded::Error => false,
    }
}

async fn handle_command_request(
    command_request: CommandRequest,
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
    device_state: TP25State,
) -> btleplug::Result<()> {
    match command_request {
        CommandRequest::ToggleTempMode => {
            let mode = match device_state.temperature_mode {
                Some(TemperatureMode::Celsius) => TemperatureMode::Fahrenheit,
                _ => TemperatureMode::Celsius,
            };
            send_temp_mode_cmd(mode, device, transfer_tx).await?;
        }
        CommandRequest::SetTempMode(celsius) => {
            send_temp_mode_cmd(
                if celsius {
                    TemperatureMode::Celsius
                } else {
                    TemperatureMode::Fahrenheit
                },
                device,
                transfer_tx,
            )
            .await?;
        }
        CommandRequest::ReportAllProfiles => {
            send_query_profile(device, transfer_tx, Probe1).await?;
            send_query_profile(device, transfer_tx, Probe2).await?;
            send_query_profile(device, transfer_tx, Probe3).await?;
            send_query_profile(device, transfer_tx, Probe4).await?;
        }
        CommandRequest::ReportProfile(idx) => {
            send_query_profile(device, transfer_tx, idx).await?;
        }
        CommandRequest::SetProfile(idx, profile) => {
            send_set_profile(device, transfer_tx, idx, profile).await?;
        }
        CommandRequest::AckAlarm => {
            send_alarm_ack_cmd(device, transfer_tx).await?;
        }
        CommandRequest::CustomCommand(bytes) => {
            send_custom_cmd(device, transfer_tx, bytes).await?;
        }
    };
    Ok(())
}

async fn send_startup_cmd(
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
) -> btleplug::Result<()> {
    send_cmd(device, transfer_tx, build_startup_command()).await
}

async fn send_temp_mode_cmd(
    mode: TemperatureMode,
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
) -> btleplug::Result<()> {
    send_cmd(device, transfer_tx, build_set_temp_mode_command(mode)).await
}

async fn send_state_update(ui_cmd_tx: &Sender<TP25State>, device_state: TP25State) {
    // TODO: handle when this send fails, as the receiver has gone away
    let _ = ui_cmd_tx.send(device_state).await;
}

async fn send_alarm_ack_cmd(
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
) -> btleplug::Result<()> {
    send_cmd(device, transfer_tx, build_alarm_ack_cmd()).await
}

async fn send_custom_cmd(
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
    bytes: Vec<u8>,
) -> btleplug::Result<()> {
    send_cmd(device, transfer_tx, build_custom_cmd(bytes)).await
}

async fn get_device_state(protected: &ProtectedDeviceState) -> TP25State {
    protected.lock().await.clone()
}

fn handle_temps(temps: &TemperatureData, device_state: &mut TP25State) {
    for i in 0..4 {
        device_state.probes[i].temperature = temps.temps[i].temp;
        device_state.probes[i].alarm = if temps.temps[i].alarm {
            AlarmState::Alarm
        } else {
            AlarmState::NoAlarm
        }
    }
    device_state.temperature_mode = Some(temps.temp_mode);
}

fn handle_probe_profile(profile_data: &ProbeProfileData, device_state: &mut TP25State) {
    device_state.probes[profile_data.idx.as_zero_based() as usize].alarm_threshold =
        Some(profile_data.threshold);
}

async fn send_cmd(
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
    command: Command,
) -> btleplug::Result<()> {
    transfer_tx
        .send(Transfer::Command(command.clone()))
        .await
        .unwrap();
    device.send_cmd(command).await
}

async fn send_query_profile(
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
    idx: ProbeIdx,
) -> btleplug::Result<()> {
    send_cmd(device, transfer_tx, build_report_profile_cmd(idx)).await
}

async fn send_set_profile(
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
    idx: ProbeIdx,
    threshold: AlarmThreshold,
) -> btleplug::Result<()> {
    send_cmd(device, transfer_tx, build_set_profile_cmd(idx, threshold)).await
}
