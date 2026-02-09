use crate::run_git;
use std::path::Path;
use toad_core::ToadResult;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GitStatus {
    Clean,
    Dirty,
    Untracked,
    NoRepo,
}

pub fn check_status(path: &Path) -> ToadResult<GitStatus> {
    if !path.join(".git").exists() {
        return Ok(GitStatus::NoRepo);
    }

    let res = run_git(path, &["status", "--porcelain"], "internal")?;
    if !res.success {
        return Ok(GitStatus::NoRepo);
    }

    if res.stdout.trim().is_empty() {
        Ok(GitStatus::Clean)
    } else if res.stdout.contains("??") {
        Ok(GitStatus::Untracked)
    } else {
        Ok(GitStatus::Dirty)
    }
}
