pub struct Viewport {
    pub top_line: usize,
    pub height: usize,
    pub col_offset: usize,
    pub width: usize,
}

const H_SCROLL_MARGIN: usize = 6;

impl Viewport {
    pub fn new() -> Self {
        Self {
            top_line: 0,
            height: 0,
            col_offset: 0,
            width: 0,
        }
    }

    pub fn ensure_cursor_visible(&mut self, cursor_line: usize) {
        if cursor_line < self.top_line {
            self.top_line = cursor_line;
        } else if cursor_line >= self.top_line + self.height {
            self.top_line = cursor_line - self.height + 1;
        }
    }

    pub fn ensure_cursor_col_visible(&mut self, cursor_col: usize) {
        if self.width == 0 {
            return;
        }

        let margin = H_SCROLL_MARGIN.min(self.width / 2);

        if cursor_col < self.col_offset.saturating_add(margin) {
            self.col_offset = cursor_col.saturating_sub(margin);
        } else if cursor_col >= self.col_offset + self.width.saturating_sub(margin) {
            self.col_offset = cursor_col.saturating_sub(self.width.saturating_sub(margin + 1));
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
