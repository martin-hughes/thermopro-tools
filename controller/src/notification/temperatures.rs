use crate::device::TempMode;
use crate::notification::content::NotificationTryFromErr;
use crate::notification::content::NotificationTryFromErr::{FieldError, WrongLength};
use crate::transfer::RawTransfer;
use bcd_convert::BcdNumber;
use std::convert::TryFrom;
// Example temperature notification:
// 300f5a0c00ffffffffffff0281ffffffff1e0140

#[derive(Debug)]
pub struct Temperatures {
    #[allow(unused)]
    unknown_a: u8,
    pub celsius: Option<TempMode>,
    pub alarms: u8,
    pub temps: [Option<u16>; 4], // Temperature in tenths of Celsius
    #[allow(unused)]
    unknown_b: [u8; 4],
}

impl Default for Temperatures {
    fn default() -> Self {
        Temperatures {
            unknown_a: UNKNOWN_A,
            celsius: None,
            alarms: 0,
            temps: [None; 4],
            unknown_b: UNKNOWN_B,
        }
    }
}

const LENGTH: u8 = 0x0f;

const UNKNOWN_A: u8 = 0x5a;

const UNKNOWN_B: [u8; 4] = [0xff, 0xff, 0xff, 0xff];

fn temp_from_bytes(val: [u8; 2]) -> Result<Option<u16>, &'static str> {
    if (val[0] == 0xff) && (val[1] == 0xff) {
        Ok(None)
    } else {
        Ok(Some(
            BcdNumber::try_from(&val as &[u8])
                .map_err(|_| "Temperature not valid BCD")?
                .to_u64()
                .unwrap() as u16,
        ))
    }
}

impl TryFrom<&RawTransfer> for Temperatures {
    type Error = NotificationTryFromErr;
    fn try_from(val: &RawTransfer) -> Result<Self, Self::Error> {
        if val.length != LENGTH {
            return Err(WrongLength);
        }

        let data_part = &val.value;
        let mut t = Temperatures {
            celsius: match data_part[1] {
                0x0c => Some(TempMode::Celsius),
                0x0f => Some(TempMode::Fahrenheit),
                _ => None,
            },
            alarms: data_part[2],
            ..Default::default()
        };

        for i in 0..4 {
            let offset = (i * 2) + 3;
            let maybe_t = temp_from_bytes([data_part[offset], data_part[offset + 1]]);
            match maybe_t {
                Ok(tt) => t.temps[i] = tt,
                Err(e) => return Err(FieldError(e)),
            }
        }

        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn basic_single_temp_from_bytes() {
        assert_eq!(Some(1234u16), temp_from_bytes([0x12, 0x34]).unwrap());
    }

    #[test]
    fn temp_rejects_non_bcd() {
        let e = temp_from_bytes([0x1a, 0x34]);
        assert!(e.is_err());
    }

    #[test]
    fn null_temp_returns_none() {
        let e = temp_from_bytes([0xff, 0xff]).unwrap();
        assert_eq!(None, e);
    }

    #[test]
    fn basic_temp_struct_from_bytes() {
        let bytes = Bytes::from(vec![
            0x30, 0x0f, 0x5a, 0x0c, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02, 0x81, 0xff,
            0xff, 0xff, 0xff, 0x1e, 0x01, 0x40,
        ]);
        let raw = RawTransfer::try_from(bytes).unwrap();
        let t = Temperatures::try_from(&raw);
        assert!(t.is_ok());
    }
}
