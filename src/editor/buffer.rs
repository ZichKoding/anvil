use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use ropey::Rope;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB guard

pub struct Buffer {
    pub rope: Rope,
    pub file_path: PathBuf,
    pub modified: bool,
}

impl Buffer {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        // File size guard
        let metadata = std::fs::metadata(path)?;
        if metadata.len() > MAX_FILE_SIZE {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("File too large ({} bytes, max {})", metadata.len(), MAX_FILE_SIZE),
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
}
