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

            // Apply software cursor cell styling on the cursor line
            if is_cursor_line {
                let cursor_vis_col = self.cursor.col.saturating_sub(col_offset);
                result_spans = apply_cursor_cell(result_spans, cursor_vis_col, theme);
            }

            lines.push(Line::from(result_spans));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner);
    }
}

/// Split spans to insert cursor cell styling at the given visible column.
/// `cursor_vis_col` is relative to the text area (after gutter).
/// `gutter_w` is the width of the gutter span (always the first span).
fn apply_cursor_cell<'a>(
    spans: Vec<Span<'a>>,
    cursor_vis_col: usize,
    theme: &Theme,
) -> Vec<Span<'a>> {
    let cursor_style = Style::default().fg(theme.cursor_fg).bg(theme.cursor_bg);

    // The first span is the gutter — skip it for column counting
    let mut result: Vec<Span<'a>> = Vec::with_capacity(spans.len() + 2);
    if spans.is_empty() {
        return spans;
    }
    result.push(spans[0].clone());

    let mut col = 0usize;
    let mut cursor_applied = false;

    for span in spans.into_iter().skip(1) {
        let span_len = span.content.len();

        if cursor_applied || col + span_len <= cursor_vis_col {
            // Cursor is not in this span — emit as-is
            col += span_len;
            result.push(span);
            continue;
        }

        // Cursor falls within this span
        let offset_in_span = cursor_vis_col - col;
        let content: &str = &span.content;

        // Before cursor
        if offset_in_span > 0 {
            result.push(Span::styled(
                content[..offset_in_span].to_string(),
                span.style,
            ));
        }

        // Cursor cell
        if offset_in_span < span_len {
            // Cursor is on an existing character
            let ch = &content[offset_in_span..offset_in_span + 1];
            result.push(Span::styled(ch.to_string(), cursor_style));

            // After cursor
            if offset_in_span + 1 < span_len {
                result.push(Span::styled(
                    content[offset_in_span + 1..].to_string(),
                    span.style,
                ));
            }
        }

        cursor_applied = true;
        col += span_len;
    }

    // If cursor is past end of all text, append a styled space
    if !cursor_applied {
        result.push(Span::styled(" ".to_string(), cursor_style));
    }

    result
}

#[cfg(test)]
mod cursor_cell_tests {
    use super::*;
    use ratatui::style::{Color, Style};
    use ratatui::text::Span;

    fn test_theme() -> Theme {
        crate::theme::retroterm::retroterm_theme()
    }

    fn gutter_span() -> Span<'static> {
        Span::styled("  1 ".to_string(), Style::default())
    }

    #[test]
    fn cursor_at_start_of_span() {
        let theme = test_theme();
        let text_style = Style::default().fg(theme.fg);
        let spans = vec![gutter_span(), Span::styled("hello".to_string(), text_style)];
        let result = apply_cursor_cell(spans, 0, &theme);
        // gutter + cursor_cell("h") + rest("ello")
        assert_eq!(result.len(), 3);
        assert_eq!(result[1].content.as_ref(), "h");
        assert_eq!(result[1].style.fg, Some(theme.cursor_fg));
        assert_eq!(result[1].style.bg, Some(theme.cursor_bg));
        assert_eq!(result[2].content.as_ref(), "ello");
    }

    #[test]
    fn cursor_at_middle_of_span() {
        let theme = test_theme();
        let text_style = Style::default().fg(theme.fg);
        let spans = vec![gutter_span(), Span::styled("hello".to_string(), text_style)];
        let result = apply_cursor_cell(spans, 2, &theme);
        // gutter + before("he") + cursor_cell("l") + after("lo")
        assert_eq!(result.len(), 4);
        assert_eq!(result[1].content.as_ref(), "he");
        assert_eq!(result[2].content.as_ref(), "l");
        assert_eq!(result[2].style.fg, Some(theme.cursor_fg));
        assert_eq!(result[3].content.as_ref(), "lo");
    }

    #[test]
    fn cursor_at_end_of_span() {
        let theme = test_theme();
        let text_style = Style::default().fg(theme.fg);
        let spans = vec![gutter_span(), Span::styled("hello".to_string(), text_style)];
        let result = apply_cursor_cell(spans, 4, &theme);
        // gutter + before("hell") + cursor_cell("o")
        assert_eq!(result.len(), 3);
        assert_eq!(result[1].content.as_ref(), "hell");
        assert_eq!(result[2].content.as_ref(), "o");
        assert_eq!(result[2].style.fg, Some(theme.cursor_fg));
    }

    #[test]
    fn cursor_past_end_of_line() {
        let theme = test_theme();
        let text_style = Style::default().fg(theme.fg);
        let spans = vec![gutter_span(), Span::styled("hi".to_string(), text_style)];
        let result = apply_cursor_cell(spans, 5, &theme);
        // gutter + "hi" + cursor space
        assert_eq!(result.len(), 3);
        assert_eq!(result[2].content.as_ref(), " ");
        assert_eq!(result[2].style.fg, Some(theme.cursor_fg));
        assert_eq!(result[2].style.bg, Some(theme.cursor_bg));
    }

    #[test]
    fn cursor_across_multiple_spans() {
        let theme = test_theme();
        let s1 = Style::default().fg(theme.keyword);
        let s2 = Style::default().fg(theme.string);
        let spans = vec![
            gutter_span(),
            Span::styled("fn".to_string(), s1),
            Span::styled(" main".to_string(), s2),
        ];
        // cursor at col 3 => in second text span at offset 1 ('m')
        let result = apply_cursor_cell(spans, 3, &theme);
        // gutter + "fn" (untouched) + " " (before) + cursor("m") + "ain" (after)
        assert_eq!(result.len(), 5);
        assert_eq!(result[1].content.as_ref(), "fn");
        assert_eq!(result[2].content.as_ref(), " ");
        assert_eq!(result[3].content.as_ref(), "m");
        assert_eq!(result[3].style.fg, Some(theme.cursor_fg));
        assert_eq!(result[4].content.as_ref(), "ain");
    }

    #[test]
    fn theme_has_cursor_fields() {
        let theme = test_theme();
        // Verify cursor colors are set (not default)
        assert!(matches!(theme.cursor_fg, Color::Rgb(_, _, _)));
        assert!(matches!(theme.cursor_bg, Color::Rgb(_, _, _)));
    }
}
