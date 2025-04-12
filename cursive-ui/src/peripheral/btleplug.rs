use crate::peripheral::command::Command;
use crate::peripheral::interface::{TP25Receiver, TP25Writer};
use crate::peripheral::notification::Notification;
use btleplug::api::{Characteristic, Peripheral as _, ValueNotification, WriteType};
use btleplug::platform::Peripheral;
use bytes::Bytes;
use futures::Stream;
use futures::StreamExt;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::timeout;

type BtleNotificationStream = Pin<Box<dyn Stream<Item = ValueNotification> + Send>>;

pub struct BtleplugReceiver {
    receiver: BtleNotificationStream,
}

impl BtleplugReceiver {
    pub fn new(receiver: BtleNotificationStream) -> BtleplugReceiver {
        BtleplugReceiver { receiver }
    }
}

impl TP25Receiver for BtleplugReceiver {
    async fn get_notification(&mut self) -> Option<Notification> {
        let Ok(Some(vn)) = timeout(Duration::from_secs(4), self.receiver.next()).await else {
            return None;
        };

        let d: Bytes = vn.value.into();
        Some(Notification::from(d))
    }
}

pub struct BtleplugWriter {
    device: Peripheral,
    write_characteristic: Characteristic,
}

impl BtleplugWriter {
    pub fn new(device: Peripheral, write_characteristic: Characteristic) -> BtleplugWriter {
        BtleplugWriter {
            device,
            write_characteristic,
        }
    }
}

impl TP25Writer for BtleplugWriter {
    async fn send_cmd(&self, command: Command) {
        if self
            .device
            .write(
                &self.write_characteristic,
                command.raw.iter().as_slice(),
                WriteType::WithoutResponse,
            )
            .await
            .is_err()
        {
            panic!("Failed to send command");
        }
    }
}
