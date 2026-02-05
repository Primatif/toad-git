use super::status::*;
use anyhow::Result;
use std::process::Command;
use tempfile::tempdir;

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
    Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output()?;
    let status = check_status(dir.path())?;
    assert_eq!(status, GitStatus::Clean);
    Ok(())
}

#[test]
fn test_check_status_untracked() -> Result<()> {
    let dir = tempdir()?;
    Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output()?;
    std::fs::write(dir.path().join("file.txt"), "hello")?;
    let status = check_status(dir.path())?;
    assert_eq!(status, GitStatus::Untracked);
    Ok(())
}

#[test]
fn test_check_status_dirty() -> Result<()> {
    let dir = tempdir()?;
    Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output()?;
    std::fs::write(dir.path().join("file.txt"), "hello")?;
    // Stage it
    Command::new("git")
        .arg("add")
        .arg("file.txt")
        .current_dir(dir.path())
        .output()?;
    let status = check_status(dir.path())?;
    assert_eq!(status, GitStatus::Dirty);
    Ok(())
}
