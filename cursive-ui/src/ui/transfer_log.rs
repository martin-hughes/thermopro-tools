use crate::model::transfer_log::TransferLog;
use crate::ui::views::TransferLogView;
use cursive::traits::Nameable;
use cursive::views::NamedView;
use cursive::Cursive;

const TABLE_NAME: &str = "transfer_log";

pub fn make_transfer_log() -> NamedView<TransferLogView> {
    TransferLogView::new().with_name(TABLE_NAME)
}

pub fn update_transfer_log(transfer_log: TransferLog, c: &mut Cursive) {
    c.call_on_name(TABLE_NAME, |view: &mut TransferLogView| {
        view.set_items_from_log(transfer_log);
    });
}
