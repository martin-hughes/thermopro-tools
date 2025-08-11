use crate::model::transfer_log::TransferLog;
use crate::ui::main::run_ui;
use crate::ui::ui_command::{UiCommand, UpdateStateDetails};
use device_controller::controller::command_request::CommandRequest;
use device_controller::controller::default::Controller;
use device_controller::dev_finder::DeviceFinder;
use std::sync::mpsc::{channel as std_channel, Sender};
use tokio::select;
use tokio::sync::mpsc::{channel as tokio_channel, Receiver};

mod model;
mod ui;

fn main() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        default_panic(info);
        std::process::exit(1);
    }));

    let (ui_cmd_tx, ui_cmd_rx) = std_channel();
    let (ui_request_tx, ui_request_rx) = tokio_channel(10);

    let _ = std::thread::spawn(move || tokio_thread(ui_cmd_tx, ui_request_rx));

    // Run the UI in the main thread.
    run_ui(ui_cmd_rx, ui_request_tx);
}

fn tokio_thread(ui_cmd_tx: Sender<UiCommand>, ui_request_rx: Receiver<CommandRequest>) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            tokio_main_loop(ui_cmd_tx, ui_request_rx).await;
        });
}

async fn tokio_main_loop(ui_cmd_tx: Sender<UiCommand>, ui_request_rx: Receiver<CommandRequest>) {
    let (state_tx, mut state_rx) = tokio_channel(10);
    let (transfer_tx, mut transfer_rx) = tokio_channel(10);
    let finder = DeviceFinder {};

    let task_a = Controller::run(finder, state_tx, transfer_tx, ui_request_rx);

    let ui_cmd_tx_2 = ui_cmd_tx.clone();
    let ui_cmd_tx_3 = ui_cmd_tx.clone();

    let task_b = tokio::spawn(async move {
        loop {
            let Some(new_state) = state_rx.recv().await else {
                return;
            };
            if ui_cmd_tx
                .send(UiCommand::UpdateState(UpdateStateDetails {
                    device_state: new_state,
                }))
                .is_err()
            {
                // The UI has apparently shut down.
                return;
            }
        }
    });

    let task_c = tokio::spawn(async move {
        let transfer_log = TransferLog::new();
        loop {
            let Some(transfer) = transfer_rx.recv().await else {
                return;
            };
            transfer_log.push_transfer(transfer);
            if ui_cmd_tx_2
                .send(UiCommand::UpdateTransferLog(transfer_log.clone()))
                .is_err()
            {
                // The UI has shut down
                return;
            }
        }
    });

    select!(_ = task_a => {}, _ = task_b => {}, _ = task_c => {});

    // No need to handle errors here, we're about to exit anyway.
    ui_cmd_tx_3.send(UiCommand::Quit).unwrap_or_default();
}
