use anyhow::Result;
use std::path::Path;
use toad_core::GitOpResult;
use crate::run_git;

/// Aligns a submodule to the SHA expected by the parent repository.
pub fn align_submodule(parent_path: &Path, submodule_rel_path: &Path, project_name: &str) -> Result<GitOpResult> {
    // git submodule update --init -- <path>
    run_git(
        parent_path,
        &["submodule", "update", "--init", "--", submodule_rel_path.to_str().unwrap()],
        project_name
    )
}

/// Resets a repository to its upstream state (DANGEROUS).
pub fn reset_to_upstream(path: &Path, project_name: &str) -> Result<GitOpResult> {
    run_git(path, &["reset", "--hard", "@{u}"], project_name)
}
