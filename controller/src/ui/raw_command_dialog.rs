use crossterm::event::KeyEvent;
use crate::ui::dialog::{Dialog, KeypressResult};

struct RawCommandDialog {

}

impl Dialog for RawCommandDialog {
    fn handle_normal_keypress(&mut self, keypress: KeyEvent) -> KeypressResult {
        KeypressResult::NotHandled
    }

    fn draw(&self) {

    }
}
