use crate::app::{App, Focus, Mode};
use crate::config::keybindings::KeybindingMode;
use crate::input::command::{self, CommandResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    // Global keys (work in all modes)
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
            return;
        }
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
            return;
        }
        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_sidebar();
            return;
        }
        // Tab switching: Ctrl+PageDown/PageUp or Ctrl+Tab not available in terminals,
        // so use Ctrl+N/P for next/prev tab
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.next_editor();
            return;
        }
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.prev_editor();
            return;
        }
        KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.close_active_editor();
            return;
        }
        _ => {}
    }

    // Handle command mode before focus dispatch (works regardless of focus)
    if app.mode == Mode::Command {
        handle_command_mode(app, key);
        return;
    }

    match app.focus {
        Focus::Tree => handle_tree_keys(app, key),
        Focus::Editor => {
            let is_vim = app.config.general.keybinding_mode == KeybindingMode::Vim;
            if is_vim {
                match app.mode {
                    Mode::Normal => handle_normal_mode(app, key),
                    Mode::Insert => handle_insert_mode(app, key),
                    Mode::Command => unreachable!("command mode is intercepted before focus dispatch"),
                }
            } else {
                // VS Code mode: always insert
                handle_insert_mode(app, key);
            }
        }
    }
}

fn handle_tree_keys(app: &mut App, key: KeyEvent) {
    let is_vim = app.config.general.keybinding_mode == KeybindingMode::Vim;
    match key.code {
        KeyCode::Char('q') if is_vim => app.quit(),
        KeyCode::Tab => app.toggle_focus(),
        KeyCode::Up | KeyCode::Char('k') => app.file_tree.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.file_tree.move_down(),
        KeyCode::Enter | KeyCode::Char('l') => {
            if app.file_tree.selected_is_file() {
                if let Some(path) = app.file_tree.selected_path().map(|p| p.to_path_buf()) {
                    app.open_file(&path);
                }
            } else {
                app.file_tree.toggle_expand();
            }
        }
        KeyCode::Right => {
            if app.file_tree.selected_is_file() {
                if let Some(path) = app.file_tree.selected_path().map(|p| p.to_path_buf()) {
                    app.open_file(&path);
                }
            } else if let Some(idx) = app.file_tree.state.selected()
                && !app.file_tree.is_expanded(idx)
            {
                app.file_tree.toggle_expand();
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if let Some(idx) = app.file_tree.state.selected()
                && app.file_tree.is_expanded(idx)
            {
                app.file_tree.toggle_expand();
            }
        }
        _ => {}
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(':') => {
            app.mode = Mode::Command;
            app.command_buffer.clear();
            app.status_message = String::from(":");
        }
        KeyCode::Char('q') => app.quit(),
        KeyCode::Tab => app.toggle_focus(),
        KeyCode::Char('i') => {
            app.mode = Mode::Insert;
            app.status_message = String::from("-- INSERT --");
        }
        KeyCode::Char('a') => {
            // Append: enter insert after cursor
            if let Some(editor) = app.active_editor_mut() {
                let len = editor.current_line_len();
                if editor.cursor.col < len {
                    editor.cursor.col += 1;
                }
            }
            app.mode = Mode::Insert;
            app.status_message = String::from("-- INSERT --");
        }
        KeyCode::Char('A') => {
            // Append at end of line
            if let Some(editor) = app.active_editor_mut() {
                let len = editor.current_line_len();
                editor.cursor.col = len;
            }
            app.mode = Mode::Insert;
            app.status_message = String::from("-- INSERT --");
        }
        KeyCode::Char('o') => {
            // Open line below
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let line_len = editor.buffer.line_len_chars(line);
                editor.buffer.insert_newline(line, line_len);
                editor.cursor.line += 1;
                editor.cursor.col = 0;
                reparse_highlighter(editor);
            }
            app.mode = Mode::Insert;
            app.status_message = String::from("-- INSERT --");
        }
        KeyCode::Char('O') => {
            // Open line above
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                editor.buffer.insert_newline(line, 0);
                editor.cursor.col = 0;
                reparse_highlighter(editor);
            }
            app.mode = Mode::Insert;
            app.status_message = String::from("-- INSERT --");
        }
        KeyCode::Char('x') => {
            // Delete char under cursor
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let col = editor.cursor.col;
                editor.buffer.delete_char_at(line, col);
                let len = editor.current_line_len();
                if editor.cursor.col >= len && len > 0 {
                    editor.cursor.col = len - 1;
                }
                reparse_highlighter(editor);
            }
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::NONE) => {
            // dd would delete line, but we'll handle single 'd' as noop for now
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            save_current_file(app);
        }
        // Navigation
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
        KeyCode::Char('G') => {
            // Go to end of file
            if let Some(editor) = app.active_editor_mut() {
                let last = editor.buffer.line_count().saturating_sub(1);
                editor.cursor.line = last;
                editor.cursor.col = 0;
            }
        }
        KeyCode::Char('g') => {
            // gg = go to top (simplified: single g goes to top)
            if let Some(editor) = app.active_editor_mut() {
                editor.cursor.line = 0;
                editor.cursor.col = 0;
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
        _ => {}
    }
}

