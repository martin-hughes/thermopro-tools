use crate::model::probe::AlarmThreshold;

pub enum CommandRequest {
    ToggleTempMode,
    SetTempMode(bool), // True => celsius, false => Fahrenheit
    ReportAllProfiles,
    ReportProfile(u8),
    SetProfile(u8, AlarmThreshold),
}
