use crate::model::probe::{AlarmThreshold, ProbeIdx};

pub enum CommandRequest {
    ToggleTempMode,
    SetTempMode(bool), // True => celsius, false => Fahrenheit
    ReportAllProfiles,
    ReportProfile(ProbeIdx),
    SetProfile(ProbeIdx, AlarmThreshold),
    AckAlarm,
    CustomCommand(Vec<u8>),
}
