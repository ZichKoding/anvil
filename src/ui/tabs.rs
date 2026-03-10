use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::theme::Theme;

pub fn render_tabs(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    if app.editors.is_empty() || area.width < 10 {
        return;
    }

    let mut spans: Vec<Span> = Vec::new();

    for (i, editor) in app.editors.iter().enumerate() {
        let name = editor.buffer.filename();
        let modified = if editor.buffer.modified { " +" } else { "" };
        let is_active = i == app.active_editor;

        let label = format!(" {name}{modified} ");

        let style = if is_active {
            Style::default()
                .fg(theme.statusbar_fg)
                .bg(theme.border_focused)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.sidebar_fg).bg(theme.sidebar_bg)
        };

        spans.push(Span::styled(label, style));
        spans.push(Span::styled(" ", Style::default().bg(theme.bg)));
    }

    let line = Line::from(spans);
    let tabs = Paragraph::new(line).style(Style::default().bg(theme.bg));

    frame.render_widget(tabs, area);
}
