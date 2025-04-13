use crate::model::probe::AlarmThreshold;

pub enum CommandRequest {
    ToggleTempMode,
    ReportAllProfiles,
    ReportProfile(u8),
    SetProfile(u8, AlarmThreshold),
}
