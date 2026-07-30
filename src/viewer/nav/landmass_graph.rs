//! Pure `bevy_landmass` conversion (issue #112, M4 wave 3 spike): prepared
//! per-cell nav-graph data (issue #111's `PreparedNavGraph`) -> a validated
//! `bevy_landmass` 3D navigation mesh, plus a deterministic door-link
//! descriptor list and a `landmass` agent-state -> project-native status
//! mapping.
//!
//! Deliberately decoupled from `vsa::prepare::nav_graph::PreparedNavGraph`:
//! this file is included verbatim by `tests/features.rs` via `#[path]` (see
//! that file's module doc comment -- `src/lib.rs` keeps `vsa`/`viewer`
//! private, so the integration-test binary cannot `use bevyout::viewer::...`
//! and instead compiles the exact same source in via `#[path]`), and that
//! `#[path]` inclusion has no access to the `vsa` module tree from this
//! file's real directory depth (`src/viewer/nav/`, three levels from crate
//! root, versus `vsa::prepare::nav_graph`'s two -- there is no shared
//! relative-`super::` path that resolves in both trees, unlike
//! `vsa::prepare::nav_graph` itself reusing `vsa::paths` via `super::super`).
//! So this module takes its own small plain-data `MeshInput`/`PolygonInput`/
//! `DoorInput` shape instead of `PreparedNavMesh`/`PreparedNavPolygon`/
//! `PreparedNavDoor` directly; `nav/mod.rs` (Bevy-visible, not
//! cucumber-included) does the trivial field-for-field boundary conversion,
//! exactly the shape `vsa::prepare::nav_graph.rs` itself uses to decouple
//! from `openmw_esm4`'s raw record types.
//!
//! No `bevy` (full engine) import: `bevy_landmass`/`glam` types are used
//! directly instead, per the wave plan's explicit allowance ("Add `landmass`
//! directly only if the pure conversion module needs types `bevy_landmass`
//! does not re-export"). In practice this module needs `bevy_landmass`
//! itself, not the plain `landmass` crate: `bevy_landmass::NavigationMesh3d`
//! (a `landmass::NavigationMesh<bevy_landmass::coords::ThreeD>` alias) is
//! the type the runtime side (`nav/agent.rs`) needs back out of this
//! conversion to build an `Archipelago3d`/`Island3dBundle`, and there is no
//! way to convert a `landmass::NavigationMesh<MyOwnCoordinateSystem>` into
//! that type after the fact -- `ThreeD` is a concrete marker type, not just
//! a shape. `bevy_landmass`'s `coords`/`nav_mesh`/`agent` split keeps this
//! import free of any ECS/`Component`/`System`/`World` type, so it stays
//! pure data-in-data-out and testable without spinning up a `World`.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use bevy_landmass::{AgentState, NavigationMesh3d, ValidNavigationMesh3d, ValidationError};
use glam::Vec3;

// ---------------------------------------------------------------------
// Conversion inputs (boundary conversion from `PreparedNavGraph` happens in
// `nav/mod.rs`)
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct MeshInput {
    pub(crate) form_id: u32,
    /// Bevy metres (already converted at prepare time by #111's
    /// `to_bevy_position`); no further coordinate conversion happens here.
    pub(crate) vertices: Vec<[f32; 3]>,
    pub(crate) polygons: Vec<PolygonInput>,
    pub(crate) doors: Vec<DoorInput>,
    /// Issue #177: collision-derived blocker -> polygon associations
    /// (`PreparedNavMesh::derived_doors`), deliberately kept out of `doors`.
    /// The authored list's per-door triangle *count* is load bearing
    /// (`door_link_descriptors` treats exactly two as a two-sided link,
    /// `single_sided_doors` exactly one as a travel/crossing candidate), so
    /// folding derived associations in would silently reclassify authored
    /// doors -- a travel door with one authored triangle plus three derived
    /// ones would stop being a travel door.
    pub(crate) derived_doors: Vec<DerivedDoorInput>,
}

