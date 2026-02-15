use crate::run_git;
use std::path::Path;
use toad_core::{GitOpResult, ToadResult};

pub fn commit(path: &Path, message: &str, project_name: &str) -> ToadResult<GitOpResult> {
    // 1. Add all changes
    run_git(path, &["add", "."], project_name)?;

    // 2. Commit
    let mut res = run_git(path, &["commit", "-m", message], project_name)?;

    // Treat exit code 1 (nothing to commit) as success
    if res.exit_code == 1
        && (res.stdout.contains("nothing to commit") || res.stderr.contains("nothing to commit"))
    {
        res.success = true;
    }

    Ok(res)
}

pub fn is_dirty(path: &Path) -> ToadResult<bool> {
    let res = run_git(path, &["status", "--porcelain"], "internal")?;
    Ok(!res.stdout.trim().is_empty())
}
