#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub line: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { line: 0, col: 0 }
    }

    pub fn move_up(&mut self) {
        self.line = self.line.saturating_sub(1);
    }

    pub fn move_down(&mut self, max_line: usize) {
        if self.line + 1 < max_line {
            self.line += 1;
        }
    }

    pub fn move_left(&mut self) {
        self.col = self.col.saturating_sub(1);
    }

    pub fn move_right(&mut self, line_len: usize) {
        if self.col < line_len {
            self.col += 1;
        }
    }

    pub fn clamp_col(&mut self, line_len: usize) {
        if self.col > line_len {
            self.col = line_len;
        }
    }
}
