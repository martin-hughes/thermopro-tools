use device_controller::peripheral::transfer::Transfer;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Internal {
    transfers: Vec<(u64, Transfer)>,
    index: u64,
}

#[derive(Clone)]
pub struct TransferLog {
    internal: Arc<Mutex<Internal>>,
}

impl TransferLog {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(Mutex::new(Internal::default())),
        }
    }

    pub fn push_transfer(&self, transfer: Transfer) {
        let mut v = self.internal.lock().unwrap();
        v.index += 1;
        let i = v.index;
        v.transfers.push((i, transfer));
    }

    pub fn get_transfers(&self) -> Vec<(u64, Transfer)> {
        self.internal.lock().unwrap().transfers.clone()
    }
}
