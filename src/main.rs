use anyhow::{Context, Result};
use clap::Parser;
use std::process::Command;

mod interactive;
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

    /// Start interactive mode
    #[arg(short = 'I', long)]
    interactive: bool,

    /// Show only staged files
    #[arg(short, long)]
    staged_only: bool,

    /// Show only modified files (hide untracked)
    #[arg(short, long)]
    modified_only: bool,
}

fn get_git_config(key: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["config", "--global", key])
        .output()
        .ok()?;

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    } else {
        None
    }
}

fn determine_indent(arg_indent: Option<usize>) -> usize {
    let indent = arg_indent
        .or_else(|| get_git_config("twig.indent").and_then(|s| s.parse().ok()))
        .unwrap_or(3);

    indent.clamp(2, 10)
}

fn determine_collapse(arg_collapse: bool) -> bool {
    if arg_collapse {
        return true;
    }
    get_git_config("twig.collapse")
        .map(|s| s == "true")
        .unwrap_or(false)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let indent = determine_indent(args.indent);
    let collapse = determine_collapse(args.collapse);

    if args.interactive {
        // Interactive mode currently does not support filtering (not requested in roadmap yet)
        return interactive::run(indent, collapse);
    }

    let result_node = match build_tree_from_git(args.staged_only, args.modified_only) {
        Ok(Some(node)) => node,
        Ok(None) => {
            println!("(working directory clean)");
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    print!("{}", result_node.render_tree(indent, collapse));

    Ok(())
}

pub fn build_tree_from_git(staged_only: bool, modified_only: bool) -> Result<Option<node::Node>> {
    // Run git status --porcelain -b (to get branch info)
    let status_output = Command::new("git")
        .args(["status", "--porcelain", "-b"])
        .output()
        .context("Failed to execute git status")?;

    if !status_output.status.success() {
        let err = String::from_utf8_lossy(&status_output.stderr);
        return Err(anyhow::anyhow!("Git status failed: {}", err));
    }

    let status_stdout = String::from_utf8_lossy(&status_output.stdout);
    let mut lines: Vec<String> = status_stdout.lines().map(|s| s.to_string()).collect();

    if lines.is_empty() {
        return Ok(None);
    }

    // Process header
    if let Some(first) = lines.first() {
        if first.starts_with("##") {
            let header = first.clone();
            // Remove header from lines to be processed by tree parser
            lines.remove(0);
            
            print_context_header(&header);
            
            if lines.is_empty() {
                println!("(working directory clean)");
                return Ok(None);
            }
        }
    }

    // Collect diff stats
    let mut stats = std::collections::HashMap::new();
    collect_diff_stats(&mut stats, &["diff", "--numstat"])?;
    collect_diff_stats(&mut stats, &["diff", "--cached", "--numstat"])?;

    let result_node = parser::build_tree(lines, &stats, staged_only, modified_only)?;
    Ok(Some(result_node))
}

fn print_context_header(line: &str) {
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
        (&content[..idx], Some(&content[idx+3..]))
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

fn collect_diff_stats(
    stats: &mut std::collections::HashMap<String, (usize, usize)>,
    args: &[&str],
) -> Result<()> {
    let output = Command::new("git").args(args).output()?;

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
