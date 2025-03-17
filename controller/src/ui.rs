use crate::device::AlarmState::Alarm;
use crate::device::{AlarmThreshold, Device, DeviceConnectedState, DeviceState, Probe};
//use crossterm::style::Stylize;
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Span;
use ratatui::widgets::Padding;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

pub fn draw_ui(terminal: &mut DefaultTerminal, state: DeviceState, pulse_on: bool) {
    terminal
        .draw(|frame| draw(frame, &state, pulse_on))
        .unwrap();
}

fn draw(frame: &mut Frame, state: &DeviceState, pulse_on: bool) {
    let area = frame.area();
    let title = Line::from(" ThermoPro TP25 ".bold());
    let block = Block::bordered().title(title.centered());

    let chunks = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Min(0),
    ])
    .margin(1)
    .split(area);

    frame.render_widget(block, area);

    render_status(state, pulse_on, frame, chunks[0]);

    match state {
        DeviceState::NotConnected => (),
        DeviceState::Connected(state) => render_probes(state, pulse_on, frame, chunks[1]),
    }
}

fn render_status(state: &DeviceState, pulse_on: bool, frame: &mut Frame, area: Rect) {
    let title = Line::from(" Device status ");
    let block = Block::bordered()
        .title(title.left_aligned())
        .padding(Padding::left(1));

    let status_str = match state {
        DeviceState::NotConnected => "Disconnected",
        DeviceState::Connected(_) => "Connected",
    };

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        "Device status: ".into(),
        status_str.into(),
    ]));

    if let DeviceState::Connected(s) = state {
        let mode_str = if s.celsius { "Celsius" } else { "Fahrenheit" };
        lines.push(Line::from(vec![
            "Temperature mode: ".into(),
            mode_str.into(),
        ]));
        lines.push(Line::from(vec![
            "Alarm status: ".into(),
            if s.has_alarm() {
                let t = "Alarming";
                if pulse_on {
                    t.bold()
                } else {
                    t.into()
                }
            } else {
                "No alarm".into()
            },
        ]))
    }
    frame.render_widget(Paragraph::new(lines).block(block), area);
}
fn render_probes(state: &DeviceConnectedState, pulse_on: bool, frame: &mut Frame, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(area);

    for idx in 0..4 {
        render_probe(
            frame,
            chunks[idx],
            idx,
            state.probes[idx],
            pulse_on,
            state.celsius,
        );
    }
}

fn render_probe(
    frame: &mut Frame,
    area: Rect,
    idx: usize,
    probe: Probe,
    pulse_on: bool,
    celsius: bool,
) {
    let alarm_state = if let Alarm = probe.alarm { true } else { false };
    let bold = pulse_on && alarm_state;

    let mut title_str = " Probe: ".to_string();
    title_str.push_str((idx + 1).to_string().as_str());
    title_str.push(' ');

    let title = Line::from(if bold {
        title_str.bold()
    } else {
        title_str.not_bold()
    });
    let mut block = Block::bordered().title(title.centered());

    if bold {
        block = block.border_set(border::THICK);
    }

    let tb = temp_line(&probe, celsius);
    let mut lines = Vec::new();
    lines.push(tb);
    lines.append(alarm_threshold_lines(&probe, celsius).as_mut());
    frame.render_widget(Paragraph::new(lines).centered().block(block), area);
}

fn temp_line(probe: &Probe, celsius: bool) -> Line {
    let mut pieces = Vec::new();
    pieces.push("Temp: ".into());
    match probe.temperature {
        Some(temp) => {
            pieces.append(temp_to_spans(temp, celsius).as_mut());
        }
        None => pieces.push("--".into()),
    };

    Line::from(pieces)
}

fn temp_to_spans<'a>(temp: u16, celsius: bool) -> Vec<Span<'a>> {
    let mut pieces = Vec::new();
    let f = (temp as f64 * 0.18) + 32.0;
    let f_str = format!("{:.1}", f);

    // Use this special conversion as it is a direct conversion of the thermometer's
    // value with no rounding
    let c_str = bcdish_to_string(temp);

    if celsius {
        pieces.push(c_str.bold());
        pieces.push(" C ".bold());
        pieces.push("/ ".into());
        pieces.push(f_str.dim());
        pieces.push(" F".dim());
    } else {
        pieces.push(c_str.dim());
        pieces.push(" C ".dim());
        pieces.push("/ ".into());
        pieces.push(f_str.bold());
        pieces.push(" F".bold());
    }
    pieces
}

fn alarm_threshold_lines(probe: &Probe, celsius: bool) -> Vec<Line> {
    let mut pieces = Vec::new();

    match probe.alarm_threshold {
        AlarmThreshold::Unknown => pieces.push("Threshold unknown".into()),
        AlarmThreshold::NoneSet => pieces.push("No threshold set".into()),
        AlarmThreshold::UpperLimit(max) => pieces.push(alarm_upper_threshold_line(max, celsius)),
        AlarmThreshold::RangeLimit(max, min) => {
            pieces.push(alarm_upper_threshold_line(max, celsius));
            pieces.push(alarm_lower_threshold_line(min, celsius));
        }
    };

    pieces
}

fn alarm_upper_threshold_line<'a>(l: u16, celsius: bool) -> Line<'a> {
    let mut pieces = Vec::new();

    pieces.push("Max: ".into());
    pieces.append(temp_to_spans(l, celsius).as_mut());

    Line::from(pieces)
}

fn alarm_lower_threshold_line<'a>(l: u16, celsius: bool) -> Line<'a> {
    let mut pieces = Vec::new();

    pieces.push("Min: ".into());
    pieces.append(temp_to_spans(l, celsius).as_mut());

    Line::from(pieces)
}

fn bcdish_to_string(bcdish: u16) -> String {
    let whole = bcdish / 10;
    let tenths = bcdish % 10;

    let mut result = whole.to_string();
    result.push('.');
    result.push_str(&tenths.to_string());

    result
}
