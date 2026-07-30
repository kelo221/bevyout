//! Pure, std-only door/activator animation policy (issue #57): clip
//! selection for open/close transitions, the deferred-travel "open lead"
//! that lets a door's `Open` clip play before an instant cell swap fires,
//! and the mid-animation reversal seek decision.
//!
//! Kept dependency-free (mirrors `world::policy`'s own doc comment) so
//! `tests/features.rs` can include it verbatim via `#[path]` without
//! compiling any Bevy code.

/// Which transition a placement's open-state change or activation requests.
/// Doors and containers pick `Closing` on their second activation; an
/// activator (no open/close concept of its own) always requests `Opening`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClipTransition {
    Opening,
    Closing,
}

/// F57.3's cap: a travel door's `Open` clip may run for up to this long
/// before `DoorTravelRequested` fires regardless of clip length, so the
/// swap still feels responsive on long door animations.
pub(crate) const OPEN_LEAD_CAP_SECONDS: f32 = 0.6;

/// T57.1: picks a clip name for `transition` out of `clip_names` (arbitrary
/// order -- glTF's `named_animations` is a `HashMap`, so `clip_names` may
/// arrive in any order). Opening prefers an exact `"Open"` match; if none
/// exists (an activator whose only clip isn't named `"Open"`), falls back to
/// the alphabetically-first clip name so the choice is deterministic without
/// depending on glTF export order. Closing prefers `"Close"` and has no
/// fallback: a door/container with no `Close` clip simply doesn't animate
/// its close (F57.2's "assets without clips behave exactly as today").
pub(crate) fn select_clip(transition: ClipTransition, clip_names: &[String]) -> Option<String> {
    if clip_names.is_empty() {
        return None;
    }
    let target = match transition {
        ClipTransition::Opening => "Open",
        ClipTransition::Closing => "Close",
    };
    if let Some(name) = clip_names.iter().find(|name| name.as_str() == target) {
        return Some(name.clone());
    }
    match transition {
        ClipTransition::Opening => clip_names.iter().min().cloned(),
        ClipTransition::Closing => None,
    }
}

/// T57.2: the travel lead in seconds -- how long a travel door's `Open` clip
/// gets to play before the deferred `DoorTravelRequested` write fires.
/// `open_clip_seconds` is `None` when the door has no `Open` clip (or its
/// animation hasn't been discovered yet), which yields zero lead: travel
/// fires the same frame, bit-for-bit the wave-2 behavior this issue must not
/// regress.
pub(crate) fn open_lead_seconds(open_clip_seconds: Option<f32>, cap_seconds: f32) -> f32 {
    open_clip_seconds
        .map(|seconds| seconds.max(0.0).min(cap_seconds.max(0.0)))
        .unwrap_or(0.0)
}

/// T57.3 / F57.4: mid-animation reversal decision. A true continuation from
/// the current pose would need to blend between two different clips' curves
/// frame-by-frame; ponytail: this mirrors the elapsed time onto the new
/// clip instead (`new_duration - previous_seek_seconds`, clamped into the
/// clip's range), which reads as a clean reversal for the symmetric
/// swing/slide timings Fallout door animations actually use, without
/// building a cross-clip blend.
pub(crate) fn reversal_seek_seconds(new_clip_duration: f32, previous_seek_seconds: f32) -> f32 {
    let duration = new_clip_duration.max(0.0);
    (duration - previous_seek_seconds).clamp(0.0, duration)
}

#[cfg(test)]
#[path = "tests/policy.rs"]
mod tests;
