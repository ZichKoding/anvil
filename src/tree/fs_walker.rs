use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

pub fn read_directory(path: &Path) -> Vec<DirEntry> {
    let mut entries = Vec::new();

    let Ok(read_dir) = fs::read_dir(path) else {
        return entries;
    };

    for entry in read_dir.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_dir = path.is_dir();
        entries.push(DirEntry { path, name, is_dir });
    }

    // Sort: directories first, then files, alphabetical within each group
    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    entries
}