/// One derived blocker -> polygon association (issue #177). Unlike
/// [`DoorInput`] the FormID is always present: a derived association exists
/// only because a blocking placement produced it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DerivedDoorInput {
    pub(crate) triangle_index: u32,
    pub(crate) door_reference_form_id: u32,
    /// `true` when the polygon lies wholly inside the blocker's collision
    /// volume, so it must not be freely traversable while the blocker is
    /// closed. `false` is the doorway *crossing* itself, which stays
    /// ordinary walkable ground so the runtime crossing gate can fire on it.
    pub(crate) blocks_when_closed: bool,
    /// Whether this blocker owns a runtime open/close FSM. Decides the
    /// closed-state cost rather than the classification -- see
    /// `nav/agent.rs`'s `CLOSED_DOOR_TYPE_INDEX_COST`.
    pub(crate) openable: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct PolygonInput {
    pub(crate) index: u32,
    pub(crate) vertex_indices: [u32; 3],
    pub(crate) is_water: bool,
    /// `NVTR`'s `PREFERRED_PATHING` flag (issue #156 feature 1): resolved to
    /// a cheaper-cost landmass polygon type by `resolve_polygon_type_index`
    /// -- see that function's doc comment for the precedence rule when a
    /// triangle is both a door and preferred-pathing.
    pub(crate) is_preferred_pathing: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DoorInput {
    pub(crate) triangle_index: u32,
    pub(crate) door_reference_form_id: Option<u32>,
}

// ---------------------------------------------------------------------
// Conversion diagnostics
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConversionDiagnostic {
    pub(crate) severity: Severity,
    pub(crate) message: String,
}

impl ConversionDiagnostic {
    fn warning(message: String) -> Self {
        Self {
            severity: Severity::Warning,
            message,
        }
    }

    fn error(message: String) -> Self {
        Self {
            severity: Severity::Error,
            message,
        }
    }
}

/// Result of converting one [`MeshInput`] into a validated landmass mesh.
/// `nav_mesh` is `None` when every polygon was excluded (all water, all
/// invalid/degenerate, or validation failed both winding attempts) --
/// `nav/agent.rs` simply skips spawning an island for that mesh rather than
/// treating it as fatal, mirroring `nav_overlay.rs`'s empty-mesh skip.
#[derive(Debug, Default)]
pub(crate) struct BuildResult {
    pub(crate) nav_mesh: Option<ValidNavigationMesh3d>,
    pub(crate) diagnostics: Vec<ConversionDiagnostic>,
}

/// Global door FormID -> `landmass` polygon type index (issue #155 feature
/// 1), computed once across every mesh in one archipelago build (not
/// per-mesh): `override_type_index_cost`/`AgentTypeIndexCostOverrides`
/// (`nav/agent.rs`) key by type index against the whole archipelago's
/// shared `NavigationData`, not per-island, so the *same* door referenced
/// from two meshes (a two-sided `DoorLinkDescriptor`'s `side_a`/`side_b`,
/// or the rarer case of one door's triangle appearing in more than one
/// mesh) must resolve to the *same* type index everywhere or locking it
/// would only exclude one side. Type index `0` is left for ordinary
/// walkable ground (every polygon's default, unchanged) since
/// `landmass::pathfinding`'s `type_index_to_cost` already treats an
/// unlisted index as cost `1.0` -- doors get `1..`, assigned in ascending
/// door-FormID order so the mapping is deterministic across calls on the
/// same graph (the same determinism rule `door_link_descriptors`/
/// `single_sided_doors` already follow).
pub(crate) fn door_type_indices(meshes: &[MeshInput]) -> BTreeMap<u32, usize> {
    let mut door_form_ids: BTreeSet<u32> = BTreeSet::new();
    for mesh in meshes {
        for door in &mesh.doors {
            if let Some(door_form_id) = door.door_reference_form_id {
                door_form_ids.insert(door_form_id);
            }
        }
        // Issue #177: a derived-only blocker (an ordinary in-cell door the
        // authored `NVDP` never associated with a triangle) needs a type
        // index for exactly the same reason an authored one does -- it is
        // the identity `apply_door_lock_overrides` prices.
        for door in &mesh.derived_doors {
            door_form_ids.insert(door.door_reference_form_id);
        }
    }
    door_form_ids
        .into_iter()
        .enumerate()
        .map(|(offset, door_form_id)| (door_form_id, offset + 1))
        .collect()
}

/// Every blocker FormID whose derived associations report it as openable
/// (`DerivedDoorInput::openable`) -- a real `DOOR` record with a runtime
/// open/close FSM. `nav/agent.rs` prices a *closed* openable blocker's
/// interior expensive-but-passable so the solver still routes through the
/// doorway and the crossing gate can open it, and a non-openable one
/// impassable.
pub(crate) fn openable_blockers(meshes: &[MeshInput]) -> BTreeSet<u32> {
    let mut form_ids = BTreeSet::new();
    for mesh in meshes {
        for door in &mesh.derived_doors {
            if door.openable {
                form_ids.insert(door.door_reference_form_id);
            }
        }
    }
    form_ids
}

/// Global blocker FormID -> `landmass` polygon type index for the *blocking*
/// derived association class (issue #177): the polygons that lie wholly
/// inside a blocker's collision volume. They need their own type index,
/// separate from [`door_type_indices`]'s, because the two classes are priced
/// on different state: a door's ordinary (gate) polygons are impassable only
/// while it is *locked*, while a polygon inside the solid slab is impassable
/// whenever the door is *closed*, lock or no lock. `landmass` stores exactly
/// one `type_index` per polygon, so one door needing both rules needs two
/// indices.
///
/// Allocated above every index [`door_type_indices`] assigned, in ascending
/// FormID order, so the two maps can never collide and both are deterministic
/// across calls on the same graph.
pub(crate) fn closed_door_type_indices(
    meshes: &[MeshInput],
    door_type_indices: &BTreeMap<u32, usize>,
) -> BTreeMap<u32, usize> {
    let mut form_ids: BTreeSet<u32> = BTreeSet::new();
    for mesh in meshes {
        for door in &mesh.derived_doors {
            if door.blocks_when_closed {
                form_ids.insert(door.door_reference_form_id);
            }
        }
    }
    let base = door_type_indices.values().max().copied().unwrap_or(0) + 1;
    form_ids
        .into_iter()
        .enumerate()
        .map(|(offset, form_id)| (form_id, base + offset))
        .collect()
}

/// The single landmass polygon type index every preferred-pathing triangle
/// (issue #156 feature 1, `NVTR`'s `PREFERRED_PATHING` flag) that is not
/// itself a door triangle resolves to: one past the highest door type index
/// this archipelago-wide `door_type_indices` map assigned (issue #155
/// feature 1), so it can never collide with a door's own type index
/// regardless of how many distinct doors this cell's meshes reference.
/// Deterministic: depends only on `door_type_indices`'s own contents, and
/// that map is itself computed deterministically once per archipelago build
/// (`door_type_indices`'s own doc comment) and passed unchanged to every
/// mesh's `build_navigation_mesh` call, so every mesh in one build agrees on
/// the same preferred-pathing index. `1` when there are no doors at all
/// (the type-index space starts fresh at `1`, same as `door_type_indices`
/// itself).
pub(crate) fn preferred_pathing_type_index(
    door_type_indices: &BTreeMap<u32, usize>,
    closed_door_type_indices: &BTreeMap<u32, usize>,
) -> usize {
    door_type_indices
        .values()
        .chain(closed_door_type_indices.values())
        .max()
        .copied()
        .unwrap_or(0)
        + 1
}

/// Resolves one triangle's landmass polygon type index (issue #156 feature
/// 1 / issue #155 feature 1's coexistence rule): a door-typed triangle
/// always wins over a preferred-pathing triangle when a triangle is both,
/// falling back to ordinary walkable ground (`0`) when neither applies.
///
/// landmass 0.9.2 stores exactly one `type_index` per polygon
/// (`ValidPolygon::type_index`, fed by `NavigationMesh::polygon_type_indices`
/// -- see `build_navigation_mesh`'s doc comment) -- there is no way to stack
/// a door's lockable type and a preferred-path's cheaper-cost type on the
/// same triangle, so precedence has to pick one. Door wins: a locked door
/// must always block the route through that exact triangle regardless of
/// its preferred-pathing flag (a preferred-pathing type's whole purpose is a
/// *cheaper*, not an *infinite*, cost -- letting it silently override the
/// door's lockable type would reopen a "locked" doorway), and real FO3 NAVM
/// data already treats a door triangle as special-purposed distinctly from
/// ordinary preferred-path corridor filler.
fn resolve_polygon_type_index(
    polygon_index: u32,
    is_preferred_pathing: bool,
    door_type_index_by_triangle: &HashMap<u32, usize>,
    preferred_pathing_type_index: usize,
) -> usize {
    if let Some(&type_index) = door_type_index_by_triangle.get(&polygon_index) {
        return type_index;
    }
    if is_preferred_pathing {
        return preferred_pathing_type_index;
    }
    0
}

/// Converts `mesh` into a validated `bevy_landmass` navigation mesh.
///
/// Non-walkable exclusion: `is_water` polygons are dropped rather than
/// mapped to a distinct type index. NAVM triangle flags decoded by #111
/// carry no separate "disabled"/ground bit at this layer (only
/// `is_water`/`is_preferred_pathing`/`contains_door` are extracted) -- water
/// is the only flag clearly meaning "a ground-walking humanoid should not
/// path across this triangle" in Fallout 3, so it stands in for
/// "non-walkable" for this spike's polygon-type-index question. A distinct
/// type index (rather than exclusion) would let a future wave give water a
/// non-infinite cost for wading, which this spike does not need.
///
/// Winding: `bevy_landmass`'s `ThreeD` coordinate system always reverses
/// polygon vertex order internally before storing (`FLIP_POLYGONS = true`,
/// compensating for the chirality difference between Bevy's Y-up and
/// landmass's internal Z-up "standard" coordinates) -- so this function does
/// not need to guess a fixed convention. It tries the polygon order exactly
/// as authored by #111's `PreparedNavPolygon::vertex_indices` first; if
/// `validate()` rejects it (concave/clockwise), it retries once with every
/// included polygon's vertex order reversed, and only then gives up. This
/// keeps both real Fallout NAVM winding conventions supported without
/// requiring advance knowledge of which one the source data uses, fixed
/// once per mesh (a mix of the two within one mesh would corrupt
/// `validate()`'s doubly-connected-edge adjacency detection).
///
/// Clearance (issue #153, M4 wave 10): agent-radius clearance and
/// collision-derived validation now run prepare-side in
/// `vsa::prepare::nav_clearance` and are baked into `mesh.vertices` (offset
/// positions, seam/door edges pinned) and a per-polygon `walkable` flag
/// `viewer::nav::mesh_inputs` filters on before this function ever sees the
/// mesh. The interim runtime erosion pass this used to call is retired, so
/// vertices are taken as-is here. `_merges` is retained on the signature for
/// call-site compatibility but is no longer consumed (the seam/door
/// protected-edge computation it fed moved to `nav_clearance`).
///
/// `door_type_indices` (issue #155 feature 1): every door-associated
/// triangle (`mesh.doors`) gets the polygon type index its door FormID
/// resolves to in this map, instead of the flat `0` every polygon used
/// before. `is_preferred_pathing` (issue #156 feature 1): every non-door
/// preferred-pathing triangle gets the single shared
/// `preferred_pathing_type_index` -- everything else stays type `0`. A
/// triangle that is both resolves via `resolve_polygon_type_index`'s
/// documented precedence (door wins). This only changes which
/// `landmass::pathfinding` cost a polygon looks up (`type_index_to_cost`,
/// overridden per agent by `nav/agent.rs`'s `AgentTypeIndexCostOverrides`
/// when a door is locked, or -- for the preferred-pathing type, issue #156 --
/// by a base `Archipelago::set_type_index_cost` a future wave wires up on
/// the archipelago itself); it does not remove the polygon or touch its
/// vertices/winding, so a typed triangle stays exactly as connected to its
/// neighbours as before -- confirmed against `landmass` 0.9.2's own
/// `NavigationMesh::validate()` (`nav_mesh.rs`): region/adjacency
/// (`DisjointSet`/`connectivity`) is computed purely from shared vertex
/// indices between polygons, and `polygon_type_indices[i]` only ever feeds
/// `ValidPolygon::type_index`, read solely by the cost lookup in
/// `pathfinding.rs`. The live-Archipelago tests that actually run a solve
/// against a typed mesh to confirm this live in `agent.rs`'s own test
/// module instead of here, since this module stays Bevy-engine-free (no
/// `bevy::app`/`Landmass3dPlugin` -- see the module doc comment).
pub(crate) fn build_navigation_mesh(
    mesh: &MeshInput,
    _merges: &[MergeInput],
    door_type_indices: &BTreeMap<u32, usize>,
    closed_door_type_indices: &BTreeMap<u32, usize>,
) -> BuildResult {
    let mut diagnostics = Vec::new();
    let vertex_count = mesh.vertices.len();

    // Issue #155 feature 1: triangle index -> door FormID, resolved to this
    // build's polygon type index (falling back to `0`, ordinary walkable
    // ground, for any triangle not in `mesh.doors` or whose FormID this
    // archipelago-wide map has no entry for -- e.g. a door with no
    // `door_reference_form_id` decoded).
    let mut door_type_index_by_triangle: HashMap<u32, usize> = HashMap::new();
    for door in &mesh.doors {
        let Some(door_form_id) = door.door_reference_form_id else {
            continue;
        };
        if let Some(&type_index) = door_type_indices.get(&door_form_id) {
            door_type_index_by_triangle.insert(door.triangle_index, type_index);
        }
    }
    // Issue #177: derived associations type the same way authored ones do,
    // with the blocking class taking its own index (see
    // `closed_door_type_indices`). A derived association is applied after the
    // authored ones so a blocking classification wins over a plain gate
    // classification on the same triangle -- the stricter rule must not be
    // silently downgraded.
    for door in &mesh.derived_doors {
        let index = if door.blocks_when_closed {
            closed_door_type_indices.get(&door.door_reference_form_id)
        } else {
            door_type_indices.get(&door.door_reference_form_id)
        };
        if let Some(&type_index) = index {
            let slot = door_type_index_by_triangle.entry(door.triangle_index);
            match slot {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    if door.blocks_when_closed {
                        occupied.insert(type_index);
                    }
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(type_index);
                }
            }
        }
    }
    // Issue #156 feature 1: computed once from the same archipelago-wide
    // `door_type_indices` map every mesh in this build shares, so every
    // mesh agrees on the same preferred-pathing index (see
    // `preferred_pathing_type_index`'s doc comment).
    let preferred_pathing_index =
        preferred_pathing_type_index(door_type_indices, closed_door_type_indices);

    let mut included_polygons: Vec<&PolygonInput> = Vec::new();
    let mut included_type_indices: Vec<usize> = Vec::new();
    for polygon in &mesh.polygons {
        if polygon.is_water {
            continue;
        }
        let invalid = polygon
            .vertex_indices
            .iter()
            .any(|&index| index as usize >= vertex_count);
        if invalid {
            diagnostics.push(ConversionDiagnostic::error(format!(
                "mesh {:08x} polygon {}: invalid vertex index, excluded from navigation mesh",
                mesh.form_id, polygon.index
            )));
            continue;
        }
        let [a, b, c] = polygon.vertex_indices;
        if a == b || b == c || a == c {
            diagnostics.push(ConversionDiagnostic::warning(format!(
                "mesh {:08x} polygon {}: degenerate triangle, excluded from navigation mesh",
                mesh.form_id, polygon.index
            )));
            continue;
        }
        included_type_indices.push(resolve_polygon_type_index(
            polygon.index,
            polygon.is_preferred_pathing,
            &door_type_index_by_triangle,
            preferred_pathing_index,
        ));
        included_polygons.push(polygon);
    }

    if included_polygons.is_empty() {
        diagnostics.push(ConversionDiagnostic::warning(format!(
            "mesh {:08x}: no walkable polygons after conversion",
            mesh.form_id
        )));
        return BuildResult {
            nav_mesh: None,
            diagnostics,
        };
    }

    // Issue #153 (M4 wave 10): agent-radius clearance (with collision-derived
    // validation) is now applied prepare-side in `vsa::prepare::nav_clearance`
    // and baked into `mesh.vertices` (offset positions) + a per-polygon
    // `walkable` flag that `viewer::nav::mesh_inputs` has already filtered on.
    // The runtime erosion pass this used to call is retired, so vertices are
    // taken as-is.
    let vertices: Vec<Vec3> = mesh
        .vertices
        .iter()
        .map(|v| Vec3::new(v[0], v[1], v[2]))
        .collect();

    let mut last_error: Option<ValidationError> = None;
    for reversed in [false, true] {
        let polygons: Vec<Vec<usize>> = included_polygons
            .iter()
            .map(|polygon| {
                let mut indices: Vec<usize> = polygon
                    .vertex_indices
                    .iter()
                    .map(|&index| index as usize)
                    .collect();
                if reversed {
                    indices.reverse();
                }
                indices
            })
            .collect();
        // Issue #155 feature 1: `included_type_indices` was built in the
        // same order/filter as `included_polygons` above, so it lines up
        // 1:1 with `polygons` here regardless of winding (reversal only
        // touches vertex order within a polygon, never polygon order).
        let candidate = NavigationMesh3d {
            vertices: vertices.clone(),
            polygons,
            polygon_type_indices: included_type_indices.clone(),
            height_mesh: None,
        };
        match candidate.validate() {
            Ok(valid) => {
                if reversed {
                    diagnostics.push(ConversionDiagnostic::warning(format!(
                        "mesh {:08x}: source polygon winding required reversal to validate",
                        mesh.form_id
                    )));
                }
                return BuildResult {
                    nav_mesh: Some(valid),
                    diagnostics,
                };
            }
            Err(error) => last_error = Some(error),
        }
    }

    diagnostics.push(ConversionDiagnostic::error(format!(
        "mesh {:08x}: landmass validation failed with both polygon windings: {}",
        mesh.form_id,
        last_error.expect("both winding attempts ran and recorded an error"),
    )));
    BuildResult {
        nav_mesh: None,
        diagnostics,
    }
}

