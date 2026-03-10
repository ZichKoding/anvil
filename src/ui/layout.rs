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
        render_editor_area(frame, app, h_chunks[1]);
    } else {
        render_editor_area(frame, app, content_area);
    }

    render_status_bar(frame, status_area, app);
}

fn render_editor_area(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let focused = app.focus == Focus::Editor;

    if app.editors.is_empty() {
        render_welcome(frame, area, focused);
        return;
    }

    let idx = app.active_editor;
    app.editors[idx].render(frame, area, focused);
}

fn render_welcome(frame: &mut Frame, area: ratatui::layout::Rect, focused: bool) {
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Editor ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let content = Paragraph::new("  Welcome to Anvil v0.1.0\n\n  Open a file from the tree to begin editing.\n\n  Keys:\n    Tab     - switch focus\n    Enter   - open file / expand folder\n    q       - quit\n    Ctrl+B  - toggle sidebar")
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

    let file_info = if let Some(editor) = app.active_editor() {
        let cursor = &editor.cursor;
        format!("{} | {}:{} | {} lines",
            editor.buffer.filename(),
            cursor.line + 1,
            cursor.col + 1,
            editor.buffer.line_count(),
        )
    } else {
        String::from("No file")
    };

    let status = format!(" Anvil v0.1.0 | {} | {} | {} ", mode_str, file_info, app.status_message);
    let bar = Paragraph::new(status)
        .style(Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD));

    frame.render_widget(bar, area);
}
