use crate::run_git;
use anyhow::Result;
use std::path::Path;
use toad_core::GitOpResult;

/// Pushes changes to the remote.
pub fn push(
    path: &Path,
    project_name: &str,
    upstream: Option<&str>,
    branch: Option<&str>,
) -> Result<GitOpResult> {
    if let (Some(u), Some(b)) = (upstream, branch) {
        run_git(path, &["push", "-u", u, b], project_name)
    } else {
        run_git(path, &["push"], project_name)
    }
}

/// Pulls changes from the remote.
pub fn pull(path: &Path, project_name: &str) -> Result<GitOpResult> {
    run_git(path, &["pull"], project_name)
}
