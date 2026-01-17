use crate::node::Node;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
struct BuilderNode {
    name: String,
    full_path: String,
    children: HashMap<String, BuilderNode>,
    file_status: Option<String>,
    stats: Option<(usize, usize)>,
}

pub fn parse_status_line(line: &str) -> Option<(String, String)> {
    if line.len() < 4 {
        return None;
    }

    let x = line.chars().next()?;
    let y = line.chars().nth(1)?;
    let rest = &line[3..];

    // Determine status string
    let status = if x == '?' && y == '?' {
        "??".to_string()
    } else if y == ' ' {
        format!("{}+", x)
    } else {
        y.to_string()
    };

    // Handle renames
    Some((rest.to_string(), status))
}

pub fn build_tree(
    lines: Vec<String>,
    stats: &HashMap<String, (usize, usize)>,
    staged_only: bool,
    modified_only: bool,
) -> Result<Node> {
    let mut root = BuilderNode {
        name: ".".to_string(),
        full_path: ".".to_string(),
        children: HashMap::new(),
        file_status: None,
        stats: None,
    };

    for line in lines {
        if let Some((path_str, status)) = parse_status_line(&line) {
            // Filter logic
            if staged_only && !status.ends_with('+') {
                continue;
            }
            // "modified_only" means hide untracked (??)
            // status for untracked is "??"
            if modified_only && status == "??" {
                continue;
            }

            // Handle rename special display
            // If status has 'R' and path has " -> ", we need to split
            let (effective_path, display_name) =
                if status.contains('R') && path_str.contains(" -> ") {
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
                        (path_str.clone(), path_str.clone())
                    }
                } else {
                    (
                        path_str.clone(),
                        std::path::Path::new(&path_str)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                    )
                };

            // Look up stats for the *full path* (which is effective_path generally, or keys in stats map)
            // Note: if rename, stats map is keyed by which path?
            // `git diff --numstat` shows "old => new" or just "new"?
            // git status --porcelain shows "old -> new".
            // git diff usually uses post-image name for working tree?
            // If I modify 'a', it shows 'a'.
            // If I renamed 'a' -> 'b', and modified 'b', `git diff` shows 'b'.
            // If I renamed 'a' -> 'b' (staged), and modified 'b' (unstaged).
            // `stats` collection logic:
            // "path" = "b".
            // `effective_path` logic uses "a" (old path) as the location in tree if we used `old` in rename block above.
            // In the rename block above: `(old.to_string(), name)`.
            // So we are inserting into `old` path location.
            // But stats might be keyed by `new` path?
            // Let's check keys in stats map.
            // If I key by `path_str` (raw from porcelain e.g. "old -> new"), I won't match "new".
            // I should try to lookup both or use fuzzy logic?
            // Simplest: lookup `effective_path`. If it's a rename, effective_path is `old`.
            // But diff stats will likely list `new`.

            // Let's extract `new` path from rename string if needed.
            let lookup_path = if status.contains('R') && path_str.contains(" -> ") {
                let parts: Vec<&str> = path_str.split(" -> ").collect();
                if parts.len() == 2 {
                    parts[1]
                } else {
                    &path_str
                }
            } else {
                &path_str
            };

            let file_stats = stats.get(lookup_path).cloned();

            // Insert into tree
            // "a/b/c.txt" -> traverse "a", "b", insert "c.txt"
            let path = std::path::Path::new(&effective_path);
            let components: Vec<_> = path
                .components()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .collect();

            let mut current = &mut root;
            let mut current_path = String::new();

            // Navigate directories
            for part in &components[..components.len() - 1] {
                if !current_path.is_empty() {
                    current_path.push('/');
                }
                current_path.push_str(part);

                current = current
                    .children
                    .entry(part.clone())
                    .or_insert_with(|| BuilderNode {
                        name: part.clone(),
                        full_path: current_path.clone(),
                        children: HashMap::new(),
                        file_status: None,
                        stats: None,
                    });
            }

            // Insert leaf
            let leaf = BuilderNode {
                name: display_name,
                full_path: effective_path, // This is the full original path
                children: HashMap::new(),
                file_status: Some(status),
                stats: file_stats,
            };

            current.children.insert(leaf.name.clone(), leaf);
        }
    }

    Ok(convert_builder(root))
}

