use crate::model::device_temperature::DeviceTemperature::{InRange, OutOfRange};
use bcd_convert::BcdNumber;

#[derive(Clone, Copy, Debug, Default)]
pub enum DeviceTemperature {
    #[default]
    OutOfRange,
    InRange(InRangeDeviceTemperature),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InRangeDeviceTemperature {
    degrees: u16,
    tenths: u8,
}

impl InRangeDeviceTemperature {
    pub fn try_new(degrees: u16, tenths: u8) -> Result<InRangeDeviceTemperature, &'static str> {
        if tenths > 9 {
            Err("Tenths must be less than 10")
        } else {
            Ok(InRangeDeviceTemperature { degrees, tenths })
        }
    }

    pub fn new(degrees: u16, tenths: u8) -> InRangeDeviceTemperature {
        Self::try_new(degrees, tenths).expect("Tenths must be less than 10")
    }
}

impl TryFrom<[u8; 2]> for DeviceTemperature {
    type Error = &'static str;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        let raw = temp_from_bytes(value)?;

        Ok(match raw {
            None => OutOfRange,
            Some(t) => InRange(InRangeDeviceTemperature {
                degrees: t / 10,
                tenths: (t % 10) as u8,
            }),
        })
    }
}

impl From<DeviceTemperature> for [u8; 2] {
    fn from(value: DeviceTemperature) -> [u8; 2] {
        match value {
            InRange(t) => t.into(),
            OutOfRange => [0xff, 0xff],
        }
    }
}

impl From<InRangeDeviceTemperature> for [u8; 2] {
    fn from(value: InRangeDeviceTemperature) -> [u8; 2] {
        [
            ((((value.degrees / 100) % 10) << 4) + ((value.degrees / 10) % 10)) as u8,
            ((value.degrees % 10) << 4) as u8 + (value.tenths % 10),
        ]
    }
}

impl From<InRangeDeviceTemperature> for f32 {
    fn from(value: InRangeDeviceTemperature) -> f32 {
        value.degrees as f32 + (value.tenths as f32 / 10.0)
    }
}

impl From<f32> for InRangeDeviceTemperature {
    fn from(value: f32) -> InRangeDeviceTemperature {
        InRangeDeviceTemperature {
            degrees: value as u16,
            tenths: ((value * 10.0).round() % 10.0) as u8,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_from_bytes() {
        let t = DeviceTemperature::try_from([0x03u8, 0x46u8]);
        assert!(t.is_ok());

        let t = t.unwrap();
        let InRange(t2) = t else {
            assert!(false);
            return;
        };

        assert_eq!(t2.degrees, 34);
        assert_eq!(t2.tenths, 6);
    }

    #[test]
    fn basic_from_f32() {
        let t = InRangeDeviceTemperature::from(34.6_f32);
        assert_eq!(t.degrees, 34);
        assert_eq!(t.tenths, 6);
    }

    #[test]
    fn f32_from_bytes() {
        let t = DeviceTemperature::try_from([0x03u8, 0x46u8]);
        assert!(t.is_ok());
        let t = t.unwrap();

        let InRange(t2) = t else {
            assert!(false);
            return;
        };

        let f: f32 = f32::from(t2);
        assert_eq!((f * 10.0).round(), 346.0);
    }
}