fn handle_insert_mode(app: &mut App, key: KeyEvent) {
    let is_vim = app.config.general.keybinding_mode == KeybindingMode::Vim;

    match key.code {
        KeyCode::Esc => {
            if is_vim {
                app.mode = Mode::Normal;
                if let Some(editor) = app.active_editor_mut()
                    && editor.cursor.col > 0
                {
                    editor.cursor.col -= 1;
                }
                app.status_message = String::from("Ready");
            }
            // In VS Code mode, Esc does nothing (always in insert)
        }
        KeyCode::Char('[') if key.modifiers.contains(KeyModifiers::CONTROL) && is_vim => {
            app.mode = Mode::Normal;
            if let Some(editor) = app.active_editor_mut()
                && editor.cursor.col > 0
            {
                editor.cursor.col -= 1;
            }
            app.status_message = String::from("Ready");
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            save_current_file(app);
        }
        KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let col = editor.cursor.col;
                editor.buffer.insert_char(line, col, ch);
                editor.cursor.col += 1;
                reparse_highlighter(editor);
            }
        }
        KeyCode::Enter => {
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let col = editor.cursor.col;
                editor.buffer.insert_newline(line, col);
                editor.cursor.line += 1;
                editor.cursor.col = 0;
                reparse_highlighter(editor);
            }
        }
        KeyCode::Backspace => {
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let col = editor.cursor.col;
                if let Some((new_line, new_col)) = editor.buffer.delete_char_before(line, col) {
                    editor.cursor.line = new_line;
                    editor.cursor.col = new_col;
                    reparse_highlighter(editor);
                }
            }
        }
        KeyCode::Delete => {
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let col = editor.cursor.col;
                editor.buffer.delete_char_at(line, col);
                reparse_highlighter(editor);
            }
        }
        KeyCode::Tab => {
            let tab_size = app.config.editor.tab_size;
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let col = editor.cursor.col;
                for i in 0..tab_size {
                    editor.buffer.insert_char(line, col + i, ' ');
                }
                editor.cursor.col += tab_size;
                reparse_highlighter(editor);
            }
        }
        // Arrow key navigation in insert mode
        KeyCode::Up => {
            if let Some(editor) = app.active_editor_mut() {
                editor.cursor.move_up();
                let len = editor.current_line_len();
                editor.cursor.clamp_col(len);
            }
        }
        KeyCode::Down => {
            if let Some(editor) = app.active_editor_mut() {
                let max = editor.buffer.line_count();
                editor.cursor.move_down(max);
                let len = editor.current_line_len();
                editor.cursor.clamp_col(len);
            }
        }
        KeyCode::Left => {
            if let Some(editor) = app.active_editor_mut() {
                editor.cursor.move_left();
            }
        }
        KeyCode::Right => {
            if let Some(editor) = app.active_editor_mut() {
                let len = editor.current_line_len();
                editor.cursor.move_right(len);
            }
        }
        KeyCode::Home => {
            if let Some(editor) = app.active_editor_mut() {
                editor.cursor.col = 0;
            }
        }
        KeyCode::End => {
            if let Some(editor) = app.active_editor_mut() {
                let len = editor.current_line_len();
                editor.cursor.col = len;
            }
        }
        _ => {}
    }
}

fn handle_command_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            app.command_buffer.clear();
            app.status_message = String::from("Ready");
        }
        KeyCode::Enter => {
            let input = app.command_buffer.clone();
            app.mode = Mode::Normal;
            let result = command::execute_command(&input, app);
            match result {
                CommandResult::Saved => {}
                CommandResult::Quit => app.quit(),
                CommandResult::Error(msg) => {
                    app.status_message = msg;
                }
            }
            app.command_buffer.clear();
        }
        KeyCode::Backspace => {
            if app.command_buffer.is_empty() {
                app.mode = Mode::Normal;
                app.status_message = String::from("Ready");
            } else {
                app.command_buffer.pop();
            }
        }
        KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.command_buffer.push(ch);
        }
        _ => {}
    }
}

