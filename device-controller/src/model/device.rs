use crate::model::probe::Probe;

#[derive(Clone, Copy, Debug)]
pub enum TemperatureMode {
    Celsius,
    Fahrenheit,
}

#[derive(Clone, Default)]
pub struct TP25State {
    pub probes: [Probe; 4],
    pub temperature_mode: Option<TemperatureMode>,
    pub connected: bool,
}
