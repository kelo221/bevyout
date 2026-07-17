//! Pure navmesh erosion policy (issue #136): shrinks the walkable polygon
//! boundary inward by the agent radius so `bevy_landmass`'s path smoothing
//! does not string-pull a route within less than a capsule-width of
//! wall/prop geometry.
//!
//! Context: FO3 `NAVM` meshes are authored without any agent-radius margin,
//! and since M4 wave 5 made agent movement physics-authoritative (a real
//! 0.35 m kinematic capsule character controller, `nav::agent`'s
//! `AGENT_RADIUS`/`player::CAPSULE_RADIUS`), a route that hugs the
//! un-eroded navmesh boundary can put the capsule's collider inside a wall
//! or prop, wedging it (`collision-blocked` -> `stuck`). This module is the
//! fix: it moves each navmesh vertex on the walkable-region boundary inward
//! by up to the agent radius before the mesh reaches `bevy_landmass`.
//!
//! **Landmass-native option checked first, per the wave plan (F136.1):**
//! `landmass` 0.9.1 / `bevy_landmass` 0.12.0 (this project's pinned
//! versions, `Cargo.toml`) were read before writing this module.
//! `landmass::Agent::radius` (`ArchipelagoOptions::from_agent_radius`, used
//! by `nav::agent::ensure_archipelago`) feeds only local avoidance
//! (RVO-style agent/agent separation, `landmass::avoidance`) and
//! `PointSampleDistance3d::from_agent_radius`'s off-navmesh sampling
//! envelope -- neither erodes navmesh polygon boundaries or offsets path
//! smoothing away from static geometry. `landmass::nav_mesh`/`pathfinding`
//! have no wall-clearance or polygon-offset option at all. So there is no
//! lazy landmass-side switch to flip; this module does the erosion itself.
//!
//! **Why erosion moves vertex positions, not polygon topology:** the
//! prepared navmesh is a triangle soup where adjacent triangles literally
//! share a vertex *index* into a common vertex array -- that shared index
//! is exactly what `bevy_landmass`'s `NavigationMesh3d::validate()` uses to
//! detect two polygons as connected. Eroding each polygon independently
//! (producing per-polygon private vertices) would sever every triangle from
//! its neighbours and disconnect nearly the whole mesh. Instead this module
//! only ever *moves* existing vertex positions -- indices, and therefore
//! connectivity, are untouched by construction, which is what makes
//! F136.2's "must not disconnect a previously connected walkable region"
//! trivially true rather than something the algorithm has to prove per
//! call.
//!
//! **Algorithm.** An edge shared by exactly one polygon is a walkable-region
//! boundary edge (a wall, prop cutout, or non-walkable/water exclusion --
//! `landmass_graph::build_navigation_mesh` already excludes water before
//! calling this, so those edges show up as boundary edges here too). Each
//! boundary edge contributes an inward-facing unit normal (2D, horizontal
//! X/Z plane -- matching wave 5's horizontal-plane treatment of nav-point
//! distance, since FO3 floors are near-flat and vertical erosion is not
//! this issue's problem) to both of its endpoint vertices; a vertex's raw
//! offset is the average of its incident boundary normals, renormalized to
//! length `radius`. Interior vertices (touching no boundary edge) get a
//! zero offset and never move.
//!
//! **Corridor-pinch safety (F136.2).** Applying every vertex's raw offset
//! at full strength can invert or degenerate a polygon whose available
//! width is less than `2 * radius` (a corridor narrower than the agent).
//! Each vertex carries a `[0, 1]` erosion factor, all starting at `1.0`.
//! Every polygon whose *current* (factor-scaled) shape has flipped winding
//! sign or dropped below a minimum area relative to its original shape has
//! its three vertices' factors halved, repeated to a fixed pass limit. This
//! is monotonic (factors only shrink) and deterministic (fixed iteration
//! order, no hashing), and always converges since halving a factor that is
//! already at the floor clamps it to exactly zero (the guaranteed-safe
//! original position).
//!
//! Std-only (no `bevy`/`bevy_landmass`/`glam`, not even `serde`): this file
//! is included verbatim by `tests/features.rs` via `#[path]` -- see
//! `src/viewer/world/policy.rs`'s module doc comment for why modules driven
//! from cucumber take this shape, and `landmass_graph.rs`'s module doc
//! comment for the sibling flat top-level `#[path]` inclusion this depends
//! on lining up against.
//!
//! **Agent radius constant.** `nav::agent::AGENT_RADIUS` (0.35 m) is
//! private to `agent.rs`, which is out of this issue's file-ownership
//! boundary (`agent.rs`'s runtime call site,
//! `landmass_graph::build_navigation_mesh(mesh)`, must keep its existing
//! signature so this erosion pass wires in without an `agent.rs` edit) --
//! so it cannot be imported here without either widening that private
//! const's visibility or duplicating the literal a second time in
//! `landmass_graph.rs`. This module holds the single copy for both of its
//! own files; unifying it with `nav::agent::AGENT_RADIUS` /
//! `player::CAPSULE_RADIUS` (also 0.35 m) is left to a follow-up that
//! touches `agent.rs`.
use std::collections::BTreeMap;

