use anyhow::Result;
use std::path::Path;
use toad_core::GitOpResult;
use crate::run_git;

/// Checks out a branch (creating it if it doesn't exist).
pub fn checkout(path: &Path, branch_name: &str, project_name: &str, create: bool) -> Result<GitOpResult> {
    if create {
        run_git(path, &["checkout", "-b", branch_name], project_name)
    } else {
        run_git(path, &["checkout", branch_name], project_name)
    }
}

/// Returns the name of the current branch.
pub fn current_branch(path: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(path)
        .output()?;
    
    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(name)
}
