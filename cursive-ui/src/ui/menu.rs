use crate::ui::report_profile_dialog::report_profile_cb;
use crate::ui::set_profile_dialog::set_profile_cb;
use cursive::{menu, CursiveRunnable};
use device_controller::controller::command_request::CommandRequest;
use tokio::sync::mpsc::Sender;

pub fn install_menu(c: &mut CursiveRunnable, request_tx: Sender<CommandRequest>) {
    let tx_a = request_tx.clone();
    let tx_b = request_tx.clone();
    let tx_c = request_tx.clone();
    let tx_d = request_tx.clone();
    let tx_e = request_tx.clone();

    c.menubar()
        .add_subtree(
            "Commands",
            menu::Tree::new()
                // Trees are made of leaves, with are directly actionable...
                .leaf("Toggle Temp Mode", move |_| {
                    tx_a.blocking_send(CommandRequest::ToggleTempMode).unwrap();
                })
                .leaf("Report all profiles", move |_| {
                    tx_b.blocking_send(CommandRequest::ReportAllProfiles)
                        .unwrap();
                })
                .leaf("Report profile", move |c| report_profile_cb(c, &tx_c))
                .leaf("Set profile", move |c| set_profile_cb(c, &tx_d))
                .leaf("Acknowledge alarm", move |_| {
                    tx_e.blocking_send(CommandRequest::AckAlarm).unwrap();
                }),
        )
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());
}
