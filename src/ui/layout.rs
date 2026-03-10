use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Modifier};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, Focus};

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.area();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(size);

    let content_area = main_chunks[0];
    let status_area = main_chunks[1];

    if app.sidebar_visible {
        let sidebar_width = (content_area.width as u32 * 25 / 100)
            .max(20)
            .min(50) as u16;
        let sidebar_width = sidebar_width.min(content_area.width.saturating_sub(20));

        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(sidebar_width),
                Constraint::Min(20),
            ])
            .split(content_area);

        app.file_tree.render(frame, h_chunks[0], app.focus == Focus::Tree);
        render_editor_pane(frame, h_chunks[1], app.focus == Focus::Editor);
    } else {
        render_editor_pane(frame, content_area, true);
    }

    render_status_bar(frame, status_area, app);
}

fn render_editor_pane(frame: &mut Frame, area: ratatui::layout::Rect, focused: bool) {
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Editor ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let content = Paragraph::new("  Welcome to Anvil v0.1.0\n\n  Open a file from the tree to begin editing.")
        .style(Style::default().fg(Color::Gray))
        .block(block);

    frame.render_widget(content, area);
}

fn render_status_bar(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mode_str = match app.mode {
        crate::app::Mode::Normal => "NORMAL",
        crate::app::Mode::Insert => "INSERT",
        crate::app::Mode::Command => "COMMAND",
    };

    let status = format!(" Anvil v0.1.0 | {} | Ready ", mode_str);
    let bar = Paragraph::new(status)
        .style(Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD));

    frame.render_widget(bar, area);
}
