use anyhow::Result;
use std::path::Path;
use std::process::Command;

#[derive(Debug, PartialEq, Eq)]
pub enum GitStatus {
    Clean,
    Dirty,
    Untracked,
    NoRepo,
}

/// Checks the git status of a directory.
pub fn check_status(path: &Path) -> Result<GitStatus> {
    if !path.join(".git").exists() {
        return Ok(GitStatus::NoRepo);
    }

    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        return Ok(GitStatus::NoRepo);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.is_empty() {
        Ok(GitStatus::Clean)
    } else {
        // Check if there are only untracked files
        let mut only_untracked = true;
        for line in stdout.lines() {
            if !line.starts_with("??") {
                only_untracked = false;
                break;
            }
        }

        if only_untracked {
            Ok(GitStatus::Untracked)
        } else {
            Ok(GitStatus::Dirty)
        }
    }
}
