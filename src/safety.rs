// SPDX-License-Identifier: BUSL-1.1

pub fn is_destructive(command: &str) -> bool {
    let cmd = command.to_lowercase();
    let dangerous = [
        "rm ",
        "rf ",
        "delete ",
        "drop ",
        "truncate ",
        "wipe ",
        "format ",
    ];
    dangerous.iter().any(|d| cmd.contains(d))
}
