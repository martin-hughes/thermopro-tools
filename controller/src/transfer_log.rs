use crate::command::Command;
use crate::notification::Notification;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum Transfer {
    Command(Command),
    Notification(Notification),
}

#[derive(Clone)]
pub struct TransferLog {
    log: Arc<Mutex<Vec<Transfer>>>,
}

impl TransferLog {
    pub fn new() -> Self {
        Self {
            log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push_transfer(&self, transfer: Transfer) {
        let mut v = self.log.lock().unwrap();
        v.push(transfer);
    }

    pub fn get_transfers(&self) -> Vec<Transfer> {
        self.log.lock().unwrap().clone()
    }
}
