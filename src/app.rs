use ratatui::layout::Rect;

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
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            focus: Focus::Tree,
            mode: Mode::Normal,
            terminal_size: Rect::default(),
            sidebar_visible: true,
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
}
