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

    let stdout = res.stdout.trim();
    if stdout.is_empty() {
        return Ok(GitStatus::Clean);
    }

    let lines: Vec<&str> = stdout.lines().collect();
    let all_untracked = lines.iter().all(|line| line.starts_with("??"));

    if all_untracked {
        Ok(GitStatus::Untracked)
    } else {
        Ok(GitStatus::Dirty)
    }
}
