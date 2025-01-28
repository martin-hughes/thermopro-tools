use crate::command::Command;
use crate::notification::Notification;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn controller(mut incoming: Receiver<Notification>, outgoing: Sender<Command>) {
    outgoing.send(Command::Connect).await.unwrap();
    loop {
        let n = incoming.recv().await.unwrap();
        println!("Received notification: {:?}", n);
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
        tokio::spawn(controller(write_rx, read_tx));

        let c = read_rx.recv().await.unwrap();
        assert!(matches!(c, Command::Connect { .. }));
    }
}
