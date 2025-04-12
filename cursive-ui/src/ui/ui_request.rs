use crate::model::probe::AlarmThreshold;

pub enum UiRequest {
    ToggleTempMode,
    ReportAllProfiles,
    ReportProfile(u8),
    SetProfile(u8, AlarmThreshold),
}
