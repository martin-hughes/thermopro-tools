use crate::peripheral::command::Command;
use crate::peripheral::notification::Notification;

#[trait_variant::make(TP25Receiver: Send)]
pub trait LocalTP25Receiver {
    #[allow(unused)] // Needed because we always used the variant constructed above
    async fn get_notification(&mut self) -> Option<Notification>;
}

#[trait_variant::make(TP25Writer: Send)]
pub trait LocalTP25Writer {
    #[allow(unused)] // Needed because we always used the variant constructed above
    async fn send_cmd(&self, command: Command);
}
