pub mod keybindings;

use keybindings::KeybindingMode;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // --- Default values ---

    #[test]
    fn test_general_config_default_theme() {
        let cfg = GeneralConfig::default();
        assert_eq!(cfg.theme, "retroterm");
    }

    #[test]
    fn test_general_config_default_mouse_enabled() {
        let cfg = GeneralConfig::default();
        assert!(cfg.mouse_enabled);
    }

    #[test]
    fn test_general_config_default_keybinding_mode_is_vim() {
        use keybindings::KeybindingMode;
        let cfg = GeneralConfig::default();
        assert_eq!(cfg.keybinding_mode, KeybindingMode::Vim);
    }

    #[test]
    fn test_sidebar_config_default_width() {
        let cfg = SidebarConfig::default();
        assert_eq!(cfg.width, 25);
    }

    #[test]
    fn test_sidebar_config_default_show_icons() {
        let cfg = SidebarConfig::default();
        assert!(cfg.show_icons);
    }

    #[test]
    fn test_editor_config_default_show_line_numbers() {
        let cfg = EditorConfig::default();
        assert!(cfg.show_line_numbers);
    }

    #[test]
    fn test_editor_config_default_tab_size() {
        let cfg = EditorConfig::default();
        assert_eq!(cfg.tab_size, 4);
    }

    // --- TOML parsing ---

    #[test]
    fn test_toml_parse_overrides_theme() {
        let toml_str = r#"
[general]
theme = "gruvbox"
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.general.theme, "gruvbox");
    }

    #[test]
    fn test_toml_parse_overrides_tab_size() {
        let toml_str = r#"
[editor]
tab_size = 2
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.editor.tab_size, 2);
    }

    #[test]
    fn test_toml_parse_overrides_sidebar_width() {
        let toml_str = r#"
[sidebar]
width = 40
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.sidebar.width, 40);
    }

    #[test]
    fn test_toml_parse_partial_uses_defaults_for_missing_fields() {
        let toml_str = r#"
[general]
theme = "solarized"
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        // editor fields not in toml -> use defaults
        assert_eq!(cfg.editor.tab_size, 4);
        assert!(cfg.editor.show_line_numbers);
    }

    #[test]
    fn test_toml_parse_empty_string_uses_all_defaults() {
        let cfg: Config = toml::from_str("").unwrap();
        assert_eq!(cfg.general.theme, "retroterm");
        assert_eq!(cfg.sidebar.width, 25);
        assert_eq!(cfg.editor.tab_size, 4);
    }

    #[test]
    fn test_toml_parse_invalid_returns_error() {
        let bad_toml = "[[[[not valid toml";
        let result: Result<Config, _> = toml::from_str(bad_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_toml_keybinding_mode_vscode() {
        use keybindings::KeybindingMode;
        let toml_str = r#"
[general]
keybinding_mode = "vscode"
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.general.keybinding_mode, KeybindingMode::Vscode);
    }
}
