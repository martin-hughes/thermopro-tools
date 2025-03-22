use crate::command::Command;
use crate::device::TempMode;
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

    pub async fn set_temp_mode(&self, mode: TempMode) {
        self.sender.send(Command::SetTempUnit(mode)).await.unwrap();
    }
}
