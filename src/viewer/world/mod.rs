//! Runtime cell map and predictive door-graph neighbor preloader (issue #51).
//!
//! `policy.rs` is a pure, std-only planner (mirrors `vsa::cell_map`'s
//! dependency-free design so `tests/features.rs` can include it verbatim);
//! `preload.rs` is the Bevy-side glue that reads `<asset_root>/cellmap.ron`
//! into `policy::CellGraph`, tracks `ActiveCell`/`ResidentCells`, and drives
//! background manifest parsing plus hidden per-cell root spawning for
//! planned loads.

mod policy;
mod preload;

pub(crate) use preload::{ResidentCell, ResidentCells, ResidentState, install};