fn convert_builder(builder: BuilderNode) -> Node {
    if let Some(status) = builder.file_status {
        Node::new_file(builder.name, builder.full_path, status, builder.stats)
    } else {
        // Directory
        let children: Vec<Node> = builder
            .children
            .into_values()
            .map(convert_builder)
            .collect();
        Node::new_dir(builder.name, builder.full_path, children)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status_simple() {
        assert_eq!(
            parse_status_line("M  file.txt").unwrap(),
            ("file.txt".to_string(), "M+".to_string())
        );
        assert_eq!(
            parse_status_line("?? file.txt").unwrap(),
            ("file.txt".to_string(), "??".to_string())
        );
        assert_eq!(
            parse_status_line("D  file.txt").unwrap(),
            ("file.txt".to_string(), "D+".to_string())
        );
        assert_eq!(
            parse_status_line("A  file.txt").unwrap(),
            ("file.txt".to_string(), "A+".to_string())
        );
    }

    #[test]
    fn test_parse_status_modified_staged() {
        assert_eq!(
            parse_status_line("MM file.txt").unwrap(),
            ("file.txt".to_string(), "M".to_string())
        );
        assert_eq!(
            parse_status_line("AM file.txt").unwrap(),
            ("file.txt".to_string(), "M".to_string())
        );
    }

    #[test]
    fn test_parse_status_rename() {
        assert_eq!(
            parse_status_line("R  old.txt -> new.txt").unwrap(),
            ("old.txt -> new.txt".to_string(), "R+".to_string())
        );
    }

    #[test]
    fn test_parse_status_spaces() {
        assert_eq!(
            parse_status_line("M  my file with spaces.txt").unwrap(),
            ("my file with spaces.txt".to_string(), "M+".to_string())
        );
    }

    use crate::node::NodeType;

    #[test]
    fn test_parse_short_line() {
        assert_eq!(parse_status_line(""), None);
        assert_eq!(parse_status_line("M"), None);
        assert_eq!(parse_status_line("M  "), None); // len 3
    }

    #[test]
    fn test_build_tree_filtering_staged() {
        let lines = vec![
            "M  staged.txt".to_string(), // M+
            " M unstaged.txt".to_string(), // M
        ];
        let stats = HashMap::new();
        
        // Filter staged only
        let node = build_tree(lines.clone(), &stats, true, false).unwrap();
        // Should only contain staged.txt
        if let NodeType::Directory { children } = node.node_type {
            assert_eq!(children.len(), 1);
            assert!(children.iter().any(|c| c.name == "staged.txt"));
        } else {
            panic!("Root should be a directory");
        }
        
        // No filter
        let node = build_tree(lines, &stats, false, false).unwrap();
        if let NodeType::Directory { children } = node.node_type {
            assert_eq!(children.len(), 2);
        } else {
            panic!("Root should be a directory");
        }
    }

    #[test]
    fn test_build_tree_filtering_modified() {
        let lines = vec![
            "?? untracked.txt".to_string(),
            " M modified.txt".to_string(),
        ];
        let stats = HashMap::new();
        
        // Filter modified only (hide untracked)
        let node = build_tree(lines.clone(), &stats, false, true).unwrap();
        
        if let NodeType::Directory { children } = node.node_type {
            assert_eq!(children.len(), 1);
            assert!(children.iter().any(|c| c.name == "modified.txt"));
        } else {
            panic!("Root should be a directory");
        }
        
        // No filter
        let node = build_tree(lines, &stats, false, false).unwrap();
        if let NodeType::Directory { children } = node.node_type {
            assert_eq!(children.len(), 2);
        } else {
            panic!("Root should be a directory");
        }
    }
}
