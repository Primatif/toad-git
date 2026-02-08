use anyhow::{bail, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use toad_core::VcsStatus;

#[derive(Debug, Clone)]
pub struct SubmoduleInfo {
    pub name: String,
    pub path: PathBuf,
    pub url: String,
}

/// Parses the .gitmodules file in the given repository path.
pub fn parse_gitmodules(repo_path: &Path) -> Result<Vec<SubmoduleInfo>> {
    let gitmodules_path = repo_path.join(".gitmodules");
    if !gitmodules_path.exists() {
        return Ok(Vec::new());
    }

    // We use `git config -f .gitmodules --list` to parse the file reliably
    let output = Command::new("git")
        .args([
            "config",
            "-f",
            ".gitmodules",
            "--get-regexp",
            r"^submodule\..*\.(path|url)$",
        ])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        if output.status.code() == Some(1) {
            // git config returns 1 if no matches are found, which is fine
            return Ok(Vec::new());
        }
        bail!(
            "Failed to parse .gitmodules: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut submodules = std::collections::HashMap::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0];
        let value = parts[1];

        // Key format: submodule.<name>.<property>
        let key_parts: Vec<&str> = key.split('.').collect();
        if key_parts.len() < 3 {
            continue;
        }

        let name = key_parts[1].to_string();
        let prop = key_parts[2];

        let entry = submodules
            .entry(name.clone())
            .or_insert_with(|| (String::new(), String::new()));
        if prop == "path" {
            entry.0 = value.to_string();
        } else if prop == "url" {
            entry.1 = value.to_string();
        }
    }

    let result = submodules
        .into_iter()
        .map(|(name, (path, url))| SubmoduleInfo {
            name,
            path: PathBuf::from(path),
            url,
        })
        .collect();

    Ok(result)
}

/// Checks the status of a submodule.
pub fn check_submodule_status(
    repo_path: &Path,
    submodule_path: &Path,
) -> Result<(bool, VcsStatus, Option<String>, Option<String>)> {
    let abs_path = repo_path.join(submodule_path);

    // 1. Check if initialized
    let initialized = abs_path.join(".git").exists();

    // 2. Get expected commit (from parent index)
    let expected_commit = get_expected_sha(repo_path, submodule_path)?;

    if !initialized {
        return Ok((false, VcsStatus::None, expected_commit, None));
    }

    // 3. Get actual commit (from submodule HEAD)
    let actual_commit = get_actual_sha(&abs_path)?;

    // 4. Get VCS status
    let vcs_status = match crate::status::check_status(&abs_path)? {
        crate::status::GitStatus::Clean => VcsStatus::Clean,
        crate::status::GitStatus::Dirty => VcsStatus::Dirty,
        crate::status::GitStatus::Untracked => VcsStatus::Untracked,
        crate::status::GitStatus::NoRepo => VcsStatus::None,
    };

    Ok((true, vcs_status, expected_commit, actual_commit))
}

fn get_expected_sha(repo_path: &Path, submodule_path: &Path) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["ls-tree", "HEAD", submodule_path.to_str().unwrap()])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Format: 160000 commit <sha>    <path>
    let parts: Vec<&str> = stdout.split_whitespace().collect();
    if parts.len() >= 3 {
        Ok(Some(parts[2].to_string()))
    } else {
        Ok(None)
    }
}

fn get_actual_sha(path: &Path) -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if sha.is_empty() {
        Ok(None)
    } else {
        Ok(Some(sha))
    }
}
