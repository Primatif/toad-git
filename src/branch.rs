use anyhow::Result;
use std::path::Path;
use std::process::Command;
use toad_core::GitOpResult;

/// Checks out a branch (creating it if it doesn't exist).
pub fn checkout(path: &Path, branch_name: &str, project_name: &str, create: bool) -> Result<GitOpResult> {
    let mut cmd = Command::new("git");
    cmd.arg("checkout");
    if create {
        cmd.arg("-b");
    }
    cmd.arg(branch_name);
    cmd.current_dir(path);

    let output = cmd.output()?;

    Ok(GitOpResult {
        project_name: project_name.to_string(),
        command: format!("git checkout {}", branch_name),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(0),
    })
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
