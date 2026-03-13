use ropey::Rope;
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB guard

pub struct Buffer {
    pub rope: Rope,
    pub file_path: PathBuf,
    pub modified: bool,
}

impl Buffer {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        if metadata.len() > MAX_FILE_SIZE {
            return Err(io::Error::other(format!(
                "File too large ({} bytes, max {})",
                metadata.len(),
                MAX_FILE_SIZE
            )));
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader)?;

        Ok(Self {
            rope,
            file_path: path.to_path_buf(),
            modified: false,
        })
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line(&self, idx: usize) -> Option<ropey::RopeSlice<'_>> {
        if idx < self.rope.len_lines() {
            Some(self.rope.line(idx))
        } else {
            None
        }
    }

    pub fn filename(&self) -> &str {
        self.file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("[unnamed]")
    }

    pub fn insert_char(&mut self, line: usize, col: usize, ch: char) {
        let line_start = self.rope.line_to_char(line);
        let char_idx = line_start + col;
        if char_idx <= self.rope.len_chars() {
            self.rope.insert_char(char_idx, ch);
            self.modified = true;
        }
    }

    pub fn insert_newline(&mut self, line: usize, col: usize) {
        let line_start = self.rope.line_to_char(line);
        let char_idx = line_start + col;
        if char_idx <= self.rope.len_chars() {
            self.rope.insert_char(char_idx, '\n');
            self.modified = true;
        }
    }

    pub fn delete_char_before(&mut self, line: usize, col: usize) -> Option<(usize, usize)> {
        let line_start = self.rope.line_to_char(line);
        let char_idx = line_start + col;

        if char_idx == 0 {
            return None;
        }

        self.rope.remove(char_idx - 1..char_idx);
        self.modified = true;

        if col == 0 && line > 0 {
            // Joined with previous line
            let prev_line_len = self.line_len_chars(line - 1);
            Some((line - 1, prev_line_len))
        } else {
            Some((line, col - 1))
        }
    }

    pub fn delete_char_at(&mut self, line: usize, col: usize) {
        let line_start = self.rope.line_to_char(line);
        let char_idx = line_start + col;

        if char_idx < self.rope.len_chars() {
            self.rope.remove(char_idx..char_idx + 1);
            self.modified = true;
        }
    }

    pub fn line_len_chars(&self, line: usize) -> usize {
        if line >= self.rope.len_lines() {
            return 0;
        }
        let s = self.rope.line(line);
        let text = s.as_str().unwrap_or("");
        text.trim_end_matches('\n').trim_end_matches('\r').len()
    }

    /// Atomic save: write to temp file then rename (crash-safe, SD card safe)
    pub fn save(&mut self) -> io::Result<()> {
        let dir = self.file_path.parent().unwrap_or(Path::new("."));
        let temp_path = dir.join(format!(".anvil-save-{}", std::process::id()));

        // Write to temp file
        {
            let mut file = File::create(&temp_path)?;
            for chunk in self.rope.chunks() {
                file.write_all(chunk.as_bytes())?;
            }
            file.sync_all()?;
        }

        // Atomic rename
        fs::rename(&temp_path, &self.file_path)?;
        self.modified = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn make_buffer(content: &str) -> Buffer {
        Buffer {
            rope: Rope::from_str(content),
            file_path: PathBuf::from("test_buffer.txt"),
            modified: false,
        }
    }

    fn write_temp_file(content: &str) -> (std::path::PathBuf, std::fs::File) {
        let path = std::env::temp_dir().join(format!(
            "anvil_test_{}.txt",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f.flush().unwrap();
        (path, f)
    }

    // --- from_file ---

    #[test]
    fn test_from_file_loads_content() {
        let (path, _f) = write_temp_file("hello\nworld\n");
        let buf = Buffer::from_file(&path).unwrap();
        assert_eq!(buf.line_count(), 3); // ropey counts trailing newline as extra line
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_from_file_not_modified() {
        let (path, _f) = write_temp_file("abc");
        let buf = Buffer::from_file(&path).unwrap();
        assert!(!buf.modified);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_from_file_missing_returns_error() {
        let path = std::path::Path::new("/nonexistent/path/that/does/not/exist.txt");
        assert!(Buffer::from_file(path).is_err());
    }

    // --- line_count ---

    #[test]
    fn test_line_count_single_line_no_newline() {
        let buf = make_buffer("hello");
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn test_line_count_two_lines() {
        let buf = make_buffer("hello\nworld");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_line_count_trailing_newline() {
        let buf = make_buffer("hello\nworld\n");
        // ropey treats a trailing newline as an extra (empty) line
        assert_eq!(buf.line_count(), 3);
    }

    #[test]
    fn test_line_count_empty_buffer() {
        let buf = make_buffer("");
        assert_eq!(buf.line_count(), 1);
    }

    // --- line_len_chars ---

    #[test]
    fn test_line_len_chars_basic() {
        let buf = make_buffer("hello\nworld\n");
        assert_eq!(buf.line_len_chars(0), 5); // "hello" without \n
        assert_eq!(buf.line_len_chars(1), 5); // "world" without \n
    }

    #[test]
    fn test_line_len_chars_out_of_bounds_returns_zero() {
        let buf = make_buffer("hi\n");
        assert_eq!(buf.line_len_chars(99), 0);
    }

    #[test]
    fn test_line_len_chars_empty_line() {
        let buf = make_buffer("\n");
        assert_eq!(buf.line_len_chars(0), 0);
    }

    // --- insert_char ---

    #[test]
    fn test_insert_char_appends_to_line() {
        let mut buf = make_buffer("ab\n");
        buf.insert_char(0, 2, 'c');
        assert_eq!(buf.line_len_chars(0), 3);
        let line: String = buf.rope.line(0).chars().collect();
        assert!(line.starts_with("abc"));
    }

    #[test]
    fn test_insert_char_sets_modified() {
        let mut buf = make_buffer("hello");
        assert!(!buf.modified);
        buf.insert_char(0, 0, 'X');
        assert!(buf.modified);
    }

    #[test]
    fn test_insert_char_at_beginning() {
        let mut buf = make_buffer("bc\n");
        buf.insert_char(0, 0, 'a');
        let line: String = buf.rope.line(0).chars().collect();
        assert!(line.starts_with("abc"));
    }

    // --- delete_char_before ---

    #[test]
    fn test_delete_char_before_decrements_col() {
        let mut buf = make_buffer("abc\n");
        let result = buf.delete_char_before(0, 2); // delete 'b'
        assert_eq!(result, Some((0, 1)));
        assert_eq!(buf.line_len_chars(0), 2);
    }

    #[test]
    fn test_delete_char_before_at_col_zero_line_zero_returns_none() {
        let mut buf = make_buffer("abc\n");
        let result = buf.delete_char_before(0, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_delete_char_before_joins_lines() {
        let mut buf = make_buffer("ab\ncd\n");
        // cursor at line 1, col 0 — deletes the newline, joining lines
        let result = buf.delete_char_before(1, 0);
        assert!(result.is_some());
        let (new_line, _new_col) = result.unwrap();
        assert_eq!(new_line, 0);
    }

    #[test]
    fn test_delete_char_before_sets_modified() {
        let mut buf = make_buffer("abc");
        buf.delete_char_before(0, 1);
        assert!(buf.modified);
    }

    // --- delete_char_at ---

    #[test]
    fn test_delete_char_at_removes_character() {
        let mut buf = make_buffer("abc\n");
        buf.delete_char_at(0, 1); // delete 'b'
        let line: String = buf.rope.line(0).chars().collect();
        assert!(line.starts_with("ac"));
        assert_eq!(buf.line_len_chars(0), 2);
    }

    #[test]
    fn test_delete_char_at_out_of_bounds_noop() {
        let mut buf = make_buffer("a");
        buf.delete_char_at(0, 5); // beyond end, no panic
        assert_eq!(buf.line_len_chars(0), 1);
    }

    #[test]
    fn test_delete_char_at_sets_modified() {
        let mut buf = make_buffer("abc");
        buf.delete_char_at(0, 0);
        assert!(buf.modified);
    }

    // --- insert_newline ---

    #[test]
    fn test_insert_newline_increases_line_count() {
        let mut buf = make_buffer("hello");
        let before = buf.line_count();
        buf.insert_newline(0, 3);
        assert_eq!(buf.line_count(), before + 1);
    }

    #[test]
    fn test_insert_newline_sets_modified() {
        let mut buf = make_buffer("hello");
        buf.insert_newline(0, 2);
        assert!(buf.modified);
    }

    #[test]
    fn test_insert_newline_splits_line_content() {
        let mut buf = make_buffer("abcd");
        buf.insert_newline(0, 2);
        let line0: String = buf.rope.line(0).chars().filter(|&c| c != '\n').collect();
        let line1: String = buf.rope.line(1).chars().filter(|&c| c != '\n').collect();
        assert_eq!(line0, "ab");
        assert_eq!(line1, "cd");
    }

    // --- filename ---

    #[test]
    fn test_filename_returns_file_stem() {
        let buf = make_buffer("x");
        assert_eq!(buf.filename(), "test_buffer.txt");
    }
}
