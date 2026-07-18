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

/// Parses a raw FormID argument: 1..=8 hex digits, with or without a `0x`
/// prefix -- the Bethesda console accepts bare short hex like
/// `player.additem f 100`. Deliberately looser than `console::executor`'s
/// private reference-selector parser (which requires a prefix or exactly 8
/// digits to disambiguate from EditorIDs): callers here target a catalog
/// item FormID (`additem`, `equipitem`, ...) or a prepared-cell FormID
/// (`tp`'s optional destination argument) rather than a live reference, so
/// there is no EditorID form to collide with and no `RefRegistry`
/// resolution.
pub(super) fn parse_item_form_id(value: &str) -> Option<u32> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    ((1..=8).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(digits, 16).ok())
        .flatten()
}
