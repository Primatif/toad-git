use crate::run_git;
use std::path::Path;
use toad_core::ToadResult;

pub fn is_merged(path: &Path, a: &str, b: &str) -> ToadResult<bool> {
    let res = run_git(path, &["merge-base", "--is-ancestor", a, b], "internal")?;
    Ok(res.success)
}

pub fn has_unmerged_changes(path: &Path) -> ToadResult<bool> {
    let res = run_git(
        path,
        &["diff", "--name-only", "--diff-filter=U"],
        "internal",
    )?;
    Ok(res.success && !res.stdout.trim().is_empty())
}
