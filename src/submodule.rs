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
    let (initialized, vcs_status, actual_sha) = match parse_submodule_status_line(status_line) {
        Some(v) => v,
        None => return Ok((false, VcsStatus::None, None, None)),
    };

    let expected_sha = get_expected_sha(repo_path, Path::new(sub_path))?;

    Ok((initialized, vcs_status, expected_sha, actual_sha))
}

// Status format: [-| |+]<sha> <path> (<version>)
// Some git versions print a leading status prefix, others may not.
fn parse_submodule_status_line(status_line: &str) -> Option<(bool, VcsStatus, Option<String>)> {
    let status_line = status_line.trim();
    if status_line.is_empty() {
        return None;
    }

    let first = status_line.split_whitespace().next().unwrap_or("");
    if first.is_empty() {
        return None;
    }

    let (prefix, sha_str) = match first.chars().next() {
        Some('+') | Some('-') | Some(' ') => (&first[0..1], &first[1..]),
        _ => ("", first),
    };

    let initialized = prefix != "-";
    let actual_sha = if sha_str.is_empty() {
        None
    } else {
        Some(sha_str.trim().to_string())
    };

    let vcs_status = if prefix == "+" {
        VcsStatus::Dirty
    } else if initialized {
        VcsStatus::Clean
    } else {
        VcsStatus::None
    };

    Some((initialized, vcs_status, actual_sha))
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

#[cfg(test)]
mod tests {
    use super::parse_submodule_status_line;
    use toad_core::VcsStatus;

    #[test]
    fn parse_submodule_status_line_prefixed_dirty() {
        let sha = "6278232be17af5b63892b287475662fc31c4c0a4";
        let line = format!("+{} bin/toad-mcp (heads/dev)", sha);
        let (initialized, status, actual) = parse_submodule_status_line(&line).unwrap();
        assert!(initialized);
        assert_eq!(status, VcsStatus::Dirty);
        assert_eq!(actual.as_deref(), Some(sha));
    }

    #[test]
    fn parse_submodule_status_line_prefixed_uninitialized() {
        let sha = "6278232be17af5b63892b287475662fc31c4c0a4";
        let line = format!("-{} bin/toad-mcp (heads/dev)", sha);
        let (initialized, status, actual) = parse_submodule_status_line(&line).unwrap();
        assert!(!initialized);
        assert_eq!(status, VcsStatus::None);
        assert_eq!(actual.as_deref(), Some(sha));
    }

    #[test]
    fn parse_submodule_status_line_unprefixed_clean() {
        let sha = "6278232be17af5b63892b287475662fc31c4c0a4";
        let line = format!("{} bin/toad-mcp (heads/dev)", sha);
        let (initialized, status, actual) = parse_submodule_status_line(&line).unwrap();
        assert!(initialized);
        assert_eq!(status, VcsStatus::Clean);
        assert_eq!(actual.as_deref(), Some(sha));
    }
}
