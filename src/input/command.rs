use crate::app::App;

#[derive(Debug, PartialEq)]
pub enum CommandResult {
    Ok,
    Quit,
    Error(String),
}

pub fn execute_command(input: &str, app: &mut App) -> CommandResult {
    match input.trim() {
        "w" => {
            if let Some(editor) = app.active_editor_mut() {
                match editor.buffer.save() {
                    std::result::Result::Ok(()) => {
                        app.status_message = format!("Saved {}", editor.buffer.filename());
                        CommandResult::Ok
                    }
                    Err(e) => CommandResult::Error(format!("Save failed: {e}")),
                }
            } else {
                CommandResult::Error("No file to save".to_string())
            }
        }
        "q" => {
            if app.editors.iter().any(|e| e.buffer.modified) {
                CommandResult::Error("Unsaved changes. Use :q! to force quit.".to_string())
            } else {
                CommandResult::Quit
            }
        }
        "wq" => {
            if let Some(editor) = app.active_editor_mut() {
                if let Err(e) = editor.buffer.save() {
                    return CommandResult::Error(format!("Save failed: {e}"));
                }
                app.status_message = format!("Saved {}", editor.buffer.filename());
            }
            CommandResult::Quit
        }
        "q!" => CommandResult::Quit,
        other => CommandResult::Error(format!("Unknown command: {}", other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::editor::EditorPane;
    use crate::editor::buffer::Buffer;
    use ropey::Rope;
    use std::path::PathBuf;

    fn make_test_app() -> App {
        App::new(PathBuf::from("."))
    }

    fn make_test_buffer(path: &str, modified: bool) -> Buffer {
        Buffer {
            rope: Rope::from_str("test content"),
            file_path: PathBuf::from(path),
            modified,
        }
    }

    // --- execute_command: :w ---

    #[test]
    fn test_command_w_no_file_returns_error() {
        let mut app = make_test_app();
        let result = execute_command("w", &mut app);
        assert!(matches!(result, CommandResult::Error(_)));
    }

    // --- execute_command: :q ---

    #[test]
    fn test_command_q_clean_buffer_returns_quit() {
        let mut app = make_test_app();
        let buf = make_test_buffer("test.txt", false);
        app.editors.push(EditorPane::new(buf));
        let result = execute_command("q", &mut app);
        assert_eq!(result, CommandResult::Quit);
    }

    #[test]
    fn test_command_q_dirty_buffer_returns_error() {
        let mut app = make_test_app();
        let buf = make_test_buffer("test.txt", true);
        app.editors.push(EditorPane::new(buf));
        let result = execute_command("q", &mut app);
        assert!(matches!(result, CommandResult::Error(_)));
        if let CommandResult::Error(msg) = result {
            assert!(msg.contains("Unsaved changes"));
        }
    }

    #[test]
    fn test_command_q_no_editors_returns_quit() {
        let mut app = make_test_app();
        let result = execute_command("q", &mut app);
        assert_eq!(result, CommandResult::Quit);
    }

    // --- execute_command: :q! ---

    #[test]
    fn test_command_q_bang_dirty_buffer_returns_quit() {
        let mut app = make_test_app();
        let buf = make_test_buffer("test.txt", true);
        app.editors.push(EditorPane::new(buf));
        let result = execute_command("q!", &mut app);
        assert_eq!(result, CommandResult::Quit);
    }

    #[test]
    fn test_command_q_bang_clean_buffer_returns_quit() {
        let mut app = make_test_app();
        let buf = make_test_buffer("test.txt", false);
        app.editors.push(EditorPane::new(buf));
        let result = execute_command("q!", &mut app);
        assert_eq!(result, CommandResult::Quit);
    }

    // --- execute_command: unknown ---

    #[test]
    fn test_command_unknown_returns_error() {
        let mut app = make_test_app();
        let result = execute_command("unknown", &mut app);
        assert!(matches!(result, CommandResult::Error(_)));
        if let CommandResult::Error(msg) = result {
            assert!(msg.contains("Unknown command"));
        }
    }

    #[test]
    fn test_command_empty_returns_error() {
        let mut app = make_test_app();
        let result = execute_command("", &mut app);
        assert!(matches!(result, CommandResult::Error(_)));
    }

    // --- execute_command: whitespace trimming ---

    #[test]
    fn test_command_with_whitespace_trimmed() {
        let mut app = make_test_app();
        let result = execute_command("  q!  ", &mut app);
        assert_eq!(result, CommandResult::Quit);
    }
}
