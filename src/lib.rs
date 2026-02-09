use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;
use std::process::Command;
use toad_core::{
    BranchGroup, BranchPresence, GitOpResult, MultiRepoGitReport, MultiRepoStatusItem,
    MultiRepoStatusReport, ProjectDetail,
};

pub mod align;
pub mod branch;
pub mod branches;
pub mod commit;
pub mod init;
pub mod merge_status;
pub mod remote;
pub mod safety;
pub mod status;
pub mod submodule;
pub mod sync;

/// Generates a consolidated branch report for multiple projects.
pub fn generate_multi_repo_branch_report(
    projects: &[ProjectDetail],
    _show_remote: bool,
) -> Result<Vec<BranchGroup>> {
    let mut groups = Vec::new();

    // 1. Collect all branch names across the ecosystem
    let mut branch_names = HashSet::new();
    for p in projects {
        let local = branches::list_local_branches(&p.path)?;
        for b in local {
            branch_names.insert(b.name);
        }
    }

    // 2. For each branch name, check presence in every project
    let mut sorted_branches: Vec<_> = branch_names.into_iter().collect();
    sorted_branches.sort();

    for name in sorted_branches {
        let mut presence_list = Vec::new();
        for p in projects {
            let local = branches::list_local_branches(&p.path)?;
            let remote = branches::list_remote_branches(&p.path)?;

            let exists_locally = local.iter().any(|b| b.name == name);
            let exists_remotely = remote.iter().any(|b| b.name == name);

            if exists_locally || exists_remotely {
                presence_list.push(BranchPresence {
                    project_name: p.name.clone(),
                    exists_locally,
                    exists_remotely,
                    pr_status: toad_core::PrStatus::None,
                    pr_url: None,
                });
            }
        }

        if !presence_list.is_empty() {
            groups.push(BranchGroup {
                name,
                projects: presence_list,
            });
        }
    }

    Ok(groups)
}

/// Executes a commit across multiple repositories.
pub fn execute_multi_repo_commit(
    projects: &[ProjectDetail],
    message: &str,
    _cascade: bool,
    fail_fast: bool,
) -> Result<MultiRepoGitReport> {
    let mut results = Vec::new();
    let mut submodule_failed = false;

    for p in projects {
        let mut project_sub_failed = false;
        // 1. Commit submodules first
        for sub in &p.submodules {
            let sub_path = p.path.join(&sub.path);
            if commit::is_dirty(&sub_path)? {
                let res = commit::commit(&sub_path, message, &sub.name)?;
                if !res.success {
                    submodule_failed = true;
                    project_sub_failed = true;
                }
                results.push(res);

                if fail_fast && submodule_failed {
                    return Ok(MultiRepoGitReport {
                        title: "MULTI-REPO GIT COMMIT".to_string(),
                        results,
                    });
                }
            }
        }

        if fail_fast && submodule_failed {
            break;
        }

        // 2. Commit the project itself
        if !project_sub_failed && commit::is_dirty(&p.path)? {
            let res = commit::commit(&p.path, message, &p.name)?;
            let success = res.success;
            results.push(res);
            if !success && fail_fast {
                break;
            }
        }
    }

    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT COMMIT".to_string(),
        results,
    })
}

/// Executes a push across multiple repositories.
pub fn execute_multi_repo_push(
    projects: &[ProjectDetail],
    fail_fast: bool,
) -> Result<MultiRepoGitReport> {
    let mut results = Vec::new();

    for p in projects {
        // 1. Push submodules first
        for sub in &p.submodules {
            let sub_path = p.path.join(&sub.path);
            let res = remote::push(&sub_path, &sub.name, None, None)?;
            let success = res.success;
            results.push(res);
            if !success && fail_fast {
                return Ok(MultiRepoGitReport {
                    title: "MULTI-REPO GIT PUSH".to_string(),
                    results,
                });
            }
        }

        // 2. Push project
        let res = remote::push(&p.path, &p.name, None, None)?;
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

/// Executes a pull across multiple repositories.
pub fn execute_multi_repo_pull(
    projects: &[ProjectDetail],
    fail_fast: bool,
) -> Result<MultiRepoGitReport> {
    let mut results = Vec::new();

    for p in projects {
        // 1. Pull project
        let res = remote::pull(&p.path, &p.name)?;
        let success = res.success;
        results.push(res);
        if !success && fail_fast {
            break;
        }

        // 2. Pull submodules
        for sub in &p.submodules {
            let sub_path = p.path.join(&sub.path);
            let res = remote::pull(&sub_path, &sub.name)?;
            let success = res.success;
            results.push(res);
            if !success && fail_fast {
                return Ok(MultiRepoGitReport {
                    title: "MULTI-REPO GIT PULL".to_string(),
                    results,
                });
            }
        }
    }

    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT PULL".to_string(),
        results,
    })
}

