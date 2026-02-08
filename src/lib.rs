pub mod align;
pub mod branch;
pub mod branches;
pub mod commit;
pub mod init;
pub mod merge_status;
pub mod remote;
pub mod safety;
pub mod status;
pub mod submodule;
pub mod sync;

use anyhow::Result;
use std::path::Path;
use std::process::Command;
use toad_core::GitOpResult;

/// A central helper to execute Git commands across the ecosystem.
pub fn run_git(path: &Path, args: &[&str], project_name: &str) -> Result<GitOpResult> {
    let output = Command::new("git").args(args).current_dir(path).output()?;

    Ok(GitOpResult {
        project_name: project_name.to_string(),
        command: format!("git {}", args.join(" ")),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(0),
    })
}

#[cfg(test)]
mod tests;
