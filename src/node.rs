use colored::*;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum NodeType {
    File { status: String },
    Directory { children: Vec<Node> },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub node_type: NodeType,
}

impl Node {
    pub fn new_file(name: String, status: String) -> Self {
        Node {
            name,
            node_type: NodeType::File { status },
        }
    }

    pub fn new_dir(name: String, mut children: Vec<Node>) -> Self {
        children.sort_by(|a, b| a.cmp(b));
        Node {
            name,
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

    fn format_name(&self) -> String {
        match &self.node_type {
            NodeType::Directory { .. } => self.name.bold().to_string(),
            NodeType::File { status } => {
                let staged = status.contains('+');
                let color_name = if staged {
                    self.name.green()
                } else {
                    self.name.red()
                };
                // Example: name (M)
                format!("{} ({})", color_name, status)
            }
        }
    }

    pub fn render_tree(&self, indent: usize) -> String {
        let mut out = String::new();
        // Root is always printed as just the name (usually ".")
        out.push_str(&format!("{}\n", self.format_name()));
        
        if let NodeType::Directory { children } = &self.node_type {
            self.render_children(children, indent, "", &mut out);
        }
        out
    }

    fn render_children(&self, children: &[Node], indent: usize, prefix: &str, out: &mut String) {
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            let is_last = i == count - 1;
            
            // Prepare the connector
            let connector = if is_last { "└" } else { "├" };
            let dashes = "─".repeat(indent - 2);
            
            // Print current child line
            out.push_str(&format!("{}{}{} {}\n", prefix, connector, dashes, child.format_name()));
            
            // Recurse if child is directory
            if let NodeType::Directory { children: subchildren } = &child.node_type {
                let extension = if is_last { " " } else { "│" };
                let new_prefix = format!("{}{}{}", prefix, extension, " ".repeat(indent - 1));
                self.render_children(subchildren, indent, &new_prefix, out);
            }
        }
    }
}
