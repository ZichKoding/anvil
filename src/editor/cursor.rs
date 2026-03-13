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

#[cfg(test)]
mod tests {
    use super::*;

    // --- new ---

    #[test]
    fn test_new_starts_at_origin() {
        let c = Cursor::new();
        assert_eq!(c.line, 0);
        assert_eq!(c.col, 0);
    }

    // --- move_up ---

    #[test]
    fn test_move_up_decrements_line() {
        let mut c = Cursor { line: 3, col: 0 };
        c.move_up();
        assert_eq!(c.line, 2);
    }

    #[test]
    fn test_move_up_saturates_at_zero() {
        let mut c = Cursor::new();
        c.move_up(); // already at 0
        assert_eq!(c.line, 0);
    }

    // --- move_down ---

    #[test]
    fn test_move_down_increments_line() {
        let mut c = Cursor { line: 0, col: 0 };
        c.move_down(5);
        assert_eq!(c.line, 1);
    }

    #[test]
    fn test_move_down_does_not_exceed_max() {
        let mut c = Cursor { line: 4, col: 0 };
        c.move_down(5); // max_line is 5, line+1 == 5, not < 5 -> no move
        assert_eq!(c.line, 4);
    }

    #[test]
    fn test_move_down_at_last_line_stays() {
        let mut c = Cursor { line: 2, col: 0 };
        c.move_down(3); // 2+1 == 3, not < 3
        assert_eq!(c.line, 2);
    }

    // --- move_left ---

    #[test]
    fn test_move_left_decrements_col() {
        let mut c = Cursor { line: 0, col: 5 };
        c.move_left();
        assert_eq!(c.col, 4);
    }

    #[test]
    fn test_move_left_saturates_at_zero() {
        let mut c = Cursor::new();
        c.move_left();
        assert_eq!(c.col, 0);
    }

    // --- move_right ---

    #[test]
    fn test_move_right_increments_col() {
        let mut c = Cursor { line: 0, col: 0 };
        c.move_right(5);
        assert_eq!(c.col, 1);
    }

    #[test]
    fn test_move_right_does_not_exceed_line_len() {
        let mut c = Cursor { line: 0, col: 5 };
        c.move_right(5); // col == line_len, no move
        assert_eq!(c.col, 5);
    }

    #[test]
    fn test_move_right_allowed_up_to_line_len() {
        let mut c = Cursor { line: 0, col: 4 };
        c.move_right(5); // col < line_len -> move
        assert_eq!(c.col, 5);
    }

    // --- clamp_col ---

    #[test]
    fn test_clamp_col_within_bounds_unchanged() {
        let mut c = Cursor { line: 0, col: 3 };
        c.clamp_col(5);
        assert_eq!(c.col, 3);
    }

    #[test]
    fn test_clamp_col_beyond_line_len_clamped() {
        let mut c = Cursor { line: 0, col: 10 };
        c.clamp_col(4);
        assert_eq!(c.col, 4);
    }

    #[test]
    fn test_clamp_col_equal_to_line_len_unchanged() {
        let mut c = Cursor { line: 0, col: 5 };
        c.clamp_col(5);
        assert_eq!(c.col, 5);
    }
}
