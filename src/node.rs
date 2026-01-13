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

    pub fn render_tree(&self, indent: usize, collapse: bool) -> String {
        let mut out = String::new();
        // Root is usually "." 
        // If we are root, we don't collapse ourselves generally (unless we are just a wrapper?). 
        // The Ruby version behaves slightly differently for root.
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
                        node_type: child_collapsed.node_type.clone(),
                    };
                    return (combined, None);
                }
            }
        }
        
        (self.clone(), None)
    }
}
