use crate::run_git;
use std::path::Path;
use toad_core::{GitOpResult, ToadResult};

pub fn align_submodule(
    _root_path: &Path,
    sub_path: &Path,
    project_name: &str,
) -> ToadResult<GitOpResult> {
    // Basic implementation: just reset to whatever the index thinks it should be
    // or do a simple fetch/checkout if we have more context.
    // For now, let's just do a pull.
    run_git(sub_path, &["pull"], project_name)
}

pub fn reset_to_upstream(path: &Path, project_name: &str) -> ToadResult<GitOpResult> {
    run_git(path, &["reset", "--hard", "origin/HEAD"], project_name)
}
