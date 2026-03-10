use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::{App, Focus, Mode};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    // Global keys (work in all modes)
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
            return;
        }
        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.toggle_sidebar();
            return;
        }
        _ => {}
    }

    match app.focus {
        Focus::Tree => handle_tree_keys(app, key),
        Focus::Editor => match app.mode {
            Mode::Normal => handle_normal_mode(app, key),
            Mode::Insert => handle_insert_mode(app, key),
            Mode::Command => {}
        },
    }
}

fn handle_tree_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
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

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
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
    match key.code {
        KeyCode::Esc | KeyCode::Char('[') if key.code == KeyCode::Esc || key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.mode = Mode::Normal;
            // Move cursor back one (Vim behavior)
            if let Some(editor) = app.active_editor_mut() {
                if editor.cursor.col > 0 {
                    editor.cursor.col -= 1;
                }
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
            // Insert 4 spaces
            if let Some(editor) = app.active_editor_mut() {
                let line = editor.cursor.line;
                let col = editor.cursor.col;
                for i in 0..4 {
                    editor.buffer.insert_char(line, col + i, ' ');
                }
                editor.cursor.col += 4;
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
