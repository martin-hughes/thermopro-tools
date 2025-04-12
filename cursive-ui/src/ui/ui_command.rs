use crate::model::device::TP25State;
use crate::model::transfer_log::TransferLog;

pub struct UpdateStateDetails {
    // TransferLog is already uses an Arc<Mutex<...>> wrapper internally so not too worried about copies etc.
    pub transfer_log: TransferLog,
    pub device_state: TP25State,
}

pub enum UiCommand {
    UpdateState(UpdateStateDetails),
    Quit,
}
