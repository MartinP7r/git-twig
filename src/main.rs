use clap::Parser;
use anyhow::{Context, Result};
use std::process::Command;

mod node;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Set indentation (2-10 spaces)
    #[arg(short, long)]
    indent: Option<usize>,

    /// Collapse directories containing only another directory
    #[arg(short, long)]
    collapse: bool,
}

fn get_git_config(key: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--global", key])
        .output()
        .ok()?;
    
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if s.is_empty() { None } else { Some(s) }
    } else {
        None
    }
}

fn determine_indent(arg_indent: Option<usize>) -> usize {
    let indent = arg_indent.or_else(|| {
        get_git_config("status-tree.indent")
            .and_then(|s| s.parse().ok())
    }).unwrap_or(4);
    
    indent.clamp(2, 10)
}

fn determine_collapse(arg_collapse: bool) -> bool {
    if arg_collapse {
        return true;
    }
    get_git_config("status-tree.collapse")
        .map(|s| s == "true")
        .unwrap_or(false)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let indent = determine_indent(args.indent);
    // let collapse = determine_collapse(args.collapse); // TODO: pass to render/build if needed

    // Run git status --porcelain
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .context("Failed to execute git status")?;

    if !output.status.success() {
        // Just print stderr and exit
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(output.status.code().unwrap_or(1));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

    if lines.is_empty() {
        println!("(working directory clean)");
        return Ok(());
    }

    let result_node = parser::build_tree(lines)?;
    print!("{}", result_node.render_tree(indent));

    Ok(())
}
