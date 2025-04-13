#[cfg(feature = "dummy_device")]
use crate::controller::dummy_device_finder::get_device;

#[cfg(not(feature = "dummy_device"))]
use crate::controller::btleplug_device_finder::get_device;
use crate::controller::command_request::CommandRequest;
use crate::model::device::{TP25State, TemperatureMode};
use crate::model::probe::{AlarmState, AlarmThreshold};
use crate::peripheral::command::{
    build_report_profile_cmd, build_set_profile_cmd, build_set_temp_mode_command,
    build_startup_command, Command,
};
use crate::peripheral::interface::{TP25Receiver, TP25Writer};
use crate::peripheral::notification::{Decoded, Notification, ProbeProfileData, TemperatureData};
use crate::peripheral::transfer::Transfer;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Controller {}

type ProtectedDeviceState = Arc<Mutex<TP25State>>;

impl Controller {
    pub async fn run(
        state_update_tx: Sender<TP25State>,
        transfer_tx: Sender<Transfer>,
        mut command_request_rx: Receiver<CommandRequest>,
    ) {
        let (mut peripheral_rx, peripheral_tx) = get_device().await;
        let device_state = TP25State {
            connected: true,
            ..TP25State::default()
        };
        let protected_device_state = Arc::new(Mutex::new(device_state));

        let protected_device_state_b = protected_device_state.clone();
        let transfer_tx_b = transfer_tx.clone();

        let receiver_task = tokio::spawn(async move {
            loop {
                let Some(n) = peripheral_rx.get_notification().await else {
                    // If we stop receiving notifications then just exit.
                    // TODO: Update the screen to say disconnected, try to reconnect, etc.
                    return;
                };
                let device_state = &mut protected_device_state.lock().unwrap().clone();
                transfer_tx
                    .send(Transfer::Notification(n.clone()))
                    .await
                    .unwrap();
                handle_notification(n, &state_update_tx, device_state).await;
            }
        });

        let ui_task = tokio::spawn(async move {
            // TODO: The next line doesn't really sit well here, conceptually.
            send_startup_cmd(&peripheral_tx, &transfer_tx_b).await;

            loop {
                let Some(r) = command_request_rx.recv().await else {
                    return;
                };
                handle_command_request(
                    r,
                    &peripheral_tx,
                    &transfer_tx_b,
                    get_device_state(&protected_device_state_b),
                )
                .await;
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
) {
    match command_request {
        CommandRequest::ToggleTempMode => {
            let mode = match device_state.temperature_mode {
                Some(TemperatureMode::Celsius) => TemperatureMode::Fahrenheit,
                _ => TemperatureMode::Celsius,
            };
            send_temp_mode_cmd(mode, device, transfer_tx).await;
        }
        CommandRequest::ReportAllProfiles => {
            send_query_profile(device, transfer_tx, 0).await;
            send_query_profile(device, transfer_tx, 1).await;
            send_query_profile(device, transfer_tx, 2).await;
            send_query_profile(device, transfer_tx, 3).await;
        }
        CommandRequest::ReportProfile(idx) => {
            send_query_profile(device, transfer_tx, idx).await;
        }
        CommandRequest::SetProfile(idx, profile) => {
            let profile = match profile {
                AlarmThreshold::Unknown => AlarmThreshold::NoneSet,
                _ => profile,
            };
            send_set_profile(device, transfer_tx, idx, profile).await;
        }
    }
}

async fn send_startup_cmd(device: &impl TP25Writer, transfer_tx: &Sender<Transfer>) {
    send_cmd(device, transfer_tx, build_startup_command()).await;
}

async fn send_temp_mode_cmd(
    mode: TemperatureMode,
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
) {
    send_cmd(device, transfer_tx, build_set_temp_mode_command(mode)).await;
}

async fn send_state_update(ui_cmd_tx: &Sender<TP25State>, device_state: TP25State) {
    // TODO: handle when this send fails, as the receiver has gone away
    let _ = ui_cmd_tx.send(device_state).await;
}

fn get_device_state(protected: &ProtectedDeviceState) -> TP25State {
    protected.lock().unwrap().clone()
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
    device_state.probes[(profile_data.idx - 1) as usize].alarm_threshold = profile_data.threshold;
}

async fn send_cmd(device: &impl TP25Writer, transfer_tx: &Sender<Transfer>, command: Command) {
    transfer_tx
        .send(Transfer::Command(command.clone()))
        .await
        .unwrap();
    device.send_cmd(command).await;
}

async fn send_query_profile(device: &impl TP25Writer, transfer_tx: &Sender<Transfer>, idx: u8) {
    if idx > 3 {
        panic!("Invalid profile index");
    }
    send_cmd(device, transfer_tx, build_report_profile_cmd(idx)).await;
}

async fn send_set_profile(
    device: &impl TP25Writer,
    transfer_tx: &Sender<Transfer>,
    idx: u8,
    threshold: AlarmThreshold,
) {
    if idx > 3 {
        panic!("Invalid profile index");
    }
    if matches!(threshold, AlarmThreshold::Unknown) {
        // TODO: Should probably enforce this with a different type...
        panic!("Can't send Unknown alarm threshold");
    }
    send_cmd(device, transfer_tx, build_set_profile_cmd(idx, threshold)).await;
}
