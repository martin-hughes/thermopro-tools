use crate::controller::default::Controller;
use crate::ui::main::run_ui;
use crate::ui::ui_command::UiCommand;
use crate::ui::ui_request::UiRequest;
use std::sync::mpsc::{channel as std_channel, Sender};
use tokio::sync::mpsc::{channel as tokio_channel, Receiver};

mod controller;
mod model;
mod peripheral;
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

fn tokio_thread(ui_cmd_tx: Sender<UiCommand>, ui_request_rx: Receiver<UiRequest>) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            Controller::run(ui_cmd_tx, ui_request_rx).await;
        });
}
