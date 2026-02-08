use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// Checks if branch 'a' is merged into branch 'b'.
pub fn is_merged(path: &Path, a: &str, b: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["merge-base", "--is-ancestor", a, b])
        .current_dir(path)
        .output()?;

    Ok(output.status.success())
}

/// Checks if the current branch has unmerged changes relative to its upstream.
pub fn has_unmerged_changes(path: &Path) -> Result<bool> {
    let output = Command::new("git")
        .args(["log", "@{u}..HEAD"])
        .current_dir(path)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(!stdout.trim().is_empty())
}
