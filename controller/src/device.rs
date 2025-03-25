use crate::device::DeviceState::{Connected, NotConnected};
use crate::notification::content::NotificationContent;
use crate::notification::Notification;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum TempMode {
    Celsius,
    Fahrenheit,
}

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
        let l = &mut self.state.lock().unwrap();
        match l.deref_mut() {
            Connected(ref mut state) => self.handle_notification_is_connected(notification, state),
            NotConnected => {
                panic!("Received notification in not connected state")
            }
        }
    }

    fn handle_notification_is_connected(
        &self,
        notification: Notification,
        state: &mut DeviceConnectedState,
    ) -> bool {
        match notification.content {
            Some(NotificationContent::ConnectResponse) => false,
            Some(NotificationContent::Temperatures(temps)) => {
                for i in 0..4 {
                    state.celsius = matches!(temps.celsius, Some(TempMode::Celsius));
                    state.probes[i].temperature = temps.temps[i];
                    state.probes[i].alarm = if (temps.alarms & (1 << i)) != 0 {
                        AlarmState::Alarm
                    } else {
                        AlarmState::NoAlarm
                    };
                }

                true
            }
            Some(NotificationContent::TwoSixResponse) => false,
            Some(NotificationContent::SetTempUnit) => false,
            _ => false,
        }
    }

    pub fn get_state(&self) -> DeviceState {
        self.state.lock().unwrap().clone()
    }

    #[allow(unused)]
    pub fn set_state(&self, state: DeviceState) {
        *self.state.lock().unwrap() = state;
    }
}
