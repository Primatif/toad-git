pub mod align;
pub mod branch;
pub mod branches;
pub mod commit;
pub mod init;
pub mod merge_status;
pub mod remote;
pub mod status;
pub mod submodule;
pub mod sync;

use std::path::Path;
use std::process::Command;
use toad_core::{
    BranchGroup, GitOpResult, MultiRepoGitReport, MultiRepoStatusReport, ProjectDetail, ToadError,
    ToadResult,
};

pub fn generate_multi_repo_branch_report(
    projects: &[ProjectDetail],
    _all: bool,
) -> ToadResult<Vec<BranchGroup>> {
    let mut groups = std::collections::HashMap::new();

    for p in projects {
        let branch = branch::current_branch(&p.path)?;
        let group = groups.entry(branch.clone()).or_insert_with(|| BranchGroup {
            name: branch,
            projects: Vec::new(),
        });

        group.projects.push(toad_core::BranchPresence {
            project_name: p.name.clone(),
            exists_locally: true,
            exists_remotely: false, // Need logic to check remote
            pr_status: toad_core::PrStatus::None,
            pr_url: None,
        });
    }

    Ok(groups.into_values().collect())
}

pub fn execute_multi_repo_commit(
    projects: &[ProjectDetail],
    message: &str,
    _cascade: bool,
    fail_fast: bool,
) -> ToadResult<MultiRepoGitReport> {
    let mut results = Vec::new();
    for p in projects {
        let res = commit::commit(&p.path, message, &p.name)?;
        let success = res.success;
        results.push(res);
        if !success && fail_fast {
            break;
        }
    }
    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT COMMIT".to_string(),
        results,
    })
}

pub fn execute_multi_repo_push(
    projects: &[ProjectDetail],
    fail_fast: bool,
) -> ToadResult<MultiRepoGitReport> {
    let mut results = Vec::new();
    for p in projects {
        let res = remote::push(&p.path, &p.name)?;
        let success = res.success;
        results.push(res);
        if !success && fail_fast {
            break;
        }
    }
    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT PUSH".to_string(),
        results,
    })
}

pub fn execute_multi_repo_pull(
    projects: &[ProjectDetail],
    fail_fast: bool,
) -> ToadResult<MultiRepoGitReport> {
    let mut results = Vec::new();
    for p in projects {
        let res = remote::pull(&p.path, &p.name)?;
        let success = res.success;
        results.push(res);
        if !success && fail_fast {
            break;
        }
    }
    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT PULL".to_string(),
        results,
    })
}

pub fn execute_multi_repo_checkout(
    projects: &[ProjectDetail],
    branch: &str,
    create: bool,
    fail_fast: bool,
) -> ToadResult<MultiRepoGitReport> {
    let mut results = Vec::new();
    for p in projects {
        let res = branch::checkout(&p.path, branch, &p.name, create)?;
        let success = res.success;
        results.push(res);
        if !success && fail_fast {
            break;
        }
    }
    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT CHECKOUT".to_string(),
        results,
    })
}

pub fn execute_multi_repo_sync(
    projects: &[ProjectDetail],
    force: bool,
    _quiet: bool,
) -> ToadResult<MultiRepoGitReport> {
    let mut results = Vec::new();
    for p in projects {
        // Run preflight
        let preflight = sync::preflight_check(&p.path, &p.name, None, None)?;
        if !preflight.issues.is_empty() && !force {
            return Err(ToadError::Git(format!(
                "Preflight check failed for {}: {}",
                p.name,
                preflight.issues.join(", ")
            )));
        }

        let res = remote::pull(&p.path, &p.name)?;
        results.push(res);
    }
    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT SYNC".to_string(),
        results,
    })
}

pub fn execute_multi_repo_align(
    projects: &[ProjectDetail],
    _quiet: bool,
) -> ToadResult<MultiRepoGitReport> {
    let mut results = Vec::new();
    for p in projects {
        for sub in &p.submodules {
            let res = align::align_submodule(&p.path, &sub.path, &sub.name)?;
            results.push(res);
        }
    }
    Ok(MultiRepoGitReport {
        title: "SUBMODULE ALIGNMENT".to_string(),
        results,
    })
}

pub fn generate_multi_repo_status(projects: &[ProjectDetail]) -> ToadResult<MultiRepoStatusReport> {
    let mut items = Vec::new();
    for p in projects {
        let branch = branch::current_branch(&p.path)?;
        items.push(toad_core::MultiRepoStatusItem {
            name: p.name.clone(),
            status: p.vcs_status.clone(),
            branch,
        });
    }
    Ok(MultiRepoStatusReport { items })
}

pub fn run_git(path: &Path, args: &[&str], project_name: &str) -> ToadResult<GitOpResult> {
    let output = Command::new("git")
        .args(args)
        .current_dir(path)
        .output()
        .map_err(|e| ToadError::Git(format!("Failed to execute git: {}", e)))?;

    Ok(GitOpResult {
        project_name: project_name.to_string(),
        command: format!("git {}", args.join(" ")),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
    })
}

#[cfg(test)]
mod tests;
