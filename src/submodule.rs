use crate::run_git;
use std::path::Path;
use toad_core::{ToadResult, VcsStatus};

#[derive(Debug, Clone)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: String,
    pub url: String,
}

pub fn parse_gitmodules(repo_path: &Path) -> ToadResult<Vec<SubmoduleInfo>> {
    let gitmodules_path = repo_path.join(".gitmodules");
    if !gitmodules_path.exists() {
        return Ok(Vec::new());
    }

    let res = run_git(
        repo_path,
        &["config", "--file", ".gitmodules", "--get-regexp", "path"],
        "internal",
    )?;
    if !res.success {
        return Ok(Vec::new());
    }

    let mut submodules = Vec::new();
    for line in res.stdout.lines() {
        // Line format: submodule.<name>.path <path>
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let key = parts[0];
            let path = parts[1];
            let name = key
                .strip_prefix("submodule.")
                .and_then(|s| s.strip_suffix(".path"))
                .unwrap_or(key);

            let url_res = run_git(
                repo_path,
                &[
                    "config",
                    "--file",
                    ".gitmodules",
                    &format!("submodule.{}.url", name),
                ],
                "internal",
            )?;
            let url = if url_res.success {
                url_res.stdout.trim().to_string()
            } else {
                String::new()
            };

            submodules.push(SubmoduleInfo {
                name: name.to_string(),
                path: path.to_string(),
                url,
            });
        }
    }

    Ok(submodules)
}

pub fn check_submodule_status(
    repo_path: &Path,
    sub_path: &str,
) -> ToadResult<(bool, VcsStatus, Option<String>, Option<String>)> {
    let res = run_git(repo_path, &["submodule", "status", sub_path], "internal")?;
    if !res.success {
        return Ok((false, VcsStatus::None, None, None));
    }

    let status_line = res.stdout.trim();
    if status_line.is_empty() {
        return Ok((false, VcsStatus::None, None, None));
    }

    // Status format: [-| |+]<sha> <path> (<version>)
    let initialized = !status_line.starts_with('-');
    let actual_sha = Some(status_line[1..41].to_string());

    let expected_sha = get_expected_sha(repo_path, Path::new(sub_path))?;

    let vcs_status = if status_line.starts_with('+') {
        VcsStatus::Dirty
    } else {
        VcsStatus::Clean
    };

    Ok((initialized, vcs_status, expected_sha, actual_sha))
}

fn get_expected_sha(repo_path: &Path, submodule_path: &Path) -> ToadResult<Option<String>> {
    let res = run_git(
        repo_path,
        &["ls-tree", "HEAD", submodule_path.to_str().unwrap()],
        "internal",
    )?;
    if res.success && !res.stdout.is_empty() {
        // Output format: 160000 commit <sha>    <path>
        let parts: Vec<&str> = res.stdout.split_whitespace().collect();
        if parts.len() >= 3 {
            return Ok(Some(parts[2].to_string()));
        }
    }
    Ok(None)
}
