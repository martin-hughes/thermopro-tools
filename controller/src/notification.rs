use crate::notifications::temperatures::Temperatures;
use bytes::Bytes;

#[derive(Debug)]
pub enum Notification {
    ConnectResponse,
    SetTempUnit,
    Temperatures(Temperatures),
    TwoSixResponse,
}

impl TryFrom<Bytes> for Notification {
    type Error = &'static str;
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value[0] {
            0x01 => Ok(Notification::ConnectResponse),
            0x20 => Ok(Notification::SetTempUnit),
            0x30 => Ok(Notification::Temperatures(value.try_into()?)),
            0x26 => Ok(Notification::TwoSixResponse),
            _ => Err("Invalid notification type"),
        }
    }
}
