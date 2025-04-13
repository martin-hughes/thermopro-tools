use cursive::align::HAlign;
use cursive::traits::Nameable;
use cursive::utils::markup::StyledString;
use cursive::views::{NamedView, Panel, TextView};
use cursive::Cursive;
use device_controller::model::device::{TP25State, TemperatureMode};

const STATUS_TEXT_NAME: &str = "status_text";
pub fn make_status_view() -> Panel<NamedView<TextView>> {
    Panel::new(TextView::new(status_view_message(false, None)).with_name(STATUS_TEXT_NAME))
        .title("Device status")
        .title_position(HAlign::Left)
}

pub fn update_status_view(c: &mut Cursive, device_state: &TP25State) {
    c.call_on_name(STATUS_TEXT_NAME, |v: &mut TextView| {
        v.set_content(status_view_message(
            device_state.connected,
            device_state.temperature_mode,
        ))
    });
}

fn status_view_message(connected: bool, temperature_mode: Option<TemperatureMode>) -> StyledString {
    let mut connection_text = StyledString::plain(if connected {
        "Device: Connected"
    } else {
        "Device: DISCONNECTED"
    });

    let temp_mode_text = match temperature_mode {
        None => "Unknown",
        Some(TemperatureMode::Celsius) => "Celsius",
        Some(TemperatureMode::Fahrenheit) => "Fahrenheit",
    };

    connection_text.append("\n");
    connection_text.append(format!("Temperature unit: {}", temp_mode_text));

    connection_text
}
