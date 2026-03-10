use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::{App, Focus};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    // Global keys
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
            return;
        }
        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_sidebar();
            return;
        }
        KeyCode::Tab => {
            app.toggle_focus();
            return;
        }
        _ => {}
    }

    // Focus-specific keys
    match app.focus {
        Focus::Tree => handle_tree_keys(app, key),
        Focus::Editor => handle_editor_keys(app, key),
    }
}

fn handle_tree_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Up | KeyCode::Char('k') => app.file_tree.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.file_tree.move_down(),
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
            app.file_tree.toggle_expand();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            // Collapse if expanded, otherwise do nothing
            if let Some(idx) = app.file_tree.state.selected() {
                if app.file_tree.is_expanded(idx) {
                    app.file_tree.toggle_expand();
                }
            }
        }
        _ => {}
    }
}

fn handle_editor_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        _ => {}
    }
}
