//! Pure swap-eligibility and save-application seam for issue #52's instant
//! door transition. Std-only (no Bevy, no additional crates) so
//! `tests/features.rs` can pull this in verbatim exactly like `policy.rs`
//! (issue #51) does -- see that module's doc comment for the pattern this
//! follows.
//!
//! Nothing here is ported from OpenMW; it is bevyout's own design against
//! the door-transition flow read (not copied) from
//! `apps/openmw/mwworld/scene.cpp`'s `changeCellByMovingPlayer`.
//!
//! `ReferenceDelta`/`TransformDelta` deliberately mirror the
//! runtime-relevant fields of `save::PersistentReferenceDelta`/
//! `save::SavedTransform` with their own plain types rather than depending
//! on `src/save` (which is itself std/serde-only but pulls in `anyhow` and
//! `sha2`), the same stand-in approach `policy::DoorLink` uses for
//! `vsa::cell_map::DoorEdge`.

use std::collections::{HashMap, VecDeque};

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

/// Mirrors the runtime-relevant fields of `save::SavedTransform`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TransformDelta {
    pub(crate) translation: [f32; 3],
    pub(crate) rotation_xyzw: [f32; 4],
    pub(crate) scale: [f32; 3],
}

/// Mirrors the runtime-relevant fields of `save::PersistentReferenceDelta`
/// (F52.3): only `enabled`/`deleted`/`transform` affect the spawned
/// placement's visibility and pose, so inventory/lock/body deltas are
/// deliberately left out of this seam.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct ReferenceDelta {
    pub(crate) enabled: Option<bool>,
    pub(crate) deleted: bool,
    pub(crate) transform: Option<TransformDelta>,
}

/// One spawned placement's reference FormID, for matching against
/// `ReferenceDelta`s keyed the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PlacementRef {
    pub(crate) reference_form_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VisibilityDecision {
    Visible,
    Hidden,
}

/// One placement's activation-time decision (F52.3): whether to show it and
/// what transform delta (if any) to apply.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PlacementApplication {
    pub(crate) reference_form_id: u32,
    pub(crate) visibility: VisibilityDecision,
    pub(crate) transform: Option<TransformDelta>,
}

/// Pure per-reference visibility/transform decision (F52.3, T52.2): deleted
/// or explicitly-disabled references are hidden, everything else stays
/// visible; a saved transform delta is carried through untouched. A
/// placement with no matching delta is returned unchanged (visible, no
/// transform).
pub(crate) fn apply_persistent_cell_state(
    deltas: &HashMap<u32, ReferenceDelta>,
    placements: &[PlacementRef],
) -> Vec<PlacementApplication> {
    placements
        .iter()
        .map(|placement| {
            let delta = deltas.get(&placement.reference_form_id);
            let hidden = delta.is_some_and(|delta| delta.deleted || delta.enabled == Some(false));
            PlacementApplication {
                reference_form_id: placement.reference_form_id,
                visibility: if hidden {
                    VisibilityDecision::Hidden
                } else {
                    VisibilityDecision::Visible
                },
                transform: delta.and_then(|delta| delta.transform),
            }
        })
        .collect()
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

    fn transform_delta() -> TransformDelta {
        TransformDelta {
            translation: [1.0, 2.0, 3.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    // T52.2: disabled -> hidden.
    #[test]
    fn a_disabled_reference_is_hidden() {
        let deltas = HashMap::from([(
            0x100,
            ReferenceDelta {
                enabled: Some(false),
                ..Default::default()
            },
        )]);
        let placements = [PlacementRef {
            reference_form_id: 0x100,
        }];
        let applications = apply_persistent_cell_state(&deltas, &placements);
        assert_eq!(applications[0].visibility, VisibilityDecision::Hidden);
    }

    // T52.2: deleted -> hidden.
    #[test]
    fn a_deleted_reference_is_hidden() {
        let deltas = HashMap::from([(
            0x100,
            ReferenceDelta {
                deleted: true,
                ..Default::default()
            },
        )]);
        let placements = [PlacementRef {
            reference_form_id: 0x100,
        }];
        let applications = apply_persistent_cell_state(&deltas, &placements);
        assert_eq!(applications[0].visibility, VisibilityDecision::Hidden);
    }

    // T52.2: transform delta applied.
    #[test]
    fn a_transform_delta_is_carried_through() {
        let deltas = HashMap::from([(
            0x100,
            ReferenceDelta {
                transform: Some(transform_delta()),
                ..Default::default()
            },
        )]);
        let placements = [PlacementRef {
            reference_form_id: 0x100,
        }];
        let applications = apply_persistent_cell_state(&deltas, &placements);
        assert_eq!(applications[0].visibility, VisibilityDecision::Visible);
        assert_eq!(applications[0].transform, Some(transform_delta()));
    }

    // T52.2: untouched refs (no matching delta) are unchanged.
    #[test]
    fn an_untouched_reference_stays_visible_with_no_transform() {
        let deltas = HashMap::new();
        let placements = [PlacementRef {
            reference_form_id: 0x999,
        }];
        let applications = apply_persistent_cell_state(&deltas, &placements);
        assert_eq!(applications[0].visibility, VisibilityDecision::Visible);
        assert_eq!(applications[0].transform, None);
    }

    #[test]
    fn enabled_true_with_no_other_flags_stays_visible() {
        let deltas = HashMap::from([(
            0x100,
            ReferenceDelta {
                enabled: Some(true),
                ..Default::default()
            },
        )]);
        let placements = [PlacementRef {
            reference_form_id: 0x100,
        }];
        let applications = apply_persistent_cell_state(&deltas, &placements);
        assert_eq!(applications[0].visibility, VisibilityDecision::Visible);
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
}
