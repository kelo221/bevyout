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
