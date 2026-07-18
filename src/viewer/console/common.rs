//! Shared parsing and response helpers for viewer command families.

use super::*;

pub(super) fn no_args(invocation: &ConsoleInvocation) -> Result<(), ConsoleError> {
    if invocation.args.is_empty() {
        Ok(())
    } else {
        Err(ConsoleError::new(
            "bad_arity",
            format!("{} does not accept arguments", invocation.command),
        ))
    }
}

pub(super) fn toggle_result(value: Value, label: &str, enabled: bool) -> ConsoleCommandResult {
    ConsoleCommandResult::new(
        value,
        vec![format!(
            "{label} {}.",
            if enabled { "enabled" } else { "disabled" }
        )],
    )
}