// ---------------------------------------------------------------------
// Door-link descriptors
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DoorLinkSide {
    pub(crate) mesh_form_id: u32,
    pub(crate) polygon_index: u32,
    /// Centroid of the door triangle, standing in for an exact edge crossing
    /// point -- #111's `PreparedNavDoor` carries only a triangle index, not
    /// which edge of that triangle faces the door, so there is no exact
    /// edge-crossing geometry to derive here. #113 can refine this once a
    /// per-edge door association exists.
    pub(crate) midpoint: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DoorLinkDescriptor {
    pub(crate) door_form_id: u32,
    pub(crate) side_a: DoorLinkSide,
    pub(crate) side_b: DoorLinkSide,
}

/// The polygon's own three vertex positions, in `vertex_indices` order.
/// `None` if any index is out of bounds (same defensive check
/// `build_navigation_mesh` applies before excluding an invalid polygon).
fn polygon_vertices(mesh: &MeshInput, polygon: &PolygonInput) -> Option<[[f32; 3]; 3]> {
    if polygon
        .vertex_indices
        .iter()
        .any(|&index| index as usize >= mesh.vertices.len())
    {
        return None;
    }
    Some(
        polygon
            .vertex_indices
            .map(|index| mesh.vertices[index as usize]),
    )
}

fn polygon_centroid(mesh: &MeshInput, polygon: &PolygonInput) -> Option<[f32; 3]> {
    let vertices = polygon_vertices(mesh, polygon)?;
    let mut sum = [0.0f32; 3];
    for vertex in vertices {
        sum[0] += vertex[0];
        sum[1] += vertex[1];
        sum[2] += vertex[2];
    }
    Some([sum[0] / 3.0, sum[1] / 3.0, sum[2] / 3.0])
}

/// Builds `(door_form_id, side, vertices)` for every door-triangle
/// association across `meshes`, ordered by `(door_form_id, mesh_form_id,
/// polygon_index)` -- the shared grouping key `door_link_descriptors` and
/// `single_sided_doors` both partition, so a door's real FO3 triangle count
/// (usually exactly 1; see wave 3's finding that every real door triangle
/// is single-sided) only needs computing once. `vertices` (issue #155
/// feature 3) is the door triangle's own three vertex positions --
/// `single_sided_doors` carries it forward on `SingleSidedDoor` for the
/// corridor-based mid-route crossing gate's point-in-triangle test;
/// `door_link_descriptors` ignores it (`DoorLinkSide`'s `midpoint` is all a
/// two-sided animation-link endpoint ever needed).
fn door_sides(meshes: &[MeshInput]) -> Vec<(u32, DoorLinkSide, [[f32; 3]; 3])> {
    let mut sides: Vec<(u32, DoorLinkSide, [[f32; 3]; 3])> = Vec::new();
    for mesh in meshes {
        for door in &mesh.doors {
            let Some(door_form_id) = door.door_reference_form_id else {
                continue;
            };
            let Some(polygon) = mesh
                .polygons
                .iter()
                .find(|polygon| polygon.index == door.triangle_index)
            else {
                continue;
            };
            let Some(vertices) = polygon_vertices(mesh, polygon) else {
                continue;
            };
            let Some(midpoint) = polygon_centroid(mesh, polygon) else {
                continue;
            };
            sides.push((
                door_form_id,
                DoorLinkSide {
                    mesh_form_id: mesh.form_id,
                    polygon_index: polygon.index,
                    midpoint,
                },
                vertices,
            ));
        }
    }
    sides.sort_by_key(|(door_form_id, side, _)| {
        (*door_form_id, side.mesh_form_id, side.polygon_index)
    });
    sides
}

/// Deterministic door-link descriptor list (issue #112 feature 4): a door
/// FormID becomes a descriptor only when it is associated with *exactly*
/// two triangles across the graph's meshes -- "both sides resolve to
/// polygons in the loaded cell's graph" per the wave plan. A door
/// associated with zero, one, or more than two triangles (a travel door
/// touching only this cell's side, a decode oddity) is skipped rather than
/// guessed at. Ordered by `(door_form_id, mesh_form_id, polygon_index)` so
/// repeated calls on the same graph produce byte-identical output.
pub(crate) fn door_link_descriptors(meshes: &[MeshInput]) -> Vec<DoorLinkDescriptor> {
    let sides = door_sides(meshes);
    let mut descriptors = Vec::new();
    let mut index = 0;
    while index < sides.len() {
        let door_form_id = sides[index].0;
        let mut group_end = index + 1;
        while group_end < sides.len() && sides[group_end].0 == door_form_id {
            group_end += 1;
        }
        if group_end - index == 2 {
            descriptors.push(DoorLinkDescriptor {
                door_form_id,
                side_a: sides[index].1,
                side_b: sides[index + 1].1,
            });
        }
        index = group_end;
    }
    descriptors
}

/// One door associated with exactly one triangle across the graph's meshes
/// -- the shape every real FO3 door triangle takes (wave 3's finding).
/// Distinct from `door_link_descriptors`'s two-sided group: `nav/agent.rs`
/// treats a single-sided door as a travel-door candidate when the manifest
/// resolves it to a destination cell (issue #113 feature 3).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SingleSidedDoor {
    pub(crate) door_form_id: u32,
    pub(crate) side: DoorLinkSide,
    /// The door triangle's own three vertex positions (issue #155 feature
    /// 3): `nav/agent.rs`'s corridor-based mid-route crossing gate needs
    /// the actual polygon footprint for a point-in-triangle containment
    /// test (`point_in_door_triangle`), not just `side.midpoint`'s
    /// centroid -- issue #137's proximity scan this replaces only ever
    /// needed the midpoint.
    pub(crate) vertices: [[f32; 3]; 3],
}

