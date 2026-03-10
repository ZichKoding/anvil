use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, Focus};

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.area();

    // Fill background
    let bg_block = Block::default().style(Style::default().bg(app.theme.bg));
    frame.render_widget(bg_block, size);

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

        let tree_focused = app.focus == Focus::Tree;
        let theme = &app.theme;
        let border_color = if tree_focused { theme.border_focused } else { theme.border_unfocused };
        let sidebar_bg = theme.sidebar_bg;
        let sidebar_fg = theme.sidebar_fg;
        let tree_dir = theme.tree_dir;
        let tree_file = theme.tree_file;
        let tree_dotfile = theme.tree_dotfile;
        let tree_selected_bg = theme.tree_selected_bg;

        app.file_tree.render_themed(
            frame, h_chunks[0], tree_focused,
            border_color, sidebar_bg, sidebar_fg,
            tree_dir, tree_file, tree_dotfile, tree_selected_bg,
        );
        render_editor_area(frame, app, h_chunks[1]);
    } else {
        render_editor_area(frame, app, content_area);
    }

    render_status_bar(frame, status_area, app);
}

fn render_editor_area(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let focused = app.focus == Focus::Editor;

    if app.editors.is_empty() {
        render_welcome(frame, area, app);
        return;
    }

    let theme = &app.theme;
    let idx = app.active_editor;
    app.editors[idx].render_themed(frame, area, focused, theme);
}

fn render_welcome(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let focused = app.focus == Focus::Editor;
    let border_color = if focused { app.theme.border_focused } else { app.theme.border_unfocused };

    let block = Block::default()
        .title(" Editor ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(app.theme.bg));

    let content = Paragraph::new("  Welcome to Anvil v0.1.0\n\n  Open a file from the tree to begin editing.\n\n  Keys:\n    Tab     - switch focus\n    Enter   - open file / expand folder\n    q       - quit\n    Ctrl+B  - toggle sidebar")
        .style(Style::default().fg(app.theme.fg).bg(app.theme.bg))
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

    let lang = app.active_editor()
        .map(|e| e.highlighter.lang_name())
        .unwrap_or("--");

    let status = format!(" Anvil | {} | {} | {} | {} ", mode_str, file_info, lang, app.status_message);
    let bar = Paragraph::new(status)
        .style(Style::default()
            .fg(app.theme.statusbar_fg)
            .bg(app.theme.statusbar_bg)
            .add_modifier(Modifier::BOLD));

    frame.render_widget(bar, area);
}
