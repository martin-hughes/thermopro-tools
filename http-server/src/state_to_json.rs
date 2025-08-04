use device_controller::model::device::{TP25State, TemperatureMode};
use device_controller::model::probe::{AlarmState, AlarmThreshold, Probe};
use serde_json::{json, Value};

pub fn state_to_json(state: &TP25State) -> Value {
    if state.connected {
        json!({
            "connected": true,
            "temp_mode": temp_mode_to_string(state.temperature_mode),
            "probes": probes_to_json(&state.probes),
        })
    } else {
        json!({
            "connected": false,
        })
    }
}

fn temp_mode_to_string(mode: Option<TemperatureMode>) -> &'static str {
    match mode {
        Some(TemperatureMode::Celsius) => "celsius",
        Some(TemperatureMode::Fahrenheit) => "fahrenheit",
        None => "unknown",
    }
}

fn alarm_state_to_string(alarm: AlarmState) -> &'static str {
    match alarm {
        AlarmState::Unknown => "unknown",
        AlarmState::Alarm => "alarm",
        AlarmState::NoAlarm => "no_alarm",
    }
}

fn temp_option_to_string(temp: Option<u16>) -> String {
    match temp {
        None => "unknown".to_string(),
        Some(t) => temp_to_string(t),
    }
}

fn temp_to_string(temp: u16) -> String {
    format!("{:.1}", temp as f32 / 10.0)
}

fn alarm_threshold_to_json(threshold: Option<AlarmThreshold>) -> Value {
    match threshold {
        None => json!({"mode": "unknown"}),
        Some(AlarmThreshold::NoneSet) => json!({"mode": "none_set"}),
        Some(AlarmThreshold::UpperLimit(u)) => {
            json!({"mode": "upper_only", "upper": temp_to_string(u.max)})
        }
        Some(AlarmThreshold::RangeLimit(r)) => {
            json!({"mode": "range", "upper": temp_to_string(r.max), "lower": temp_to_string(r.min)})
        }
    }
}

fn probe_to_json(probe: &Probe) -> Value {
    json!({
        "alarm": alarm_state_to_string(probe.alarm),
        "temp": temp_option_to_string(probe.temperature),
        "alarm_threshold": alarm_threshold_to_json(probe.alarm_threshold),
    })
}

fn probes_to_json(probes: &[Probe]) -> Vec<Value> {
    probes.iter().map(probe_to_json).collect()
}
