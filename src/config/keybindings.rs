use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeybindingMode {
    Vim,
    Vscode,
}

impl Default for KeybindingMode {
    fn default() -> Self {
        Self::Vim
    }
}
