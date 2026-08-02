//! Runtime lifecycle state. Logical cancellation is generation based so an
//! uncancellable filesystem task can never resurrect an evicted cell.

use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::{Entity, Resource};
use bevyout_core::manifest::exterior::{
    ExteriorCellLifecycle, ExteriorCellPackage, ExteriorCellState, GridCoordinate,
};

#[derive(Debug)]
pub(crate) struct RuntimeCell {
    pub(crate) state: ExteriorCellState,
    pub(crate) root: Option<Entity>,
    pub(crate) task: Option<Entity>,
    pub(crate) package: Option<ExteriorCellPackage>,
    pub(crate) collision_ready: bool,
    pub(crate) eviction_restore: Option<ExteriorCellLifecycle>,
}

impl RuntimeCell {
    pub(crate) fn owns_runtime_state(&self, collision_owned: bool) -> bool {
        self.root.is_some() || self.package.is_some() || self.collision_ready || collision_owned
    }

    /// Mark a cell for ordered teardown and invalidate every completion from
    /// its previous load generation. Repeated eviction actions are idempotent.
    pub(crate) fn begin_eviction(&mut self) -> bool {
        if self.state.lifecycle == ExteriorCellLifecycle::Evicting {
            return false;
        }
        self.eviction_restore = Some(self.state.lifecycle);
        self.state.lifecycle = ExteriorCellLifecycle::Evicting;
        self.state.generation = self.state.generation.saturating_add(1);
        true
    }

    /// Undo an eviction before its final exclusive teardown phase. The
    /// generation remains bumped so an older filesystem completion cannot
    /// repopulate the cell while it is being restored.
    pub(crate) fn cancel_eviction(
        &mut self,
        current_grid: GridCoordinate,
        collision_owned: bool,
    ) -> bool {
        if self.state.lifecycle != ExteriorCellLifecycle::Evicting {
            return false;
        }
        let fallback = if self.root.is_some() || self.package.is_some() {
            ExteriorCellLifecycle::Loading
        } else if self.collision_ready || collision_owned {
            if self.state.grid == current_grid {
                ExteriorCellLifecycle::Resident
            } else {
                ExteriorCellLifecycle::Ready
            }
        } else {
            ExteriorCellLifecycle::Unloaded
        };
        let restored = self.eviction_restore.take().unwrap_or(fallback);
        self.state.lifecycle = if matches!(
            restored,
            ExteriorCellLifecycle::Queued | ExteriorCellLifecycle::Loading
        ) && self.task.is_none()
            && self.root.is_none()
            && self.package.is_none()
            && !self.collision_ready
            && !collision_owned
        {
            ExteriorCellLifecycle::Unloaded
        } else {
            restored
        };
        if collision_owned {
            self.collision_ready = true;
        }
        true
    }
}

#[derive(Resource, Debug, Default)]
pub(crate) struct ExteriorStreamState {
    pub(crate) initialized: bool,
    pub(crate) player_positioned: bool,
    pub(crate) asset_root: Option<std::path::PathBuf>,
    pub(crate) worldspace_form_id: Option<u32>,
    pub(crate) index: Option<bevyout_core::manifest::exterior::ExteriorWorldspaceIndex>,
    pub(crate) current_grid: GridCoordinate,
    pub(crate) previous_grid: Option<GridCoordinate>,
    pub(crate) cells: BTreeMap<GridCoordinate, RuntimeCell>,
    pub(crate) collision_cells: BTreeMap<GridCoordinate, u32>,
    pub(crate) persistence_applied: BTreeSet<GridCoordinate>,
    pub(crate) trace: bool,
    pub(crate) requests: u64,
    pub(crate) ready: u64,
    pub(crate) evictions: u64,
    pub(crate) cancellations: u64,
    pub(crate) stale_completions: u64,
    pub(crate) failures: u64,
    /// Rejected or otherwise invalid unload attempts. Expected stale task
    /// completions are tracked separately and do not increment this counter.
    pub(crate) invalid_unload_count: u64,
    pub(crate) resident_budget: usize,
    pub(crate) byte_budget: u64,
    pub(crate) resident_bytes: u64,
    pub(crate) peak_resident_cells: usize,
    pub(crate) peak_memory: u64,
}

impl ExteriorStreamState {
    pub(crate) fn states(&self) -> Vec<ExteriorCellState> {
        self.cells.values().map(|cell| cell.state).collect()
    }

    /// Records the high-water marks that the runtime can actually derive.
    /// A package owns resident entities as soon as its root is spawned, even
    /// while collision attachment keeps its logical lifecycle at `Loading`.
    pub(crate) fn record_peaks(&mut self) {
        let spawned_roots = self
            .cells
            .values()
            .filter(|cell| cell.root.is_some())
            .count();
        self.peak_resident_cells = self.peak_resident_cells.max(spawned_roots);
        self.peak_memory = self.peak_memory.max(self.resident_bytes);
    }
}
