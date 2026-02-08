use anyhow::Result;
use std::path::Path;
use std::process::Command;
use toad_core::GitOpResult;

/// Stages all changes and commits them in the given repository.
pub fn commit(path: &Path, message: &str, project_name: &str) -> Result<GitOpResult> {
    // 1. Git Add .
    let add_output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(path)
        .output()?;

    if !add_output.status.success() {
        return Ok(GitOpResult {
            project_name: project_name.to_string(),
            command: "git add .".to_string(),
            success: false,
            stdout: String::from_utf8_lossy(&add_output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&add_output.stderr).to_string(),
            exit_code: add_output.status.code().unwrap_or(1),
        });
    }

    // 2. Git Commit
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(path)
        .output()?;

    Ok(GitOpResult {
        project_name: project_name.to_string(),
        command: format!("git commit -m \"{}\"", message),
        success: commit_output.status.success(),
        stdout: String::from_utf8_lossy(&commit_output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&commit_output.stderr).to_string(),
        exit_code: commit_output.status.code().unwrap_or(0),
    })
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
