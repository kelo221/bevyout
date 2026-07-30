//! Deterministic cell map artifact (issue #45): exterior grid coordinates,
//! worldspace membership, and content-set-wide door connectivity, emitted as
//! RON by `cells --map` (see `src/vsa/catalog.rs`).
//!
//! This module is bevyout's own -- nothing here is ported from OpenMW, so it
//! does not live under `src/vsa/openmw_esm4/`. It is deliberately kept
//! dependency-free (only `serde`/`ron`/`std`) so it can be pulled in verbatim
//! by `tests/features.rs` the same way `src/vsa/manifest/mod.rs` is, without
//! that suite needing to compile the much larger attributed ESM4 reader.
//! Boundary conversion from the parser's `ParsedContentSet` types into the
//! plain inputs `CellMap::build` accepts happens in `catalog.rs`, mirroring
//! how `CellCatalog::build` already consumes the parser for the existing
//! cell catalogue.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct WorldspaceEntry {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CellMapEntry {
    pub(crate) form_id: u32,
    pub(crate) editor_id: Option<String>,
    pub(crate) interior: bool,
    /// `WRLD` FormID this cell belongs to; `None` for interiors.
    pub(crate) worldspace_form_id: Option<u32>,
    /// Exterior `XCLC` grid coordinates; `None` for interiors, or exteriors
    /// missing the subrecord.
    pub(crate) grid: Option<(i32, i32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct DoorEdge {
    pub(crate) source_cell_form_id: u32,
    pub(crate) door_reference_form_id: u32,
    pub(crate) destination_cell_form_id: u32,
    pub(crate) destination_door_reference_form_id: u32,
    pub(crate) position: [f32; 3],
    pub(crate) rotation: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct CellMap {
    pub(crate) content_fingerprint: String,
    pub(crate) worldspaces: Vec<WorldspaceEntry>,
    pub(crate) cells: Vec<CellMapEntry>,
    pub(crate) doors: Vec<DoorEdge>,
    /// Teleports (`XTEL`) whose destination reference could not be found in
    /// the loaded content set. Counted, not fatal (F45.3).
    pub(crate) unresolved_door_count: u32,
}

impl CellMap {
    /// Builds a `CellMap` from plain inputs, sorting every collection by
    /// FormID (doors by `(source_cell_form_id, door_reference_form_id)`) so
    /// the result -- and its RON serialization -- is byte-identical across
    /// runs regardless of the input order (T45.4).
    pub(crate) fn build(
        content_fingerprint: String,
        mut worldspaces: Vec<WorldspaceEntry>,
        mut cells: Vec<CellMapEntry>,
        mut doors: Vec<DoorEdge>,
        unresolved_door_count: u32,
    ) -> Self {
        worldspaces.sort_by_key(|worldspace| worldspace.form_id);
        cells.sort_by_key(|cell| cell.form_id);
        doors.sort_by_key(|door| (door.source_cell_form_id, door.door_reference_form_id));
        Self {
            content_fingerprint,
            worldspaces,
            cells,
            doors,
            unresolved_door_count,
        }
    }

    pub(crate) fn to_ron(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
    }
}

#[cfg(test)]
#[path = "tests/cell_map.rs"]
mod tests;
