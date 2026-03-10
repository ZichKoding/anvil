pub struct Viewport {
    pub top_line: usize,
    pub height: usize,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            top_line: 0,
            height: 0,
        }
    }

    pub fn ensure_cursor_visible(&mut self, cursor_line: usize) {
        if cursor_line < self.top_line {
            self.top_line = cursor_line;
        } else if cursor_line >= self.top_line + self.height {
            self.top_line = cursor_line - self.height + 1;
        }
    }

    pub fn page_up(&mut self) -> usize {
        let scroll = self.height.saturating_sub(2);
        self.top_line = self.top_line.saturating_sub(scroll);
        scroll
    }

    pub fn page_down(&mut self, total_lines: usize) -> usize {
        let scroll = self.height.saturating_sub(2);
        let max_top = total_lines.saturating_sub(self.height);
        self.top_line = (self.top_line + scroll).min(max_top);
        scroll
    }

    pub fn visible_range(&self) -> std::ops::Range<usize> {
        self.top_line..self.top_line + self.height
    }
}