/// Deterministic single-sided-door list, ordered by `(door_form_id,
/// mesh_form_id, polygon_index)`. A door with more than one triangle
/// association is not single-sided and is left to `door_link_descriptors`
/// (exactly two) or skipped entirely (three or more, a decode oddity).
pub(crate) fn single_sided_doors(meshes: &[MeshInput]) -> Vec<SingleSidedDoor> {
    let sides = door_sides(meshes);
    let mut result = Vec::new();
    let mut index = 0;
    while index < sides.len() {
        let door_form_id = sides[index].0;
        let mut group_end = index + 1;
        while group_end < sides.len() && sides[group_end].0 == door_form_id {
            group_end += 1;
        }
        if group_end - index == 1 {
            result.push(SingleSidedDoor {
                door_form_id,
                side: sides[index].1,
                vertices: sides[index].2,
            });
        }
        index = group_end;
    }
    result
}

/// Deterministic mid-route crossing-gate candidates recovered from the
/// *derived* associations (issue #177), ordered by `(door_form_id,
/// mesh_form_id, polygon_index)`.
///
/// Only the gate class (`!blocks_when_closed`) is returned: a polygon wholly
/// inside a closed blocker's solid is priced impassable instead
/// (`closed_door_type_indices`), and gating on it would ask an agent to stop
/// inside a wall. Unlike `single_sided_doors` this places no cardinality
/// restriction on a door's association count -- a derived doorway routinely
/// overlaps several post-clip sub-polygons (issue #171 re-triangulates), and
/// every one of them is a legitimate place for the crossing gate to fire.
/// Authored associations are untouched here: they keep flowing through
/// `door_link_descriptors`/`single_sided_doors` exactly as before, so a door
/// with both authored and derived associations cannot be reclassified.
pub(crate) fn derived_door_gates(meshes: &[MeshInput]) -> Vec<SingleSidedDoor> {
    let mut gates = Vec::new();
    for mesh in meshes {
        for door in &mesh.derived_doors {
            if door.blocks_when_closed {
                continue;
            }
            let Some(polygon) = mesh
                .polygons
                .iter()
                .find(|polygon| polygon.index == door.triangle_index)
            else {
                continue;
            };
            let (Some(vertices), Some(midpoint)) = (
                polygon_vertices(mesh, polygon),
                polygon_centroid(mesh, polygon),
            ) else {
                continue;
            };
            gates.push(SingleSidedDoor {
                door_form_id: door.door_reference_form_id,
                side: DoorLinkSide {
                    mesh_form_id: mesh.form_id,
                    polygon_index: polygon.index,
                    midpoint,
                },
                vertices,
            });
        }
    }
    gates.sort_by_key(|gate| {
        (
            gate.door_form_id,
            gate.side.mesh_form_id,
            gate.side.polygon_index,
        )
    });
    gates
}

