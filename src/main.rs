use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;

mod config;
mod git;
mod icons;
mod node;
mod parser;
mod theme;
mod tui;

use crate::theme::{Theme, ThemeType};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Set indentation (2-10 spaces)
    #[arg(short, long)]
    indent: Option<usize>,

    /// Collapse directories containing only another directory
    #[arg(short, long)]
    collapse: bool,

    /// Start interactive mode
    #[arg(short = 'I', long)]
    interactive: bool,

    /// Show only staged files
    #[arg(short, long)]
    staged_only: bool,

    /// Show only modified files (hide untracked)
    #[arg(short, long)]
    modified_only: bool,

    /// Open all modified files in $EDITOR
    #[arg(short, long)]
    open: bool,

    /// Show only untracked files
    #[arg(long)]
    untracked_only: bool,

    /// Visual theme (ascii, unicode, nerd)
    #[arg(long, value_enum)]
    theme: Option<ThemeType>,

    /// Use simple icons (generic folder/file) instead of rich Nerd Font icons
    #[arg(long)]
    simple_icons: bool,
}

fn determine_indent(arg_indent: Option<usize>) -> usize {
    let indent = arg_indent
        .or_else(|| git::get_config("twig.indent").and_then(|s| s.parse().ok()))
        .unwrap_or(3);

    indent.clamp(2, 10)
}

fn determine_collapse(arg_collapse: bool) -> bool {
    if arg_collapse {
        return true;
    }
    git::get_config("twig.collapse")
        .map(|s| s == "true")
        .unwrap_or(false)
}

fn determine_theme(arg_theme: Option<ThemeType>) -> Theme {
    let mut theme = if let Some(t) = arg_theme {
        Theme::new(t)
    } else if let Some(val) = git::get_config("twig.theme") {
        match val.as_str() {
            "unicode" => Theme::unicode(),
            "nerd" => Theme::nerd(),
            "ascii" => Theme::ascii(),
            "rounded" => Theme::rounded(),
            _ => Theme::unicode(),
        }
    } else {
        Theme::unicode()
    };

    theme.load_overrides();
    theme
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.interactive && args.open {
        anyhow::bail!("Cannot use both --interactive and --open");
    }

    let indent = determine_indent(args.indent);
    let collapse = determine_collapse(args.collapse);
    let theme = determine_theme(args.theme).with_simple_icons(args.simple_icons);

    if args.interactive {
        return tui::run(indent, collapse, theme);
    }

    let result_node =
        match git::build_tree_from_git(args.staged_only, args.modified_only, args.untracked_only) {
            Ok(Some(node)) => node,
            Ok(None) => {
                if !args.open {
                    if let Ok(header) = git::get_status_header() {
                        print_context_header(&header);
                    }
                    println!("(working directory clean)");
                }
                return Ok(());
            }
            Err(e) => return Err(e),
        };

    if args.open {
        let collapsed_paths = std::collections::HashSet::new();
        let files: Vec<String> = result_node
            .flatten(indent, collapse, &theme, &collapsed_paths)
            .into_iter()
            .filter(|node| !node.is_dir)
            .map(|node| node.full_path)
            .collect();

        if files.is_empty() {
            println!("No modified files to open.");
            return Ok(());
        }

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());

        let status = Command::new(&editor)
            .args(&files)
            .status()
            .with_context(|| format!("Failed to launch editor: {}", editor))?;

        if !status.success() {
            anyhow::bail!("Editor exited with error");
        }
        return Ok(());
    }

    if let Ok(header) = git::get_status_header() {
        print_context_header(&header);
    }
    print!("{}", result_node.render_tree(indent, collapse, &theme));

    Ok(())
}

fn print_context_header(line: &str) {
    if line.is_empty() {
        return;
    }
    // Format: ## <local>...<remote> [ahead <N>, behind <M>]
    // or: ## <local>
    // or: ## No commits yet on <local>

    let content = line.trim_start_matches("## ").trim();

    // Parse
    // 1. Check for "No commits yet on "
    if let Some(branch) = content.strip_prefix("No commits yet on ") {
        println!("On branch {} (No commits yet)", branch);
        return;
    }

    // 2. Split by ... to find remote
    // "main...origin/main [ahead 1]"
    let (local_part, rest) = if let Some(idx) = content.find("...") {
        (&content[..idx], Some(&content[idx + 3..]))
    } else {
        (content, None)
    };

    // 3. Process rest (remote + counts)
    let (remote, counts) = if let Some(r) = rest {
        if let Some(bracket_idx) = r.find(" [") {
            (Some(&r[..bracket_idx]), Some(&r[bracket_idx..]))
        } else {
            (Some(r), None)
        }
    } else {
        (None, None)
    };

    print!("On branch \x1b[1m{}\x1b[0m", local_part);

    if let Some(remote_name) = remote {
        print!(" -> {}", remote_name);
    }

    if let Some(counts_str) = counts {
        // [ahead 1, behind 2] or [ahead 1]
        let stripped = counts_str.trim_matches(|c| c == '[' || c == ']');
        let parts: Vec<&str> = stripped.split(", ").collect();

        for part in parts {
            if let Some(ahead) = part.strip_prefix("ahead ") {
                print!(" \x1b[32m⬆️ {}\x1b[0m", ahead);
            } else if let Some(behind) = part.strip_prefix("behind ") {
                print!(" \x1b[31m⬇️ {}\x1b[0m", behind);
            } else if part == "gone" {
                print!(" \x1b[31m(gone)\x1b[0m");
            }
        }
    }

    println!(); // Newline
}
