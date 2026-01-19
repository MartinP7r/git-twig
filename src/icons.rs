pub fn get_icon(name: &str) -> &'static str {
    // Special files first
    match name {
        "LICENSE" => "",
        "Makefile" => "",
        "Dockerfile" => "",
        "Cargo.toml" => "",
        "package.json" => "",
        ".env" => "",
        "src" => "",
        "tests" => "",
        "build" | "dist" | "target" => "",
        "docs" => "",
        "config" => "",
        "scripts" => "",
        "assets" => "",
        ".git" => "",
        ".github" => "",
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
                    "gitignore" => "",
                    _ => "",
                }
            } else {
                ""
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_extensions() {
        assert_eq!(get_icon("file.rs"), "");
        assert_eq!(get_icon("test.md"), "");
        assert_eq!(get_icon("script.sh"), "");
    }

    #[test]
    fn test_special_files() {
        assert_eq!(get_icon("LICENSE"), "");
        assert_eq!(get_icon("Makefile"), "");
        assert_eq!(get_icon("Cargo.toml"), "");
    }

    #[test]
    fn test_special_directories() {
        assert_eq!(get_icon("src"), "");
        assert_eq!(get_icon("target"), "");
        assert_eq!(get_icon(".git"), "");
    }

    #[test]
    fn test_unknown_file() {
        assert_eq!(get_icon("unknown.xyz"), "");
        assert_eq!(get_icon("README"), ""); // No extension and not special
    }
}
