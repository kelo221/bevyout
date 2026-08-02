//! Pure breath timing policy for the existing exterior swimming resource.
//!
//! This module intentionally returns a value instead of mutating
//! `SwimmingState`. The W4-C runtime adapter will apply the result to that
//! existing resource and decide how an exhausted player is presented or
//! whether movement is blocked.

/// Breath consumed per second while submerged.
pub(crate) const BREATH_DRAIN_PER_SECOND: f32 = 1.0;
/// Breath recovered per second while not submerged.
pub(crate) const BREATH_RECOVERY_PER_SECOND: f32 = 1.5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BreathConsequence {
    None,
    Exhausted,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BreathUpdate {
    pub(crate) remaining_seconds: f32,
    pub(crate) consequence: BreathConsequence,
}

/// Advance one breath clock by elapsed time.
///
/// The calculation stays in the existing resource-sized `f32` representation;
/// clamping at both bounds makes exhaustion and recovery independent of the
/// final chunk that crosses a bound.
pub(crate) fn advance_breath(
    breath_seconds: f32,
    max_breath_seconds: f32,
    submerged: bool,
    elapsed_seconds: f32,
) -> BreathUpdate {
    let max = finite_nonnegative(max_breath_seconds);
    let current = finite_nonnegative(breath_seconds).min(max);
    let elapsed = finite_nonnegative(elapsed_seconds);
    let rate = if submerged {
        BREATH_DRAIN_PER_SECOND
    } else {
        BREATH_RECOVERY_PER_SECOND
    };
    let next = if submerged {
        (current - elapsed * rate).max(0.0)
    } else {
        (current + elapsed * rate).min(max)
    };
    let consequence = if submerged && next == 0.0 {
        BreathConsequence::Exhausted
    } else {
        BreathConsequence::None
    };
    BreathUpdate {
        remaining_seconds: next,
        consequence,
    }
}

fn finite_nonnegative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