fn save_current_file(app: &mut App) {
    if let Some(editor) = app.active_editor_mut() {
        match editor.buffer.save() {
            Ok(()) => {
                app.status_message = format!("Saved {}", editor.buffer.filename());
            }
            Err(e) => {
                app.status_message = format!("Save failed: {e}");
            }
        }
    }
}

fn reparse_highlighter(editor: &mut crate::editor::EditorPane) {
    let source = editor.buffer.rope.to_string();
    editor.highlighter.parse(&source);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, Mode};
    use crate::editor::EditorPane;
    use crate::editor::buffer::Buffer;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ropey::Rope;
    use std::path::PathBuf;

    fn make_test_app() -> App {
        App::new(PathBuf::from("."))
    }

    fn make_test_buffer(modified: bool) -> Buffer {
        Buffer {
            rope: Rope::from_str("test content"),
            file_path: PathBuf::from("test.txt"),
            modified,
        }
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    // --- Mode transitions ---

    #[test]
    fn test_colon_enters_command_mode() {
        let mut app = make_test_app();
        app.mode = Mode::Normal;
        app.focus = crate::app::Focus::Editor;
        handle_key_event(&mut app, key(KeyCode::Char(':')));
        assert_eq!(app.mode, Mode::Command);
        assert!(app.command_buffer.is_empty());
    }

    #[test]
    fn test_esc_exits_command_mode() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        handle_key_event(&mut app, key(KeyCode::Esc));
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn test_enter_exits_command_mode() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        app.command_buffer = String::from("q!");
        handle_key_event(&mut app, key(KeyCode::Enter));
        assert_eq!(app.mode, Mode::Normal);
    }

    // --- Command buffer typing ---

    #[test]
    fn test_typing_appends_to_command_buffer() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        handle_key_event(&mut app, key(KeyCode::Char('w')));
        assert_eq!(app.command_buffer, "w");
        handle_key_event(&mut app, key(KeyCode::Char('q')));
        assert_eq!(app.command_buffer, "wq");
    }

    #[test]
    fn test_backspace_removes_from_command_buffer() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        app.command_buffer = String::from("wq");
        handle_key_event(&mut app, key(KeyCode::Backspace));
        assert_eq!(app.command_buffer, "w");
    }

    #[test]
    fn test_backspace_empty_buffer_exits_command_mode() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        app.command_buffer.clear();
        handle_key_event(&mut app, key(KeyCode::Backspace));
        assert_eq!(app.mode, Mode::Normal);
    }

    // --- Command execution via Enter ---

    #[test]
    fn test_enter_q_bang_quits() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        app.command_buffer = String::from("q!");
        handle_key_event(&mut app, key(KeyCode::Enter));
        assert!(!app.running);
    }

    #[test]
    fn test_enter_q_with_dirty_buffer_shows_error() {
        let mut app = make_test_app();
        let buf = make_test_buffer(true);
        app.editors.push(EditorPane::new(buf));
        app.mode = Mode::Command;
        app.command_buffer = String::from("q");
        handle_key_event(&mut app, key(KeyCode::Enter));
        assert!(app.running); // Should NOT quit
        assert!(app.status_message.contains("Unsaved changes"));
    }

    #[test]
    fn test_enter_unknown_command_shows_error() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        app.command_buffer = String::from("xyz");
        handle_key_event(&mut app, key(KeyCode::Enter));
        assert!(app.running);
        assert!(app.status_message.contains("Unknown command"));
    }

    // --- Ctrl+key guard in command mode ---

    #[test]
    fn test_ctrl_a_in_command_mode_does_not_append() {
        let mut app = make_test_app();
        app.mode = Mode::Command;
        app.command_buffer.clear();
        let ctrl_a = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        handle_key_event(&mut app, ctrl_a);
        assert!(
            app.command_buffer.is_empty(),
            "Ctrl+A should not append to command buffer"
        );
    }

    // --- Command mode works regardless of focus ---

    #[test]
    fn test_command_mode_works_with_tree_focus() {
        let mut app = make_test_app();
        app.focus = crate::app::Focus::Tree;
        app.mode = Mode::Command;
        app.command_buffer = String::from("q!");
        handle_key_event(&mut app, key(KeyCode::Enter));
        assert!(!app.running);
    }
}
