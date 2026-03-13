use crate::config::Config;
use crate::config::keybindings::KeybindingMode;
use crate::editor::EditorPane;
use crate::editor::buffer::Buffer;
use crate::theme::Theme;
use crate::tree::FileTree;
use ratatui::layout::Rect;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Tree,
    Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

pub struct App {
    pub running: bool,
    pub focus: Focus,
    pub mode: Mode,
    pub terminal_size: Rect,
    pub sidebar_visible: bool,
    pub file_tree: FileTree,
    pub editors: Vec<EditorPane>,
    pub active_editor: usize,
    pub status_message: String,
    pub command_buffer: String,
    pub theme: Theme,
    pub config: Config,
}

impl App {
    pub fn new(root: PathBuf) -> Self {
        let config = Config::load();

        // Set initial mode based on keybinding config
        let mode = match config.general.keybinding_mode {
            KeybindingMode::Vim => Mode::Normal,
            KeybindingMode::Vscode => Mode::Insert, // VS Code is always insert
        };

        Self {
            running: true,
            focus: Focus::Tree,
            mode,
            terminal_size: Rect::default(),
            sidebar_visible: true,
            file_tree: FileTree::new(root),
            editors: Vec::new(),
            active_editor: 0,
            status_message: String::from("Ready"),
            command_buffer: String::new(),
            theme: Theme::default_theme(),
            config,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Tree => Focus::Editor,
            Focus::Editor => Focus::Tree,
        };
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
        if !self.sidebar_visible && self.focus == Focus::Tree {
            self.focus = Focus::Editor;
        }
    }

    pub fn open_file(&mut self, path: &Path) {
        for (i, editor) in self.editors.iter().enumerate() {
            if editor.buffer.file_path == path {
                self.active_editor = i;
                self.focus = Focus::Editor;
                self.status_message = format!("Switched to {}", editor.buffer.filename());
                return;
            }
        }

        match Buffer::from_file(path) {
            Ok(buffer) => {
                let name = buffer.filename().to_string();
                let lines = buffer.line_count();
                self.editors.push(EditorPane::new(buffer));
                self.active_editor = self.editors.len() - 1;
                self.focus = Focus::Editor;
                self.status_message = format!("{} ({} lines)", name, lines);
            }
            Err(e) => {
                self.status_message = format!("Error: {e}");
            }
        }
    }

    pub fn active_editor(&self) -> Option<&EditorPane> {
        self.editors.get(self.active_editor)
    }

    pub fn active_editor_mut(&mut self) -> Option<&mut EditorPane> {
        self.editors.get_mut(self.active_editor)
    }

    pub fn next_editor(&mut self) {
        if !self.editors.is_empty() {
            self.active_editor = (self.active_editor + 1) % self.editors.len();
        }
    }

    pub fn prev_editor(&mut self) {
        if !self.editors.is_empty() {
            self.active_editor = if self.active_editor == 0 {
                self.editors.len() - 1
            } else {
                self.active_editor - 1
            };
        }
    }

    pub fn close_active_editor(&mut self) {
        if self.editors.is_empty() {
            return;
        }
        self.editors.remove(self.active_editor);
        if self.active_editor >= self.editors.len() && !self.editors.is_empty() {
            self.active_editor = self.editors.len() - 1;
        }
        if self.editors.is_empty() {
            self.focus = Focus::Tree;
        }
    }
}
