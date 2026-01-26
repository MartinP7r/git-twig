use ratatui::style::Color;

pub fn get_icon(name: &str, is_dir: bool) -> &'static str {
    if is_dir {
        return match name {
            "src" => "",
            "tests" => "",
            "build" | "dist" | "target" => "",
            "docs" => "",
            "config" => "",
            "scripts" => "",
            "assets" => "",
            ".git" => "",
            ".github" => "",
            _ => "", // Default folder icon
        };
    }

    // Special files first
    match name {
        "LICENSE" => "",
        "Makefile" => "",
        "Dockerfile" => "",
        "Cargo.toml" => "",
        "package.json" => "",
        ".env" => "",
        ".gitignore" => "",
        _ => {
            if let Some(ext) = std::path::Path::new(name)
                .extension()
                .and_then(|s| s.to_str())
            {
                match ext {
                    "rs" => "",
                    "toml" => "",
                    "md" => "",
                    "json" => "",
                    "yml" | "yaml" => "",
                    "lock" => "",
                    "sh" => "",
                    "py" => "",
                    "js" => "",
                    "ts" => "",
                    "go" => "",
                    "rb" => "",
                    "java" => "",
                    "c" => "",
                    "cpp" => "",
                    "swift" => "",
                    "kt" => "",
                    "css" => "",
                    "html" => "",
                    "sql" => "",
                    "png" | "jpg" | "jpeg" | "gif" | "svg" => "",
                    _ => "",
                }
            } else {
                ""
            }
        }
    }
}

pub fn get_icon_color(name: &str, is_dir: bool) -> Option<Color> {
    if is_dir {
        return match name {
            "src" | "scripts" | "tests" => Some(Color::Cyan),
            "build" | "dist" | "target" => Some(Color::Yellow),
            "docs" => Some(Color::Blue),
            ".git" | ".github" => Some(Color::Gray),
            _ => Some(Color::Blue),
        };
    }

    match name {
        "LICENSE" => Some(Color::Yellow),
        "Makefile" | "Cargo.toml" | "package.json" => Some(Color::Red),
        ".env" | ".gitignore" => Some(Color::DarkGray),
        _ => {
            if let Some(ext) = std::path::Path::new(name)
                .extension()
                .and_then(|s| s.to_str())
            {
                match ext {
                    "rs" => Some(Color::Red),                                // Rust
                    "js" | "ts" => Some(Color::Yellow),                      // JS/TS
                    "py" => Some(Color::Blue),                               // Python
                    "go" => Some(Color::Cyan),                               // Go
                    "c" | "cpp" | "h" | "hpp" => Some(Color::Blue),          // C/C++
                    "java" | "kt" => Some(Color::Red),                       // JVM
                    "rb" => Some(Color::Red),                                // Ruby
                    "md" => Some(Color::Blue),                               // Markdown
                    "json" | "yaml" | "yml" | "toml" => Some(Color::Yellow), // Config
                    "sh" => Some(Color::Green),                              // Shell
                    "html" => Some(Color::Red),                              // HTML
                    "css" => Some(Color::Blue),                              // CSS
                    "lock" => Some(Color::Gray),
                    "png" | "jpg" | "jpeg" | "gif" | "svg" => Some(Color::Magenta), // Images
                    _ => Some(Color::White),
                }
            } else {
                Some(Color::White)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_extensions() {
        assert_eq!(get_icon("file.rs", false), "");
        assert_eq!(get_icon("test.md", false), "");
        assert_eq!(get_icon("script.sh", false), "");
    }

    #[test]
    fn test_special_files() {
        assert_eq!(get_icon("LICENSE", false), "");
        assert_eq!(get_icon("Makefile", false), "");
        assert_eq!(get_icon("Cargo.toml", false), "");
    }

    #[test]
    fn test_special_directories() {
        assert_eq!(get_icon("src", true), "");
        assert_eq!(get_icon("target", true), "");
        assert_eq!(get_icon(".git", true), "");
        assert_eq!(get_icon("other", true), "");
    }

    #[test]
    fn test_unknown_file() {
        assert_eq!(get_icon("unknown.xyz", false), "");
        assert_eq!(get_icon("README", false), ""); // No extension and not special
    }
}
