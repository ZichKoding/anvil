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
use crate::syntax::highlighter::SyntaxHighlighter;

pub struct EditorPane {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub viewport: Viewport,
    pub highlighter: SyntaxHighlighter,
}

impl EditorPane {
    pub fn new(buffer: Buffer) -> Self {
        let mut highlighter = SyntaxHighlighter::new();
        highlighter.set_language_from_path(&buffer.file_path);

        // Parse entire file for initial highlighting
        let source = buffer.rope.to_string();
        highlighter.parse(&source);

        Self {
            buffer,
            cursor: Cursor::new(),
            viewport: Viewport::new(),
            highlighter,
        }
    }

    pub fn gutter_width(&self) -> usize {
        let digits = if self.buffer.line_count() == 0 {
            1
        } else {
            (self.buffer.line_count() as f64).log10() as usize + 1
        };
        digits.max(3) + 1
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

        let lang_name = self.highlighter.lang_name();
        let title = format!(" {} [{}] ", self.buffer.filename(), lang_name);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        self.viewport.height = inner.height as usize;
        self.viewport.ensure_cursor_visible(self.cursor.line);

        let gutter_w = self.gutter_width();
        let avail_width = (inner.width as usize).saturating_sub(gutter_w);

        let mut lines: Vec<Line> = Vec::with_capacity(inner.height as usize);
        for row in 0..inner.height as usize {
            let line_idx = self.viewport.top_line + row;
            if line_idx >= self.buffer.line_count() {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{:>width$} ", "~", width = gutter_w - 1),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));
                continue;
            }

            let is_cursor_line = line_idx == self.cursor.line && focused;

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

            // Get syntax highlighting spans for this line
            let line_byte_start = self.buffer.rope.line_to_byte(line_idx);
            let line_byte_end = if line_idx + 1 < self.buffer.line_count() {
                self.buffer.rope.line_to_byte(line_idx + 1)
            } else {
                self.buffer.rope.len_bytes()
            };
            // Adjust end to exclude newline for display
            let display_len = content.len();

            let hl_spans = self.highlighter.highlight_line(
                &self.buffer.rope.to_string(),
                line_byte_start,
                line_byte_end,
            );

            let mut result_spans: Vec<Span> = vec![Span::styled(line_num, gutter_style)];

            if hl_spans.is_empty() {
                // No syntax info - plain text
                let display: String = content.chars().take(avail_width).collect();
                let style = if is_cursor_line {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };
                result_spans.push(Span::styled(display, style));
            } else {
                // Build colored spans
                let mut pos = 0;
                for (start, end, group) in &hl_spans {
                    let start = (*start).min(display_len);
                    let end = (*end).min(display_len);
                    if start > pos {
                        // Gap before this span
                        let gap: String = content[pos..start].chars().take(avail_width).collect();
                        let style = if is_cursor_line {
                            Style::default().fg(Color::White)
                        } else {
                            Style::default().fg(Color::Gray)
                        };
                        result_spans.push(Span::styled(gap, style));
                    }
                    if start < end {
                        let text: String = content[start..end].to_string();
                        result_spans.push(Span::styled(text, Style::default().fg(group.default_color())));
                    }
                    pos = end;
                }
                // Remaining text after last span
                if pos < display_len {
                    let rest: String = content[pos..].to_string();
                    let style = if is_cursor_line {
                        Style::default().fg(Color::White)
                    } else {
                        Style::default().fg(Color::Gray)
                    };
                    result_spans.push(Span::styled(rest, style));
                }
            }

            lines.push(Line::from(result_spans));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);

        if focused {
            let cursor_x = inner.x + gutter_w as u16 + self.cursor.col as u16;
            let cursor_y = inner.y + (self.cursor.line - self.viewport.top_line) as u16;
            if cursor_x < inner.x + inner.width && cursor_y < inner.y + inner.height {
                frame.set_cursor_position((cursor_x, cursor_y));
            }
        }
    }
}
