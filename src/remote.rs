use anyhow::Result;
use std::path::Path;
use std::process::Command;
use toad_core::GitOpResult;

/// Pushes changes to the remote.
pub fn push(path: &Path, project_name: &str, upstream: Option<&str>, branch: Option<&str>) -> Result<GitOpResult> {
    let mut cmd = Command::new("git");
    cmd.arg("push");
    
    if let (Some(u), Some(b)) = (upstream, branch) {
        cmd.arg("-u").arg(u).arg(b);
    }
    
    cmd.current_dir(path);
    let output = cmd.output()?;

    Ok(GitOpResult {
        project_name: project_name.to_string(),
        command: "git push".to_string(),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(0),
    })
}

/// Pulls changes from the remote.
pub fn pull(path: &Path, project_name: &str) -> Result<GitOpResult> {
    let output = Command::new("git")
        .arg("pull")
        .current_dir(path)
        .output()?;

    Ok(GitOpResult {
        project_name: project_name.to_string(),
        command: "git pull".to_string(),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(0),
    })
}
