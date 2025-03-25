pub mod content;
pub mod temperatures;

use bytes::Bytes;
use content::NotificationContent;
use crate::notification::content::NotificationTryFromErr;
use crate::notification::Status::{InvalidContent, InvalidLength, InvalidType};
use crate::transfer::RawTransfer;

#[derive(Clone)]
pub enum Status {
    Ok,
    InvalidType,
    InvalidLength,
    InvalidChecksum,
    InvalidContent(&'static str),
}

impl From<NotificationTryFromErr> for Status {
    fn from(value: NotificationTryFromErr) -> Self {
        match value {
            NotificationTryFromErr::UnknownType => InvalidType,
            NotificationTryFromErr::WrongLength => InvalidLength,
            NotificationTryFromErr::FieldError(s) => InvalidContent(s),
        }
    }
}

#[derive(Clone)]
pub struct Notification {
    pub raw_notification: RawTransfer,
    pub content: Option<NotificationContent>,
    pub status: Status,
}

impl TryFrom<Bytes> for Notification {
    type Error = &'static str;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        let raw = RawTransfer::try_from(value)?;
        if raw.checksum.valid {
            let content = NotificationContent::try_from(&raw);
            match content {
                Ok(c) => {
                    Ok(Notification {
                        raw_notification: raw,
                        content: Some(c),
                        status: Status::Ok,
                    })
                },
                Err(e) => {
                    Ok(Notification {
                        raw_notification: raw,
                        content: None,
                        status: e.into()
                    })
                }
            }
        } else {
            Ok(Notification {
                raw_notification: raw,
                content: None,
                status: Status::InvalidChecksum,
            })
        }
    }
}
