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
        KeyCode::Enter | KeyCode::Char('l') => {
            // If it's a file, open it; if it's a directory, toggle expand
            if app.file_tree.selected_is_file() {
                if let Some(path) = app.file_tree.selected_path().map(|p| p.to_path_buf()) {
                    app.open_file(&path);
                }
            } else {
                app.file_tree.toggle_expand();
            }
        }
        KeyCode::Right => {
            // Expand directory or open file
            if app.file_tree.selected_is_file() {
                if let Some(path) = app.file_tree.selected_path().map(|p| p.to_path_buf()) {
                    app.open_file(&path);
                }
            } else if let Some(idx) = app.file_tree.state.selected() {
                if !app.file_tree.is_expanded(idx) {
                    app.file_tree.toggle_expand();
                }
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
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
        KeyCode::Up | KeyCode::Char('k') => {
            if let Some(editor) = app.active_editor_mut() {
                editor.cursor.move_up();
                let len = editor.current_line_len();
                editor.cursor.clamp_col(len);
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(editor) = app.active_editor_mut() {
                let max = editor.buffer.line_count();
                editor.cursor.move_down(max);
                let len = editor.current_line_len();
                editor.cursor.clamp_col(len);
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if let Some(editor) = app.active_editor_mut() {
                editor.cursor.move_left();
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            if let Some(editor) = app.active_editor_mut() {
                let len = editor.current_line_len();
                editor.cursor.move_right(len);
            }
        }
        KeyCode::PageUp => {
            if let Some(editor) = app.active_editor_mut() {
                let scroll = editor.viewport.page_up();
                editor.cursor.line = editor.cursor.line.saturating_sub(scroll);
            }
        }
        KeyCode::PageDown => {
            if let Some(editor) = app.active_editor_mut() {
                let total = editor.buffer.line_count();
                let scroll = editor.viewport.page_down(total);
                editor.cursor.line = (editor.cursor.line + scroll).min(total.saturating_sub(1));
            }
        }
        KeyCode::Home | KeyCode::Char('0') => {
            if let Some(editor) = app.active_editor_mut() {
                editor.cursor.col = 0;
            }
        }
        KeyCode::End | KeyCode::Char('$') => {
            if let Some(editor) = app.active_editor_mut() {
                let len = editor.current_line_len();
                editor.cursor.col = len;
            }
        }
        _ => {}
    }
}
