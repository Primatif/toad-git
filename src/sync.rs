use anyhow::Result;
use std::path::Path;
use std::process::Command;
use toad_core::PreflightResult;

/// Performs a safety check on a repository before synchronization.
pub fn preflight_check(
    path: &Path,
    project_name: &str,
    parent_path: Option<&Path>,
    submodule_rel_path: Option<&Path>,
) -> Result<PreflightResult> {
    let mut issues = Vec::new();
    let mut is_aligned = true;

    // 1. Check for dirty state
    let status_output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(path)
        .output()?;
    let is_clean = String::from_utf8_lossy(&status_output.stdout)
        .trim()
        .is_empty();
    if !is_clean {
        issues.push("Repository has uncommitted changes (dirty)".to_string());
    }

    // 2. Check for unpushed commits (Ghost Commit Prevention)
    let log_output = Command::new("git")
        .arg("log")
        .arg("@{u}..")
        .current_dir(path)
        .output()?;

    // If command fails, it might mean no upstream branch
    let unpushed_count = if log_output.status.success() {
        String::from_utf8_lossy(&log_output.stdout).lines().count()
    } else {
        0
    };

    if unpushed_count > 0 {
        issues.push(format!(
            "Repository has {} unpushed commits",
            unpushed_count
        ));
    }

    // 3. Check for SHA alignment (if it's a submodule)
    if let (Some(parent), Some(rel_path)) = (parent_path, submodule_rel_path) {
        if let Ok((init, _, expected, actual)) =
            crate::submodule::check_submodule_status(parent, rel_path)
        {
            if init && expected != actual {
                is_aligned = false;
                issues.push(format!(
                    "Submodule is drifted (Expected: {}, Actual: {})",
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
