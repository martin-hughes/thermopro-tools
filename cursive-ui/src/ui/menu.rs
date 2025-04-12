use crate::ui::report_profile_dialog::report_profile_cb;
use crate::ui::set_profile_dialog::set_profile_cb;
use crate::ui::ui_request::UiRequest;
use cursive::{menu, CursiveRunnable};
use tokio::sync::mpsc::Sender;

pub fn install_menu(c: &mut CursiveRunnable, request_tx: Sender<UiRequest>) {
    let tx_a = request_tx.clone();
    let tx_b = request_tx.clone();
    let tx_c = request_tx.clone();
    let tx_d = request_tx.clone();

    c.menubar()
        .add_subtree(
            "Commands",
            menu::Tree::new()
                // Trees are made of leaves, with are directly actionable...
                .leaf("Toggle Temp Mode", move |_| {
                    tx_a.blocking_send(UiRequest::ToggleTempMode).unwrap();
                })
                .leaf("Report all profiles", move |_| {
                    tx_b.blocking_send(UiRequest::ReportAllProfiles).unwrap();
                })
                .leaf("Report profile", move |c| report_profile_cb(c, &tx_c))
                .leaf("Set profile", move |c| set_profile_cb(c, &tx_d)),
        )
        .add_delimiter()
        .add_leaf("Quit", |s| s.quit());
}