// ---------------------------------------------------------------------
// Corridor-based mid-route door crossing gate (issue #155 feature 3)
// ---------------------------------------------------------------------

/// Whether `point` (an agent's current world position) lies within
/// `triangle`'s horizontal (XZ) footprint, within `max_vertical_gap` of it
/// vertically -- the corridor-based mid-route door crossing gate `nav/
/// agent.rs`'s `drive_door_link_for_agent` feeds this with a `SingleSidedDoor
/// ::vertices` candidate and the agent's own `Transform::translation` each
/// tick, replacing issue #137's `MID_ROUTE_DOOR_GATE_DISTANCE`
/// centroid-proximity scan (a route merely passing *near* a doorway used to
/// gate even when its corridor never actually crossed it -- see this
/// module's `door_type_indices` doc comment's sibling problem for locking).
///
/// `triangle` should be the door polygon's exact door-footprint vertices:
/// the prepare-side clearance pass (`vsa::prepare::nav_clearance`) pins
/// seam/door triangle vertices in place rather than offsetting them by the
/// agent radius like every other walkable polygon, so this is always the
/// authored NAVM footprint, not a shrunk approximation -- an agent's capsule
/// *centre* threading through a real doorway gap will cross exactly this
/// triangle.
///
/// The vertical guard mirrors `movement_policy::nav_point_reached`'s
/// same-XZ-different-floor rejection (a horizontal-only test alone cannot
/// tell a door triangle from an unrelated one stacked directly above/below
/// it on another storey) without importing that module -- this one stays
/// Bevy-engine-free (see the module doc comment), the same small
/// duplication precedent as `MERGE_PORTAL_STEP_HEIGHT`.
pub(crate) fn point_in_door_triangle(
    point: [f32; 3],
    triangle: [[f32; 3]; 3],
    max_vertical_gap: f32,
) -> bool {
    let vertical_ok = triangle
        .iter()
        .any(|vertex| (point[1] - vertex[1]).abs() <= max_vertical_gap);
    vertical_ok && point_in_triangle_xz(point, triangle)
}

