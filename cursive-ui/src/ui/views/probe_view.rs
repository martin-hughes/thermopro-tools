use crate::model::temperatures::Temperature;
use cursive::utils::markup::StyledString;
use cursive::view::ViewWrapper;
use cursive::views::TextView;
use device_controller::model::device::TemperatureMode;
use device_controller::model::device_temperature::DeviceTemperature::{InRange, OutOfRange};
use device_controller::model::probe::{AlarmState, AlarmThreshold, Probe};

pub struct ProbeView {
    inner: TextView,
    probe: Probe,
}

impl ProbeView {
    pub fn new(probe: &Probe) -> Self {
        Self {
            inner: TextView::new(Self::probe_to_content(probe, &TemperatureMode::Celsius)),
            probe: *probe,
        }
    }

    fn probe_to_content(p: &Probe, temp_mode: &TemperatureMode) -> StyledString {
        let t = match p.temperature {
            OutOfRange => StyledString::plain("--"),
            InRange(t) => StyledString::plain(Temperature(t, *temp_mode).to_string()),
        };

        let a = match p.alarm {
            AlarmState::Unknown => StyledString::plain("Alarm state unknown"),
            AlarmState::NoAlarm => StyledString::plain("No alarm"),
            AlarmState::Alarm => StyledString::plain("ALARM"),
        };

        let at = match p.alarm_threshold {
            None => StyledString::plain("Alarm threshold unknown"),
            Some(AlarmThreshold::NoneSet) => StyledString::plain("No alarm set"),
            Some(AlarmThreshold::UpperLimit(ult)) => {
                let mut l = StyledString::plain("Upper limit alarm ");
                l.append(Temperature(ult.max, *temp_mode).to_string());
                l
            }
            Some(AlarmThreshold::RangeLimit(rlt)) => {
                let mut l = StyledString::plain("Range limit alarm ");
                l.append(Temperature(rlt.min, *temp_mode).to_string());
                l.append(" -> ");
                l.append(Temperature(rlt.max, *temp_mode).to_string());
                l
            }
        };

        let mut styled = t;
        styled.append("\n");
        styled.append(a);
        styled.append("\n");
        styled.append(at);

        styled
    }

    pub fn update_probe(&mut self, p: &Probe, temp_mode: &TemperatureMode) {
        self.probe = *p;
        self.inner.set_content(Self::probe_to_content(p, temp_mode));
    }
}

impl ViewWrapper for ProbeView {
    cursive::wrap_impl!(self.inner: TextView);
}
