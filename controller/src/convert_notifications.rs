use crate::notification::Notification;
use bytes::Bytes;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn convert_notifications(mut incoming: Receiver<Bytes>, outgoing: Sender<Notification>) {
    while let Some(n) = incoming.recv().await {
        outgoing
            .send(Notification::try_from(n).unwrap())
            .await
            .unwrap();
    }
}
