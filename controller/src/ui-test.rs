mod device;
mod device_types;
mod notification;
mod notifications;
mod transfer_log;
mod ui;
mod ui_state;

use crate::device::DeviceState::Connected;
use crate::device::{
    AlarmState, AlarmThreshold, Device, DeviceConnectedState, DeviceState, RangeLimitThreshold,
    UpperLimitThreshold,
};
use crate::transfer_log::{TransferLog, TransferType};
use crate::ui::draw_ui;
use crate::ui_state::{UiCommands, UiState};
use bytes::Bytes;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    event, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use std::error::Error;
use std::io::stdout;
use std::panic::{set_hook, take_hook};
use std::time::{Duration, Instant};

struct State {
    device: Device,
    transfers: TransferLog,
    ui_state: UiState,
}

impl State {
    fn new() -> State {
        State {
            device: Device::new(),
            transfers: TransferLog::new(),
            ui_state: UiState::default(),
        }
    }
}

fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

pub fn restore_tui() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    ratatui::restore();
    Ok(())
}

fn handle_keyboard(state: &mut UiState) -> Result<Option<UiCommands>, Box<dyn Error>> {
    let timeout = Duration::from_secs_f64(1.0 / 50.0);
    if !event::poll(timeout)? {
        return Ok(None);
    }
    let e = event::read()?;
    Ok(state.handle_event(e))
}

fn toggle_celsius(state: &mut State) {
    let d = state.device.get_state();
    if let DeviceState::Connected(mut dcs) = d {
        dcs.celsius = !dcs.celsius;
        state.device.set_state(DeviceState::Connected(dcs));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    init_panic_hook();

    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");

    let mut state = State::new();

    let mut dcs = DeviceConnectedState::default();

    dcs.probes[0].alarm_threshold = AlarmThreshold::Unknown;
    dcs.probes[1].temperature = Some(238);
    dcs.probes[1].alarm = AlarmState::Alarm;
    dcs.probes[1].alarm_threshold = AlarmThreshold::RangeLimit(RangeLimitThreshold {
        idx: 2,
        min: 200,
        max: 2999,
    });
    dcs.probes[2].temperature = Some(2999);
    dcs.probes[2].alarm_threshold = AlarmThreshold::NoneSet;
    dcs.probes[3].alarm_threshold =
        AlarmThreshold::UpperLimit(UpperLimitThreshold { idx: 5, max: 300 });
    dcs.celsius = true;

    for i in 0..35 {
        // I feel like this is probably not very neat, because it took ChatGPT three attempts to
        // write it! Probably ought to do some digging into random numbers...
        let mut random_bytes = vec![0u8; 20];
        rand::rng().fill(&mut random_bytes[..]);
        let random_bytes = Bytes::from(random_bytes);

        state.transfers.push_transfer(
            if i % 2 == 0 {
                TransferType::Command
            } else {
                TransferType::Notification
            },
            random_bytes,
        );
    }

    let d = Connected(dcs);

    state.device.set_state(d);

    let start = Instant::now();

    loop {
        draw_ui(
            &mut terminal,
            state.device.get_state(),
            state.transfers.get_transfers(),
            start.elapsed().as_millis() % 1500 > 750,
        );
        if let Some(cmd) = handle_keyboard(&mut state.ui_state)? {
            match cmd {
                UiCommands::ToggleCelsius => {
                    toggle_celsius(&mut state);
                }
                UiCommands::Quit => {
                    break;
                }
            }
        }
    }

    restore_tui()?;

    Ok(())
}
