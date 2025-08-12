use cursive::utils::markup::StyledString;
use cursive::view::ViewWrapper;
use cursive::views::TextView;
use device_controller::model::probe::{AlarmState, AlarmThreshold, Probe};

pub struct ProbeView {
    inner: TextView,
    probe: Probe,
}

impl ProbeView {
    pub fn new(probe: &Probe) -> Self {
        Self {
            inner: TextView::new(Self::probe_to_content(probe)),
            probe: *probe,
        }
    }

    fn probe_to_content(p: &Probe) -> StyledString {
        let t = match p.temperature {
            None => StyledString::plain("--"),
            Some(t) => StyledString::plain(t.to_string()),
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
                l.append(ult.max.to_string());
                l
            }
            Some(AlarmThreshold::RangeLimit(rlt)) => {
                let mut l = StyledString::plain("Range limit alarm ");
                l.append(rlt.min.to_string());
                l.append(" -> ");
                l.append(rlt.max.to_string());
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

    pub fn update_probe(&mut self, p: &Probe) {
        self.probe = *p;
        self.inner.set_content(Self::probe_to_content(p));
    }
}

impl ViewWrapper for ProbeView {
    cursive::wrap_impl!(self.inner: TextView);
}
