use serde::Deserialize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeybindingMode {
    #[default]
    Vim,
    Vscode,
}
