#[derive(Clone, Copy, Debug)]
pub struct UpperLimitThreshold {
    pub max: u16,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum AlarmState {
    #[default]
    Unknown,
    NoAlarm,
    Alarm,
}

#[derive(Clone, Copy, Debug)]
pub struct RangeLimitThreshold {
    pub min: u16,
    pub max: u16,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum AlarmThreshold {
    #[default]
    Unknown,
    NoneSet,
    UpperLimit(UpperLimitThreshold),
    RangeLimit(RangeLimitThreshold),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Probe {
    pub temperature: Option<u16>,
    pub alarm: AlarmState,
    pub alarm_threshold: AlarmThreshold,
}
