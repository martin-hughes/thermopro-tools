use bytes::Bytes;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum TransferType {
    Command,
    Notification,
}

#[derive(Clone)]
pub struct DeviceTransfer {
    pub transfer_type: TransferType,
    pub bytes: Bytes,
}

#[derive(Clone)]
pub struct TransferLog {
    log: Arc<Mutex<Vec<DeviceTransfer>>>,
}

impl TransferLog {
    pub fn new() -> Self {
        Self {
            log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push_transfer(&self, transfer_type: TransferType, bytes: Bytes) {
        let mut v = self.log.lock().unwrap();
        v.push(DeviceTransfer {
            transfer_type,
            bytes,
        });
    }

    pub fn get_transfers(&self) -> Vec<DeviceTransfer> {
        self.log.lock().unwrap().clone()
    }
}
