use crate::notifications::temperatures::Temperatures;
use bytes::Bytes;

#[derive(Debug)]
pub enum Notification {
    ConnectResponse,
    Temperatures(Temperatures),
}

impl TryFrom<Bytes> for Notification {
    type Error = &'static str;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value[0] {
            0x01 => Ok(Notification::ConnectResponse),
            0x30 => Ok(Notification::Temperatures(value.try_into()?)),
            _ => Err("Invalid notification type"),
        }
    }
}
