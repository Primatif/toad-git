use crate::commit::is_dirty;
use std::path::Path;
use toad_core::{PreflightResult, ToadResult};

pub fn preflight_check(
    path: &Path,
    project_name: &str,
    _parent_path: Option<&Path>,
    _submodule_path: Option<&Path>,
) -> ToadResult<PreflightResult> {
    let mut issues = Vec::new();
    let is_clean = !is_dirty(path)?;

    if !is_clean {
        issues.push("Repository has uncommitted changes".to_string());
    }

    Ok(PreflightResult {
        project_name: project_name.to_string(),
        is_clean,
        is_aligned: true,  // Need logic for alignment
        unpushed_count: 0, // Need logic for unpushed
        issues,
    })
}