/// Agent capsule radius (metres) navmesh polygons are eroded by. See the
/// module doc comment for why this is not imported from `nav::agent`.
pub(crate) const AGENT_RADIUS: f32 = 0.35;

/// A triangle-soup navmesh, in the same plain shape
/// `landmass_graph::MeshInput`/`PolygonInput` already use (vertex array +
/// index triples), but without any of that module's water/door/form-id
/// fields this erosion pass has no use for.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ErosionMeshInput {
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygons: Vec<[u32; 3]>,
}

/// Result of eroding one [`ErosionMeshInput`]: the eroded vertex positions
/// (same length/order as the input -- indices are untouched, only
/// positions move), plus the F136.3 diagnostic counters.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ErosionResult {
    pub(crate) vertices: Vec<[f32; 3]>,
    /// Total polygon count considered (the walkable polygons fed in).
    pub(crate) polygon_count: usize,
    /// Polygons with at least one vertex that actually moved.
    pub(crate) eroded_count: usize,
    /// Polygons where the corridor-pinch guard reduced erosion below full
    /// strength for at least one of their vertices.
    pub(crate) pinch_guard_count: usize,
}

/// Below this scaled-triangle area (2D, horizontal plane, same units as
/// `vertices`), a candidate erosion is treated as degenerate.
const MIN_TRIANGLE_AREA: f32 = 1.0e-4;
/// Below this per-vertex erosion factor, treat the vertex as fully
/// clamped back to its original position rather than continuing to halve
/// an already-negligible offset forever.
const MIN_FACTOR: f32 = 1.0 / 1024.0;
/// Fixed pass limit for the pinch-guard shrink loop (see module doc
/// comment: monotonic, so this is a safety cap, not a tuning knob real
/// data is expected to hit).
const MAX_SHRINK_PASSES: usize = 32;

