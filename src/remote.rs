use crate::run_git;
use std::path::Path;
use toad_core::{GitOpResult, ToadResult};

pub fn push(path: &Path, project_name: &str) -> ToadResult<GitOpResult> {
    run_git(path, &["push"], project_name)
}

pub fn pull(path: &Path, project_name: &str) -> ToadResult<GitOpResult> {
    run_git(path, &["pull"], project_name)
}
