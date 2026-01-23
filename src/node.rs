use crate::icons;
use crate::theme::Theme;
use colored::*;
use std::cmp::Ordering;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub enum NodeType {
    File {
        status: String,
        stats: Option<(usize, usize)>,
    },
    Directory {
        children: Vec<Node>,
    },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub full_path: String,
    pub node_type: NodeType,
}

impl Node {
    pub fn new_file(
        name: String,
        full_path: String,
        status: String,
        stats: Option<(usize, usize)>,
    ) -> Self {
        Node {
            name,
            full_path,
            node_type: NodeType::File { status, stats },
        }
    }

    pub fn new_dir(name: String, full_path: String, mut children: Vec<Node>) -> Self {
        children.sort_by(|a, b| a.cmp(b));
        Node {
            name,
            full_path,
            node_type: NodeType::Directory { children },
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self.node_type, NodeType::Directory { .. })
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_file() == other.is_file() {
            self.name.cmp(&other.name)
        } else if self.is_dir() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn format_name(&self, theme: &Theme) -> String {
        match &self.node_type {
            NodeType::Directory { .. } => {
                let icon = if theme.is_nerd && !theme.simple_icons {
                    format!("{} ", icons::get_icon(&self.name, true))
                } else {
                    theme.icon_dir.to_string()
                };
                format!("{}{}", icon, self.name.bold())
            }

            NodeType::File { status, stats: _ } => {
                let staged = status.contains('+');
                let color_name = if staged {
                    self.name.green()
                } else {
                    self.name.red()
                };

                let icon = if theme.is_nerd && !theme.simple_icons {
                    format!("{} ", icons::get_icon(&self.name, false))
                } else {
                    theme.icon_file.to_string()
                };

                let s_base = format!("{}{}", icon, color_name);
                let s = format!("{} ({})", s_base, status);

                s
            }
        }
    }

    pub fn render_tree(&self, indent: usize, collapse: bool, theme: &Theme) -> String {
        let collapsed_paths = std::collections::HashSet::new();
        let flattened = self.flatten(indent, collapse, theme, &collapsed_paths);

        let max_width = flattened
            .iter()
            .map(|n| n.connector.width() + n.name.width())
            .max()
            .unwrap_or(0);

        let mut out = String::new();

        for node in flattened {
            let width = node.connector.width() + node.name.width();
            let padding = if max_width > width {
                " ".repeat(max_width - width)
            } else {
                String::new()
            };

            let stats_bar = if let Some((added, deleted)) = node.stats {
                let total = added + deleted;
                if total > 0 {
                    let max_bar_width = 10;
                    let (plus_chars, minus_chars) = if total <= max_bar_width {
                        (added, deleted)
                    } else {
                        let ratio = added as f64 / total as f64;
                        let p = (ratio * max_bar_width as f64).round() as usize;
                        let m = max_bar_width - p;
                        (p, m)
                    };

                    format!(
                        " | {} {}{}",
                        total,
                        theme.diff_bar_plus.to_string().repeat(plus_chars).green(),
                        theme.diff_bar_minus.to_string().repeat(minus_chars).red()
                    )
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            out.push_str(&format!(
                "{}{}{}{}\n",
                node.connector, node.name_colored, padding, stats_bar
            ));
        }
        out
    }

    fn get_collapsed_view(&self, theme: &Theme) -> (Node, Option<Vec<Node>>) {
        if !self.is_dir() {
            return (self.clone(), None);
        }

        if let NodeType::Directory { children } = &self.node_type {
            if children.len() == 1 {
                let only_child = &children[0];
                if only_child.is_dir() {
                    let (child_collapsed, _) = only_child.get_collapsed_view(theme);

                    let new_name = format!(
                        "{}{}{}",
                        self.name, theme.path_divider, child_collapsed.name
                    );

                    let combined = Node {
                        name: new_name,
                        full_path: self.full_path.clone(),
                        node_type: child_collapsed.node_type.clone(),
                    };
                    return (combined, None);
                }
            }
        }

        (self.clone(), None)
    }

    pub fn flatten(
        &self,
        indent_size: usize,
        collapse: bool,
        theme: &Theme,
        collapsed_paths: &std::collections::HashSet<String>,
    ) -> Vec<FlatNode> {
        let mut flattened = Vec::new();
        // Add root
        flattened.push(FlatNode {
            name: self.get_display_name_clean(theme),
            name_colored: self.format_name(theme),
            full_path: self.full_path.clone(),
            is_dir: self.is_dir(),
            status: self.get_status_char(),
            raw_status: self.get_raw_status(),
            connector: String::new(),
            stats: self.get_stats(),
        });

        if let NodeType::Directory { children } = &self.node_type {
            self.flatten_children(
                children,
                indent_size,
                collapse,
                "",
                &mut flattened,
                theme,
                collapsed_paths,
            );
        }
        flattened
    }

    #[allow(clippy::too_many_arguments)]
    fn flatten_children(
        &self,
        children: &[Node],
        indent_size: usize,
        collapse: bool,
        prefix: &str,
        out: &mut Vec<FlatNode>,
        theme: &Theme,
        collapsed_paths: &std::collections::HashSet<String>,
    ) {
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            let is_last = i == count - 1;

            let (display_node, _) = if collapse {
                child.get_collapsed_view(theme)
            } else {
                (child.clone(), None)
            };

            let children_to_render = match &display_node.node_type {
                NodeType::Directory { children } => Some(children),
                _ => None,
            };

            let connector_symbol = if is_last {
                theme.tree_end
            } else {
                theme.tree_branch
            };
            let dashes = theme.tree_dash.to_string().repeat(indent_size - 2);
            let full_connector = format!("{}{}{} ", prefix, connector_symbol, dashes);

            out.push(FlatNode {
                name: display_node.get_display_name_clean(theme),
                name_colored: display_node.format_name(theme),
                full_path: display_node.full_path.clone(),
                is_dir: display_node.is_dir(),
                status: display_node.get_status_char(),
                raw_status: display_node.get_raw_status(),
                connector: full_connector,
                stats: display_node.get_stats(),
            });

            if let Some(grand_children) = children_to_render {
                if !collapsed_paths.contains(&display_node.full_path) {
                    let new_prefix = if is_last {
                        format!("{}  {}", prefix, " ".repeat(indent_size - 2))
                    } else {
                        format!(
                            "{}{} {}",
                            prefix,
                            theme.tree_vertical,
                            " ".repeat(indent_size - 2)
                        )
                    };

                    self.flatten_children(
                        grand_children,
                        indent_size,
                        collapse,
                        &new_prefix,
                        out,
                        theme,
                        collapsed_paths,
                    );
                }
            }
        }
    }

    pub fn get_display_name_clean(&self, theme: &Theme) -> String {
        let icon = match &self.node_type {
            NodeType::Directory { .. } => {
                let icon = if theme.is_nerd && !theme.simple_icons {
                    format!("{} ", icons::get_icon(&self.name, true))
                } else {
                    theme.icon_dir.to_string()
                };
                icon
            }
            NodeType::File { .. } => {
                if theme.is_nerd && !theme.simple_icons {
                    format!("{} ", icons::get_icon(&self.name, false))
                } else {
                    theme.icon_file.to_string()
                }
            }
        };

        match &self.node_type {
            NodeType::Directory { .. } => format!("{}{}", icon, self.name),
            NodeType::File { status, .. } => {
                format!("{}{}{} ({})", icon, self.name, "", status)
            }
        }
    }

    pub fn get_status_char(&self) -> char {
        match &self.node_type {
            NodeType::File { status, .. } => {
                if status.contains('+') {
                    '+'
                } else if status.contains('?') {
                    '?'
                } else {
                    'M'
                }
            }
            NodeType::Directory { .. } => ' ',
        }
    }

    pub fn get_raw_status(&self) -> String {
        match &self.node_type {
            NodeType::File { status, .. } => status.clone(),
            NodeType::Directory { children } => {
                if children.is_empty() {
                    return String::new();
                }

                let mut all_staged = true;
                for child in children {
                    let s = child.get_raw_status();
                    if !s.contains('+') {
                        all_staged = false;
                        break;
                    }
                }

                if all_staged {
                    "M+".to_string()
                } else {
                    "M".to_string()
                }
            }
        }
    }
    pub fn get_stats(&self) -> Option<(usize, usize)> {
        match &self.node_type {
            NodeType::File { stats, .. } => *stats,
            NodeType::Directory { .. } => None,
        }
    }

    pub fn get_all_dir_paths(&self, paths: &mut std::collections::HashSet<String>) {
        if let NodeType::Directory { children } = &self.node_type {
            if !self.full_path.is_empty() && self.full_path != "." {
                paths.insert(self.full_path.clone());
            }
            for child in children {
                child.get_all_dir_paths(paths);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_directory_status() {
        let child1 = Node::new_file("a".to_string(), "a".to_string(), "M+".to_string(), None);
        let child2 = Node::new_file("b".to_string(), "b".to_string(), "A+".to_string(), None);
        let dir_staged = Node::new_dir("dir".to_string(), "dir".to_string(), vec![child1, child2]);
        assert_eq!(dir_staged.get_raw_status(), "M+");

        let child3 = Node::new_file("c".to_string(), "c".to_string(), "M+".to_string(), None);
        let child4 = Node::new_file("d".to_string(), "d".to_string(), "M".to_string(), None);
        let dir_mixed = Node::new_dir(
            "dir_mixed".to_string(),
            "dir_mixed".to_string(),
            vec![child3, child4],
        );
        assert_eq!(dir_mixed.get_raw_status(), "M");

        let child5 = Node::new_file("e".to_string(), "e".to_string(), "??".to_string(), None);
        let dir_unstaged = Node::new_dir(
            "dir_unstaged".to_string(),
            "dir_unstaged".to_string(),
            vec![child5],
        );
        assert_eq!(dir_unstaged.get_raw_status(), "M");

        let nested_dir = Node::new_dir(
            "nested".to_string(),
            "nested".to_string(),
            vec![Node::new_file(
                "f".to_string(),
                "f".to_string(),
                "M".to_string(),
                None,
            )],
        );
        let parent_dir =
            Node::new_dir("parent".to_string(), "parent".to_string(), vec![nested_dir]);
        assert_eq!(parent_dir.get_raw_status(), "M");

        let nested_dir_staged = Node::new_dir(
            "nested_s".to_string(),
            "nested_s".to_string(),
            vec![Node::new_file(
                "g".to_string(),
                "g".to_string(),
                "M+".to_string(),
                None,
            )],
        );
        let parent_dir_staged = Node::new_dir(
            "parent_s".to_string(),
            "parent_s".to_string(),
            vec![nested_dir_staged],
        );
        assert_eq!(parent_dir_staged.get_raw_status(), "M+");
    }

    #[test]
    fn test_flatten_collapsed() {
        let theme = Theme::ascii();

        let grandchild = Node {
            name: "grandchild_file".to_string(),
            full_path: "root/child_dir/grandchild_file".to_string(),
            node_type: NodeType::File {
                status: "M ".to_string(),
                stats: None,
            },
        };

        let child_dir = Node {
            name: "child_dir".to_string(),
            full_path: "root/child_dir".to_string(),
            node_type: NodeType::Directory {
                children: vec![grandchild],
            },
        };

        let child_file = Node {
            name: "child_file".to_string(),
            full_path: "root/child_file".to_string(),
            node_type: NodeType::File {
                status: "??".to_string(),
                stats: None,
            },
        };

        let root = Node {
            name: "root".to_string(),
            full_path: "root".to_string(),
            node_type: NodeType::Directory {
                children: vec![child_dir, child_file],
            },
        };

        let empty_set = std::collections::HashSet::new();
        let flattened_full = root.flatten(2, false, &theme, &empty_set);
        assert_eq!(flattened_full.len(), 4);
        assert_eq!(flattened_full[1].name, "child_dir");
        assert_eq!(flattened_full[2].name, "grandchild_file (M )");

        let mut collapsed = std::collections::HashSet::new();
        collapsed.insert("root/child_dir".to_string());
        let flattened_collapsed = root.flatten(2, false, &theme, &collapsed);
        assert_eq!(flattened_collapsed.len(), 3);
        assert_eq!(flattened_collapsed[1].name, "child_dir");
        assert_eq!(flattened_collapsed[2].name, "child_file (??)");
    }

    #[test]
    fn test_icon_spacing() {
        let theme = Theme::nerd();
        let node = Node::new_file("test.rs".into(), "test.rs".into(), "M".into(), None);
        let display = node.format_name(&theme);
        // Nerd theme should have some icon
        assert!(display.contains('') || display.contains('🦀'));
    }

    #[test]
    fn test_display_name_clean() {
        let theme = Theme::unicode();
        let node = Node::new_file("test.rs".into(), "test.rs".into(), "M".into(), None);
        let clean = node.get_display_name_clean(&theme);
        // Clean name for files includes the status code
        assert_eq!(clean, "test.rs (M)");
    }

    #[test]
    fn test_render_tree_simple() {
        let theme = Theme::ascii();
        let file = Node::new_file("a.txt".into(), "a.txt".into(), "M".into(), None);
        let dir = Node::new_dir("src".into(), "src".into(), vec![file]);
        let root = Node::new_dir(".".into(), ".".into(), vec![dir]);

        let rendered = root.render_tree(4, false, &theme);
        assert!(rendered.contains("."));
        assert!(rendered.contains("`--"));
        assert!(rendered.contains("src"));
        assert!(rendered.contains("a.txt"));
    }
}

#[derive(Debug, Clone)]
pub struct FlatNode {
    pub name: String,
    pub name_colored: String,
    pub full_path: String,
    pub is_dir: bool,
    pub status: char,
    pub raw_status: String,
    pub connector: String,
    pub stats: Option<(usize, usize)>,
}