/// XZ distance from `point` to `triangle`'s footprint -- `0.0` when the point
/// is inside it, otherwise the distance to the nearest edge. `None` when the
/// same vertical guard [`point_in_door_triangle`] applies rejects the pair
/// (a triangle stacked on another storey).
///
/// Issue #177: `nav/agent.rs`'s stalled-approach crossing gate
/// (`door_link::approach_gate`) needs "how far short of this doorway did the
/// agent stop", which containment alone cannot express -- containment is a
/// yes/no that a stalled agent short of the crossing answers "no" forever.
pub(crate) fn distance_to_door_triangle(
    point: [f32; 3],
    triangle: [[f32; 3]; 3],
    max_vertical_gap: f32,
) -> Option<f32> {
    let vertical_ok = triangle
        .iter()
        .any(|vertex| (point[1] - vertex[1]).abs() <= max_vertical_gap);
    if !vertical_ok {
        return None;
    }
    if point_in_triangle_xz(point, triangle) {
        return Some(0.0);
    }
    let mut nearest = f32::INFINITY;
    for index in 0..3 {
        let a = triangle[index];
        let b = triangle[(index + 1) % 3];
        nearest = nearest.min(distance_to_segment_xz(point, a, b));
    }
    Some(nearest)
}

fn distance_to_segment_xz(point: [f32; 3], a: [f32; 3], b: [f32; 3]) -> f32 {
    let (px, pz) = (point[0], point[2]);
    let (ax, az) = (a[0], a[2]);
    let (bx, bz) = (b[0], b[2]);
    let (dx, dz) = (bx - ax, bz - az);
    let length_squared = dx * dx + dz * dz;
    let t = if length_squared <= f32::EPSILON {
        0.0
    } else {
        (((px - ax) * dx + (pz - az) * dz) / length_squared).clamp(0.0, 1.0)
    };
    let (cx, cz) = (ax + dx * t, az + dz * t);
    ((px - cx).powi(2) + (pz - cz).powi(2)).sqrt()
}

