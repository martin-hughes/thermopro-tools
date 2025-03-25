use crate::notification::content::NotificationTryFromErr::UnknownType;
use crate::notification::temperatures::Temperatures;
use crate::transfer::RawTransfer;

#[derive(Clone, Debug)]
pub enum NotificationContent {
    ConnectResponse,
    SetTempUnit,
    Temperatures(Temperatures),
    TwoSixResponse,
}

pub enum NotificationTryFromErr {
    UnknownType,
    WrongLength,
    FieldError(&'static str),
}

impl TryFrom<&RawTransfer> for NotificationContent {
    type Error = NotificationTryFromErr;
    fn try_from(value: &RawTransfer) -> Result<Self, Self::Error> {
        match value.notification_type {
            0x01 => Ok(NotificationContent::ConnectResponse),
            0x20 => Ok(NotificationContent::SetTempUnit),
            0x30 => Ok(NotificationContent::Temperatures(value.try_into()?)),
            0x26 => Ok(NotificationContent::TwoSixResponse),
            _ => Err(UnknownType),
        }
    }
}