/// Executes a checkout across multiple repositories.
pub fn execute_multi_repo_checkout(
    projects: &[ProjectDetail],
    branch: &str,
    create: bool,
    fail_fast: bool,
) -> Result<MultiRepoGitReport> {
    let mut results = Vec::new();

    for p in projects {
        // 1. Checkout project
        let res = branch::checkout(&p.path, branch, &p.name, create)?;
        let success = res.success;
        results.push(res);
        if !success && fail_fast {
            return Ok(MultiRepoGitReport {
                title: "MULTI-REPO GIT CHECKOUT".to_string(),
                results,
            });
        }

        // 2. Checkout submodules
        for sub in &p.submodules {
            let sub_path = p.path.join(&sub.path);
            let sub_res = branch::checkout(&sub_path, branch, &sub.name, create)?;
            let sub_success = sub_res.success;
            results.push(sub_res);
            if !sub_success && fail_fast {
                return Ok(MultiRepoGitReport {
                    title: "MULTI-REPO GIT CHECKOUT".to_string(),
                    results,
                });
            }
        }
    }

    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT CHECKOUT".to_string(),
        results,
    })
}

/// Executes a synchronization and alignment across multiple repositories.
pub fn execute_multi_repo_sync(
    projects: &[ProjectDetail],
    force: bool,
    fail_fast: bool,
) -> Result<MultiRepoGitReport> {
    let mut results = Vec::new();

    for p in projects {
        // 1. Preflight safety checks
        if !force {
            let preflight = sync::preflight_check(&p.path, &p.name, None, None)?;
            if !preflight.is_clean || !preflight.is_aligned || preflight.unpushed_count > 0 {
                let mut stderr = Vec::new();
                for issue in preflight.issues {
                    stderr.push(issue);
                }
                results.push(GitOpResult {
                    project_name: p.name.clone(),
                    command: "sync (preflight)".to_string(),
                    success: false,
                    stdout: String::new(),
                    stderr: stderr.join("\n"),
                    exit_code: 1,
                });
                if fail_fast {
                    break;
                }
                continue;
            }
        }

        // 2. Pull project
        let pull_res = remote::pull(&p.path, &p.name)?;
        let pull_success = pull_res.success;
        results.push(pull_res);
        if !pull_success && fail_fast {
            break;
        }

        // 3. Align submodules
        if !p.submodules.is_empty() {
            for sub in &p.submodules {
                let res = align::align_submodule(&p.path, &sub.path, &sub.name)?;
                let success = res.success;
                results.push(res);
                if !success && fail_fast {
                    return Ok(MultiRepoGitReport {
                        title: "MULTI-REPO GIT SYNC".to_string(),
                        results,
                    });
                }
            }
        }
    }

    Ok(MultiRepoGitReport {
        title: "MULTI-REPO GIT SYNC".to_string(),
        results,
    })
}

/// Executes an alignment of submodules across multiple repositories.
pub fn execute_multi_repo_align(
    projects: &[ProjectDetail],
    fail_fast: bool,
) -> Result<MultiRepoGitReport> {
    let mut results = Vec::new();

    for p in projects {
        if p.submodules.is_empty() {
            continue;
        }

        for sub in &p.submodules {
            let res = align::align_submodule(&p.path, &sub.path, &sub.name)?;
            let success = res.success;
            results.push(res);
            if !success && fail_fast {
                return Ok(MultiRepoGitReport {
                    title: "SUBMODULE ALIGNMENT".to_string(),
                    results,
                });
            }
        }
    }

    Ok(MultiRepoGitReport {
        title: "SUBMODULE ALIGNMENT".to_string(),
        results,
    })
}

/// Generates a consolidated Git status report for multiple projects.
pub fn generate_multi_repo_status(projects: &[ProjectDetail]) -> Result<MultiRepoStatusReport> {
    let mut items = Vec::new();

    for p in projects {
        // 1. Project Status
        let branch = branch::current_branch(&p.path).unwrap_or_else(|_| "unknown".to_string());
        items.push(MultiRepoStatusItem {
            name: p.name.clone(),
            status: p.vcs_status.clone(),
            branch,
        });

        // 2. Submodule Status
        for sub in &p.submodules {
            let sub_path = p.path.join(&sub.path);
            let sub_branch = branch::current_branch(&sub_path).unwrap_or_else(|_| "unknown".to_string());
            items.push(MultiRepoStatusItem {
                name: format!("{} > {}", p.name, sub.name),
                status: sub.vcs_status.clone(),
                branch: sub_branch,
            });
        }
    }

    Ok(MultiRepoStatusReport { items })
}

/// A central helper to execute Git commands across the ecosystem.
pub fn run_git(path: &Path, args: &[&str], project_name: &str) -> Result<GitOpResult> {
    let output = Command::new("git").args(args).current_dir(path).output()?;

    Ok(GitOpResult {
        project_name: project_name.to_string(),
        command: format!("git {}", args.join(" ")),
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(0),
    })
}

#[cfg(test)]
mod tests;
