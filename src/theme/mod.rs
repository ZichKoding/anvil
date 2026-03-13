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
        retroterm::retroterm_theme()
    }
}
