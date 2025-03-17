use crate::command::Command;
use bytes::Bytes;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn convert_commands(mut incoming: Receiver<Command>, outgoing: Sender<Bytes>) {
    loop {
        let cmd = incoming.recv().await.unwrap();
        let bytes = Bytes::try_from(cmd).unwrap();
        //println!("Raw command: {:x}", bytes);
        outgoing.send(bytes).await.unwrap();
    }
}
