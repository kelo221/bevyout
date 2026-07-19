//! Retired runtime navmesh erosion (issue #136 -> superseded by issue #153,
//! M4 wave 10).
//!
//! The interim boundary-erosion pass (#136, always explicitly interim) moved
//! walkable-boundary vertices inward by the agent radius at runtime, just
//! before handing the mesh to `bevy_landmass`. Wave 9's root-causing (issues
//! #148/#164) showed boundary erosion is structurally blind to the two real
//! defect classes -- walkable mesh over a *missing floor*, and *interior*
//! colliders the authored NAVM paved over -- because it only moves boundary
//! edges. It was retired in favour of collision-derived validation +
//! clearance done **prepare-side** in `vsa::prepare::nav_clearance`, which
//! consumes the cell's cooked static collision to remove unsupported
//! triangles (F153.1), cut interior obstructions (F153.2), and offset the
//! validated boundary with miter-corrected corners while disconnecting
//! sub-diameter corridors (F153.3). Those decisions are baked into the
//! prepared graph (offset vertex positions + a per-polygon `walkable` flag)
//! and consumed by `viewer::nav::mesh_inputs`.
//!
//! [`erode`] is kept as a **no-op passthrough** only because
//! `landmass_graph::build_navigation_mesh` still calls it on its owned seam
//! (which this issue's file-ownership boundary does not include): it returns
//! the input vertices unchanged with zero counters, so no clearance happens
//! twice. The types stay so that call site keeps compiling; a follow-up that
//! owns `landmass_graph.rs` can drop the call and this module together.
//!
//! Std-only (no `bevy`/`bevy_landmass`/`glam`/`serde`): still included
//! verbatim by `tests/features.rs` via `#[path]`.

/// Agent capsule radius (metres). Retained for `landmass_graph`'s existing
/// `erode(&input, erosion_policy::AGENT_RADIUS)` call site; the value that
/// actually drives clearance now lives in `vsa::prepare::nav_clearance`.
pub(crate) const AGENT_RADIUS: f32 = 0.35;

/// A triangle-soup navmesh (vertex array + index triples), plus the
/// seam/portal protected edges `landmass_graph::protected_edges_for_mesh`
/// still computes and passes. Kept for the retired call site; the no-op
/// [`erode`] ignores every field but `vertices`/`polygons`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ErosionMeshInput {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygons: Vec<[u32; 3]>,
    pub(crate) protected_edges: Vec<(u32, u32)>,
}

/// Result of the retired erosion pass: the (now unchanged) vertex positions
/// plus the diagnostic counters `landmass_graph`'s tracing line still reads.
/// Every counter is zero now that erosion is a no-op.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ErosionResult {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygon_count: usize,
    pub(crate) eroded_count: usize,
    pub(crate) pinch_guard_count: usize,
    pub(crate) relax_passes: usize,
    pub(crate) protected_count: usize,
}

/// No-op passthrough (issue #153): returns `mesh`'s vertices unchanged with
/// zero counters. Clearance now happens prepare-side in
/// `vsa::prepare::nav_clearance`; this exists only so the still-present
/// `landmass_graph` call site compiles and never double-erodes the already
/// clearance-offset prepared vertices.
pub(crate) fn erode(mesh: &ErosionMeshInput, _radius: f32) -> ErosionResult {
    ErosionResult {
        vertices: mesh.vertices.clone(),
        polygon_count: mesh.polygons.len(),
        eroded_count: 0,
        pinch_guard_count: 0,
        relax_passes: 0,
        protected_count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corner_room() -> ErosionMeshInput {
        ErosionMeshInput {
            vertices: vec![
                [0.0, 0.0, 0.0],
                [4.0, 0.0, 0.0],
                [4.0, 0.0, 4.0],
                [0.0, 0.0, 4.0],
            ],
            polygons: vec![[0, 1, 2], [0, 2, 3]],
            protected_edges: Vec::new(),
        }
    }

    #[test]
    fn erode_is_a_no_op_leaving_vertices_unchanged() {
        // Clearance is retired here (moved to prepare-side nav_clearance):
        // erode must never move a vertex, regardless of radius.
        let mesh = corner_room();
        let result = erode(&mesh, AGENT_RADIUS);
        assert_eq!(result.vertices, mesh.vertices);
        assert_eq!(result.polygon_count, mesh.polygons.len());
        assert_eq!(result.eroded_count, 0);
        assert_eq!(result.pinch_guard_count, 0);
        assert_eq!(result.relax_passes, 0);
        assert_eq!(result.protected_count, 0);
    }

    #[test]
    fn erode_is_deterministic_across_calls() {
        let mesh = corner_room();
        assert_eq!(erode(&mesh, AGENT_RADIUS), erode(&mesh, AGENT_RADIUS));
    }
}
