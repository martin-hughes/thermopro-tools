use crate::model::device::TemperatureMode;
use crate::model::probe::AlarmThreshold;
use crate::peripheral::command::{Command, Decoded};
use crate::peripheral::interface::{TP25Receiver, TP25Writer};
use crate::peripheral::notification::Decoded::{
    ReportProbeProfile, SetProbeProfile, SetTempMode, Startup, Temperatures,
};
use crate::peripheral::notification::{
    Notification, ProbeProfileData, RawTemperature, TemperatureData,
};
use bytes::Bytes;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

struct InternalState {
    temp: u16,
    thresholds: [AlarmThreshold; 4],
    mode: TemperatureMode,
    queued_notifications: VecDeque<Notification>,
}

#[derive(Clone)]
pub struct Peripheral {
    internal: Arc<Mutex<InternalState>>,
}

impl Peripheral {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(Mutex::new(InternalState {
                temp: 0,
                mode: TemperatureMode::Celsius,
                queued_notifications: VecDeque::new(),
                thresholds: [AlarmThreshold::default(); 4],
            })),
        }
    }
}

impl TP25Receiver for Peripheral {
    async fn get_notification(&mut self) -> Option<Notification> {
        let n = get_queued_notification(&self.internal);

        match n {
            Some(n) => Some(n),
            None => {
                sleep(Duration::from_secs(1)).await;
                let mut state = self.internal.lock().unwrap();
                state.temp += 1;
                let t = state.temp;
                Some(build_temp_notification(t, state.mode))
            }
        }
    }
}

impl TP25Writer for Peripheral {
    async fn send_cmd(&self, command: Command) -> Result<(), btleplug::Error> {
        let mut state = self.internal.lock().unwrap();
        match command.decoded {
            Decoded::Startup => state.queued_notifications.push_back(Notification {
                raw: mock_raw_bytes(),
                decoded: Startup,
            }),
            Decoded::SetTempMode(mode) => {
                state.queued_notifications.push_back(Notification {
                    raw: mock_raw_bytes(),
                    decoded: SetTempMode,
                });
                state.mode = mode;
            }
            Decoded::ReportProfile(idx) => {
                let t = state.thresholds[idx as usize];
                state.queued_notifications.push_back(Notification {
                    raw: mock_raw_bytes(),
                    decoded: ReportProbeProfile(ProbeProfileData {
                        idx: idx + 1,
                        threshold: t,
                    }),
                });
            }
            Decoded::SetProbeProfile(idx, profile) => {
                state.thresholds[idx as usize] = profile;
                state.queued_notifications.push_back(Notification {
                    raw: mock_raw_bytes(),
                    decoded: SetProbeProfile,
                });
            }
        };
        Ok(())
    }
}

fn get_queued_notification(internal: &Arc<Mutex<InternalState>>) -> Option<Notification> {
    let mut state = internal.lock().unwrap();
    state.queued_notifications.pop_front()
}

fn build_temp_notification(t: u16, mode: TemperatureMode) -> Notification {
    let temps = (0..4)
        .map(|i| RawTemperature {
            temp: Some(t + i),
            alarm: false,
        })
        .collect::<Vec<_>>();
    Notification {
        raw: mock_raw_bytes(),
        decoded: Temperatures(TemperatureData {
            temps: temps.try_into().unwrap(),
            temp_mode: mode,
        }),
    }
}

fn mock_raw_bytes() -> Bytes {
    Bytes::from([1u8, 2u8].as_slice())
}
