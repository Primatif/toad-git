use anyhow::Result;
use std::path::Path;
use std::process::Command;
use toad_core::BranchInfo;

/// Returns a list of all local branches in the repository.
pub fn list_local_branches(path: &Path) -> Result<Vec<BranchInfo>> {
    let output = Command::new("git")
        .args(["branch", "--format=%(refname:short)|%(upstream:short)"])
        .current_dir(path)
        .output()?;

    let current = crate::branch::current_branch(path).unwrap_or_default();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut branches = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let name = parts[0].to_string();
        let upstream = if parts.len() > 1 && !parts[1].is_empty() {
            Some(parts[1].to_string())
        } else {
            None
        };

        branches.push(BranchInfo {
            is_current: name == current,
            name,
            is_remote: false,
            upstream,
            ahead: 0,  // Calculated separately if needed
            behind: 0, // Calculated separately if needed
        });
    }

    Ok(branches)
}

/// Returns a list of all remote branches in the repository.
pub fn list_remote_branches(path: &Path) -> Result<Vec<BranchInfo>> {
    let output = Command::new("git")
        .args(["branch", "-r", "--format=%(refname:short)"])
        .current_dir(path)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut branches = Vec::new();

    for line in stdout.lines() {
        let name = line.trim().to_string();
        if name.is_empty() || name.contains("HEAD ->") {
            continue;
        }

        branches.push(BranchInfo {
            name,
            is_current: false,
            is_remote: true,
            upstream: None,
            ahead: 0,
            behind: 0,
        });
    }

    Ok(branches)
}
