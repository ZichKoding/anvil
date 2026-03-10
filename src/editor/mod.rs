pub mod buffer;
pub mod cursor;
pub mod viewport;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use buffer::Buffer;
use cursor::Cursor;
use viewport::Viewport;

pub struct EditorPane {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub viewport: Viewport,
}

impl EditorPane {
    pub fn new(buffer: Buffer) -> Self {
        Self {
            buffer,
            cursor: Cursor::new(),
            viewport: Viewport::new(),
        }
    }

    pub fn gutter_width(&self) -> usize {
        let digits = if self.buffer.line_count() == 0 {
            1
        } else {
            (self.buffer.line_count() as f64).log10() as usize + 1
        };
        digits.max(3) + 1 // min 3 digits + 1 space
    }

    pub fn current_line_len(&self) -> usize {
        self.buffer
            .line(self.cursor.line)
            .map(|l| {
                let s = l.as_str().unwrap_or("");
                s.trim_end_matches('\n').trim_end_matches('\r').len()
            })
            .unwrap_or(0)
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let title = format!(" {} ", self.buffer.filename());
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Update viewport height
        self.viewport.height = inner.height as usize;
        self.viewport.ensure_cursor_visible(self.cursor.line);

        let gutter_w = self.gutter_width();

        let mut lines: Vec<Line> = Vec::with_capacity(inner.height as usize);
        for row in 0..inner.height as usize {
            let line_idx = self.viewport.top_line + row;
            if line_idx >= self.buffer.line_count() {
                // Tilde for empty lines past end of file
                let spans = vec![
                    Span::styled(
                        format!("{:>width$} ", "~", width = gutter_w - 1),
                        Style::default().fg(Color::DarkGray),
                    ),
                ];
                lines.push(Line::from(spans));
                continue;
            }

            let is_cursor_line = line_idx == self.cursor.line && focused;

            // Line number
            let line_num = format!("{:>width$} ", line_idx + 1, width = gutter_w - 1);
            let gutter_style = if is_cursor_line {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let content = self.buffer.line(line_idx)
                .map(|l| {
                    let s = l.as_str().unwrap_or("");
                    s.trim_end_matches('\n').trim_end_matches('\r').to_string()
                })
                .unwrap_or_default();

            // Truncate to fit in available width
            let avail_width = (inner.width as usize).saturating_sub(gutter_w);
            let display: String = content.chars().take(avail_width).collect();

            let content_style = if is_cursor_line {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            };

            lines.push(Line::from(vec![
                Span::styled(line_num, gutter_style),
                Span::styled(display, content_style),
            ]));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);

        // Show cursor
        if focused {
            let cursor_x = inner.x + gutter_w as u16 + self.cursor.col as u16;
            let cursor_y = inner.y + (self.cursor.line - self.viewport.top_line) as u16;
            if cursor_x < inner.x + inner.width && cursor_y < inner.y + inner.height {
                frame.set_cursor_position((cursor_x, cursor_y));
            }
        }
    }
}
