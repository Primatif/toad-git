# toad-git

Git status analysis and multi-repo VCS orchestration for the
[Primatif Toad](https://github.com/Primatif/Primatif_Toad) ecosystem.

## What It Does

`toad-git` provides **all git intelligence** for Toad. It analyzes repository
state, orchestrates multi-repo git operations, and manages submodule
relationships.

- **Status Analysis** — Detects dirty/clean state, uncommitted changes, and
  branch information for any git repository.
- **Branch Management** — `branch.rs` provides current branch detection, branch
  listing, and branch grouping across multiple repos.
- **Commit Operations** — `commit.rs` handles staging, committing, and log
  retrieval across repositories.
- **Remote Operations** — `remote.rs` manages push, pull, and fetch across
  multiple repos in parallel.
- **Submodule Discovery** — `submodule.rs` parses `.gitmodules`, analyzes
  submodule status, and detects alignment issues between parent and child repos.
- **Multi-Repo Sync** — `sync.rs` orchestrates `toad repo sync` which updates
  parent submodule references after child commits.
- **Merge Status** — `merge_status.rs` detects ahead/behind counts and PR status
  for branches.

## Role in the Ecosystem

`toad-git` is the VCS layer. It depends only on `toad-core` for data models. It
is consumed by `toad-discovery` (for VCS status during scanning) and the CLI
(`toad repo` commands for multi-repo git orchestration).

```text
toad-core ── toad-git ──┬── toad-discovery
                        └── bin/toad (toad repo commands)
```

## License

BUSL-1.1
