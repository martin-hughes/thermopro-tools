use crate::command::Command;
use bytes::Bytes;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::transfer::RawTransfer;

pub async fn convert_commands(mut incoming: Receiver<Command>, outgoing: Sender<Bytes>) {
    loop {
        let cmd = incoming.recv().await.unwrap();
        let bytes = Bytes::from(&RawTransfer::from(&cmd));
        //println!("Raw command: {:x}", bytes);
        outgoing.send(bytes).await.unwrap();
    }
}