fn signed_area_xz(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> f32 {
    0.5 * ((b[0] - a[0]) * (c[2] - a[2]) - (c[0] - a[0]) * (b[2] - a[2]))
}

/// Erodes `mesh`'s walkable-region boundary inward by `radius` (metres),
/// with the corridor-pinch guard described in the module doc comment.
/// `radius <= 0.0` or an empty mesh is a no-op (original vertices
/// returned, all counters zero).
pub(crate) fn erode(mesh: &ErosionMeshInput, radius: f32) -> ErosionResult {
    let vertex_count = mesh.vertices.len();
    let polygon_count = mesh.polygons.len();
    if radius <= 0.0 || vertex_count == 0 || polygon_count == 0 {
        return ErosionResult {
            vertices: mesh.vertices.clone(),
            polygon_count,
            eroded_count: 0,
            pinch_guard_count: 0,
        };
    }

    // 1. Boundary-edge detection: an edge (unordered vertex-index pair)
    // referenced by exactly one polygon. `BTreeMap` (not `HashMap`) keeps
    // iteration order fixed by key regardless of process hash-seed, so the
    // per-vertex normal accumulation below sums in the same order on every
    // call -- matching this codebase's existing "deterministic across
    // calls" testing convention (see `landmass_graph::door_link_descriptors`
    // and its tests) rather than relying on incidental hash stability.
    let mut edge_polygons: BTreeMap<(u32, u32), Vec<usize>> = BTreeMap::new();
    for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
        for &(a, b) in &[
            (triangle[0], triangle[1]),
            (triangle[1], triangle[2]),
            (triangle[2], triangle[0]),
        ] {
            let key = if a <= b { (a, b) } else { (b, a) };
            edge_polygons.entry(key).or_default().push(poly_index);
        }
    }

    // 2. Per-vertex raw inward offset (horizontal X/Z plane only -- Y is
    // left untouched, see module doc comment).
    let mut accum: Vec<(f32, f32)> = vec![(0.0, 0.0); vertex_count];
    for (&(a, b), polys) in &edge_polygons {
        if polys.len() != 1 {
            continue; // interior edge: shared by two polygons, not a wall.
        }
        let Some(&poly_index) = polys.first() else {
            continue;
        };
        let Some(triangle) = mesh.polygons.get(poly_index) else {
            continue;
        };
        let Some(&third) = triangle.iter().find(|&&v| v != a && v != b) else {
            continue;
        };
        let (Some(&pa), Some(&pb), Some(&pthird)) = (
            mesh.vertices.get(a as usize),
            mesh.vertices.get(b as usize),
            mesh.vertices.get(third as usize),
        ) else {
            continue;
        };
        let ex = pb[0] - pa[0];
        let ez = pb[2] - pa[2];
        // Perpendicular-to-edge candidate; pick whichever of the two signs
        // points toward the triangle's third vertex (i.e. into the
        // polygon's interior, away from the wall).
        let (n1x, n1z) = (-ez, ex);
        let mid_x = (pa[0] + pb[0]) * 0.5;
        let mid_z = (pa[2] + pb[2]) * 0.5;
        let to_third_x = pthird[0] - mid_x;
        let to_third_z = pthird[2] - mid_z;
        let dot = n1x * to_third_x + n1z * to_third_z;
        let (nx, nz) = if dot >= 0.0 { (n1x, n1z) } else { (-n1x, -n1z) };
        let len = (nx * nx + nz * nz).sqrt();
        if len <= f32::EPSILON {
            continue; // degenerate edge (zero length in the X/Z plane).
        }
        let (ux, uz) = (nx / len, nz / len);
        if let Some(entry) = accum.get_mut(a as usize) {
            entry.0 += ux;
            entry.1 += uz;
        }
        if let Some(entry) = accum.get_mut(b as usize) {
            entry.0 += ux;
            entry.1 += uz;
        }
    }

    let mut offsets: Vec<(f32, f32)> = vec![(0.0, 0.0); vertex_count];
    for (index, &(sx, sz)) in accum.iter().enumerate() {
        let len = (sx * sx + sz * sz).sqrt();
        if len > f32::EPSILON {
            offsets[index] = (sx / len * radius, sz / len * radius);
        }
    }

    // 3. Corridor-pinch guard: shrink a polygon's vertex factors whenever
    // its current (factor-scaled) shape would invert or degenerate,
    // deterministically (fixed polygon/vertex order, no randomness).
    let original_areas: Vec<f32> = mesh
        .polygons
        .iter()
        .map(|triangle| {
            let a = mesh.vertices.get(triangle[0] as usize).copied();
            let b = mesh.vertices.get(triangle[1] as usize).copied();
            let c = mesh.vertices.get(triangle[2] as usize).copied();
            match (a, b, c) {
                (Some(a), Some(b), Some(c)) => signed_area_xz(a, b, c),
                _ => 0.0,
            }
        })
        .collect();

    let mut factors = vec![1.0f32; vertex_count];
    let mut pinch_guard_marked = vec![false; polygon_count];

    let eroded_position = |mesh: &ErosionMeshInput,
                           offsets: &[(f32, f32)],
                           factors: &[f32],
                           index: u32|
     -> Option<[f32; 3]> {
        let base = *mesh.vertices.get(index as usize)?;
        let &(ox, oz) = offsets.get(index as usize)?;
        let f = *factors.get(index as usize)?;
        Some([base[0] + ox * f, base[1], base[2] + oz * f])
    };

    for _pass in 0..MAX_SHRINK_PASSES {
        let mut changed = false;
        for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
            let Some(&original_area) = original_areas.get(poly_index) else {
                continue;
            };
            if original_area.abs() < MIN_TRIANGLE_AREA {
                // Already-degenerate input: upstream (`landmass_graph`)
                // should have filtered this before calling erosion; skip
                // defensively rather than divide-by-zero-style reasoning
                // about a shape with no meaningful sign.
                continue;
            }
            let (Some(a), Some(b), Some(c)) = (
                eroded_position(mesh, &offsets, &factors, triangle[0]),
                eroded_position(mesh, &offsets, &factors, triangle[1]),
                eroded_position(mesh, &offsets, &factors, triangle[2]),
            ) else {
                continue;
            };
            let area = signed_area_xz(a, b, c);
            let same_sign = (area > 0.0) == (original_area > 0.0);
            if !same_sign || area.abs() < MIN_TRIANGLE_AREA {
                pinch_guard_marked[poly_index] = true;
                for &v in triangle {
                    if let Some(factor) = factors.get_mut(v as usize) {
                        if *factor > MIN_FACTOR {
                            *factor *= 0.5;
                            changed = true;
                        } else {
                            *factor = 0.0;
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    // 4. Finalize eroded vertex positions.
    let mut eroded_vertices = mesh.vertices.clone();
    for (index, eroded) in eroded_vertices.iter_mut().enumerate() {
        let (ox, oz) = offsets[index];
        let factor = factors[index];
        if ox != 0.0 || oz != 0.0 {
            eroded[0] += ox * factor;
            eroded[2] += oz * factor;
        }
    }

    let mut eroded_count = 0usize;
    let mut pinch_guard_count = 0usize;
    for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
        let moved = triangle.iter().any(|&v| {
            let Some(&(ox, oz)) = offsets.get(v as usize) else {
                return false;
            };
            let Some(&factor) = factors.get(v as usize) else {
                return false;
            };
            (ox * factor).abs() > 1.0e-6 || (oz * factor).abs() > 1.0e-6
        });
        if moved {
            eroded_count += 1;
        }
        if pinch_guard_marked[poly_index] {
            pinch_guard_count += 1;
        }
    }

    ErosionResult {
        vertices: eroded_vertices,
        polygon_count,
        eroded_count,
        pinch_guard_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A single right-angle room corner: wall along X (A-B) and wall along
    /// Z (D-A) meeting at vertex A, split by diagonal A-C so the room is
    /// two triangles. Vertex A is the corner under test; B/C/D make the
    /// room's other three sides boundary too (an isolated room, not a
    /// corridor continuing further -- fine, this test only asserts A's
    /// movement).
    fn corner_room() -> ErosionMeshInput {
        ErosionMeshInput {
            vertices: vec![
                [0.0, 0.0, 0.0], // 0: A, the corner
                [4.0, 0.0, 0.0], // 1: B
                [4.0, 0.0, 4.0], // 2: C
                [0.0, 0.0, 4.0], // 3: D
            ],
            polygons: vec![[0, 1, 2], [0, 2, 3]],
        }
    }

    #[test]
    fn l_corner_vertex_is_pulled_diagonally_off_the_corner() {
        let mesh = corner_room();
        let result = erode(&mesh, AGENT_RADIUS);
        let original = mesh.vertices[0];
        let eroded = result.vertices[0];
        // A sits at the meeting point of the X=0 and Z=0 walls; clearance
        // means moving strictly into the room on both axes.
        assert!(
            eroded[0] > original[0] && eroded[2] > original[2],
            "corner vertex must move into the room on both axes: {eroded:?}"
        );
        // A large room at this radius should not trip the pinch guard, so
        // the corner should get its full-strength diagonal offset (unit
        // normals from the two walls average to length 1 after
        // renormalizing, then scale by radius).
        let dx = eroded[0] - original[0];
        let dz = eroded[2] - original[2];
        let displacement = (dx * dx + dz * dz).sqrt();
        assert!(
            (displacement - AGENT_RADIUS).abs() < 1.0e-4,
            "expected displacement close to the agent radius, got {displacement}"
        );
        assert_eq!(result.pinch_guard_count, 0);
        assert!(result.eroded_count > 0);
    }

    #[test]
    fn interior_only_vertex_never_moves() {
        // The diagonal A-C is an interior edge shared by both triangles;
        // neither of its endpoints is boundary-only, but they are also
        // corners here so they do move via the *other* boundary edges they
        // touch. To isolate a genuinely interior-only vertex, add a fan of
        // four triangles around a single non-boundary centre vertex.
        let mesh = ErosionMeshInput {
            vertices: vec![
                [1.0, 0.0, 0.0],  // 0: centre-adjacent ring vertex (right)
                [0.0, 0.0, 1.0],  // 1: ring (top)
                [-1.0, 0.0, 0.0], // 2: ring (left)
                [0.0, 0.0, -1.0], // 3: ring (bottom)
                [0.0, 0.0, 0.0],  // 4: centre -- touches only interior edges
            ],
            polygons: vec![[4, 0, 1], [4, 1, 2], [4, 2, 3], [4, 3, 0]],
        };
        let result = erode(&mesh, AGENT_RADIUS);
        assert_eq!(result.vertices[4], mesh.vertices[4], "centre must not move");
    }

    #[test]
    fn zero_or_negative_radius_is_a_no_op() {
        let mesh = corner_room();
        for radius in [0.0, -1.0] {
            let result = erode(&mesh, radius);
            assert_eq!(result.vertices, mesh.vertices);
            assert_eq!(result.eroded_count, 0);
            assert_eq!(result.pinch_guard_count, 0);
        }
    }

    #[test]
    fn erosion_is_deterministic_across_calls() {
        let mesh = corner_room();
        let first = erode(&mesh, AGENT_RADIUS);
        let second = erode(&mesh, AGENT_RADIUS);
        assert_eq!(first, second);
    }

    /// A chain of corridor segments along X between two long walls at
    /// Z=0 and Z=width, each segment split by a diagonal. The segment
    /// seams (shared bottom/top vertices between consecutive segments)
    /// are interior edges; only the long walls and the two open ends are
    /// boundary.
    fn corridor(width: f32, segments: u32) -> ErosionMeshInput {
        let mut vertices = Vec::new();
        let mut polygons = Vec::new();
        for i in 0..=segments {
            let x = i as f32 * 2.0;
            vertices.push([x, 0.0, 0.0]); // bottom row, index 2*i
            vertices.push([x, 0.0, width]); // top row, index 2*i + 1
        }
        for i in 0..segments {
            let bottom_a = 2 * i;
            let top_a = 2 * i + 1;
            let bottom_b = 2 * (i + 1);
            let top_b = 2 * (i + 1) + 1;
            polygons.push([bottom_a, bottom_b, top_b]);
            polygons.push([bottom_a, top_b, top_a]);
        }
        ErosionMeshInput { vertices, polygons }
    }

    #[test]
    fn narrow_corridor_engages_the_pinch_guard_and_never_inverts() {
        // Width 0.3 m, radius 0.35 m: full erosion from both walls (0.35 m
        // each) would overlap by 0.4 m, well past inversion.
        let mesh = corridor(0.3, 3);
        let result = erode(&mesh, AGENT_RADIUS);
        assert!(
            result.pinch_guard_count > 0,
            "narrow corridor must engage the pinch guard"
        );
        // Connectivity is untouched by construction (see module doc
        // comment), so the only thing left to prove is that no polygon
        // inverted or degenerated.
        for (poly_index, triangle) in mesh.polygons.iter().enumerate() {
            let original_area = signed_area_xz(
                mesh.vertices[triangle[0] as usize],
                mesh.vertices[triangle[1] as usize],
                mesh.vertices[triangle[2] as usize],
            );
            let eroded_area = signed_area_xz(
                result.vertices[triangle[0] as usize],
                result.vertices[triangle[1] as usize],
                result.vertices[triangle[2] as usize],
            );
            assert!(
                (eroded_area > 0.0) == (original_area > 0.0),
                "polygon {poly_index} inverted after erosion"
            );
            assert!(
                eroded_area.abs() >= MIN_TRIANGLE_AREA,
                "polygon {poly_index} degenerated after erosion"
            );
        }
        // Same vertex count/order, indices unchanged: topology preserved.
        assert_eq!(result.vertices.len(), mesh.vertices.len());
        assert_eq!(result.polygon_count, mesh.polygons.len());
    }

    #[test]
    fn wide_corridor_erodes_fully_without_the_pinch_guard() {
        // Width 3.0 m is far wider than 2 * radius: both walls should
        // erode at full strength with no fallback needed.
        let mesh = corridor(3.0, 2);
        let result = erode(&mesh, AGENT_RADIUS);
        assert_eq!(result.pinch_guard_count, 0);
        assert!(result.eroded_count > 0);
        for (index, vertex) in mesh.vertices.iter().enumerate() {
            let eroded = result.vertices[index];
            let is_bottom = vertex[2] == 0.0;
            let is_top = vertex[2] == 3.0;
            if is_bottom {
                assert!(
                    eroded[2] > vertex[2],
                    "bottom-wall vertex {index} must move inward"
                );
            }
            if is_top {
                assert!(
                    eroded[2] < vertex[2],
                    "top-wall vertex {index} must move inward"
                );
            }
        }
    }
}
