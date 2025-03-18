use bcd_convert::BcdNumber;
use bytes::Bytes;
use std::convert::TryFrom;
// Example temperature notification:
// 300f5a0c00ffffffffffff0281ffffffff1e0140

#[derive(Debug)]
struct Checksum {
    #[allow(unused)]
    value: u8,
    #[allow(unused)]
    valid: bool,
}

#[derive(Debug)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
    Unknown,
}

#[derive(Debug)]
pub struct Temperatures {
    #[allow(unused)]
    length: u8,
    #[allow(unused)]
    unknown_a: u8,
    pub celsius: TempUnit,
    pub alarms: u8,
    pub temps: [Option<u16>; 4], // Temperature in tenths of Celsius
    #[allow(unused)]
    unknown_b: [u8; 4],
    checksum: Checksum,
    #[allow(unused)]
    suffix: [u8; 2],
}

impl Default for Temperatures {
    fn default() -> Self {
        Temperatures {
            length: LENGTH,
            unknown_a: UNKNOWN_A,
            celsius: TempUnit::Unknown,
            alarms: 0,
            temps: [None; 4],
            unknown_b: UNKNOWN_B,
            checksum: Checksum {
                value: 0,
                valid: false,
            },
            suffix: SUFFIX,
        }
    }
}

const LENGTH: u8 = 0x0f;

const UNKNOWN_A: u8 = 0x5a;

const UNKNOWN_B: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const SUFFIX: [u8; 2] = [0x01, 0x40];

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

fn calc_checksum(bytes: &[u8]) -> u8 {
    #[allow(arithmetic_overflow)]
    let sum: u64 = bytes[0..bytes.len()].iter().map(|x| *x as u64).sum();
    (sum % 256) as u8
}

impl TryFrom<Bytes> for Temperatures {
    type Error = &'static str;
    fn try_from(val: Bytes) -> Result<Self, Self::Error> {
        if val[1] != LENGTH {
            return Err("Invalid length");
        }
        if (val[2] != UNKNOWN_A) || (val[18..] != SUFFIX) || (val[13..17] != UNKNOWN_B) {
            return Err("Invalid constant parts");
        }

        let data_part = val[0..17].to_vec();
        let checksum_val = calc_checksum(&data_part);

        let mut t = Temperatures {
            celsius: match val[3] {
                0x0c => TempUnit::Celsius,
                0x0f => TempUnit::Fahrenheit,
                _ => TempUnit::Unknown,
            },
            alarms: val[4],
            checksum: Checksum {
                value: checksum_val,
                valid: checksum_val == val[17],
            },
            ..Default::default()
        };

        for i in 0..4 {
            let offset = (i * 2) + 5;
            t.temps[i] = temp_from_bytes([val[offset], val[offset + 1]])?;
        }

        Ok(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let t = Temperatures::try_from(bytes);
        assert!(t.is_ok());

        let s = t.unwrap();
        assert!(s.checksum.valid);
    }
}
