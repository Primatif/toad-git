// SPDX-License-Identifier: BUSL-1.1
use anyhow::{bail, Result};
use std::path::Path;
use std::process::Command;

pub fn init_repo(path: &Path) -> Result<()> {
    let output = Command::new("git").arg("init").current_dir(path).output()?;

    if !output.status.success() {
        bail!(
            "Failed to initialize git repository: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}
