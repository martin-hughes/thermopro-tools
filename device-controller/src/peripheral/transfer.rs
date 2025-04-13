use crate::peripheral::command::Command;
use crate::peripheral::notification::Notification;

#[derive(Clone)]
pub enum Transfer {
    Notification(Notification),
    Command(Command),
}