/// Point-in-triangle containment, projected onto the horizontal (XZ) plane --
/// Y (height) is ignored entirely here; [`point_in_door_triangle`] applies the
/// separate vertical-gap guard first. Winding-independent.
///
/// Issue #189 feature 4: this defers to the one shared containment primitive
/// (`bevyout_core::geometry`) rather than carrying its own exact-zero copy,
/// which was one of four hand-rolled point-in-polygon tests with three
/// different tolerances. See that module for the epsilon rationale.
fn point_in_triangle_xz(point: [f32; 3], triangle: [[f32; 3]; 3]) -> bool {
    bevyout_core::geometry::point_in_triangle_xz(point, triangle)
}

// ---------------------------------------------------------------------
// Same-cell cross-mesh merge links (issue #113, M4 wave 4 feature 2)
// ---------------------------------------------------------------------

/// One same-cell cross-mesh connection (boundary conversion from
/// `vsa::prepare::nav_graph::PreparedNavMeshMerge`, done in `nav/mod.rs`
/// per this module's usual `vsa`-free boundary-conversion split).
/// `interval_a`/`interval_b` are issue #154's validated, positionally-
/// corresponding portal-interval endpoints (already clamped to the two
/// edges' geometric overlap, prepare-side) -- `merge_link_descriptors`
/// below links their midpoints rather than triangle centroids.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MergeInput {
    pub(crate) mesh_a_form_id: u32,
    pub(crate) triangle_a: u32,
    pub(crate) mesh_b_form_id: u32,
    pub(crate) triangle_b: u32,
    pub(crate) interval_a: [[f32; 3]; 2],
    pub(crate) interval_b: [[f32; 3]; 2],
}

/// A resolved cross-mesh merge link, the same two-sided shape as
/// [`DoorLinkDescriptor`] minus the door FormID (a merge link is always
/// open -- there is no door to activate). `distance` is the straight-line
/// traversal distance between `side_a.midpoint` and `side_b.midpoint`
/// (issue #154 feature 3): the animation-link cost `nav/agent.rs` spawns
/// the link with, in place of the previous flat `1.0` every link (door or
/// merge) used.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MergeLinkDescriptor {
    pub(crate) side_a: DoorLinkSide,
    pub(crate) side_b: DoorLinkSide,
    pub(crate) distance: f32,
}

/// The `landmass` animation-link "kind" a validated merge-link candidate at
/// position `validated_index` in this build's own deterministic
/// `merge_link_descriptors` order gets (issue #162 feature 1): kind `0` is
/// reserved for every ordinary/door animation link (`spawn_link_pair`'s
/// existing flat default -- see `door_link_descriptors`), so merge kinds
/// start at `1` and never collide with it. Unlike door locking's polygon
/// *type-index* scheme (#155's `door_type_indices`), a merge portal has no
/// polygon to type -- `bevy_landmass::AnimationLink3d::kind` is a property
/// of the off-mesh link itself, so per-link exclusion via
/// `landmass::PermittedAnimationLinks::Kinds` achieves exact single-link
/// granularity for free, with no risk of two links sharing an entry
/// polygon the way a polygon-typing scheme would have to worry about (see
/// [`permitted_animation_link_kinds`]'s doc comment). Only meaningful
/// within one archipelago build: kind numbers are not stable across
/// rebuilds, and nothing needs them to be -- a quarantine
/// (`nav::agent::AgentRuntime::quarantined_merge_link_kinds`) is cleared on
/// every retarget/despawn (module doc comment, issue #162 feature 2) well
/// before a cell ever rebuilds its archipelago.
pub(crate) fn merge_link_kind(validated_index: usize) -> usize {
    validated_index + 1
}

/// The animation-link kinds an agent with quarantine set `quarantined` may
/// still use, given this archipelago build assigned merge portals kinds
/// `1..=merge_link_kind_count` (see [`merge_link_kind`]). Kind `0` (every
/// door link, `door_link_descriptors`) is always included -- a blocked
/// merge portal must never make an unrelated door impassable, since
/// `landmass::PermittedAnimationLinks::Kinds` is an *allow-list*, not a
/// deny-list, so a quarantine has to enumerate everything it still permits
/// rather than just the one thing it excludes.
///
/// Returns `None` when `quarantined` is empty: `bevy_landmass::
/// PermittedAnimationLinks::All` is the correct component value for an
/// unquarantined agent (the overwhelmingly common case), and callers
/// should use it directly rather than materializing the equivalent full
/// `0..=merge_link_kind_count` set on every tick.
pub(crate) fn permitted_animation_link_kinds(
    quarantined: &BTreeSet<usize>,
    merge_link_kind_count: usize,
) -> Option<BTreeSet<usize>> {
    if quarantined.is_empty() {
        return None;
    }
    Some(
        std::iter::once(0)
            .chain(1..=merge_link_kind_count)
            .filter(|kind| !quarantined.contains(kind))
            .collect(),
    )
}

fn interval_midpoint(interval: [[f32; 3]; 2]) -> [f32; 3] {
    [
        (interval[0][0] + interval[1][0]) / 2.0,
        (interval[0][1] + interval[1][1]) / 2.0,
        (interval[0][2] + interval[1][2]) / 2.0,
    ]
}

fn point_distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    Vec3::from_array(a).distance(Vec3::from_array(b))
}

