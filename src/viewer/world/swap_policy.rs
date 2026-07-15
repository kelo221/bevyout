//! Pure swap-eligibility seam for issue #52's instant door transition, plus
//! issue #59's fallback lifecycle and overlay-fade policy. Std-only (no
//! Bevy, no additional crates) so `tests/features.rs` can pull this in
//! verbatim exactly like `policy.rs` (issue #51) does -- see that module's
//! doc comment for the pattern this follows. (The save-application seam
//! that used to live here moved to `persist_policy.rs` in issues #60/#61.)
//!
//! Nothing here is ported from OpenMW; it is bevyout's own design against
//! the door-transition flow read (not copied) from
//! `apps/openmw/mwworld/scene.cpp`'s `changeCellByMovingPlayer`.

use std::collections::VecDeque;

/// Residency of the destination cell at the moment a door is activated,
/// mirroring `world::preload::ResidentState` plus an `Absent` case for a
/// cell with no resident entry at all (see `world::preload::ResidentCells`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Residency {
    Ready,
    Loading,
    Absent,
}

/// F52.1: whether a door activation can swap the active cell in the current
/// frame, or must fall back to a loading screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SwapDecision {
    Instant,
    Fallback,
}

/// Total, trivial decision table (F52.1): only a `Ready` resident
/// destination is eligible for an instant swap. `manifest_exists` does not
/// change the decision itself (a `Ready` cell obviously has a manifest) --
/// it is threaded through so callers can compute it once and also use it to
/// predict a fallback's `FallbackOutcome` before attempting the load.
pub(crate) fn swap_decision(manifest_exists: bool, residency: Residency) -> SwapDecision {
    let _ = manifest_exists;
    match residency {
        Residency::Ready => SwapDecision::Instant,
        Residency::Loading | Residency::Absent => SwapDecision::Fallback,
    }
}

/// F52.1: the outcome of a fallback load attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FallbackOutcome {
    Proceed,
    ReturnToSource,
}

pub(crate) fn fallback_outcome(load_ok: bool) -> FallbackOutcome {
    if load_ok {
        FallbackOutcome::Proceed
    } else {
        FallbackOutcome::ReturnToSource
    }
}

/// A pure FIFO queue of pending collider-build work items, drained at most
/// `budget` per call so cell-swap collider construction (F52.2) never
/// spikes a single frame. Generic over the work-item type so the Bevy-side
/// caller can queue whatever index/handle it needs without this module
/// depending on Bevy.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ColliderBuildQueue<T> {
    pending: VecDeque<T>,
}

impl<T> ColliderBuildQueue<T> {
    pub(crate) fn new(items: impl IntoIterator<Item = T>) -> Self {
        Self {
            pending: items.into_iter().collect(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.pending.len()
    }

    /// Removes and returns up to `budget` items in FIFO order. Returns
    /// fewer than `budget` (down to zero) once the queue is drained.
    pub(crate) fn drain_budget(&mut self, budget: usize) -> Vec<T> {
        let take = budget.min(self.pending.len());
        self.pending.drain(..take).collect()
    }
}

/// Conservative per-frame collider-build budget (F52.2): kept small so a
/// large destination cell's colliders never spike a single frame.
pub(crate) const COLLIDER_BUILD_BUDGET_PER_FRAME: usize = 64;

/// F59.1: whether a fallback swap is currently in flight. `swap.rs` derives
/// this from `PendingFallbackSwap` being `Some`/`None`; it is threaded
/// through explicitly here (rather than inferred inside this module) so the
/// table below is total and independently testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FallbackState {
    Idle,
    InFlight,
}

/// F59.1: an event that can resolve or interrupt an in-flight fallback
/// swap. `DestinationReady`/`ParseFailed` are `check_fallback_progress`'s
/// existing signals (see `fallback_outcome` above); `PlayerCancelled` is a
/// new Esc keypress while `GameplayModal::Loading` is up;
/// `SupersedingRequest` is a second `DoorTravelRequested` arriving before
/// the first fallback resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FallbackEvent {
    DestinationReady,
    ParseFailed,
    PlayerCancelled,
    SupersedingRequest,
}

/// F59.1: the lifecycle outcome for a `(state, event)` pair. `Ignore` covers
/// every event arriving with no fallback in flight (should not happen in
/// practice -- `swap.rs` only evaluates this table while
/// `PendingFallbackSwap` is `Some` -- but the table stays total rather than
/// partial). `Supersede` means: cancel the in-flight fallback (same as
/// `Cancel`, but silently -- the player is already requesting somewhere
/// else) and immediately evaluate the superseding request as a fresh one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FallbackLifecycleOutcome {
    Ignore,
    Proceed,
    ReturnToSource,
    Cancel,
    Supersede,
}

