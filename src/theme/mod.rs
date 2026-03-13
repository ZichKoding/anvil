pub mod palette;
pub mod retroterm;

use crate::syntax::highlighter::HighlightGroup;
use ratatui::style::Color;

pub struct Theme {
    // UI Chrome
    pub bg: Color,
    pub fg: Color,
    pub sidebar_bg: Color,
    pub sidebar_fg: Color,
    pub gutter_fg: Color,
    pub gutter_active_fg: Color,
    pub statusbar_bg: Color,
    pub statusbar_fg: Color,
    pub border_focused: Color,
    pub border_unfocused: Color,
    pub cursor_line_bg: Color,
    pub cursor_fg: Color,
    pub cursor_bg: Color,

    // Syntax groups
    pub keyword: Color,
    pub string: Color,
    pub comment: Color,
    pub function: Color,
    pub r#type: Color,
    pub number: Color,
    pub operator: Color,
    pub punctuation: Color,
    pub variable: Color,
    pub constant: Color,
    pub property: Color,

    // Tree
    pub tree_dir: Color,
    pub tree_file: Color,
    pub tree_dotfile: Color,
    pub tree_selected_bg: Color,
}

impl Theme {
    pub fn color_for_group(&self, group: HighlightGroup) -> Color {
        match group {
            HighlightGroup::Keyword => self.keyword,
            HighlightGroup::String => self.string,
            HighlightGroup::Comment => self.comment,
            HighlightGroup::Function => self.function,
            HighlightGroup::Type => self.r#type,
            HighlightGroup::Number => self.number,
            HighlightGroup::Operator => self.operator,
            HighlightGroup::Punctuation => self.punctuation,
            HighlightGroup::Variable => self.variable,
            HighlightGroup::Constant => self.constant,
            HighlightGroup::Property => self.property,
            HighlightGroup::Normal => self.fg,
        }
    }

    pub fn default_theme() -> Self {
        let theme = retroterm::retroterm_theme();
        if palette::supports_truecolor() {
            theme
        } else {
            theme.with_fallback_colors()
        }
    }

    /// Convert all Rgb colors in this theme to their ANSI fallback equivalents.
    // NOTE: When adding fields to Theme, this function must be updated.
    pub fn with_fallback_colors(self) -> Self {
        Self {
            bg: palette::to_256_fallback(self.bg),
            fg: palette::to_256_fallback(self.fg),
            sidebar_bg: palette::to_256_fallback(self.sidebar_bg),
            sidebar_fg: palette::to_256_fallback(self.sidebar_fg),
            gutter_fg: palette::to_256_fallback(self.gutter_fg),
            gutter_active_fg: palette::to_256_fallback(self.gutter_active_fg),
            statusbar_bg: palette::to_256_fallback(self.statusbar_bg),
            statusbar_fg: palette::to_256_fallback(self.statusbar_fg),
            border_focused: palette::to_256_fallback(self.border_focused),
            border_unfocused: palette::to_256_fallback(self.border_unfocused),
            cursor_line_bg: palette::to_256_fallback(self.cursor_line_bg),
            cursor_fg: palette::to_256_fallback(self.cursor_fg),
            cursor_bg: palette::to_256_fallback(self.cursor_bg),
            keyword: palette::to_256_fallback(self.keyword),
            string: palette::to_256_fallback(self.string),
            comment: palette::to_256_fallback(self.comment),
            function: palette::to_256_fallback(self.function),
            r#type: palette::to_256_fallback(self.r#type),
            number: palette::to_256_fallback(self.number),
            operator: palette::to_256_fallback(self.operator),
            punctuation: palette::to_256_fallback(self.punctuation),
            variable: palette::to_256_fallback(self.variable),
            constant: palette::to_256_fallback(self.constant),
            property: palette::to_256_fallback(self.property),
            tree_dir: palette::to_256_fallback(self.tree_dir),
            tree_file: palette::to_256_fallback(self.tree_file),
            tree_dotfile: palette::to_256_fallback(self.tree_dotfile),
            tree_selected_bg: palette::to_256_fallback(self.tree_selected_bg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "windows"))]
    use ratatui::style::Color;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_with_fallback_colors_converts_all_rgb_fields() {
        unsafe { std::env::remove_var("COLORTERM") };
        unsafe { std::env::remove_var("WT_SESSION") };
        unsafe { std::env::remove_var("ConEmuANSI") };
        unsafe { std::env::remove_var("TERM_PROGRAM") };
        let theme = retroterm::retroterm_theme();
        let fallback = theme.with_fallback_colors();

        // All fields that were Rgb should now be non-Rgb
        assert!(!matches!(fallback.bg, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.fg, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.keyword, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.string, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.comment, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.function, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.variable, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.constant, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.statusbar_bg, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.cursor_fg, Color::Rgb(_, _, _)));
        assert!(!matches!(fallback.cursor_bg, Color::Rgb(_, _, _)));
    }

    #[test]
    fn test_color_for_group_returns_correct_colors() {
        let theme = retroterm::retroterm_theme();
        assert_eq!(
            theme.color_for_group(HighlightGroup::Keyword),
            theme.keyword
        );
        assert_eq!(theme.color_for_group(HighlightGroup::String), theme.string);
        assert_eq!(theme.color_for_group(HighlightGroup::Normal), theme.fg);
    }
}