/// Vertical clearance budget (bevy metres) a merge portal's matched
/// interval may differ by in elevation between its two sides before this
/// module treats it as impassable rather than building an animation link
/// for it (issue #154 review correction). `vsa::prepare::nav_graph`'s own
/// portal validation is deliberately agent-class-agnostic pure geometry
/// (opposing directions, an overlapping interval -- see that module's
/// `PreparedNavMeshMerge` doc comment) and does *not* reject a candidate
/// for excessive vertical drop; this runtime layer is where the actual
/// agent definition (`nav::agent::AGENT_RADIUS` and friends) already lives,
/// so it is the correct place for an agent-aware "can this agent even step
/// up/down this portal" check instead. Duplicate of `viewer::openmw_player::
/// DEFAULT_STEP_HEIGHT`'s value (`34.0 / 69.991_25`), not an import: this
/// module is Bevy-engine-free (see the module doc comment) and that
/// constant's owning module pulls in `bevy::prelude::*`. Same local-copy
/// precedent as `nav::agent::AGENT_RADIUS`.
const MERGE_PORTAL_STEP_HEIGHT: f32 = 34.0 / 69.991_25;

/// Resolves each `MergeInput` into a world-space link, using its own
/// already-validated portal-interval midpoints (issue #154 feature 3) --
/// not, as before issue #154, a triangle-centroid lookup against `meshes`.
/// The prepare-side validation (`vsa::prepare::nav_graph::
/// validate_portal_candidate`) already confirmed both edges/triangles are
/// real and geometrically opposing/overlapping before a
/// `PreparedNavMeshMerge` was ever emitted, so the checks left to make here
/// are (a) both meshes are still present in the *loaded* `meshes` (a mesh
/// that failed landmass validation and so never became an island -- see
/// `build_navigation_mesh`'s doc comment -- would otherwise dangle a link
/// onto a non-existent island), never panicking on a missing one, and (b)
/// the agent-aware vertical-clearance check ([`MERGE_PORTAL_STEP_HEIGHT`])
/// prepare-time deliberately does not make. Order follows `merges`' own
/// order, which `vsa::prepare::nav_graph::compute_mesh_merges` already
/// produces deterministically.
pub(crate) fn merge_link_descriptors(
    meshes: &[MeshInput],
    merges: &[MergeInput],
) -> Vec<MergeLinkDescriptor> {
    let mut descriptors = Vec::new();
    for merge in merges {
        if !meshes
            .iter()
            .any(|mesh| mesh.form_id == merge.mesh_a_form_id)
        {
            continue;
        }
        if !meshes
            .iter()
            .any(|mesh| mesh.form_id == merge.mesh_b_form_id)
        {
            continue;
        }
        let drop = (merge.interval_a[0][1] - merge.interval_b[0][1])
            .abs()
            .max((merge.interval_a[1][1] - merge.interval_b[1][1]).abs());
        if drop > MERGE_PORTAL_STEP_HEIGHT {
            tracing::warn!(
                "nav merge portal mesh {:08x} triangle {} <-> mesh {:08x} triangle {}: skipped, vertical clearance {:.3} m exceeds the agent step height {:.3} m",
                merge.mesh_a_form_id,
                merge.triangle_a,
                merge.mesh_b_form_id,
                merge.triangle_b,
                drop,
                MERGE_PORTAL_STEP_HEIGHT,
            );
            continue;
        }
        let midpoint_a = interval_midpoint(merge.interval_a);
        let midpoint_b = interval_midpoint(merge.interval_b);
        descriptors.push(MergeLinkDescriptor {
            side_a: DoorLinkSide {
                mesh_form_id: merge.mesh_a_form_id,
                polygon_index: merge.triangle_a,
                midpoint: midpoint_a,
            },
            side_b: DoorLinkSide {
                mesh_form_id: merge.mesh_b_form_id,
                polygon_index: merge.triangle_b,
                midpoint: midpoint_b,
            },
            distance: point_distance(midpoint_a, midpoint_b),
        });
    }
    descriptors
}

// ---------------------------------------------------------------------
// Agent-state mapping
// ---------------------------------------------------------------------

/// Project-native agent status, decoupled from `bevy_landmass::AgentState`
/// so console output (`tna status`) and log lines have stable wording
/// independent of the backend's own enum naming.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavAgentStatus {
    Idle,
    Moving,
    Reached,
    Unreachable,
    Paused,
    /// A travel-door traversal completed (issue #113 feature 3): the agent
    /// stopped at the traversed door; the destination cell is unloaded.
    /// #134 turns this terminal status into an actual cell handoff.
    TravelReached,
}

impl NavAgentStatus {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            NavAgentStatus::Idle => "idle",
            NavAgentStatus::Moving => "moving",
            NavAgentStatus::Reached => "reached",
            NavAgentStatus::Unreachable => "unreachable",
            NavAgentStatus::Paused => "paused",
            NavAgentStatus::TravelReached => "travel-reached",
        }
    }
}

/// Maps a `landmass` agent state to the project-native status. Animation
/// link states count as `Moving` at this layer -- the door-link state
/// machine (`nav::door_link`) is the authority on pause/wait/resume/failure
/// around a specific door, and overrides this mapping while active (see
/// `nav/agent.rs`'s status resolution).
pub(crate) fn map_agent_state(state: AgentState) -> NavAgentStatus {
    match state {
        AgentState::Idle => NavAgentStatus::Idle,
        AgentState::Moving | AgentState::ReachedAnimationLink | AgentState::UsingAnimationLink => {
            NavAgentStatus::Moving
        }
        AgentState::ReachedTarget => NavAgentStatus::Reached,
        AgentState::AgentNotOnNavMesh | AgentState::TargetNotOnNavMesh | AgentState::NoPath => {
            NavAgentStatus::Unreachable
        }
        AgentState::Paused => NavAgentStatus::Paused,
    }
}

#[cfg(test)]
#[path = "tests/landmass_graph.rs"]
mod tests;
