use crate::model::device::TemperatureMode;
use crate::model::probe::{AlarmThreshold, ProbeIdx};
use crate::peripheral::notification::calc_checksum;
use bytes::Bytes;

#[derive(Clone)]
pub enum Decoded {
    Startup,

    // I know the "decoded" structs aren't used at present, but having written them in I think
    // they're worth keeping so we can expand the UI in the future.
    #[allow(dead_code)]
    SetTempMode(TemperatureMode),

    #[allow(dead_code)]
    ReportProfile(ProbeIdx),

    #[allow(dead_code)]
    SetProbeProfile(ProbeIdx, AlarmThreshold),

    #[allow(dead_code)]
    AlarmAck,

    #[allow(dead_code)]
    Custom(Vec<u8>),
}

#[derive(Clone)]
pub struct Command {
    pub raw: Bytes,
    pub decoded: Decoded,
}

pub fn build_startup_command() -> Command {
    Command {
        raw: Bytes::from_static(&[
            0x01, 0x09, 0x70, 0x32, 0xe2, 0xc1, 0x79, 0x9d, 0xb4, 0xd1, 0xc7, 0xb1,
        ]),
        decoded: Decoded::Startup,
    }
}

pub fn build_set_temp_mode_command(mode: TemperatureMode) -> Command {
    Command {
        raw: Bytes::from_static(if matches!(mode, TemperatureMode::Celsius) {
            &[0x20, 0x01, 0x0c, 0x2d]
        } else {
            &[0x20, 0x01, 0x0f, 0x30]
        }),
        decoded: Decoded::SetTempMode(mode),
    }
}

pub fn build_alarm_ack_cmd() -> Command {
    Command {
        raw: Bytes::from_static(&[0x27, 0x00, 0x27]),
        decoded: Decoded::AlarmAck,
    }
}

pub fn build_custom_cmd(raw: Vec<u8>) -> Command {
    Command {
        raw: raw.clone().into(),
        decoded: Decoded::Custom(raw),
    }
}

pub fn build_report_profile_cmd(probe_idx: ProbeIdx) -> Command {
    let mut raw = vec![0x24, 0x01, probe_idx.as_one_based()];
    let checksum = calc_checksum(raw.as_slice());
    raw.push(checksum);

    Command {
        raw: raw.into(),
        decoded: Decoded::ReportProfile(probe_idx),
    }
}

pub fn build_set_profile_cmd(probe_idx: ProbeIdx, threshold: AlarmThreshold) -> Command {
    let mut raw = vec![0x23, 0x06, probe_idx.as_one_based(), 0xcc];

    match threshold {
        AlarmThreshold::NoneSet => {
            raw.push(0xff);
            raw.push(0xff);
            raw.push(0xff);
            raw.push(0xff);
        }
        AlarmThreshold::UpperLimit(ult) => {
            let u: [u8; 2] = ult.max.into();
            raw.push(u[0]);
            raw.push(u[1]);
            raw.push(0xff);
            raw.push(0xff);
        }
        AlarmThreshold::RangeLimit(rlt) => {
            let min: [u8; 2] = rlt.min.into();
            let max: [u8; 2] = rlt.max.into();
            raw.push(max[0]);
            raw.push(max[1]);
            raw.push(min[0]);
            raw.push(min[1]);
        }
    }

    let checksum = calc_checksum(raw.as_slice());
    raw.push(checksum);

    Command {
        raw: raw.into(),
        decoded: Decoded::SetProbeProfile(probe_idx, threshold),
    }
}
