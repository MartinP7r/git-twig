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
    }).unwrap_or(3);
    
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
    let collapse = determine_collapse(args.collapse);

    // Run git status --porcelain
    let status_output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .context("Failed to execute git status")?;

    if !status_output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&status_output.stderr));
        std::process::exit(status_output.status.code().unwrap_or(1));
    }

    let status_stdout = String::from_utf8_lossy(&status_output.stdout);
    let lines: Vec<String> = status_stdout.lines().map(|s| s.to_string()).collect();

    if lines.is_empty() {
        println!("(working directory clean)");
        return Ok(());
    }

    // Collect diff stats
    // We need both unstaged and staged stats
    let mut stats = std::collections::HashMap::new();
    collect_diff_stats(&mut stats, &["diff", "--numstat"])?;
    collect_diff_stats(&mut stats, &["diff", "--cached", "--numstat"])?;

    let result_node = parser::build_tree(lines, &stats)?;
    print!("{}", result_node.render_tree(indent, collapse));

    Ok(())
}

fn collect_diff_stats(
    stats: &mut std::collections::HashMap<String, (usize, usize)>, 
    args: &[&str]
) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .output()?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // components: added deleted path
                // We re-parse properly below using tabs, so we just check for basic structure here.
                
                // Re-parsing line properly
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    let added = parts[0].parse::<usize>().unwrap_or(0);
                    let deleted = parts[1].parse::<usize>().unwrap_or(0);
                    let path = parts[2].to_string();
                    
                    let entry = stats.entry(path).or_insert((0, 0));
                    entry.0 += added;
                    entry.1 += deleted;
                }
            }
        }
    }
    Ok(())
}
