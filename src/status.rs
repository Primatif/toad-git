use std::path::Path;
use std::process::Command;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum GitStatus {
    Clean,
    Dirty,
    Untracked,
    NoRepo,
}

/// Checks the git status of a directory.
pub fn check_status(path: &Path) -> Result<GitStatus> {
    if !path.join(".git").exists() {
        return Ok(GitStatus::NoRepo);
    }

    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        return Ok(GitStatus::NoRepo);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.is_empty() {
        Ok(GitStatus::Clean)
    } else {
        // Check if there are only untracked files
        let mut only_untracked = true;
        for line in stdout.lines() {
            if !line.starts_with("??") {
                only_untracked = false;
                break;
            }
        }
        
        if only_untracked {
            Ok(GitStatus::Untracked)
        } else {
            Ok(GitStatus::Dirty)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::process::Command;

    #[test]
    fn test_check_status_no_repo() -> Result<()> {
        let dir = tempdir()?;
        let status = check_status(dir.path())?;
        assert_eq!(status, GitStatus::NoRepo);
        Ok(())
    }

    #[test]
    fn test_check_status_clean() -> Result<()> {
        let dir = tempdir()?;
        Command::new("git").arg("init").current_dir(dir.path()).output()?;
        let status = check_status(dir.path())?;
        assert_eq!(status, GitStatus::Clean);
        Ok(())
    }

    #[test]
    fn test_check_status_untracked() -> Result<()> {
        let dir = tempdir()?;
        Command::new("git").arg("init").current_dir(dir.path()).output()?;
        std::fs::write(dir.path().join("file.txt"), "hello")?;
        let status = check_status(dir.path())?;
        assert_eq!(status, GitStatus::Untracked);
        Ok(())
    }

    #[test]
    fn test_check_status_dirty() -> Result<()> {
        let dir = tempdir()?;
        Command::new("git").arg("init").current_dir(dir.path()).output()?;
        std::fs::write(dir.path().join("file.txt"), "hello")?;
        // Stage it
        Command::new("git").arg("add").arg("file.txt").current_dir(dir.path()).output()?;
        let status = check_status(dir.path())?;
        assert_eq!(status, GitStatus::Dirty);
        Ok(())
    }
}
