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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_viewport(height: usize, width: usize) -> Viewport {
        Viewport {
            top_line: 0,
            height,
            col_offset: 0,
            width,
        }
    }

    // --- ensure_cursor_visible ---

    #[test]
    fn test_ensure_cursor_visible_cursor_in_view_no_change() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 5;
        vp.ensure_cursor_visible(8);
        assert_eq!(vp.top_line, 5);
    }

    #[test]
    fn test_ensure_cursor_visible_cursor_above_scrolls_up() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 10;
        vp.ensure_cursor_visible(5);
        assert_eq!(vp.top_line, 5);
    }

    #[test]
    fn test_ensure_cursor_visible_cursor_below_scrolls_down() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 0;
        vp.ensure_cursor_visible(15); // 15 >= 0+10
        // top_line = 15 - 10 + 1 = 6
        assert_eq!(vp.top_line, 6);
    }

    #[test]
    fn test_ensure_cursor_visible_cursor_at_last_visible_line_no_scroll() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 0;
        vp.ensure_cursor_visible(9); // 9 == 0+10-1, still in view
        assert_eq!(vp.top_line, 0);
    }

    // --- ensure_cursor_col_visible ---

    #[test]
    fn test_ensure_cursor_col_visible_zero_width_noop() {
        let mut vp = Viewport {
            top_line: 0,
            height: 10,
            col_offset: 0,
            width: 0,
        };
        vp.ensure_cursor_col_visible(100);
        assert_eq!(vp.col_offset, 0); // unchanged
    }

    #[test]
    fn test_ensure_cursor_col_visible_col_in_view_no_change() {
        let mut vp = make_viewport(10, 80);
        vp.col_offset = 0;
        vp.ensure_cursor_col_visible(40);
        assert_eq!(vp.col_offset, 0);
    }

    #[test]
    fn test_ensure_cursor_col_visible_col_far_right_scrolls() {
        let mut vp = make_viewport(10, 80);
        vp.col_offset = 0;
        vp.ensure_cursor_col_visible(100);
        assert!(vp.col_offset > 0);
    }

    #[test]
    fn test_ensure_cursor_col_visible_col_left_of_margin_scrolls_left() {
        let mut vp = make_viewport(10, 80);
        vp.col_offset = 50;
        vp.ensure_cursor_col_visible(2); // well left of offset+margin
        assert!(vp.col_offset < 50);
    }

    // --- page_up ---

    #[test]
    fn test_page_up_scrolls_up_by_height_minus_two() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 20;
        let scrolled = vp.page_up();
        assert_eq!(scrolled, 8); // height - 2
        assert_eq!(vp.top_line, 12); // 20 - 8
    }

    #[test]
    fn test_page_up_clamps_at_zero() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 3;
        vp.page_up();
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn test_page_up_at_top_stays_at_zero() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 0;
        vp.page_up();
        assert_eq!(vp.top_line, 0);
    }

    #[test]
    fn test_page_up_height_less_than_two_returns_zero_scroll() {
        let mut vp = make_viewport(1, 80);
        vp.top_line = 5;
        let scrolled = vp.page_up();
        // height.saturating_sub(2) = 0
        assert_eq!(scrolled, 0);
        assert_eq!(vp.top_line, 5); // no scroll
    }

    // --- page_down ---

    #[test]
    fn test_page_down_scrolls_down_by_height_minus_two() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 0;
        let scrolled = vp.page_down(100);
        assert_eq!(scrolled, 8);
        assert_eq!(vp.top_line, 8);
    }

    #[test]
    fn test_page_down_clamps_to_max_top() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 90;
        vp.page_down(100); // max_top = 100 - 10 = 90
        assert_eq!(vp.top_line, 90);
    }

    #[test]
    fn test_page_down_does_not_exceed_total_lines() {
        let mut vp = make_viewport(10, 80);
        vp.top_line = 0;
        vp.page_down(5); // total 5 lines, max_top = 5-10 saturates to 0
        assert_eq!(vp.top_line, 0);
    }
}
