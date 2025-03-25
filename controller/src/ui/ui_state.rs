use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

pub enum UiCommands {
    Quit,
    ToggleCelsius,
}

#[derive(Default)]
pub struct UiState {
    //pub quit: bool,
}

impl UiState {
    pub fn handle_event(&mut self, event: Event) -> Option<UiCommands> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => None,
        }
    }

    fn handle_key_press(&mut self, key: KeyEvent) -> Option<UiCommands> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(UiCommands::Quit),
            KeyCode::Char('c') => Some(UiCommands::ToggleCelsius),
            _ => None,
        }
    }
}
