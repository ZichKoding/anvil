pub mod keybindings;

use keybindings::KeybindingMode;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub sidebar: SidebarConfig,
    pub editor: EditorConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub theme: String,
    pub keybinding_mode: KeybindingMode,
    pub mouse_enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SidebarConfig {
    pub width: u16,
    pub show_icons: bool,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct EditorConfig {
    pub show_line_numbers: bool,
    pub tab_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            sidebar: SidebarConfig::default(),
            editor: EditorConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: String::from("retroterm"),
            keybinding_mode: KeybindingMode::Vim,
            mouse_enabled: true,
        }
    }
}

impl Default for SidebarConfig {
    fn default() -> Self {
        Self {
            width: 25,
            show_icons: true,
        }
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            tab_size: 4,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = config_dir().join("anvil.toml");
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Config parse error: {e}");
                    }
                },
                Err(e) => {
                    eprintln!("Config read error: {e}");
                }
            }
        }
        Self::default()
    }
}

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("anvil")
}
