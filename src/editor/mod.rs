pub mod buffer;
pub mod cursor;
pub mod viewport;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::syntax::highlighter::SyntaxHighlighter;
use crate::theme::Theme;
use buffer::Buffer;
use cursor::Cursor;
use viewport::Viewport;

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

    pub fn render_themed(&mut self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let border_color = if focused {
            theme.border_focused
        } else {
            theme.border_unfocused
        };

        let lang_name = self.highlighter.lang_name();
        let title = format!(" {} [{}] ", self.buffer.filename(), lang_name);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(theme.bg));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let gutter_w = self.gutter_width();

        self.viewport.height = inner.height as usize;
        self.viewport.width = (inner.width as usize).saturating_sub(gutter_w);
        self.viewport.ensure_cursor_visible(self.cursor.line);
        self.viewport.ensure_cursor_col_visible(self.cursor.col);

        let mut lines: Vec<Line> = Vec::with_capacity(inner.height as usize);
        for row in 0..inner.height as usize {
            let line_idx = self.viewport.top_line + row;
            if line_idx >= self.buffer.line_count() {
                lines.push(Line::from(vec![Span::styled(
                    format!("{:>width$} ", "~", width = gutter_w - 1),
                    Style::default().fg(theme.gutter_fg),
                )]));
                continue;
            }

            let is_cursor_line = line_idx == self.cursor.line && focused;

            let line_num = format!("{:>width$} ", line_idx + 1, width = gutter_w - 1);
            let gutter_style = if is_cursor_line {
                Style::default().fg(theme.gutter_active_fg)
            } else {
                Style::default().fg(theme.gutter_fg)
            };

            let full_content = self
                .buffer
                .line(line_idx)
                .map(|l| {
                    let s = l.as_str().unwrap_or("");
                    s.trim_end_matches('\n').trim_end_matches('\r').to_string()
                })
                .unwrap_or_default();

            let col_offset = self.viewport.col_offset;
            let vis_width = self.viewport.width;

            let line_byte_start = self.buffer.rope.line_to_byte(line_idx);
            let line_byte_end = if line_idx + 1 < self.buffer.line_count() {
                self.buffer.rope.line_to_byte(line_idx + 1)
            } else {
                self.buffer.rope.len_bytes()
            };
            let full_len = full_content.len();

            let hl_spans = self
                .highlighter
                .highlight_line(line_byte_start, line_byte_end);

            let line_bg = if is_cursor_line {
                Some(theme.cursor_line_bg)
            } else {
                None
            };

            let mut result_spans: Vec<Span> = vec![Span::styled(
                line_num,
                match line_bg {
                    Some(bg) => gutter_style.bg(bg),
                    None => gutter_style,
                },
            )];

            let default_fg = theme.fg;

            // Slice content to the visible horizontal window
            let slice_start = col_offset.min(full_len);
            let slice_end = (col_offset + vis_width).min(full_len);

            if hl_spans.is_empty() {
                let visible = &full_content[slice_start..slice_end];
                let style = match line_bg {
                    Some(bg) => Style::default().fg(default_fg).bg(bg),
                    None => Style::default().fg(default_fg),
                };
                result_spans.push(Span::styled(visible.to_string(), style));
            } else {
                let mut pos = 0;
                for (start, end, group) in &hl_spans {
                    let start = (*start).min(full_len);
                    let end = (*end).min(full_len);

                    // Handle gap before this span
                    if start > pos {
                        let gap_vis_start = pos.max(slice_start);
                        let gap_vis_end = start.min(slice_end);
                        if gap_vis_start < gap_vis_end {
                            let style = match line_bg {
                                Some(bg) => Style::default().fg(default_fg).bg(bg),
                                None => Style::default().fg(default_fg),
                            };
                            result_spans.push(Span::styled(
                                full_content[gap_vis_start..gap_vis_end].to_string(),
                                style,
                            ));
                        }
                    }

                    // Handle the highlighted span
                    if start < end {
                        let span_vis_start = start.max(slice_start);
                        let span_vis_end = end.min(slice_end);
                        if span_vis_start < span_vis_end {
                            let color = theme.color_for_group(*group);
                            let style = match line_bg {
                                Some(bg) => Style::default().fg(color).bg(bg),
                                None => Style::default().fg(color),
                            };
                            result_spans.push(Span::styled(
                                full_content[span_vis_start..span_vis_end].to_string(),
                                style,
                            ));
                        }
                    }
                    pos = end;
                }
                // Handle remaining text after last highlight span
                if pos < full_len {
                    let rest_vis_start = pos.max(slice_start);
                    let rest_vis_end = full_len.min(slice_end);
                    if rest_vis_start < rest_vis_end {
                        let style = match line_bg {
                            Some(bg) => Style::default().fg(default_fg).bg(bg),
                            None => Style::default().fg(default_fg),
                        };
                        result_spans.push(Span::styled(
                            full_content[rest_vis_start..rest_vis_end].to_string(),
                            style,
                        ));
                    }
                }
            }

            lines.push(Line::from(result_spans));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);

        if focused {
            let col_offset = self.viewport.col_offset;
            let cursor_x =
                inner.x + gutter_w as u16 + (self.cursor.col.saturating_sub(col_offset)) as u16;
            let cursor_y = inner.y + (self.cursor.line - self.viewport.top_line) as u16;
            if cursor_x < inner.x + inner.width && cursor_y < inner.y + inner.height {
                frame.set_cursor_position((cursor_x, cursor_y));
            }
        }
    }
}
