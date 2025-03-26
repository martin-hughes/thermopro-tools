use crate::command::Command;
use crate::transfer::RawTransfer;
use bytes::Bytes;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn convert_commands(mut incoming: Receiver<Command>, outgoing: Sender<Bytes>) {
    while let Some(cmd) = incoming.recv().await {
        let bytes = Bytes::from(&RawTransfer::from(&cmd));
        outgoing.send(bytes).await.unwrap();
    }
}
