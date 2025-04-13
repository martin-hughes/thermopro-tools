use cursive::traits::Nameable;
use cursive::views::{Dialog, EditView};
use cursive::Cursive;
use device_controller::controller::command_request::CommandRequest;
use tokio::sync::mpsc::Sender;

pub fn report_profile_cb(c: &mut Cursive, tx: &Sender<CommandRequest>) {
    let tx_cb = tx.clone();
    c.add_layer(
        Dialog::new()
            .title("Enter probe number")
            .content(EditView::new().with_name("probe_number"))
            .button("OK", move |c2| {
                let num_str = c2
                    .call_on_name("probe_number", |view: &mut EditView| view.get_content())
                    .unwrap();
                c2.pop_layer();

                let number = num_str.parse::<u8>();
                let Ok(num) = number else {
                    c2.add_layer(Dialog::info("Probe number invalid!"));
                    return;
                };
                if num == 0 || num > 4 {
                    c2.add_layer(Dialog::info("Probe number invalid!"));
                } else {
                    tx_cb
                        .blocking_send(CommandRequest::ReportProfile(num - 1))
                        .unwrap()
                }
            }),
    )
}
