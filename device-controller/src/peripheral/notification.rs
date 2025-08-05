use crate::model::device::TemperatureMode;
use crate::model::probe::{AlarmThreshold, ProbeIdx, RangeLimitThreshold, UpperLimitThreshold};
use bcd_convert::BcdNumber;
use bytes::Bytes;

#[derive(Clone, Debug)]
pub struct Notification {
    pub raw: Bytes,
    pub decoded: Decoded,
}

impl From<Bytes> for Notification {
    fn from(raw: Bytes) -> Notification {
        if raw.len() < 3 {
            return Notification {
                raw,
                decoded: Decoded::Unknown,
            };
        }

        let decoded = match raw[0] {
            0x01 => make_notification(&raw, 1, startup),
            0x20 => make_notification(&raw, 0, set_temp_mode),
            0x23 => make_notification(&raw, 2, set_probe_profile),
            0x24 => make_notification(&raw, 6, report_probe_profile),
            0x30 => make_notification(&raw, 0x0f, temperature_report),
            0xe0 => Decoded::Error,
            _ => Decoded::Unknown,
        };

        Notification { raw, decoded }
    }
}

#[derive(Clone, Debug)]
pub enum Decoded {
    Unknown,
    Startup,                              // 0x01
    SetTempMode,                          // 0x20
    SetProbeProfile,                      // 0x23
    ReportProbeProfile(ProbeProfileData), // 0x24
    Temperatures(TemperatureData),        // 0x30
    Error,                                // 0xe0
}

#[derive(Clone, Debug)]
pub struct ProbeProfileData {
    pub idx: ProbeIdx,
    pub threshold: AlarmThreshold,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RawTemperature {
    pub temp: Option<u16>,
    pub alarm: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct TemperatureData {
    pub temps: [RawTemperature; 4],
    pub temp_mode: TemperatureMode,
}

type InnerConversion = fn(raw: &Bytes) -> Decoded;

fn make_notification(raw: &Bytes, length: usize, inner_conversion: InnerConversion) -> Decoded {
    if raw[1] != length as u8 {
        return Decoded::Unknown;
    }

    let checksum_byte = raw[2 + length];
    let calc_checksum = calc_checksum(raw.slice(0..2 + length).as_ref());

    if checksum_byte != calc_checksum {
        return Decoded::Unknown;
    }

    // At this point, we know the notification has the correct format, so these inner conversion do not need to do any
    // checking.
    inner_conversion(raw)
}

pub fn calc_checksum(bytes: &[u8]) -> u8 {
    #[allow(arithmetic_overflow)]
    let sum: u64 = bytes[0..bytes.len()].iter().map(|x| *x as u64).sum();
    (sum % 256) as u8
}

fn startup(_: &Bytes) -> Decoded {
    // There is a byte of value, but its purpose is currently unknown.
    Decoded::Startup
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

fn report_probe_profile(raw: &Bytes) -> Decoded {
    let idx = ProbeIdx::from_one_based(raw[2]);
    let high_threshold = temp_from_bytes([raw[4], raw[5]]);
    let low_threshold = temp_from_bytes([raw[6], raw[7]]);

    let Ok(high_threshold) = high_threshold else {
        return Decoded::Unknown;
    };
    let Ok(low_threshold) = low_threshold else {
        return Decoded::Unknown;
    };
    if low_threshold.is_some() && high_threshold.is_none() {
        return Decoded::Unknown;
    }

    let threshold: AlarmThreshold = match (low_threshold, high_threshold) {
        (Some(low), Some(high)) => AlarmThreshold::RangeLimit(RangeLimitThreshold {
            min: low,
            max: high,
        }),
        (Some(_), None) => {
            return Decoded::Unknown;
        }
        (None, Some(high)) => AlarmThreshold::UpperLimit(UpperLimitThreshold { max: high }),
        (None, None) => AlarmThreshold::NoneSet,
    };

    Decoded::ReportProbeProfile(ProbeProfileData { idx, threshold })
}

fn temperature_report(raw: &Bytes) -> Decoded {
    let temp_mode = if raw[3] == 0x0f {
        TemperatureMode::Fahrenheit
    } else {
        TemperatureMode::Celsius
    };
    let alarms = raw[4];

    let mut temps: [RawTemperature; 4] = [RawTemperature::default(); 4];
    for i in 0..4 {
        let Ok(t) = temp_from_bytes([raw[5 + (i * 2)], raw[6 + (i * 2)]]) else {
            return Decoded::Unknown;
        };
        temps[i].temp = t;
        temps[i].alarm = (alarms & (1 << i)) != 0;
    }

    Decoded::Temperatures(TemperatureData { temps, temp_mode })
}

fn set_temp_mode(_: &Bytes) -> Decoded {
    Decoded::SetTempMode
}

fn set_probe_profile(_: &Bytes) -> Decoded {
    // There are two bytes sent that we're discarding.
    Decoded::SetProbeProfile
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn parses_simple_startup_response() {
        let b = Bytes::from_static(&[0x01u8, 0x01u8, 0x0au8, 0x0cu8]);
        let n = Notification::from(b);
        assert_matches!(n.decoded, Decoded::Startup);
    }

    #[test]
    fn parses_simple_temp_report() {
        let b = Bytes::from_static(&[
            0x30, 0x0f, 0x5a, 0x0c, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x25, 0xff,
            0xff, 0xff, 0xff, 0xc3,
        ]);
        let n = Notification::from(b);
        let Decoded::Temperatures(d) = n.decoded else {
            assert!(false);
            return;
        };

        assert_matches!(d.temp_mode, TemperatureMode::Celsius);
        assert_matches!(d.temps[0].temp, None);
        assert_matches!(d.temps[1].temp, None);
        assert_matches!(d.temps[2].temp, None);
        assert_matches!(d.temps[3].temp, Some(325));
        assert!(!d.temps[0].alarm);
        assert!(!d.temps[1].alarm);
        assert!(!d.temps[2].alarm);
        assert!(!d.temps[3].alarm);
    }
}
