use colored::*;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum NodeType {
    File { status: String, stats: Option<(usize, usize)> },
    Directory { children: Vec<Node> },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub full_path: String,
    pub node_type: NodeType,
}

impl Node {
    pub fn new_file(name: String, full_path: String, status: String, stats: Option<(usize, usize)>) -> Self {
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

    fn format_name(&self) -> String {
        match &self.node_type {
            NodeType::Directory { .. } => self.name.bold().to_string(),
            NodeType::File { status, stats } => {
                let staged = status.contains('+');
                let color_name = if staged {
                    self.name.green()
                } else {
                    self.name.red()
                };
                
                let mut s = format!("{} ({})", color_name, status);
                
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
                        
                        let bar = format!("{}{}", "+".repeat(plus_chars).green(), "-".repeat(minus_chars).red());
                        s.push_str(&format!(" | {} {}", total, bar));
                    }
                }
                s
            }
        }
    }

    pub fn render_tree(&self, indent: usize, collapse: bool) -> String {
        let mut out = String::new();
        // Root is usually "." 
        // If we are root, we don't collapse ourselves generally (unless we are just a wrapper?). 
        // Logic for root node behavior
        // Let's assume root is never collapsed.
        
        out.push_str(&format!("{}\n", self.format_name()));
        
        if let NodeType::Directory { children } = &self.node_type {
            self.render_children(children, indent, collapse, "", &mut out);
        }
        out
    }

    fn render_children(
        &self, 
        children: &[Node], 
        indent: usize, 
        collapse: bool, 
        prefix: &str, 
        out: &mut String
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
            let connector = if is_last { "└" } else { "├" };
            let dashes = "─".repeat(indent - 2);
            
            // Print current child line
            out.push_str(&format!("{}{}{} {}\n", prefix, connector, dashes, display_node.format_name()));
            
            // Recurse if child is directory (and we have children to show)
            if let Some(recurs_children) = children_to_render {
                 let extension = if is_last { " " } else { "│" };
                 let new_prefix = format!("{}{}{}", prefix, extension, " ".repeat(indent - 1));
                 // We recurse on the ORIGINAL logic's children?
                 // No, on the children of the display node.
                 self.render_children(recurs_children, indent, collapse, &new_prefix, out);
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

    pub fn flatten(&self, indent_size: usize, collapse: bool) -> Vec<FlatNode> {
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
            self.flatten_children(children, indent_size, collapse, "", &mut flattened);
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
            let connector_symbol = if is_last { "└" } else { "├" };
            let dashes = "─".repeat(indent_size - 2);
            let full_connector = format!("{}{}{} ", prefix, connector_symbol, dashes);

            out.push(FlatNode {
                name: display_node.get_display_name_clean(),
                full_path: display_node.full_path.clone(),
                indent: 0, // Not strictly needed if we have full_connector
                is_dir: display_node.is_dir(),
                status: display_node.get_status_char(),
                raw_status: display_node.get_raw_status(),
                connector: full_connector,
            });

            if let Some(recurs_children) = children_to_render {
                let extension = if is_last { " " } else { "│" };
                let new_prefix = format!("{}{}{}", prefix, extension, " ".repeat(indent_size - 1));
                self.flatten_children(recurs_children, indent_size, collapse, &new_prefix, out);
            }
        }
    }

    pub fn get_display_name_clean(&self) -> String {
        match &self.node_type {
            NodeType::Directory { .. } => self.name.clone(),
            NodeType::File { status, .. } => {
                format!("{} ({})", self.name, status)
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
            NodeType::Directory { .. } => String::new(),
        }
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
