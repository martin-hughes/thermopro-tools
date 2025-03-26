use crate::checksum::{calc_checksum, Checksum};
use bytes::Bytes;
use std::convert::TryFrom;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawTransfer {
    pub notification_type: u8,
    pub length: u8,
    pub value: Bytes,
    pub checksum: Checksum,
    pub extra: Option<Bytes>,
}

impl TryFrom<Bytes> for RawTransfer {
    type Error = &'static str;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err("Notifications must be at least three bytes.");
        }

        if value.len() > 20 {
            return Err("Notifications must be less than 20 bytes.");
        }

        let value_length = value[1] as usize;
        if value_length > 17 {
            return Err("Value length is too long (max 17 bytes)");
        }
        if value_length > value.len() - 3 {
            return Err("Length field is greater than length of data in notification");
        }

        let value_bytes = value.slice(2..2 + value_length);
        let checksum_byte = value[2 + value_length];
        let calc_checksum = calc_checksum(value.slice(0..2 + value_length).as_ref());
        let extra = value.slice(3 + value_length..);
        Ok(RawTransfer {
            notification_type: value[0],
            length: value[1],
            value: value_bytes,
            checksum: Checksum {
                value: checksum_byte,
                valid: checksum_byte == calc_checksum,
            },
            extra: if !extra.is_empty() { Some(extra) } else { None },
        })
    }
}

impl From<&RawTransfer> for Bytes {
    fn from(value: &RawTransfer) -> Bytes {
        let mut bytes = Vec::new();
        bytes.push(value.notification_type);
        bytes.push(value.length);
        let value_bytes = &value.value;
        bytes.append(value_bytes.to_vec().as_mut());
        bytes.push(value.checksum.value);
        if let Some(extra) = &value.extra {
            bytes.append(extra.to_vec().as_mut());
        }

        bytes.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_single_byte() {
        let bytes = Bytes::from_static(&[0x01]);
        assert!(RawTransfer::try_from(bytes).is_err());
    }

    #[test]
    fn rejects_two_bytes() {
        let bytes = Bytes::from_static(&[0x01, 0x02]);
        assert!(RawTransfer::try_from(bytes).is_err());
    }

    #[test]
    fn rejects_twenty_one_bytes() {
        let bytes = Bytes::from_static(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x10, 0x11, 0x12, 0x13, 0x14,
            0x15, 0x16, 0x17, 0x18, 0x19, 0x20, 0x21,
        ]);
        assert!(RawTransfer::try_from(bytes).is_err());
    }

    #[test]
    fn rejects_length_too_long() {
        // 0x02 is the length, but the max possible data length is one.
        let bytes = Bytes::from_static(&[0x01, 0x02, 0x03, 0x04]);
        assert!(RawTransfer::try_from(bytes).is_err());
    }

    #[test]
    fn accepts_zero_length() {
        let bytes = Bytes::from_static(&[0x01, 0x00, 0x01]);
        let raw = RawTransfer::try_from(bytes).unwrap();
        let expected = RawTransfer {
            notification_type: 1,
            length: 0,
            value: Bytes::new(),
            checksum: Checksum {
                value: 1,
                valid: true,
            },
            extra: None,
        };
        assert_eq!(raw, expected);
    }

    #[test]
    fn accepts_nonzero_length() {
        let bytes = Bytes::from_static(&[0x01, 0x02, 0x11, 0x12, 0x26]);
        let raw = RawTransfer::try_from(bytes).unwrap();
        let expected = RawTransfer {
            notification_type: 1,
            length: 2,
            value: Bytes::from_static(&[0x11, 0x12]),
            checksum: Checksum {
                value: 0x26,
                valid: true,
            },
            extra: None,
        };
        assert_eq!(raw, expected);
    }

    #[test]
    fn has_correct_extra() {
        let bytes = Bytes::from_static(&[0x01, 0x02, 0x11, 0x12, 0x26, 0x3a, 0x3b]);
        let raw = RawTransfer::try_from(bytes).unwrap();
        let expected = RawTransfer {
            notification_type: 1,
            length: 2,
            value: Bytes::from_static(&[0x11, 0x12]),
            checksum: Checksum {
                value: 0x26,
                valid: true,
            },
            extra: Some(Bytes::from_static(&[0x3a, 0x3b])),
        };
        assert_eq!(raw, expected);
    }

    #[test]
    fn allows_wrong_checksum() {
        let bytes = Bytes::from_static(&[0x01, 0x02, 0x11, 0x12, 0x28, 0x3a, 0x3b]);
        let raw = RawTransfer::try_from(bytes).unwrap();
        let expected = RawTransfer {
            notification_type: 1,
            length: 2,
            value: Bytes::from_static(&[0x11, 0x12]),
            checksum: Checksum {
                value: 0x28,
                valid: false,
            },
            extra: Some(Bytes::from_static(&[0x3a, 0x3b])),
        };
        assert_eq!(raw, expected);
    }
}
