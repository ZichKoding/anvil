use super::Theme;
use super::palette::hex_to_color;

pub fn retroterm_theme() -> Theme {
    Theme {
        name: "RetroTerm".to_string(),

        // UI Chrome
        bg: hex_to_color("#1a1a2e"),
        fg: hex_to_color("#e0c097"),
        sidebar_bg: hex_to_color("#16213e"),
        sidebar_fg: hex_to_color("#a0a0b0"),
        gutter_fg: hex_to_color("#555568"),
        gutter_active_fg: hex_to_color("#e0c097"),
        statusbar_bg: hex_to_color("#0f3460"),
        statusbar_fg: hex_to_color("#e0c097"),
        border_focused: hex_to_color("#00d4ff"),
        border_unfocused: hex_to_color("#333355"),
        cursor_line_bg: hex_to_color("#222244"),
        selection_bg: hex_to_color("#3a3a5c"),

        // Syntax
        keyword: hex_to_color("#00ff9f"),
        string: hex_to_color("#ffd700"),
        comment: hex_to_color("#4a7c7c"),
        function: hex_to_color("#00d4ff"),
        r#type: hex_to_color("#ff6b9d"),
        number: hex_to_color("#ff8c42"),
        operator: hex_to_color("#e0c097"),
        punctuation: hex_to_color("#777799"),
        variable: hex_to_color("#e0c097"),
        constant: hex_to_color("#ff6b9d"),
        property: hex_to_color("#00d4ff"),

        // Tree
        tree_dir: hex_to_color("#00d4ff"),
        tree_file: hex_to_color("#a0a0b0"),
        tree_dotfile: hex_to_color("#555568"),
        tree_selected_bg: hex_to_color("#333355"),
    }
}
