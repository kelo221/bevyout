//! Runtime cell map, predictive door-graph neighbor preloader (issue #51),
//! and instant door-transition cell swap (issue #52).
//!
//! `policy.rs` is a pure, std-only planner (mirrors `vsa::cell_map`'s
//! dependency-free design so `tests/features.rs` can include it verbatim);
//! `preload.rs` is the Bevy-side glue that reads `<asset_root>/cellmap.ron`
//! into `policy::CellGraph`, tracks `ActiveCell`/`ResidentCells`, and drives
//! background manifest parsing plus hidden per-cell root spawning for
//! planned loads.
//!
//! `swap_policy.rs` is likewise a pure, std-only seam (issue #52): the
//! eligibility decision (instant vs. fallback), the fallback load outcome,
//! save-state application to a cell's placements, and the collider-build
//! stagger queue. `swap.rs` is the Bevy-side glue that turns a door
//! activation into either an instant same-frame cell swap or a
//! loading-screen fallback that reuses `preload.rs`'s background manifest
//! parse, then performs the same activation steps either way.
//!
//! `reveal_policy.rs` is a fourth pure, std-only seam (issue #55): bounded
//! reveal-chunk planning, ordered nearest-to-arrival first, for a
//! preloaded cell's placement entities. `reveal.rs` is the Bevy-side glue
//! `swap.rs` calls into during `activate_resident_cell` to flip visibility
//! in bounded chunks across a few frames instead of all at once -- see that
//! module's doc comment for the measured spike this amortizes.

mod persist;
mod persist_policy;
mod policy;
mod preload;
mod reveal;
mod reveal_policy;
mod swap;
// Issues #60/#61 moved the save-application path to `persist_policy`, which
// left `swap_policy`'s `apply_persistent_cell_state` seam (and its
// ReferenceDelta/TransformDelta/... types) dead in the lib target. Per the
// wave-4 file-ownership boundary that module is not touched here; the
// orchestrator deletes the dead seam (and this allow) at merge.
#[allow(dead_code)]
mod swap_policy;

pub(crate) use persist::{
    ActiveSaveState, DynamicBodyRestore, PersistRestores, apply_save_state_at_startup,
    write_save_slot,
};
pub(crate) use preload::{ResidentCell, ResidentCells, ResidentState};
pub(crate) use swap_policy::{COLLIDER_BUILD_BUDGET_PER_FRAME, ColliderBuildQueue};

pub(crate) fn install(app: &mut bevy::app::App, resident_cell_limit: usize) {
    persist::install(app);
    preload::install(app, resident_cell_limit);
    reveal::install(app);
    swap::install(app);
}
