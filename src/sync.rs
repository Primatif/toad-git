use crate::commit::is_dirty;
use crate::run_git;
use crate::submodule::check_submodule_status;
use std::path::Path;
use toad_core::{PreflightResult, ToadResult};

pub fn preflight_check(
    path: &Path,
    project_name: &str,
    parent_path: Option<&Path>,
    submodule_path: Option<&Path>,
) -> ToadResult<PreflightResult> {
    let mut issues = Vec::new();
    let is_clean = !is_dirty(path)?;

    if !is_clean {
        issues.push("Repository has uncommitted changes".to_string());
    }

    let unpushed_count = get_unpushed_count(path)?;
    if unpushed_count > 0 {
        issues.push(format!("{} unpushed commits", unpushed_count));
    }

    let mut is_aligned = true;
    if let (Some(parent), Some(sub)) = (parent_path, submodule_path) {
        if let Some(sub_path_str) = sub.to_str() {
            let (init, _vcs, expected, actual) = check_submodule_status(parent, sub_path_str)?;
            if !init {
                is_aligned = false;
                issues.push("Submodule is not initialized".to_string());
            } else if expected != actual {
                is_aligned = false;
                issues.push(format!(
                    "Submodule SHA mismatch (expected {}, got {})",
                    expected.unwrap_or_else(|| "none".to_string()),
                    actual.unwrap_or_else(|| "none".to_string())
                ));
            }
        }
    }

    Ok(PreflightResult {
        project_name: project_name.to_string(),
        is_clean,
        is_aligned,
        unpushed_count,
        issues,
    })
}

fn get_unpushed_count(path: &Path) -> ToadResult<usize> {
    // Check if upstream exists first
    let upstream_res = run_git(path, &["rev-parse", "--abbrev-ref", "@{u}"], "internal")?;
    if !upstream_res.success {
        return Ok(0);
    }

    let res = run_git(path, &["rev-list", "--count", "@{u}..HEAD"], "internal")?;
    if res.success {
        Ok(res.stdout.trim().parse().unwrap_or(0))
    } else {
        Ok(0)
    }
}
