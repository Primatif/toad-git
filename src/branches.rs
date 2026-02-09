use crate::run_git;
use std::path::Path;
use toad_core::{BranchInfo, ToadResult};

pub fn list_local_branches(path: &Path) -> ToadResult<Vec<BranchInfo>> {
    let res = run_git(
        path,
        &["branch", "--format=%(refname:short)|%(HEAD)"],
        "internal",
    )?;
    let mut branches = Vec::new();

    for line in res.stdout.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            branches.push(BranchInfo {
                name: parts[0].to_string(),
                is_current: parts[1] == "*",
                is_remote: false,
                upstream: None,
                ahead: 0,
                behind: 0,
            });
        }
    }

    Ok(branches)
}

pub fn list_remote_branches(path: &Path) -> ToadResult<Vec<BranchInfo>> {
    let res = run_git(
        path,
        &["branch", "-r", "--format=%(refname:short)"],
        "internal",
    )?;
    let mut branches = Vec::new();

    for line in res.stdout.lines() {
        branches.push(BranchInfo {
            name: line.trim().to_string(),
            is_current: false,
            is_remote: true,
            upstream: None,
            ahead: 0,
            behind: 0,
        });
    }

    Ok(branches)
}
