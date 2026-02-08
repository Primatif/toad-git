use crate::run_git;
use anyhow::Result;
use std::path::Path;
use std::process::Command;
use toad_core::GitOpResult;

/// Stages all changes and commits them in the given repository.
pub fn commit(path: &Path, message: &str, project_name: &str) -> Result<GitOpResult> {
    // 1. Git Add .
    let add_res = run_git(path, &["add", "."], project_name)?;
    if !add_res.success {
        return Ok(add_res);
    }

    // 2. Git Commit
    run_git(path, &["commit", "-m", message], project_name)
}

/// Checks if there are any changes to commit.
pub fn is_dirty(path: &Path) -> Result<bool> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(path)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(!stdout.trim().is_empty())
}
