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
/// Wave 4 (#55, A15): 48 was measured against 64 on the largest cell and
/// was indistinguishable from run-to-run noise (the residual variance is
/// texture-upload bursts, not collider work) -- left at 64.
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

/// The two runtime phases used when a resident cell becomes active. Static
/// and keyframed collision must exist before dynamic bodies are allowed to
/// enter gravity, otherwise a prop can cross the floor before its collider is
/// registered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ColliderBuildPhase {
    Static,
    Dynamic,
    Ready,
}

/// Partitions enabled physics work without depending on Bevy or the manifest
/// types. The boolean is true for a dynamic placement and false for static or
/// keyframed work.
pub(crate) fn partition_collider_indices(
    items: impl IntoIterator<Item = (usize, bool)>,
) -> (Vec<usize>, Vec<usize>) {
    let mut static_indices = Vec::new();
    let mut dynamic_indices = Vec::new();
    for (index, dynamic) in items {
        if dynamic {
            dynamic_indices.push(index);
        } else {
            static_indices.push(index);
        }
    }
    (static_indices, dynamic_indices)
}

/// Resolves the next collider phase from queue state. `static_ready` is kept
/// separate from queue emptiness so a just-drained static batch cannot also
/// start dynamic bodies in the same update.
pub(crate) fn next_collider_build_phase(
    static_pending: bool,
    dynamic_pending: bool,
    static_ready: bool,
) -> ColliderBuildPhase {
    if !static_ready && static_pending {
        ColliderBuildPhase::Static
    } else if dynamic_pending {
        ColliderBuildPhase::Dynamic
    } else {
        ColliderBuildPhase::Ready
    }
}

#[cfg(test)]
#[path = "tests/swap_policy.rs"]
mod tests;
