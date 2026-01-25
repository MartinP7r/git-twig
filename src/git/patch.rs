use anyhow::{Context, Result};
use std::process::Command;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Hunk {
    pub header: String,
    pub content: String,
    pub display_start: usize, // Line index in the full diff display
    pub display_end: usize,
}

pub fn parse_diff(diff_content: &str) -> (Vec<String>, Vec<Hunk>) {
    let mut headers = Vec::new();
    let mut hunks = Vec::new();

    let lines: Vec<&str> = diff_content.lines().collect();
    let mut i = 0;
    
    // 1. Parse File Headers (everything before the first @@)
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("@@") {
            break;
        }
        headers.push(line.to_string());
        i += 1;
    }

    // 2. Parse Hunks
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("@@") {
            let start = i;
            let header = line.to_string();
            let mut content = String::new();
            content.push_str(line);
            content.push('\n');
            
            i += 1;
            while i < lines.len() {
                let inner_line = lines[i];
                if inner_line.starts_with("@@") {
                    break; // Next hunk
                }
                content.push_str(inner_line);
                content.push('\n');
                i += 1;
            }
            
            hunks.push(Hunk {
                header,
                content,
                display_start: start,
                display_end: i - 1,
            });
        } else {
            i += 1;
        }
    }

    (headers, hunks)
}

pub fn apply_patch(headers: &[String], hunk: &Hunk, stage: bool) -> Result<()> {
    let mut patch_content = String::new();
    for header in headers {
        patch_content.push_str(header);
        patch_content.push('\n');
    }
    patch_content.push_str(&hunk.content);

    let mut cmd = Command::new("git");
    cmd.arg("apply");
    if stage {
        cmd.arg("--cached");
    }
    cmd.arg("-"); // Read from stdin

    let mut child = cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("Failed to spawn git apply")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(patch_content.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git apply failed: {}", err);
    }

    Ok(())
}
