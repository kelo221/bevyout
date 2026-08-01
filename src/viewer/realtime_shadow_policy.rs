//! Pure write-gating policy for the realtime point-shadow selection system
//! (`viewer::lighting::apply_realtime_shadow_light`).
//!
//! Kept free of Bevy imports so the executable-specification suite in
//! `tests/features.rs` can include this file verbatim (the same pattern as
//! `hybrid_shadow_policy.rs`).

/// True while the disabled realtime-shadow path still owes writes.
///
/// Cleanup is required on a settings change (one conditional pass clears any
/// shadow maps an earlier enabled pass switched on) or while a previous
/// selection is still recorded. When neither holds, the state is already
/// clean: candidate lights always spawn with `shadow_maps_enabled: false`
/// and the shadow system is the only writer afterwards, so skipping
/// performs zero writes without leaving stale state behind.
pub(crate) fn disabled_shadow_writes_needed(
    settings_changed: bool,
    selection_active: bool,
) -> bool {
    settings_changed || selection_active
}

#[cfg(test)]
#[path = "tests/realtime_shadow_policy.rs"]
mod tests;
