mod device;
mod notification;
mod notifications;
mod ui;

use crate::device::DeviceState::Connected;
use crate::device::{
    AlarmState, AlarmThreshold, Device, DeviceConnectedState, DeviceState, RangeLimitThreshold,
    UpperLimitThreshold,
};
use crate::ui::draw_ui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    event, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::error::Error;
use std::io::stdout;
use std::panic::{set_hook, take_hook};
use std::time::{Duration, Instant};

struct State {
    device: Device,
    quit: bool,
}

impl State {
    fn new() -> State {
        State {
            device: Device::new(),
            quit: false,
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

fn handle_keyboard(state: &mut State) -> Result<(), Box<dyn Error>> {
    let timeout = Duration::from_secs_f64(1.0 / 50.0);
    if !event::poll(timeout)? {
        return Ok(());
    }
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_key_press(state, key),
        _ => {}
    }
    Ok(())
}

fn handle_key_press(state: &mut State, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => state.quit = true,
        KeyCode::Char('c') => toggle_celsius(state),
        _ => {}
    };
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

    let d = Connected(dcs);

    state.device.set_state(d);

    let start = Instant::now();

    while !state.quit {
        draw_ui(
            &mut terminal,
            state.device.get_state(),
            start.elapsed().as_millis() % 1500 > 750,
        );
        handle_keyboard(&mut state)?
    }

    restore_tui()?;

    Ok(())
}
