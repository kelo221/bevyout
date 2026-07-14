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
mod tests {
    use super::*;

    fn names(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    // T57.1: opening picks "Open" when present.
    #[test]
    fn opening_picks_the_open_clip_when_present() {
        let clips = names(&["Close", "Open"]);
        assert_eq!(
            select_clip(ClipTransition::Opening, &clips),
            Some("Open".to_string())
        );
    }

    // T57.1: closing picks "Close" when present.
    #[test]
    fn closing_picks_the_close_clip_when_present() {
        let clips = names(&["Close", "Open"]);
        assert_eq!(
            select_clip(ClipTransition::Closing, &clips),
            Some("Close".to_string())
        );
    }

    // T57.1: an activator (opening transition) with clips but no "Open"
    // deterministically picks the first (alphabetically) clip.
    #[test]
    fn opening_without_an_open_clip_falls_back_to_the_first_clip() {
        let clips = names(&["Use", "Activate"]);
        assert_eq!(
            select_clip(ClipTransition::Opening, &clips),
            Some("Activate".to_string())
        );
    }

    // T57.1: an asset without clips picks none, regardless of transition.
    #[test]
    fn no_clips_selects_nothing_for_either_transition() {
        assert_eq!(select_clip(ClipTransition::Opening, &[]), None);
        assert_eq!(select_clip(ClipTransition::Closing, &[]), None);
    }

    // Closing with no "Close" clip has no fallback (unlike opening).
    #[test]
    fn closing_without_a_close_clip_selects_nothing() {
        let clips = names(&["Open"]);
        assert_eq!(select_clip(ClipTransition::Closing, &clips), None);
    }

    // T57.2: lead is the open clip's duration, capped.
    #[test]
    fn open_lead_is_capped_by_the_clip_duration() {
        assert_eq!(open_lead_seconds(Some(0.3), OPEN_LEAD_CAP_SECONDS), 0.3);
        assert_eq!(
            open_lead_seconds(Some(1.33), OPEN_LEAD_CAP_SECONDS),
            OPEN_LEAD_CAP_SECONDS
        );
    }

    // T57.2: no clip means zero lead -- travel fires the same frame,
    // preserving wave-2 behavior bit-for-bit.
    #[test]
    fn no_open_clip_yields_zero_lead() {
        assert_eq!(open_lead_seconds(None, OPEN_LEAD_CAP_SECONDS), 0.0);
    }

    // T57.3: reversal mirrors elapsed time onto the new clip's duration.
    #[test]
    fn reversal_mirrors_elapsed_time_onto_the_new_clip() {
        // Halfway through a 1.3s Open clip; reversing into a 1.3s Close clip
        // should resume from roughly the same halfway point.
        assert!((reversal_seek_seconds(1.3, 0.65) - 0.65).abs() < 1e-6);
        // Just started (near time 0) -> reversal should start near the end.
        assert!((reversal_seek_seconds(1.3, 0.0) - 1.3).abs() < 1e-6);
        // Already finished (seek at duration) -> reversal starts at 0.
        assert!((reversal_seek_seconds(1.3, 1.3) - 0.0).abs() < 1e-6);
    }

    // T57.3: mismatched clip durations still clamp into range.
    #[test]
    fn reversal_clamps_into_the_new_clips_range() {
        assert_eq!(reversal_seek_seconds(1.0, 5.0), 0.0);
        assert_eq!(reversal_seek_seconds(1.0, -5.0), 1.0);
    }
}
