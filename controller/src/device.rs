use crate::device::DeviceState::Connected;
use crate::notification::Notification;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum AlarmState {
    Unknown,
    NoAlarm,
    Alarm,
}

#[derive(Clone, Copy)]
pub struct UpperLimitThreshold {
    pub idx: u16,
    pub max: u16,
}

#[derive(Clone, Copy)]
pub struct RangeLimitThreshold {
    pub idx: u16,
    pub min: u16,
    pub max: u16,
}

#[derive(Clone, Copy)]
pub enum AlarmThreshold {
    Unknown,
    NoneSet,
    UpperLimit(UpperLimitThreshold),
    RangeLimit(RangeLimitThreshold),
}

#[derive(Clone, Copy)]
pub struct Probe {
    pub temperature: Option<u16>,
    pub alarm: AlarmState,
    pub alarm_threshold: AlarmThreshold,
}

impl Probe {
    fn new() -> Probe {
        Probe {
            temperature: None,
            alarm: AlarmState::Unknown,
            alarm_threshold: AlarmThreshold::Unknown,
        }
    }
}

#[derive(Clone)]
pub enum DeviceState {
    NotConnected,
    Connected(DeviceConnectedState),
}

#[derive(Clone)]
pub struct DeviceConnectedState {
    pub probes: [Probe; 4],
    pub celsius: bool,
}

impl Default for DeviceConnectedState {
    fn default() -> DeviceConnectedState {
        DeviceConnectedState {
            probes: [Probe::new(); 4],
            celsius: false,
        }
    }
}

impl DeviceConnectedState {
    pub fn has_alarm(&self) -> bool {
        self.probes
            .iter()
            .any(|p| matches!(p.alarm, AlarmState::Alarm))
    }
}

#[derive(Clone)]
pub struct Device {
    state: Arc<Mutex<DeviceState>>,
}

impl Device {
    pub fn new() -> Self {
        Device {
            state: Arc::new(Mutex::new(Connected(DeviceConnectedState::default()))),
        }
    }

    pub fn handle_notification(&self, notification: Notification) -> bool {
        let mut updated = false;
        match notification {
            Notification::ConnectResponse => {}
            Notification::Temperatures(temps) => {
                let ref mut l = self.state.lock().unwrap();
                match l.deref_mut() {
                    Connected(ref mut state) => {
                        for i in 0..4 {
                            state.probes[i].temperature = temps.temps[i];
                        }
                    }
                    DeviceState::NotConnected => {
                        panic!("Received notification in not connected state")
                    }
                }

                updated = true;
            }
            Notification::TwoSixResponse => {}
        }

        updated
    }

    pub fn get_state(&self) -> DeviceState {
        self.state.lock().unwrap().clone()
    }

    pub fn set_state(&self, state: DeviceState) {
        *self.state.lock().unwrap() = state;
    }
}
