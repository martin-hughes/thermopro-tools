#[cfg(feature = "dummy_device")]
use crate::controller::dummy_device_finder::get_device;

#[cfg(not(feature = "dummy_device"))]
use crate::controller::btleplug_device_finder::get_device;
use crate::model::device::{TP25State, TemperatureMode};
use crate::model::probe::{AlarmState, AlarmThreshold};
use crate::model::transfer_log::{Transfer, TransferLog};
use crate::peripheral::command::{
    build_report_profile_cmd, build_set_profile_cmd, build_set_temp_mode_command,
    build_startup_command, Command,
};
use crate::peripheral::interface::{TP25Receiver, TP25Writer};
use crate::peripheral::notification::{Decoded, Notification, ProbeProfileData, TemperatureData};
use crate::ui::ui_command::{UiCommand, UpdateStateDetails};
use crate::ui::ui_request::UiRequest;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::select;
use tokio::sync::mpsc::Receiver;

pub struct Controller {}

type ProtectedDeviceState = Arc<Mutex<TP25State>>;

impl Controller {
    pub async fn run(ui_command_tx: Sender<UiCommand>, mut ui_request_rx: Receiver<UiRequest>) {
        let quit_tx = ui_command_tx.clone();

        let (mut peripheral_rx, peripheral_tx) = get_device().await;
        let device_state = TP25State {
            connected: true,
            ..TP25State::default()
        };
        let protected_device_state = Arc::new(Mutex::new(device_state));

        let transfer_log = TransferLog::new();
        let transfer_log_b = transfer_log.clone();

        let ui_cmd_tx_b = ui_command_tx.clone();
        let protected_device_state_b = protected_device_state.clone();

        let receiver_task = tokio::spawn(async move {
            loop {
                let Some(n) = peripheral_rx.get_notification().await else {
                    // If we stop receiving notifications then just exit.
                    // TODO: Update the screen to say disconnected, try to reconnect, etc.
                    return;
                };
                let device_state = &mut protected_device_state.lock().unwrap();

                handle_notification(n, &transfer_log, &ui_command_tx, device_state);
            }
        });

        let ui_task = tokio::spawn(async move {
            // TODO: The next line doesn't really sit well here, conceptually.
            send_startup_cmd(&transfer_log_b, &peripheral_tx).await;

            loop {
                let Some(r) = ui_request_rx.recv().await else {
                    return;
                };
                handle_ui_request(
                    r,
                    &transfer_log_b,
                    &peripheral_tx,
                    &ui_cmd_tx_b,
                    get_device_state(&protected_device_state_b),
                )
                .await;
            }
        });

        select! { _ = receiver_task => {}, _ = ui_task => {}};

        quit_tx.send(UiCommand::Quit).unwrap()
    }
}
fn handle_notification(
    notification: Notification,
    transfer_log: &TransferLog,
    ui_cmd_tx: &Sender<UiCommand>,
    device_state: &mut TP25State,
) {
    match &notification.decoded {
        Decoded::Unknown => {}
        Decoded::Startup => {}
        Decoded::SetTempMode => {}
        Decoded::ReportProbeProfile(profile_data) => {
            handle_probe_profile(profile_data, device_state)
        }
        Decoded::Temperatures(temps) => handle_temps(temps, device_state),
        Decoded::SetProbeProfile => {}
        Decoded::Error => {}
    };
    transfer_log.push_transfer(Transfer::Notification(notification));
    update_ui(transfer_log, ui_cmd_tx, device_state.clone());
}

async fn handle_ui_request(
    ui_request: UiRequest,
    transfer_log: &TransferLog,
    device: &impl TP25Writer,
    ui_cmd_tx: &Sender<UiCommand>,
    device_state: TP25State,
) {
    match ui_request {
        UiRequest::ToggleTempMode => {
            let mode = match device_state.temperature_mode {
                Some(TemperatureMode::Celsius) => TemperatureMode::Fahrenheit,
                _ => TemperatureMode::Celsius,
            };
            send_temp_mode_cmd(mode, transfer_log, device).await;
        }
        UiRequest::ReportAllProfiles => {
            send_query_profile(transfer_log, device, 0).await;
            send_query_profile(transfer_log, device, 1).await;
            send_query_profile(transfer_log, device, 2).await;
            send_query_profile(transfer_log, device, 3).await;
        }
        UiRequest::ReportProfile(idx) => {
            send_query_profile(transfer_log, device, idx).await;
        }
        UiRequest::SetProfile(idx, profile) => {
            let profile = match profile {
                AlarmThreshold::Unknown => AlarmThreshold::NoneSet,
                _ => profile,
            };
            send_set_profile(transfer_log, device, idx, profile).await;
        }
    }

    update_ui(transfer_log, ui_cmd_tx, device_state);
}

async fn send_startup_cmd(transfer_log: &TransferLog, device: &impl TP25Writer) {
    send_cmd(transfer_log, device, build_startup_command()).await;
}

async fn send_temp_mode_cmd(
    mode: TemperatureMode,
    transfer_log: &TransferLog,
    device: &impl TP25Writer,
) {
    send_cmd(transfer_log, device, build_set_temp_mode_command(mode)).await;
}

fn update_ui(transfer_log: &TransferLog, ui_cmd_tx: &Sender<UiCommand>, device_state: TP25State) {
    ui_cmd_tx
        .send(UiCommand::UpdateState(UpdateStateDetails {
            transfer_log: transfer_log.clone(),
            device_state,
        }))
        .unwrap();
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

async fn send_cmd(transfer_log: &TransferLog, device: &impl TP25Writer, command: Command) {
    transfer_log.push_transfer(Transfer::Command(command.clone()));
    device.send_cmd(command).await;
}

async fn send_query_profile(transfer_log: &TransferLog, device: &impl TP25Writer, idx: u8) {
    if idx > 3 {
        panic!("Invalid profile index");
    }
    send_cmd(transfer_log, device, build_report_profile_cmd(idx)).await;
}

async fn send_set_profile(
    transfer_log: &TransferLog,
    device: &impl TP25Writer,
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
    send_cmd(transfer_log, device, build_set_profile_cmd(idx, threshold)).await;
}
