use crate::run_git;
use std::path::Path;
use toad_core::{GitOpResult, ToadResult};

pub fn checkout(
    path: &Path,
    branch: &str,
    project_name: &str,
    create: bool,
) -> ToadResult<GitOpResult> {
    let mut args = vec!["checkout"];
    if create {
        args.push("-b");
    }
    args.push(branch);

    run_git(path, &args, project_name)
}

pub fn current_branch(path: &Path) -> ToadResult<String> {
    let res = run_git(path, &["rev-parse", "--abbrev-ref", "HEAD"], "internal")?;
    if res.success {
        Ok(res.stdout.trim().to_string())
    } else {
        Ok("HEAD".to_string())
    }
}
