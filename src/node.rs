use crate::icons;
use crate::theme::Theme;
use colored::*;
use std::cmp::Ordering;

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

    // ... is_dir, is_file, cmp omitted (unchanged) ...

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
            NodeType::Directory { .. } => self.name.bold().to_string(),
            NodeType::File { status, stats } => {
                let staged = status.contains('+');
                let color_name = if staged {
                    self.name.green()
                } else {
                    self.name.red()
                };

                let icon = if theme.is_nerd {
                    icons::get_icon(&self.name)
                } else {
                    theme.icon_file
                };

                let s_base = format!("{}{}", icon, color_name);
                let mut s = format!("{} ({})", s_base, status);

                if let Some((added, deleted)) = stats {
                    let total = added + deleted;
                    if total > 0 {
                        // Visual bar logic
                        // Example: | 5 +++--
                        // Scale? If total is huge, we cap the bar length?
                        // Let's assume max bar width of 5-10 chars.
                        let max_width = 10;

                        // Calculate ratio of + to -
                        // If total > max, we scale down.
                        // f64 ops might be overkill but safest.

                        let (plus_chars, minus_chars) = if total <= max_width {
                            (*added, *deleted)
                        } else {
                            let ratio = *added as f64 / total as f64;
                            let p = (ratio * max_width as f64).round() as usize;
                            let m = max_width - p;
                            (p, m)
                        };

                        let bar = format!(
                            "{}{}",
                            theme.diff_bar_char.to_string().repeat(plus_chars).green(),
                            theme.diff_bar_char.to_string().repeat(minus_chars).red()
                        );
                        s.push_str(&format!(" | {} {}", total, bar));
                    }
                }
                s
            }
        }
    }

    pub fn render_tree(&self, indent: usize, collapse: bool, theme: &Theme) -> String {
        let mut out = String::new();
        // Root is usually "."
        // If we are root, we don't collapse ourselves generally (unless we are just a wrapper?).
        // Logic for root node behavior
        // Let's assume root is never collapsed.

        out.push_str(&format!("{}\n", self.format_name(theme)));

        if let NodeType::Directory { children } = &self.node_type {
            self.render_children(children, indent, collapse, "", &mut out, theme);
        }
        out
    }

    fn render_children(
        &self,
        children: &[Node],
        indent: usize,
        collapse: bool,
        prefix: &str,
        out: &mut String,
        theme: &Theme,
    ) {
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            let is_last = i == count - 1;

            // Check for collapsing
            // If collapse is ON, and child is collapsible (Dir containing 1 Dir),
            // then we should effectively "skip" printing the connector for the child,
            // and instead print the collapsed chain.

            let (display_node, _effective_children) = if collapse {
                child.get_collapsed_view()
            } else {
                (child.clone(), None)
            };

            let children_to_render = match &display_node.node_type {
                NodeType::Directory { children } => Some(children),
                _ => None,
            };
            // Note: effective_children might be the children of the bottom-most collapsed node.
            // But get_collapsed_view should return a synthetic Node that has the name "a/b/c"
            // and the children of "c".

            // Prepare the connector
            let connector = if is_last {
                theme.tree_end
            } else {
                theme.tree_branch
            };
            let dashes = theme.tree_dash.to_string().repeat(indent - 2);

            // Print current child line
            out.push_str(&format!(
                "{}{}{} {}\n",
                prefix,
                connector,
                dashes,
                display_node.format_name(theme)
            ));

            // Recurse if child is directory (and we have children to show)
            if let Some(recurs_children) = children_to_render {
                let extension = if is_last {
                    " ".to_string()
                } else {
                    theme.tree_vertical.to_string()
                };
                let new_prefix = format!("{}{}{}", prefix, extension, " ".repeat(indent - 1));
                self.render_children(recurs_children, indent, collapse, &new_prefix, out, theme);
            }
        }
    }

    // Returns (NewNode, Option<OriginalChildren>)
    // Actually just returns a Node that represents the collapsed view.
    fn get_collapsed_view(&self) -> (Node, Option<Vec<Node>>) {
        if !self.is_dir() {
            return (self.clone(), None);
        }

        // Check if collapsible: Dir with exactly 1 child which is also a Dir
        if let NodeType::Directory { children } = &self.node_type {
            if children.len() == 1 {
                let only_child = &children[0];
                if only_child.is_dir() {
                    // It is collapsible.
                    // We need to recursively collapse the child.
                    let (child_collapsed, _) = only_child.get_collapsed_view();

                    // Combine names
                    let new_name = format!("{}/{}", self.name, child_collapsed.name);

                    // Create new combined node
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

    pub fn flatten(&self, indent_size: usize, collapse: bool, theme: &Theme) -> Vec<FlatNode> {
        let mut flattened = Vec::new();
        // Add root
        flattened.push(FlatNode {
            name: self.name.clone(),
            full_path: self.full_path.clone(),
            indent: 0,
            is_dir: self.is_dir(),
            status: self.get_status_char(),
            raw_status: self.get_raw_status(),
            connector: String::new(),
        });

        if let NodeType::Directory { children } = &self.node_type {
            self.flatten_children(children, indent_size, collapse, "", &mut flattened, theme);
        }
        flattened
    }

    fn flatten_children(
        &self,
        children: &[Node],
        indent_size: usize,
        collapse: bool,
        prefix: &str,
        out: &mut Vec<FlatNode>,
        theme: &Theme,
    ) {
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            let is_last = i == count - 1;

            let (display_node, _) = if collapse {
                child.get_collapsed_view()
            } else {
                (child.clone(), None)
            };

            let children_to_render = match &display_node.node_type {
                NodeType::Directory { children } => Some(children),
                _ => None,
            };

            // Prepare the connector
            let connector_symbol = if is_last {
                theme.tree_end
            } else {
                theme.tree_branch
            };
            let dashes = theme.tree_dash.to_string().repeat(indent_size - 2);
            let full_connector = format!("{}{}{} ", prefix, connector_symbol, dashes);

            out.push(FlatNode {
                name: display_node.get_display_name_clean(theme),
                full_path: display_node.full_path.clone(),
                indent: 0, // Not strictly needed if we have full_connector
                is_dir: display_node.is_dir(),
                status: display_node.get_status_char(),
                raw_status: display_node.get_raw_status(),
                connector: full_connector,
            });

            if let Some(recurs_children) = children_to_render {
                let extension = if is_last {
                    " ".to_string()
                } else {
                    theme.tree_vertical.to_string()
                };
                let new_prefix = format!("{}{}{}", prefix, extension, " ".repeat(indent_size - 1));
                self.flatten_children(
                    recurs_children,
                    indent_size,
                    collapse,
                    &new_prefix,
                    out,
                    theme,
                );
            }
        }
    }

    pub fn get_display_name_clean(&self, theme: &Theme) -> String {
        let icon = match &self.node_type {
            NodeType::Directory { .. } => theme.icon_dir,
            NodeType::File { .. } => {
                if theme.is_nerd {
                    icons::get_icon(&self.name)
                } else {
                    theme.icon_file
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
                    'M' // Default to Modified if not staged
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
                    // If any child is NOT staged (doesn't contain '+'),
                    // then the directory is not fully staged.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_directory_status() {
        // Case 1: All children staged
        let child1 = Node::new_file("a".to_string(), "a".to_string(), "M+".to_string(), None);
        let child2 = Node::new_file("b".to_string(), "b".to_string(), "A+".to_string(), None);
        let dir_staged = Node::new_dir("dir".to_string(), "dir".to_string(), vec![child1, child2]);
        assert_eq!(dir_staged.get_raw_status(), "M+");

        // Case 2: Mixed (one staged, one unstaged)
        let child3 = Node::new_file("c".to_string(), "c".to_string(), "M+".to_string(), None);
        let child4 = Node::new_file("d".to_string(), "d".to_string(), "M".to_string(), None); // Unstaged
        let dir_mixed = Node::new_dir(
            "dir_mixed".to_string(),
            "dir_mixed".to_string(),
            vec![child3, child4],
        );
        assert_eq!(dir_mixed.get_raw_status(), "M");

        // Case 3: All unstaged
        let child5 = Node::new_file("e".to_string(), "e".to_string(), "??".to_string(), None);
        let dir_unstaged = Node::new_dir(
            "dir_unstaged".to_string(),
            "dir_unstaged".to_string(),
            vec![child5],
        );
        assert_eq!(dir_unstaged.get_raw_status(), "M");

        // Case 4: Recursive
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
        // nested child is M (unstaged), so nested is M. Parent sees M (no +), so parent is M.
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
        // nested child is M+ (staged), so nested is M+. Parent sees M+ (has +), so parent is M+.
        assert_eq!(parent_dir_staged.get_raw_status(), "M+");
    }
}

#[derive(Debug, Clone)]
pub struct FlatNode {
    pub name: String,
    pub full_path: String,
    pub indent: usize,
    pub is_dir: bool,
    pub status: char,
    pub raw_status: String,
    pub connector: String,
}
