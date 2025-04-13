use crate::model::transfer_log::TransferLog;
use device_controller::model::device::TP25State;

pub struct UpdateStateDetails {
    pub device_state: TP25State,
}

pub enum UiCommand {
    UpdateState(UpdateStateDetails),
    UpdateTransferLog(TransferLog),
    Quit,
}
