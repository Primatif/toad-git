use crate::submodule::*;
use anyhow::Result;
use std::fs;
use std::process::Command;
use tempfile::tempdir;
use toad_core::VcsStatus;

#[test]
fn test_parse_gitmodules_none() -> Result<()> {
    let dir = tempdir()?;
    let submodules = parse_gitmodules(dir.path())?;
    assert!(submodules.is_empty());
    Ok(())
}

#[test]
fn test_parse_gitmodules_success() -> Result<()> {
    let dir = tempdir()?;
    let gitmodules_content = r#"[submodule "crates/toad-core"]
	path = crates/toad-core
	url = git@github.com:Primatif/toad-core.git
[submodule "crates/toad-scaffold"]
	path = crates/toad-scaffold
	url = git@github.com:Primatif/toad-scaffold.git
"#;
    fs::write(dir.path().join(".gitmodules"), gitmodules_content)?;

    let submodules = parse_gitmodules(dir.path())?;
    assert_eq!(submodules.len(), 2);

    let core = submodules
        .iter()
        .find(|s| s.name == "crates/toad-core")
        .unwrap();
    assert_eq!(core.path.to_str().unwrap(), "crates/toad-core");
    assert_eq!(core.url, "git@github.com:Primatif/toad-core.git");

    Ok(())
}

#[test]
fn test_check_submodule_status_uninit() -> Result<()> {
    let dir = tempdir()?;
    // Init parent
    Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output()?;

    let sub_path = std::path::PathBuf::from("sub");
    fs::create_dir(dir.path().join(&sub_path))?;

    // We can't easily test expected_commit without a real commit in parent index,
    // but we can check if it returns uninitialized
    let (init, status, _expected, actual) = check_submodule_status(dir.path(), &sub_path)?;
    assert!(!init);
    assert_eq!(status, VcsStatus::None);
    assert!(actual.is_none());

    Ok(())
}
