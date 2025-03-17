use crate::command::Command;
use crate::device::Device;
use crate::notification::Notification;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum Updated {
    Updated,
}

pub async fn controller(
    device: Device,
    mut incoming: Receiver<Notification>,
    outgoing: Sender<Command>,
    updated: Sender<Updated>,
) {
    outgoing.send(Command::Connect).await.unwrap();
    loop {
        let n = incoming.recv().await.unwrap();
        if device.handle_notification(n) {
            updated.send(Updated::Updated).await.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    const CHANNEL_SIZE: usize = 10;

    use super::*;
    use tokio::sync::mpsc::channel;

    #[tokio::test(start_paused = true)]
    async fn sends_startup_message() {
        let (_, write_rx) = channel(CHANNEL_SIZE);
        let (read_tx, mut read_rx) = channel(CHANNEL_SIZE);
        let (update_tx, _) = channel(CHANNEL_SIZE);
        let device = Device::new();
        tokio::spawn(controller(device, write_rx, read_tx, update_tx));

        let c = read_rx.recv().await.unwrap();
        assert!(matches!(c, Command::Connect { .. }));
    }
}
