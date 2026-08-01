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

    pub(crate) fn set_lifecycle(&mut self, grid: GridCoordinate, lifecycle: ExteriorCellLifecycle) {
        if let Some(cell) = self.cells.get_mut(&grid) {
            cell.state.lifecycle = lifecycle;
        }
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
