use crate::run_git;
use std::path::Path;
use toad_core::{ToadError, ToadResult};

pub fn init_repo(path: &Path) -> ToadResult<()> {
    let res = run_git(path, &["init"], "internal")?;
    if !res.success {
        return Err(ToadError::Git(format!(
            "Failed to init repo at {:?}: {}",
            path, res.stderr
        )));
    }
    Ok(())
}
