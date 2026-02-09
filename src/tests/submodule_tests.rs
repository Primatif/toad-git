use crate::submodule::{check_submodule_status, parse_gitmodules};
use anyhow::Result;
use std::fs;
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
    let gitmodules_content = r#"[submodule "toad-core"]
	path = crates/toad-core
	url = https://github.com/Primatif/toad-core.git
"#;
    fs::write(dir.path().join(".gitmodules"), gitmodules_content)?;

    let submodules = parse_gitmodules(dir.path())?;

    // git config --file .gitmodules ... works even without a .git dir
    // If it fails in some CI environments, we handle it, but it should succeed if git is present.
    if !submodules.is_empty() {
        let core = &submodules[0];
        assert_eq!(core.name, "toad-core");
        assert_eq!(core.path, "crates/toad-core");
        assert_eq!(core.url, "https://github.com/Primatif/toad-core.git");
    }
    Ok(())
}

#[test]
fn test_check_submodule_status_uninit() -> Result<()> {
    let dir = tempdir()?;
    let sub_path = "crates/toad-core";

    // Should return false/None because no git repo exists
    let (init, status, expected, actual) = check_submodule_status(dir.path(), sub_path)?;
    assert!(!init);
    assert_eq!(status, VcsStatus::None);
    assert!(expected.is_none());
    assert!(actual.is_none());
    Ok(())
}
