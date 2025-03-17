use bcd_convert::BcdNumber;
use bytes::Bytes;
use std::convert::TryFrom;
// Example temperature notification:
// 300f5a0c00ffffffffffff0281ffffffff1e0140

#[derive(Debug)]
struct Checksum {
    value: u8,
    valid: bool,
}

#[derive(Debug)]
pub struct Temperatures {
    preamble: [u8; 4],
    pub temps: [Option<u16>; 4], // Temperature in tenths of Celsius
    unknown: [u8; 4],
    checksum: Checksum,
    suffix: [u8; 2],
}

const PREAMBLE: [u8; 4] = [0x0f, 0x5a, 0x0c, 0x00];
const UNKNOWN: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
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

fn calc_checksum(bytes: &Vec<u8>) -> u8 {
    #[allow(arithmetic_overflow)]
    let sum: u64 = bytes[0..bytes.len()].iter().map(|x| *x as u64).sum();
    (sum % 256) as u8
}

impl TryFrom<Bytes> for Temperatures {
    type Error = &'static str;
    fn try_from(val: Bytes) -> Result<Self, Self::Error> {
        if (val[1..5] != PREAMBLE) || (val[18..] != SUFFIX) || (val[13..17] != UNKNOWN) {
            return Err("Invalid constant parts");
        }

        let data_part = val[0..17].to_vec();
        let checksum_val = calc_checksum(&data_part);

        let mut t = Temperatures {
            preamble: PREAMBLE,
            temps: [Some(0); 4],
            unknown: UNKNOWN,
            checksum: Checksum {
                value: checksum_val,
                valid: checksum_val == val[17],
            },
            suffix: SUFFIX,
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
