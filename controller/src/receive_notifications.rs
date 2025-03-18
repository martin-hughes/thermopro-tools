use crate::command::Command;
use crate::device::Device;
use crate::notification::Notification;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum Updated {
    Updated,
}

pub async fn receive_notifications(
    device: Device,
    mut incoming: Receiver<Notification>,
    updated: Sender<Updated>,
) {
    loop {
        let n = incoming.recv().await.unwrap();
        if device.handle_notification(n) {
            updated.send(Updated::Updated).await.unwrap();
        }
    }
}
