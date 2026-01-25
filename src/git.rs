use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::process::Command;

use crate::node;
use crate::parser;

#[derive(Debug, Clone, Serialize)]
pub struct Worktree {
    pub path: String,
    pub head: String,
    pub branch: String,
}

pub fn build_tree_from_git(
    staged_only: bool,
    modified_only: bool,
    untracked_only: bool,
) -> Result<Option<node::Node>> {
    // Run git status --porcelain -b -u
    let status_output = Command::new("git")
        .args(["status", "--porcelain", "-b", "-u"])
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

    // Remove header (starts with ##)
    if let Some(first) = lines.first() {
        if first.starts_with("##") {
            lines.remove(0);
            if lines.is_empty() {
                return Ok(None);
            }
        }
    }

    // Collect diff stats
    let mut stats = HashMap::new();
    collect_diff_stats(&mut stats, &["diff", "--numstat"])?;
    collect_diff_stats(&mut stats, &["diff", "--cached", "--numstat"])?;

    let result_node =
        parser::build_tree(lines, &stats, staged_only, modified_only, untracked_only)?;
    Ok(Some(result_node))
}

pub fn get_status_header() -> Result<String> {
    let output = Command::new("git")
        .args(["status", "--porcelain", "-b"])
        .output()
        .context("Failed to get branch status")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(first) = stdout.lines().next() {
            if first.starts_with("##") {
                return Ok(first.to_string());
            }
        }
    }
    Ok(String::new())
}

pub fn collect_diff_stats(
    stats: &mut HashMap<String, (usize, usize)>,
    args: &[&str],
) -> Result<()> {
    let output = Command::new("git").args(args).output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
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
    Ok(())
}

pub fn toggle_stage(path: &str, is_staged: bool) -> Result<()> {
    let mut args = if is_staged {
        vec!["restore", "--staged"]
    } else {
        vec!["add"]
    };
    args.push(path);

    let status = Command::new("git").args(args).status()?;
    if !status.success() {
        anyhow::bail!("Git command failed");
    }
    Ok(())
}

pub fn commit(message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("Failed to execute git commit")?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git commit failed: {}", err);
    }
    Ok(())
}

pub fn get_diff(path: &str, is_staged: bool, is_untracked: bool) -> Result<String> {
    let mut args = vec!["diff", "--color=always"];

    if is_staged {
        args.push("--cached");
    }

    if is_untracked {
        args.push("--no-index");
        args.push("/dev/null");
    }

    args.push(path);

    let output = Command::new("git").args(args).output()?;

    if output.status.success() || output.status.code() == Some(1) {
        let content = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(content)
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git diff failed: {}", err)
    }
}

pub fn get_config(key: &str) -> Option<String> {
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
pub fn get_config_regexp(pattern: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let output = Command::new("git")
        .args(["config", "--get-regexp", pattern])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            for line in s.lines() {
                let parts: Vec<&str> = line.splitn(2, ' ').collect();
                if parts.len() == 2 {
                    map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }
    }
    map
}

pub fn get_worktrees() -> Result<Vec<Worktree>> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .context("Failed to list worktrees")?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git worktree list failed: {}", err);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_worktrees(&stdout))
}

fn parse_worktrees(stdout: &str) -> Vec<Worktree> {
    let mut worktrees = Vec::new();
    let mut current_path = String::new();
    let mut current_head = String::new();
    let mut current_branch = String::new();

    for line in stdout.lines() {
        if line.is_empty() {
            if !current_path.is_empty() {
                worktrees.push(Worktree {
                    path: current_path.clone(),
                    head: current_head.clone(),
                    branch: current_branch.clone(),
                });
                current_path.clear();
                current_head.clear();
                current_branch.clear();
            }
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "worktree" => current_path = parts[1].to_string(),
            "HEAD" => current_head = parts[1].to_string(),
            "branch" => current_branch = parts[1].to_string(),
            _ => {}
        }
    }

    // Push the last one if it exists
    if !current_path.is_empty() {
        worktrees.push(Worktree {
            path: current_path,
            head: current_head,
            branch: current_branch,
        });
    }

    worktrees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_worktrees() {
        let output = "worktree /path/to/repo\nHEAD 123456\nbranch refs/heads/main\n\nworktree /path/to/other\nHEAD abcdef\nbranch refs/heads/dev\n";
        let worktrees = parse_worktrees(output);
        assert_eq!(worktrees.len(), 2);
        assert_eq!(worktrees[0].path, "/path/to/repo");
        assert_eq!(worktrees[0].branch, "refs/heads/main");
        assert_eq!(worktrees[1].path, "/path/to/other");
        assert_eq!(worktrees[1].branch, "refs/heads/dev");
    }
}