/// Total decision table (F59.1, T59.1): only `FallbackState::InFlight`
/// produces a real outcome; every event against `Idle` is `Ignore`. The
/// `DestinationReady`/`ParseFailed` rows extend issue #52's two-outcome
/// `fallback_outcome` seam (delegated to, not duplicated); the
/// `PlayerCancelled`/`SupersedingRequest` rows are what issue #59 adds.
pub(crate) fn fallback_lifecycle_outcome(
    state: FallbackState,
    event: FallbackEvent,
) -> FallbackLifecycleOutcome {
    match state {
        FallbackState::Idle => FallbackLifecycleOutcome::Ignore,
        FallbackState::InFlight => match event {
            FallbackEvent::DestinationReady | FallbackEvent::ParseFailed => {
                match fallback_outcome(event == FallbackEvent::DestinationReady) {
                    FallbackOutcome::Proceed => FallbackLifecycleOutcome::Proceed,
                    FallbackOutcome::ReturnToSource => FallbackLifecycleOutcome::ReturnToSource,
                }
            }
            FallbackEvent::PlayerCancelled => FallbackLifecycleOutcome::Cancel,
            FallbackEvent::SupersedingRequest => FallbackLifecycleOutcome::Supersede,
        },
    }
}

/// F59.2: how long the loading overlay takes to fade in or out.
pub(crate) const OVERLAY_FADE_SECONDS: f32 = 0.25;

/// F59.2: the overlay's fully-opaque background alpha (matches the flat
/// value it used before this issue).
pub(crate) const OVERLAY_MAX_ALPHA: f32 = 0.85;

/// F59.2: monotonic 0->1 progress over `duration_seconds`, clamped so
/// neither endpoint is overshot. A non-positive duration is treated as
/// already complete (progress 1.0) rather than dividing by zero.
pub(crate) fn fade_progress(elapsed_seconds: f32, duration_seconds: f32) -> f32 {
    if duration_seconds <= 0.0 {
        return 1.0;
    }
    (elapsed_seconds / duration_seconds).clamp(0.0, 1.0)
}

/// F59.2: the overlay's alpha `elapsed_seconds` into fading in, from
/// transparent up to `max_alpha`.
pub(crate) fn fade_in_alpha(elapsed_seconds: f32, duration_seconds: f32, max_alpha: f32) -> f32 {
    fade_progress(elapsed_seconds, duration_seconds) * max_alpha
}

