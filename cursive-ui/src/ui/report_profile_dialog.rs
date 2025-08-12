use crate::ui::views::ProbeSelectView;
use cursive::traits::Nameable;
use cursive::views::Dialog;
use cursive::Cursive;
use device_controller::controller::command_request::CommandRequest;
use tokio::sync::mpsc::Sender;

pub fn report_profile_cb(c: &mut Cursive, tx: &Sender<CommandRequest>) {
    let tx_cb = tx.clone();
    c.add_layer(
        Dialog::new()
            .title("Enter probe number")
            .content(ProbeSelectView::new().with_name("probe_index"))
            .button("Cancel", move |c2| {
                c2.pop_layer();
            })
            .button("OK", move |c2| {
                let probe_idx = c2
                    .call_on_name("probe_index", |view: &mut ProbeSelectView| {
                        view.get_selected_probe()
                    })
                    .unwrap();
                c2.pop_layer();

                tx_cb
                    .blocking_send(CommandRequest::ReportProfile(probe_idx))
                    .unwrap()
            }),
    )
}
