//! Exterior preparation slice.
//!
//! The parser boundary stays in `openmw_esm4`; this module owns the
//! deterministic conversion of parsed records into worldspace indexes and
//! self-contained cell packages.

mod conversion;
mod index;
mod package;
mod terrain;

pub use conversion::exterior_conversion_report;
pub(crate) use index::build_worldspace_indexes;
pub use index::exterior_catalog;
pub(crate) use package::{apply_staged_assets, build_cell_package};
pub(crate) use terrain::terrain_from_land;

pub(crate) const PERSISTENT_REFERENCE_FLAG: u32 = 0x0000_0400;
pub(crate) const DISTANT_REFERENCE_FLAG: u32 = 0x0000_8000;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
