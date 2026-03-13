use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::tabs;
use crate::app::{App, Focus};

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.area();

    // Fill background
    let bg_block = Block::default().style(Style::default().bg(app.theme.bg));
    frame.render_widget(bg_block, size);

    let show_tabs = app.editors.len() > 1;

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(if show_tabs {
            vec![
                Constraint::Length(1), // tab bar
                Constraint::Min(3),    // content
                Constraint::Length(1), // status bar
            ]
        } else {
            vec![
                Constraint::Length(0), // no tab bar
                Constraint::Min(3),    // content
                Constraint::Length(1), // status bar
            ]
        })
        .split(size);

    let tab_area = main_chunks[0];
    let content_area = main_chunks[1];
    let status_area = main_chunks[2];

    // Render tab bar
    if show_tabs {
        tabs::render_tabs(frame, tab_area, app, &app.theme);
    }

    if app.sidebar_visible {
        let sidebar_pct = app.config.sidebar.width as u32;
        let sidebar_width = (content_area.width as u32 * sidebar_pct / 100).clamp(20, 50) as u16;
        let sidebar_width = sidebar_width.min(content_area.width.saturating_sub(20));

        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(sidebar_width), Constraint::Min(20)])
            .split(content_area);

        let tree_focused = app.focus == Focus::Tree;
        let theme = &app.theme;

        app.file_tree
            .render_themed(frame, h_chunks[0], tree_focused, theme);
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
    if let Some(editor) = app.editors.get_mut(app.active_editor) {
        editor.render_themed(frame, area, focused, theme);
    }
}

fn render_welcome(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let focused = app.focus == Focus::Editor;
    let border_color = if focused {
        app.theme.border_focused
    } else {
        app.theme.border_unfocused
    };

    let block = Block::default()
        .title(" Editor ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(app.theme.bg));

    let mode_info = match app.config.general.keybinding_mode {
        crate::config::keybindings::KeybindingMode::Vim => {
            "  Mode: Vim (i=insert, Esc=normal, :w=save, :q=quit)"
        }
        crate::config::keybindings::KeybindingMode::Vscode => {
            "  Mode: VS Code (Ctrl+S=save, Ctrl+Q=quit)"
        }
    };

    let content = format!(
        "  Welcome to Anvil v0.3.0\n\n  A lightweight terminal IDE\n\n{}\n\n  Keys:\n    Tab     - switch focus\n    Enter   - open file / expand folder\n    Ctrl+B  - toggle sidebar\n    Ctrl+S  - save file",
        mode_info
    );

    let paragraph = Paragraph::new(content)
        .style(Style::default().fg(app.theme.fg).bg(app.theme.bg))
        .block(block);

    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    if app.mode == crate::app::Mode::Command {
        // Render command line
        let command_line = format!(":{}", app.command_buffer);
        let bar = Paragraph::new(command_line).style(
            Style::default()
                .fg(app.theme.statusbar_fg)
                .bg(app.theme.statusbar_bg)
                .add_modifier(Modifier::BOLD),
        );
        frame.render_widget(bar, area);

        // Position cursor at end of command buffer
        let cursor_x = area.x + 1 + app.command_buffer.len() as u16;
        let cursor_y = area.y;
        if cursor_x < area.x + area.width {
            frame.set_cursor_position((cursor_x, cursor_y));
        }
        return;
    }

    let mode_str = match app.mode {
        crate::app::Mode::Normal => "NORMAL",
        crate::app::Mode::Insert => "INSERT",
        crate::app::Mode::Command => "COMMAND",
    };

    let file_info = if let Some(editor) = app.active_editor() {
        let cursor = &editor.cursor;
        let modified = if editor.buffer.modified { " [+]" } else { "" };
        format!(
            "{}{} | {}:{} | {} lines",
            editor.buffer.filename(),
            modified,
            cursor.line + 1,
            cursor.col + 1,
            editor.buffer.line_count(),
        )
    } else {
        String::from("No file")
    };

    let lang = app
        .active_editor()
        .map(|e| e.highlighter.lang_name())
        .unwrap_or("--");

    let tab_info = if app.editors.len() > 1 {
        format!("[{}/{}] ", app.active_editor + 1, app.editors.len())
    } else {
        String::new()
    };

    let status = format!(
        " Anvil | {} | {}{} | {} | {} ",
        mode_str, tab_info, file_info, lang, app.status_message
    );
    let bar = Paragraph::new(status).style(
        Style::default()
            .fg(app.theme.statusbar_fg)
            .bg(app.theme.statusbar_bg)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_widget(bar, area);
}
