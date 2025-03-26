use crossterm::event::{KeyCode, KeyEvent};
use crate::ui::ui_state::UiCommands;

pub enum KeypressResult {
    NotHandled,
    Handled,
    Cancel,
    Ok(Option<UiCommands>),
}

pub trait Dialog {
    fn handle_normal_keypress(&mut self, keypress: KeyEvent) -> KeypressResult {
        KeypressResult::NotHandled
    }

    fn draw(&self);

    fn handle_keypress(&mut self, keypress: KeyEvent) -> KeypressResult {
        if keypress.code == KeyCode::Esc {
            return KeypressResult::Cancel;
        }

        self.handle_normal_keypress(keypress)
    }
}

pub type DialogType = Box<dyn Dialog + Send>;
