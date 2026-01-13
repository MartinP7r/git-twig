use crate::node::Node;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct BuilderNode {
    name: String,
    children: HashMap<String, BuilderNode>,
    file_status: Option<String>,
}

pub fn parse_status_line(line: &str) -> Option<(String, String)> {
    if line.len() < 4 {
        return None;
    }

    let x = line.chars().nth(0)?;
    let y = line.chars().nth(1)?;
    let rest = &line[3..];

    // Determine status string like Ruby version
    let status = if y == ' ' {
        format!("{}+", x)
    } else {
        y.to_string()
    };

    // Handle renames
    Some((rest.to_string(), status))
}

pub fn build_tree(lines: Vec<String>) -> Result<Node> {
    let mut root = BuilderNode {
        name: ".".to_string(),
        children: HashMap::new(),
        file_status: None,
    };
    
    for line in lines {
        if let Some((path_str, status)) = parse_status_line(&line) {
            // Handle rename special display
            // If status has 'R' and path has " -> ", we need to split
            let (effective_path, display_name) = if status.contains('R') && path_str.contains(" -> ") {
                let parts: Vec<&str> = path_str.split(" -> ").collect();
                if parts.len() == 2 {
                    let old = parts[0];
                    let new = parts[1];
                    
                    let old_path = std::path::Path::new(old);
                    let new_path = std::path::Path::new(new);
                    let old_dir = old_path.parent().unwrap_or(std::path::Path::new(""));
                    let new_dir = new_path.parent().unwrap_or(std::path::Path::new(""));
                    
                    let old_name = old_path.file_name().unwrap().to_string_lossy();
                    // unused: let new_name = new_path.file_name().unwrap().to_string_lossy();
                    
                    let name = if old_dir == new_dir {
                         // We need just the basename of new
                         let new_base = new_path.file_name().unwrap().to_string_lossy();
                        format!("{} -> {}", old_name, new_base)
                    } else {
                        format!("{} -> {}", old_name, new)
                    };
                    
                    (old.to_string(), name)
                } else {
                    (path_str.clone(), path_str)
                }
            } else {
                (path_str.clone(), std::path::Path::new(&path_str).file_name().unwrap_or_default().to_string_lossy().to_string())
            };
            
            // Insert into tree
            // "a/b/c.txt" -> traverse "a", "b", insert "c.txt"
            let path = std::path::Path::new(&effective_path);
            let components: Vec<_> = path.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
            
            let mut current = &mut root;
            
            // Navigate directories
            for part in &components[..components.len() - 1] {
                current = current.children.entry(part.clone()).or_insert_with(|| BuilderNode {
                    name: part.clone(),
                    children: HashMap::new(),
                    file_status: None,
                });
            }
            
            // Insert leaf
            let leaf = BuilderNode {
                name: display_name,
                children: HashMap::new(),
                file_status: Some(status),
            };
            
            current.children.insert(leaf.name.clone(), leaf);
        }
    }
    
    Ok(convert_builder(root))
}

fn convert_builder(builder: BuilderNode) -> Node {
    if let Some(status) = builder.file_status {
        Node::new_file(builder.name, status)
    } else {
        // Directory
        let children: Vec<Node> = builder.children.into_values().map(convert_builder).collect();
        Node::new_dir(builder.name, children)
    }
}
