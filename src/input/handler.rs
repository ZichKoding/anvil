use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::App;

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Tab => app.toggle_focus(),
        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_sidebar();
        }
        _ => {}
    }
}
