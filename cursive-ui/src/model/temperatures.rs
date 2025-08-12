use device_controller::model::device::TemperatureMode;
use device_controller::model::device_temperature::InRangeDeviceTemperature;
use std::fmt::Display;

pub struct Celsius(pub InRangeDeviceTemperature);
pub struct Fahrenheit(pub InRangeDeviceTemperature);

pub struct Temperature(pub InRangeDeviceTemperature, pub TemperatureMode);

impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t: f32 = self.0.into();
        write!(f, "{:>1.1}C", t)
    }
}

impl Display for Fahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: f32 = self.0.into();
        let t: f32 = ((c * 9.0) / 5.0) + 32.0;
        write!(f, "{:>1.1}F", t)
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            TemperatureMode::Celsius => write!(f, "{}", Celsius(self.0)),
            TemperatureMode::Fahrenheit => write!(f, "{}", Fahrenheit(self.0)),
        }
    }
}