/// F59.2: the overlay's alpha `elapsed_seconds` into fading out, from
/// `max_alpha` back down to transparent -- the symmetric mirror of
/// `fade_in_alpha` (T59.2).
pub(crate) fn fade_out_alpha(elapsed_seconds: f32, duration_seconds: f32, max_alpha: f32) -> f32 {
    (1.0 - fade_progress(elapsed_seconds, duration_seconds)) * max_alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    // T52.1: resident Ready -> Instant.
    #[test]
    fn ready_residency_is_an_instant_swap() {
        assert_eq!(swap_decision(true, Residency::Ready), SwapDecision::Instant);
    }

    // T52.1: Loading -> Fallback.
    #[test]
    fn loading_residency_is_a_fallback_swap() {
        assert_eq!(
            swap_decision(true, Residency::Loading),
            SwapDecision::Fallback
        );
    }

    // T52.1: Absent + manifest-exists -> Fallback.
    #[test]
    fn absent_with_manifest_on_disk_is_a_fallback_swap() {
        assert_eq!(
            swap_decision(true, Residency::Absent),
            SwapDecision::Fallback
        );
    }

    // T52.1: Absent + no manifest -> Fallback, whose failure -> ReturnToSource.
    #[test]
    fn absent_with_no_manifest_is_a_fallback_swap_that_fails_to_return_to_source() {
        assert_eq!(
            swap_decision(false, Residency::Absent),
            SwapDecision::Fallback
        );
        assert_eq!(fallback_outcome(false), FallbackOutcome::ReturnToSource);
    }

    #[test]
    fn a_successful_fallback_load_proceeds() {
        assert_eq!(fallback_outcome(true), FallbackOutcome::Proceed);
    }

    #[test]
    fn collider_build_queue_drains_at_most_the_budget_per_call() {
        let mut queue = ColliderBuildQueue::new(0..10);
        assert_eq!(queue.len(), 10);
        let first = queue.drain_budget(4);
        assert_eq!(first, vec![0, 1, 2, 3]);
        assert_eq!(queue.len(), 6);
        let second = queue.drain_budget(100);
        assert_eq!(second, vec![4, 5, 6, 7, 8, 9]);
        assert!(queue.is_empty());
    }

    #[test]
    fn collider_build_queue_drain_budget_of_zero_is_a_no_op() {
        let mut queue = ColliderBuildQueue::new([1, 2, 3]);
        assert!(queue.drain_budget(0).is_empty());
        assert_eq!(queue.len(), 3);
    }

    // T59.1: no fallback in flight -> every event is ignored.
    #[test]
    fn idle_state_ignores_every_event() {
        for event in [
            FallbackEvent::DestinationReady,
            FallbackEvent::ParseFailed,
            FallbackEvent::PlayerCancelled,
            FallbackEvent::SupersedingRequest,
        ] {
            assert_eq!(
                fallback_lifecycle_outcome(FallbackState::Idle, event),
                FallbackLifecycleOutcome::Ignore
            );
        }
    }

    // T59.1: an in-flight fallback whose destination becomes ready proceeds.
    #[test]
    fn in_flight_destination_ready_proceeds() {
        assert_eq!(
            fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::DestinationReady),
            FallbackLifecycleOutcome::Proceed
        );
    }

    // T59.1: an in-flight fallback whose parse fails returns to source.
    #[test]
    fn in_flight_parse_failed_returns_to_source() {
        assert_eq!(
            fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::ParseFailed),
            FallbackLifecycleOutcome::ReturnToSource
        );
    }

    // T59.1: an in-flight fallback the player cancels (Esc) cancels cleanly.
    #[test]
    fn in_flight_player_cancelled_cancels() {
        assert_eq!(
            fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::PlayerCancelled),
            FallbackLifecycleOutcome::Cancel
        );
    }

    // T59.1: a superseding travel request cancels the old fallback and
    // starts the new one -- the caller (`swap.rs`) implements "start the
    // new one" by re-running `swap_decision` for the superseding request
    // once this table says `Supersede`.
    #[test]
    fn in_flight_superseding_request_supersedes() {
        assert_eq!(
            fallback_lifecycle_outcome(FallbackState::InFlight, FallbackEvent::SupersedingRequest),
            FallbackLifecycleOutcome::Supersede
        );
    }

    // T59.2: fade progress is monotonic and clamped to [0, 1].
    #[test]
    fn fade_progress_is_monotonic_and_clamped() {
        assert_eq!(fade_progress(-1.0, 0.25), 0.0);
        assert_eq!(fade_progress(0.0, 0.25), 0.0);
        let quarter = fade_progress(0.0625, 0.25);
        let half = fade_progress(0.125, 0.25);
        let three_quarter = fade_progress(0.1875, 0.25);
        assert!(quarter < half);
        assert!(half < three_quarter);
        assert!(three_quarter < 1.0);
        assert_eq!(fade_progress(0.25, 0.25), 1.0);
        assert_eq!(fade_progress(10.0, 0.25), 1.0);
    }

    // T59.2: a non-positive duration never divides by zero -- it is
    // instantaneous instead.
    #[test]
    fn fade_progress_treats_non_positive_duration_as_instantaneous() {
        assert_eq!(fade_progress(0.0, 0.0), 1.0);
        assert_eq!(fade_progress(0.1, -1.0), 1.0);
    }

    // T59.2: fade-in reaches 0 at the start and max_alpha at the duration.
    #[test]
    fn fade_in_alpha_spans_zero_to_max() {
        assert_eq!(fade_in_alpha(0.0, 0.25, 0.85), 0.0);
        assert_eq!(fade_in_alpha(0.25, 0.25, 0.85), 0.85);
    }

    // T59.2: fade-out is the symmetric mirror of fade-in -- at any elapsed
    // time `t`, `fade_out_alpha(t)` equals `fade_in_alpha(duration - t)`.
    #[test]
    fn fade_out_alpha_is_the_symmetric_mirror_of_fade_in_alpha() {
        let duration = 0.25;
        let max_alpha = 0.85;
        for elapsed in [0.0, 0.05, 0.125, 0.2, 0.25] {
            let out = fade_out_alpha(elapsed, duration, max_alpha);
            let mirrored_in = fade_in_alpha(duration - elapsed, duration, max_alpha);
            assert!(
                (out - mirrored_in).abs() < 1e-6,
                "fade_out_alpha({elapsed}) = {out} != fade_in_alpha({}) = {mirrored_in}",
                duration - elapsed
            );
        }
        assert_eq!(fade_out_alpha(0.0, duration, max_alpha), max_alpha);
        assert_eq!(fade_out_alpha(duration, duration, max_alpha), 0.0);
    }
}
