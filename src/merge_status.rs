use crate::run_git;
use std::path::Path;
use toad_core::ToadResult;

pub fn is_merged(path: &Path, a: &str, b: &str) -> ToadResult<bool> {
    let res = run_git(path, &["merge-base", "--is-ancestor", a, b], "internal")?;
    Ok(res.success)
}

/// Checks if there are merge conflicts (unmerged paths).
pub fn has_conflicts(path: &Path) -> ToadResult<bool> {
    let res = run_git(
        path,
        &["diff", "--name-only", "--diff-filter=U"],
        "internal",
    )?;
    Ok(res.success && !res.stdout.trim().is_empty())
}

/// Checks for divergence from upstream (unpushed or unpulled changes).
pub fn has_unmerged_changes(path: &Path) -> ToadResult<bool> {
    // Check if upstream exists
    let upstream_res = run_git(path, &["rev-parse", "--abbrev-ref", "@{u}"], "internal")?;
    if !upstream_res.success {
        return Ok(false);
    }

    // Check for ahead/behind
    let res = run_git(path, &["rev-list", "--left-right", "--count", "HEAD...@{u}"], "internal")?;
    if res.success {
        let stdout = res.stdout.trim();
        // Format is "ahead\tbehind"
        let parts: Vec<&str> = stdout.split_whitespace().collect();
        if parts.len() >= 2 {
            let ahead: usize = parts[0].parse().unwrap_or(0);
            let behind: usize = parts[1].parse().unwrap_or(0);
            return Ok(ahead > 0 || behind > 0);
        }
    }
    Ok(false)
}
