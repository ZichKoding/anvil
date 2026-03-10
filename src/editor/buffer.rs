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
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "File too large ({} bytes, max {})",
                    metadata.len(),
                    MAX_FILE_SIZE
                ),
            ));
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
