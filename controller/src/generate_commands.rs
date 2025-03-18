use crate::command::Command;
use tokio::sync::mpsc::Sender;

pub struct Commander {
    sender: Sender<Command>,
}

impl Commander {
    pub fn new(sender: Sender<Command>) -> Self {
        Self { sender }
    }

    pub async fn startup(&self) {
        self.sender.send(Command::Connect).await.unwrap();
    }
}
