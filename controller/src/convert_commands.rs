use crate::command::Command;
use bytes::Bytes;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn convert_commands(mut incoming: Receiver<Command>, outgoing: Sender<Bytes>) {
    loop {
        let cmd = incoming.recv().await.unwrap();
        outgoing.send(Bytes::try_from(cmd).unwrap()).await.unwrap();
    }
}
