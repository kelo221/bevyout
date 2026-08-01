//! Pure AO-mesh eligibility tracking for the viewer's `apply_ao_strength`
//! system (issue #270, PERF wave 1).
//!
//! Kept free of Bevy imports so the executable-specification suite in
//! `tests/features.rs` can include this file verbatim (the same pattern as
//! `realtime_shadow_policy.rs`). Keys are generic: the runtime adapter in
//! `viewer::controls` uses `Entity`/`AssetId<Mesh>`, the spec drives plain
//! integers.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Incremental, event-driven replacement for the old per-frame
/// count-sentinel scan (`AoScanState`).
///
/// The tracker is fed by three signal classes:
///
/// - `discover`: a mesh entity was added or its mesh handle changed. The
///   caller supplies the ancestor-walk outcome (`eligible`); the tracker
///   handles the bookkeeping, including a handle swap retiring the old
///   mesh's reference.
/// - `release`: the entity despawned or lost its mesh component. Reference
///   counts keep a mesh eligible while any other entity still references
///   it, and make remove+add pairs leaving equal entity/asset totals
///   impossible to miss (the exact blind spot of the count sentinel).
/// - `asset_added`: the asset store (re)inserted a mesh. A still-referenced
///   mesh must be rescaled from a fresh baseline -- a baseline captured
///   before the reload was recorded against already-scaled colors.
///
/// `pending` is the work queue for `apply_ao_strength`: only pending meshes
/// are touched on quiet frames, and the entire eligible set is re-scaled
/// from baselines when the AO strength changes.
#[derive(Clone, Debug)]
pub(crate) struct AoEligibilityTracker<E, K>
where
    E: Eq + Hash + Copy,
    K: Eq + Hash + Copy,
{
    /// Last eligible mesh each tracked entity referred to. Without a
    /// per-entity record a handle swap could not retire the old key's
    /// refcount, and entity removal could not know which mesh to decrement.
    entity_meshes: HashMap<E, K>,
    /// Meshes referenced by at least one AO-eligible entity, keyed by mesh
    /// with the number of referring entities.
    refcounts: HashMap<K, usize>,
    /// Eligible meshes still awaiting an `apply_ao_strength` pass (fresh
    /// baseline + scaling). Entries stay queued until the pass resolves
    /// them, so a mesh whose asset has not loaded yet is retried cheaply.
    pending: HashSet<K>,
}

impl<E, K> Default for AoEligibilityTracker<E, K>
where
    E: Eq + Hash + Copy,
    K: Eq + Hash + Copy,
{
    fn default() -> Self {
        Self {
            entity_meshes: HashMap::new(),
            refcounts: HashMap::new(),
            pending: HashSet::new(),
        }
    }
}

impl<E, K> AoEligibilityTracker<E, K>
where
    E: Eq + Hash + Copy,
    K: Eq + Hash + Copy,
{
    /// A mesh entity surfaced with mesh key `mesh` and ancestor-walk
    /// outcome `eligible`. Re-discovering the same entity with the same
    /// outcome is a no-op.
    pub(crate) fn discover(&mut self, entity: E, mesh: K, eligible: bool) {
        if let Some(previous) = self.entity_meshes.get(&entity).copied() {
            if eligible && previous == mesh {
                return;
            }
            self.release(entity);
        }
        if !eligible {
            return;
        }
        self.entity_meshes.insert(entity, mesh);
        *self.refcounts.entry(mesh).or_insert(0) += 1;
        self.pending.insert(mesh);
    }

    /// The entity despawned or lost its mesh component.
    pub(crate) fn release(&mut self, entity: E) {
        let Some(mesh) = self.entity_meshes.remove(&entity) else {
            return;
        };
        if let Some(count) = self.refcounts.get_mut(&mesh) {
            *count -= 1;
            if *count == 0 {
                self.refcounts.remove(&mesh);
                self.pending.remove(&mesh);
            }
        }
    }

    /// The asset store (re)inserted a mesh. Only still-referenced meshes
    /// are queued; a removal-dropped baseline is rebuilt on the next pass.
    pub(crate) fn asset_added(&mut self, mesh: K) {
        if self.refcounts.contains_key(&mesh) {
            self.pending.insert(mesh);
        }
    }

    // `is_pending`/`is_eligible` complete the tracker's observer surface
    // for the executable spec (`tests/features.rs`, which includes this
    // file verbatim) and the pure unit specs; the runtime adapter reads
    // the iterators instead (they are `#[allow]`-ed, not unused there).
    #[allow(dead_code)]
    pub(crate) fn is_pending(&self, mesh: K) -> bool {
        self.pending.contains(&mesh)
    }

    pub(crate) fn has_pending(&self) -> bool {
        !self.pending.is_empty()
    }

    #[allow(dead_code)]
    pub(crate) fn is_eligible(&self, mesh: K) -> bool {
        self.refcounts.contains_key(&mesh)
    }

    pub(crate) fn pending_meshes(&self) -> impl Iterator<Item = K> + '_ {
        self.pending.iter().copied()
    }

    pub(crate) fn eligible_meshes(&self) -> impl Iterator<Item = K> + '_ {
        self.refcounts.keys().copied()
    }

    /// `apply_ao_strength` has captured (or confirmed) this mesh's baseline
    /// and applied the current strength; no further quiet-frame work is
    /// owed for it.
    pub(crate) fn resolve_pending(&mut self, mesh: K) {
        self.pending.remove(&mesh);
    }
}

#[cfg(test)]
#[path = "tests/ao_policy.rs"]
mod tests;
