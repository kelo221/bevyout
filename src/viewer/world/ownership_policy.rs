//! Issue #63: pure per-cell collider-ownership ledger. Std-only (no Bevy,
//! no boxddd — the id types are generic parameters) so `tests/features.rs`
//! can include it verbatim like `policy.rs` and `persist_policy.rs`.
//!
//! Before this ledger existed, collider construction had no notion of which
//! cell owned what: every swap re-queued the destination's full placement
//! list, so revisits duplicated static shapes and keyframed bindings on the
//! shared static body, and a departed cell's colliders persisted for the
//! rest of the session. The ledger records what each cell's build created;
//! `release` hands back exactly that set for teardown when the cell is
//! swapped away from or evicted.
//!
//! Nothing here is ported from OpenMW; it is bevyout's own design.

use std::collections::HashMap;

/// Everything one cell's collider build registered, handed back by
/// [`CellColliderLedger::release`] for destruction.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CellColliders<S, B> {
    /// Shapes attached to the single shared static body. Teardown destroys
    /// these individually (the shared body lives on).
    pub(crate) static_shapes: Vec<S>,
    /// Shapes attached to this cell's own (keyframed/dynamic) bodies.
    /// Teardown must NOT destroy these — they die with their body, and a
    /// stale ShapeId could have been recycled — but their per-shape
    /// bookkeeping (surface-material map) still needs cleaning.
    pub(crate) body_shapes: Vec<S>,
    /// Kinematic bodies driven by door/activator animations (issue #64).
    pub(crate) keyframed_bodies: Vec<B>,
    /// Per-placement dynamic bodies.
    pub(crate) dynamic_bodies: Vec<B>,
}

// Manual impls: `derive` would put `S: Default`/`B: Default` bounds on the
// generic id types, which empty `Vec`s don't need.
impl<S, B> Default for CellColliders<S, B> {
    fn default() -> Self {
        Self {
            static_shapes: Vec::new(),
            body_shapes: Vec::new(),
            keyframed_bodies: Vec::new(),
            dynamic_bodies: Vec::new(),
        }
    }
}

impl<S, B> CellColliders<S, B> {
    pub(crate) fn shape_count(&self) -> usize {
        self.static_shapes.len()
    }

    pub(crate) fn body_count(&self) -> usize {
        self.keyframed_bodies.len() + self.dynamic_bodies.len()
    }
}

/// Which cell owns which collider ids, keyed by cell FormID.
#[derive(Debug)]
pub(crate) struct CellColliderLedger<S, B> {
    cells: HashMap<u32, CellColliders<S, B>>,
}

impl<S, B> Default for CellColliderLedger<S, B> {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}

impl<S, B> CellColliderLedger<S, B> {
    pub(crate) fn record_static_shape(&mut self, cell: u32, shape: S) {
        self.cells
            .entry(cell)
            .or_default()
            .static_shapes
            .push(shape);
    }

    pub(crate) fn record_body_shape(&mut self, cell: u32, shape: S) {
        self.cells.entry(cell).or_default().body_shapes.push(shape);
    }

    pub(crate) fn record_keyframed_body(&mut self, cell: u32, body: B) {
        self.cells
            .entry(cell)
            .or_default()
            .keyframed_bodies
            .push(body);
    }

    pub(crate) fn record_dynamic_body(&mut self, cell: u32, body: B) {
        self.cells
            .entry(cell)
            .or_default()
            .dynamic_bodies
            .push(body);
    }

    /// Removes and returns everything the cell's build registered, or `None`
    /// for a cell with nothing recorded. After this the cell is untracked;
    /// a rebuild starts a fresh entry.
    pub(crate) fn release(&mut self, cell: u32) -> Option<CellColliders<S, B>> {
        self.cells.remove(&cell)
    }

    // Exercised by `collider_ownership.feature`'s "still tracked"/"no
    // longer tracked" steps; not yet read from production Bevy glue
    // (`teardown_cell_colliders` uses `release`'s `Option` directly).
    #[allow(dead_code)]
    pub(crate) fn is_tracked(&self, cell: u32) -> bool {
        self.cells.contains_key(&cell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_returns_exactly_what_was_recorded_and_untracks_the_cell() {
        let mut ledger = CellColliderLedger::<u64, u64>::default();
        ledger.record_static_shape(0x100, 1);
        ledger.record_static_shape(0x100, 2);
        ledger.record_keyframed_body(0x100, 10);
        ledger.record_dynamic_body(0x100, 20);
        let released = ledger.release(0x100).expect("cell was tracked");
        assert_eq!(released.static_shapes, vec![1, 2]);
        assert_eq!(released.keyframed_bodies, vec![10]);
        assert_eq!(released.dynamic_bodies, vec![20]);
        assert_eq!(released.shape_count(), 2);
        assert_eq!(released.body_count(), 2);
        assert!(!ledger.is_tracked(0x100));
    }

    #[test]
    fn releasing_an_untracked_cell_returns_none() {
        let mut ledger = CellColliderLedger::<u64, u64>::default();
        assert!(ledger.release(0x999).is_none());
    }

    #[test]
    fn cells_are_independent() {
        let mut ledger = CellColliderLedger::<u64, u64>::default();
        ledger.record_static_shape(0x100, 1);
        ledger.record_static_shape(0x200, 2);
        let released = ledger.release(0x100).expect("cell was tracked");
        assert_eq!(released.static_shapes, vec![1]);
        assert!(ledger.is_tracked(0x200));
    }

    #[test]
    fn re_recording_after_release_starts_fresh() {
        let mut ledger = CellColliderLedger::<u64, u64>::default();
        ledger.record_static_shape(0x100, 1);
        ledger.release(0x100);
        ledger.record_static_shape(0x100, 7);
        let released = ledger.release(0x100).expect("cell was re-tracked");
        assert_eq!(released.static_shapes, vec![7]);
    }
}
