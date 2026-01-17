pub fn get_icon(name: &str) -> &'static str {
    if let Some(ext) = std::path::Path::new(name).extension().and_then(|s| s.to_str()) {
        match ext {
            "rs" => "🦀", // Rust (or )
            "toml" => "⚙️ ", // Config
            "md" => "📝", // Markdown (or )
            "json" => "IO",
            "yml" | "yaml" => "it",
            "lock" => "🔒",
            "sh" => "🐚",
            "png" | "jpg" | "jpeg" | "gif" | "svg" => "🖼️ ",
            "gitignore" => "🙈",
            _ => "📄", // Default file
        }
    } else {
        // Dotfiles or no extension
        if name == "LICENSE" {
            "⚖️ "
        } else if name == "Makefile" {
            "🛠️ "
        } else {
             "📄"
        }
    }
}
