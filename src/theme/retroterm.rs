use super::Theme;
use super::palette::hex_to_color;

pub fn retroterm_theme() -> Theme {
    Theme {
        // UI Chrome
        bg: hex_to_color("#1a1a2e"),
        fg: hex_to_color("#e0c097"),
        sidebar_bg: hex_to_color("#16213e"),
        sidebar_fg: hex_to_color("#a0a0b0"),
        gutter_fg: hex_to_color("#7a7a9a"),
        gutter_active_fg: hex_to_color("#e0c097"),
        statusbar_bg: hex_to_color("#0f3460"),
        statusbar_fg: hex_to_color("#e0c097"),
        border_focused: hex_to_color("#00d4ff"),
        border_unfocused: hex_to_color("#4a4a70"),
        cursor_line_bg: hex_to_color("#2a2a50"),
        cursor_fg: hex_to_color("#1a1a2e"),
        cursor_bg: hex_to_color("#e0c097"),

        // Syntax
        keyword: hex_to_color("#00ff9f"),
        string: hex_to_color("#ffd700"),
        comment: hex_to_color("#6a9a9a"),
        function: hex_to_color("#00d4ff"),
        r#type: hex_to_color("#ff6b9d"),
        number: hex_to_color("#ff8c42"),
        operator: hex_to_color("#c0a0d0"),
        punctuation: hex_to_color("#8888aa"),
        variable: hex_to_color("#d4b896"),
        constant: hex_to_color("#ff6b9d"),
        property: hex_to_color("#00d4ff"),

        // Tree
        tree_dir: hex_to_color("#00d4ff"),
        tree_file: hex_to_color("#a0a0b0"),
        tree_dotfile: hex_to_color("#6a6a85"),
        tree_selected_bg: hex_to_color("#2a2a50"),
    }
}
