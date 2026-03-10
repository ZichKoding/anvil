pub mod fs_walker;

use std::path::{Path, PathBuf};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Modifier};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

use fs_walker::{read_directory, DirEntry};

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub entry: DirEntry,
    pub depth: usize,
    pub expanded: bool,
    pub children_loaded: bool,
}

pub struct FileTree {
    root: PathBuf,
    nodes: Vec<TreeNode>,
    pub state: ListState,
}

impl FileTree {
    pub fn new(root: PathBuf) -> Self {
        let mut tree = Self {
            root: root.clone(),
            nodes: Vec::new(),
            state: ListState::default(),
        };
        tree.load_root();
        if !tree.nodes.is_empty() {
            tree.state.select(Some(0));
        }
        tree
    }

    fn load_root(&mut self) {
        let entries = read_directory(&self.root);
        self.nodes = entries
            .into_iter()
            .map(|entry| TreeNode {
                entry,
                depth: 0,
                expanded: false,
                children_loaded: false,
            })
            .collect();
    }

    pub fn selected_path(&self) -> Option<&Path> {
        self.state
            .selected()
            .and_then(|i| self.nodes.get(i))
            .map(|n| n.entry.path.as_path())
    }

    pub fn selected_is_file(&self) -> bool {
        self.state
            .selected()
            .and_then(|i| self.nodes.get(i))
            .is_some_and(|n| !n.entry.is_dir)
    }

    pub fn move_up(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected > 0 {
                self.state.select(Some(selected - 1));
            }
        }
    }

    pub fn move_down(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected + 1 < self.nodes.len() {
                self.state.select(Some(selected + 1));
            }
        }
    }

    pub fn toggle_expand(&mut self) {
        let Some(idx) = self.state.selected() else {
            return;
        };

        if !self.nodes[idx].entry.is_dir {
            return;
        }

        if self.nodes[idx].expanded {
            // Collapse: remove all children
            self.nodes[idx].expanded = false;
            let depth = self.nodes[idx].depth;
            let remove_start = idx + 1;
            let mut remove_end = remove_start;
            while remove_end < self.nodes.len() && self.nodes[remove_end].depth > depth {
                remove_end += 1;
            }
            self.nodes.drain(remove_start..remove_end);
        } else {
            // Expand: lazy load children
            self.nodes[idx].expanded = true;
            let depth = self.nodes[idx].depth + 1;
            let path = self.nodes[idx].entry.path.clone();
            let children = read_directory(&path);
            let child_nodes: Vec<TreeNode> = children
                .into_iter()
                .map(|entry| TreeNode {
                    entry,
                    depth,
                    expanded: false,
                    children_loaded: false,
                })
                .collect();
            let insert_at = idx + 1;
            for (i, node) in child_nodes.into_iter().enumerate() {
                self.nodes.insert(insert_at + i, node);
            }
            self.nodes[idx].children_loaded = true;
        }
    }

    pub fn is_expanded(&self, idx: usize) -> bool {
        self.nodes.get(idx).is_some_and(|n| n.expanded)
    }

    pub fn render_themed(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        _focused: bool,
        border_color: Color,
        bg: Color,
        fg: Color,
        dir_color: Color,
        file_color: Color,
        dotfile_color: Color,
        selected_bg: Color,
    ) {
        let block = Block::default()
            .title(" Files ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(bg));

        let items: Vec<ListItem> = self
            .nodes
            .iter()
            .map(|node| {
                let indent = "  ".repeat(node.depth);
                let icon = if node.entry.is_dir {
                    if node.expanded { "v " } else { "> " }
                } else {
                    "  "
                };

                let is_dotfile = node.entry.name.starts_with('.');
                let style = if node.entry.is_dir {
                    Style::default().fg(dir_color).add_modifier(Modifier::BOLD)
                } else if is_dotfile {
                    Style::default().fg(dotfile_color)
                } else {
                    Style::default().fg(file_color)
                };

                let line = Line::from(vec![
                    Span::styled(indent, Style::default().fg(fg)),
                    Span::styled(format!("{icon}{}", node.entry.name), style),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(selected_bg)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
