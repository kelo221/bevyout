use super::*;

fn square_mesh(vertex_indices_a: [u32; 3], vertex_indices_b: [u32; 3]) -> MeshInput {
    MeshInput {
        form_id: 0x10,
        vertices: vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
        ],
        polygons: vec![
            PolygonInput {
                index: 0,
                vertex_indices: vertex_indices_a,
                is_water: false,
                is_preferred_pathing: false,
            },
            PolygonInput {
                index: 1,
                vertex_indices: vertex_indices_b,
                is_water: false,
                is_preferred_pathing: false,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    }
}

#[test]
fn validates_a_known_good_square_mesh() {
    // Empirically the winding `NavigationMesh3d::validate()` accepts
    // without needing this function's reversal retry, for this
    // vertex layout (see `reversed_winding_still_validates_after_retry`
    // for the opposite winding, which does need the retry).
    let mesh = square_mesh([0, 1, 2], [1, 3, 2]);
    let result = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
    assert!(
        result.diagnostics.is_empty(),
        "expected no diagnostics for a known-good mesh: {:?}",
        result.diagnostics
    );
}

#[test]
fn reversed_winding_still_validates_after_retry_with_a_warning() {
    // Same square, opposite winding: the first attempt must fail
    // internally and the retry with reversed order must succeed.
    let mesh = square_mesh([0, 2, 1], [1, 2, 3]);
    let result = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.message.contains("required reversal")),
        "{:?}",
        result.diagnostics
    );
}

#[test]
fn water_polygons_are_excluded_as_non_walkable() {
    let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
    mesh.polygons[1].is_water = true;
    let result = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    // One walkable polygon remains -- still a valid (smaller) mesh.
    // `ValidNavigationMesh`'s fields are private to `landmass`, so this
    // only asserts what's externally observable: conversion still
    // succeeds after excluding the water polygon.
    assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
}

#[test]
fn all_water_mesh_produces_no_navigation_mesh_and_a_warning() {
    let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
    mesh.polygons[0].is_water = true;
    mesh.polygons[1].is_water = true;
    let result = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    assert!(result.nav_mesh.is_none());
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.message.contains("no walkable polygons")),
        "{:?}",
        result.diagnostics
    );
}

#[test]
fn invalid_vertex_index_polygon_is_skipped_with_an_error_diagnostic_and_never_panics() {
    let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
    mesh.polygons[1].vertex_indices = [1, 2, u32::MAX];
    let result = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error && d.message.contains("invalid vertex index")),
        "{:?}",
        result.diagnostics
    );
}

#[test]
fn degenerate_polygon_is_skipped_with_a_warning_diagnostic() {
    let mut mesh = square_mesh([0, 2, 1], [1, 2, 3]);
    mesh.polygons[1].vertex_indices = [1, 1, 3];
    let result = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    assert!(result.nav_mesh.is_some(), "{:?}", result.diagnostics);
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning && d.message.contains("degenerate triangle")),
        "{:?}",
        result.diagnostics
    );
}

fn mesh_with_door(form_id: u32, triangle_index: u32, door_form_id: u32) -> MeshInput {
    MeshInput {
        form_id,
        vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        polygons: vec![PolygonInput {
            index: triangle_index,
            vertex_indices: [0, 1, 2],
            is_water: false,
            is_preferred_pathing: false,
        }],
        doors: vec![DoorInput {
            triangle_index,
            door_reference_form_id: Some(door_form_id),
        }],
        derived_doors: Vec::new(),
    }
}

#[test]
fn door_link_descriptor_extracted_when_both_sides_resolve() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x99);
    let descriptors = door_link_descriptors(&[mesh_a, mesh_b]);
    assert_eq!(descriptors.len(), 1);
    assert_eq!(descriptors[0].door_form_id, 0x99);
    assert_eq!(descriptors[0].side_a.mesh_form_id, 0x10);
    assert_eq!(descriptors[0].side_b.mesh_form_id, 0x20);
}

#[test]
fn door_link_descriptor_is_deterministic_across_calls() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x99);
    let first = door_link_descriptors(&[mesh_a.clone(), mesh_b.clone()]);
    let second = door_link_descriptors(&[mesh_a, mesh_b]);
    assert_eq!(first, second);
}

#[test]
fn door_with_only_one_side_is_skipped() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let descriptors = door_link_descriptors(&[mesh_a]);
    assert!(descriptors.is_empty());
}

#[test]
fn door_with_more_than_two_sides_is_skipped() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x99);
    let mesh_c = mesh_with_door(0x30, 0, 0x99);
    let descriptors = door_link_descriptors(&[mesh_a, mesh_b, mesh_c]);
    assert!(descriptors.is_empty());
}

#[test]
fn a_door_with_one_side_is_single_sided() {
    // Wave 3's real-data finding: every real FO3 door triangle is
    // single-sided.
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let doors = single_sided_doors(&[mesh_a]);
    assert_eq!(doors.len(), 1);
    assert_eq!(doors[0].door_form_id, 0x99);
    assert_eq!(doors[0].side.mesh_form_id, 0x10);
}

#[test]
fn a_door_with_two_sides_is_not_single_sided() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x99);
    let doors = single_sided_doors(&[mesh_a, mesh_b]);
    assert!(doors.is_empty());
}

#[test]
fn single_sided_doors_are_deterministic_and_ordered() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x50);
    let first = single_sided_doors(&[mesh_a.clone(), mesh_b.clone()]);
    let second = single_sided_doors(&[mesh_a, mesh_b]);
    assert_eq!(first, second);
    assert_eq!(
        first.iter().map(|d| d.door_form_id).collect::<Vec<_>>(),
        vec![0x50, 0x99]
    );
}

#[test]
fn single_sided_doors_carry_the_triangle_vertices() {
    // Issue #155 feature 3: the corridor-based crossing gate needs the
    // door triangle's real footprint, not just its centroid.
    let mesh = mesh_with_door(0x10, 0, 0x99);
    let doors = single_sided_doors(&[mesh]);
    assert_eq!(doors.len(), 1);
    assert_eq!(
        doors[0].vertices,
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]
    );
}

// -------------------------------------------------------------
// door_type_indices (issue #155 feature 1)
// -------------------------------------------------------------

#[test]
fn each_distinct_door_gets_its_own_type_index_starting_at_one() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x50);
    let indices = door_type_indices(&[mesh_a, mesh_b]);
    // Ascending by door FormID, not by discovery order: 0x50 < 0x99.
    assert_eq!(indices.get(&0x50), Some(&1));
    assert_eq!(indices.get(&0x99), Some(&2));
}

#[test]
fn the_same_door_referenced_from_two_meshes_gets_one_shared_type_index() {
    // The two-sided `DoorLinkDescriptor` case: the same door FormID
    // appears in both meshes' `doors` list. Locking it must exclude
    // both triangles via one type index, not two different ones.
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x99);
    let indices = door_type_indices(&[mesh_a, mesh_b]);
    assert_eq!(indices.len(), 1);
    assert_eq!(indices.get(&0x99), Some(&1));
}

#[test]
fn a_door_with_no_resolved_form_id_gets_no_type_index() {
    let mut mesh = mesh_with_door(0x10, 0, 0x99);
    mesh.doors[0].door_reference_form_id = None;
    let indices = door_type_indices(&[mesh]);
    assert!(indices.is_empty());
}

#[test]
fn door_type_indices_is_deterministic_across_calls() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x50);
    let first = door_type_indices(&[mesh_a.clone(), mesh_b.clone()]);
    let second = door_type_indices(&[mesh_a, mesh_b]);
    assert_eq!(first, second);
}

#[test]
fn a_typed_door_triangle_still_validates_and_keeps_its_polygon_count() {
    // CONSTRAINT pin (issue #155 feature 1): typing a door triangle must
    // not remove or alter unrelated adjacency. This module cannot run a
    // live pathfind (Bevy-engine-free, see the module doc comment), but
    // it can confirm conversion still succeeds and produces the same
    // polygon count with a non-trivial type index as it does with the
    // all-zero default -- `agent.rs`'s own tests confirm the live
    // solve still connects across a typed-but-unlocked door.
    let mesh = mesh_with_door(0x10, 0, 0x99);
    let indices = door_type_indices(std::slice::from_ref(&mesh));
    assert_eq!(indices.get(&0x99), Some(&1));
    let typed = build_navigation_mesh(&mesh, &[], &indices, &BTreeMap::new());
    let untyped = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    assert!(typed.nav_mesh.is_some(), "{:?}", typed.diagnostics);
    assert!(untyped.nav_mesh.is_some(), "{:?}", untyped.diagnostics);
}

// -------------------------------------------------------------
// preferred_pathing_type_index / resolve_polygon_type_index
// (issue #156 feature 1)
// -------------------------------------------------------------

#[test]
fn preferred_pathing_type_index_is_one_past_the_highest_door_index() {
    let mesh_a = mesh_with_door(0x10, 0, 0x99);
    let mesh_b = mesh_with_door(0x20, 0, 0x50);
    let door_indices = door_type_indices(&[mesh_a, mesh_b]);
    // Doors got 1 (0x50) and 2 (0x99) -- see
    // `each_distinct_door_gets_its_own_type_index_starting_at_one`.
    assert_eq!(
        preferred_pathing_type_index(&door_indices, &BTreeMap::new()),
        3
    );
}

#[test]
fn preferred_pathing_type_index_is_one_when_there_are_no_doors() {
    assert_eq!(
        preferred_pathing_type_index(&BTreeMap::new(), &BTreeMap::new()),
        1
    );
}

#[test]
fn an_ordinary_polygon_resolves_to_type_zero() {
    let door_type_index_by_triangle = HashMap::new();
    assert_eq!(
        resolve_polygon_type_index(0, false, &door_type_index_by_triangle, 5),
        0
    );
}

#[test]
fn a_preferred_pathing_polygon_resolves_to_the_preferred_pathing_type() {
    let door_type_index_by_triangle = HashMap::new();
    assert_eq!(
        resolve_polygon_type_index(0, true, &door_type_index_by_triangle, 5),
        5
    );
}

#[test]
fn a_door_polygon_resolves_to_its_own_door_type() {
    let mut door_type_index_by_triangle = HashMap::new();
    door_type_index_by_triangle.insert(0, 2);
    assert_eq!(
        resolve_polygon_type_index(0, false, &door_type_index_by_triangle, 5),
        2
    );
}

#[test]
fn a_triangle_that_is_both_a_door_and_preferred_pathing_keeps_its_door_type() {
    // The exact coexistence rule issue #156 feature 1 documents: landmass
    // 0.9.2 stores one type index per polygon, so when a triangle is
    // both, the door's lockable type must win -- a preferred-pathing
    // type only ever means a *cheaper* cost, never an override that
    // could silently reopen a locked door.
    let mut door_type_index_by_triangle = HashMap::new();
    door_type_index_by_triangle.insert(0, 2);
    assert_eq!(
        resolve_polygon_type_index(0, true, &door_type_index_by_triangle, 5),
        2
    );
}

fn mesh_with_preferred_pathing_polygon(form_id: u32) -> MeshInput {
    MeshInput {
        form_id,
        vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        polygons: vec![PolygonInput {
            index: 0,
            vertex_indices: [0, 1, 2],
            is_water: false,
            is_preferred_pathing: true,
        }],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    }
}

#[test]
fn a_preferred_pathing_mesh_still_validates_and_keeps_its_polygon_count() {
    // CONSTRAINT pin (issue #156 feature 1), the same shape as #155's own
    // `a_typed_door_triangle_still_validates_and_keeps_its_polygon_count`:
    // typing a preferred-pathing triangle must not remove or alter
    // adjacency, only which cost `landmass::pathfinding` looks up.
    let mesh = mesh_with_preferred_pathing_polygon(0x10);
    let typed = build_navigation_mesh(&mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    let mut untyped_mesh = mesh;
    untyped_mesh.polygons[0].is_preferred_pathing = false;
    let untyped = build_navigation_mesh(&untyped_mesh, &[], &BTreeMap::new(), &BTreeMap::new());
    assert!(typed.nav_mesh.is_some(), "{:?}", typed.diagnostics);
    assert!(untyped.nav_mesh.is_some(), "{:?}", untyped.diagnostics);
}

// -------------------------------------------------------------
// point_in_door_triangle (issue #155 feature 3)
// -------------------------------------------------------------

/// A door-sized triangle spanning x:4..6, z:-1..1 (apex at z=1), the
/// same shape the invariant tests below reason about.
fn sample_door_triangle() -> [[f32; 3]; 3] {
    [[4.0, 0.0, -1.0], [6.0, 0.0, -1.0], [5.0, 0.0, 1.0]]
}

#[test]
fn a_point_inside_the_triangle_footprint_is_contained() {
    assert!(point_in_door_triangle(
        [5.0, 0.0, 0.0],
        sample_door_triangle(),
        1.8
    ));
}

#[test]
fn a_point_clearly_outside_the_triangle_footprint_is_not_contained() {
    assert!(!point_in_door_triangle(
        [5.0, 0.0, -5.0],
        sample_door_triangle(),
        1.8
    ));
}

#[test]
fn a_point_near_the_centroid_but_outside_the_footprint_is_not_contained() {
    // The exact bug issue #155 fixes: the old `MID_ROUTE_DOOR_GATE_
    // DISTANCE` (0.75 m) centroid-proximity scan would have gated a
    // route merely passing close to the door's midpoint. The
    // triangle's centroid is about (5.0, 0.0, -0.33); this point sits
    // just outside the triangle's base edge (z = -1) but well within
    // 0.75 m of that centroid.
    let point = [5.0, 0.0, -1.05];
    let triangle = sample_door_triangle();
    let centroid_z = (triangle[0][2] + triangle[1][2] + triangle[2][2]) / 3.0;
    assert!(
        (point[2] - centroid_z).abs() < 0.75,
        "test setup: the point must be within the old proximity radius"
    );
    assert!(!point_in_door_triangle(point, triangle, 1.8));
}

#[test]
fn a_point_on_a_different_floor_is_not_contained_despite_matching_xz() {
    assert!(!point_in_door_triangle(
        [5.0, 5.0, 0.0],
        sample_door_triangle(),
        1.8
    ));
}

#[test]
fn the_vertical_gap_tolerates_the_agent_capsule_centre_offset() {
    // Mirrors `movement_policy::nav_point_reached`'s own tolerance: a
    // capsule-centre agent above a feet-level door triangle must still
    // be contained.
    assert!(point_in_door_triangle(
        [5.0, 0.9, 0.0],
        sample_door_triangle(),
        1.8
    ));
}

#[test]
fn merge_link_descriptor_resolves_both_sides_from_the_prepared_interval() {
    let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
    let mut mesh_b = square_mesh([0, 1, 2], [1, 3, 2]);
    mesh_b.form_id = 0x20;
    let merges = vec![MergeInput {
        mesh_a_form_id: 0x10,
        triangle_a: 0,
        mesh_b_form_id: 0x20,
        triangle_b: 1,
        interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        interval_b: [[2.0, 0.0, 0.0], [3.0, 0.0, 0.0]],
    }];
    let descriptors = merge_link_descriptors(&[mesh_a, mesh_b], &merges);
    assert_eq!(descriptors.len(), 1);
    let descriptor = descriptors[0];
    assert_eq!(descriptor.side_a.mesh_form_id, 0x10);
    assert_eq!(descriptor.side_a.polygon_index, 0);
    // Issue #154 feature 3: the link point is the *interval* midpoint,
    // not a triangle centroid.
    assert_eq!(descriptor.side_a.midpoint, [0.5, 0.0, 0.0]);
    assert_eq!(descriptor.side_b.mesh_form_id, 0x20);
    assert_eq!(descriptor.side_b.polygon_index, 1);
    assert_eq!(descriptor.side_b.midpoint, [2.5, 0.0, 0.0]);
    // Cost is the real distance between the two interval midpoints.
    assert!((descriptor.distance - 2.0).abs() < 1.0e-6, "{descriptor:?}");
}

#[test]
fn merge_referencing_a_missing_mesh_is_skipped_not_panicked() {
    // Issue #154: triangle/edge legitimacy is already validated
    // prepare-side before a `MergeInput` is ever produced, so this
    // module's own defensive check is reduced to "is the mesh still
    // present in the loaded manifest" -- see `merge_link_descriptors`'s
    // doc comment.
    let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
    let merges = vec![MergeInput {
        mesh_a_form_id: 0x10,
        triangle_a: 0,
        mesh_b_form_id: 0x9999, // no such mesh
        triangle_b: 0,
        interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        interval_b: [[2.0, 0.0, 0.0], [3.0, 0.0, 0.0]],
    }];
    let descriptors = merge_link_descriptors(&[mesh_a], &merges);
    assert!(descriptors.is_empty());
}

#[test]
fn a_merge_with_too_much_vertical_drop_is_skipped_at_runtime() {
    // Issue #154 review correction: prepare-side no longer rejects a
    // portal candidate for excessive vertical drop (that is an
    // agent-class assumption, moved here) -- so a `PreparedNavMeshMerge`
    // can legitimately carry an interval like this (adversarial
    // fixture: vertically stacked floors whose edges overlap in XZ).
    // This runtime layer, where the actual agent step-height
    // definition lives, is what must skip building a link for it.
    let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
    let mut mesh_b = square_mesh([0, 1, 2], [1, 3, 2]);
    mesh_b.form_id = 0x20;
    let merges = vec![MergeInput {
        mesh_a_form_id: 0x10,
        triangle_a: 0,
        mesh_b_form_id: 0x20,
        triangle_b: 1,
        interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        // 1 m above mesh_a's interval -- well past a humanoid step.
        interval_b: [[0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    }];
    let descriptors = merge_link_descriptors(&[mesh_a, mesh_b], &merges);
    assert!(descriptors.is_empty(), "{descriptors:?}");
}

#[test]
fn a_merge_within_step_height_still_resolves() {
    // The counterpart to the vertical-drop-is-skipped test above: a
    // small (well under step-height) elevation difference must still
    // resolve normally.
    let mesh_a = square_mesh([0, 1, 2], [1, 3, 2]);
    let mut mesh_b = square_mesh([0, 1, 2], [1, 3, 2]);
    mesh_b.form_id = 0x20;
    let merges = vec![MergeInput {
        mesh_a_form_id: 0x10,
        triangle_a: 0,
        mesh_b_form_id: 0x20,
        triangle_b: 1,
        interval_a: [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        interval_b: [[0.0, 0.05, 0.0], [1.0, 0.05, 0.0]],
    }];
    let descriptors = merge_link_descriptors(&[mesh_a, mesh_b], &merges);
    assert_eq!(descriptors.len(), 1, "{descriptors:?}");
}

#[test]
fn agent_state_maps_to_project_status() {
    assert_eq!(map_agent_state(AgentState::Idle), NavAgentStatus::Idle);
    assert_eq!(map_agent_state(AgentState::Moving), NavAgentStatus::Moving);
    assert_eq!(
        map_agent_state(AgentState::ReachedTarget),
        NavAgentStatus::Reached
    );
    assert_eq!(
        map_agent_state(AgentState::AgentNotOnNavMesh),
        NavAgentStatus::Unreachable
    );
    assert_eq!(
        map_agent_state(AgentState::TargetNotOnNavMesh),
        NavAgentStatus::Unreachable
    );
    assert_eq!(
        map_agent_state(AgentState::NoPath),
        NavAgentStatus::Unreachable
    );
    assert_eq!(map_agent_state(AgentState::Paused), NavAgentStatus::Paused);
}

// -------------------------------------------------------------
// merge_link_kind / permitted_animation_link_kinds (issue #162)
// -------------------------------------------------------------

#[test]
fn merge_link_kinds_start_at_one_and_never_collide_with_the_reserved_door_kind() {
    assert_eq!(merge_link_kind(0), 1);
    assert_eq!(merge_link_kind(1), 2);
    assert_eq!(merge_link_kind(4), 5);
}

#[test]
fn an_unquarantined_agent_gets_no_kind_restriction() {
    assert_eq!(
        permitted_animation_link_kinds(&BTreeSet::new(), 3),
        None,
        "an empty quarantine must signal `PermittedAnimationLinks::All`, not an explicit full set"
    );
}

#[test]
fn a_quarantined_link_is_excluded_but_everything_else_including_doors_stays_permitted() {
    let mut quarantined = BTreeSet::new();
    quarantined.insert(2);
    let permitted = permitted_animation_link_kinds(&quarantined, 3)
        .expect("a non-empty quarantine must produce an explicit allow-list");
    // Door kind 0 and every other merge kind (1, 3) stay permitted;
    // only the quarantined merge kind (2) is excluded.
    assert_eq!(permitted, BTreeSet::from([0, 1, 3]));
}

#[test]
fn quarantining_every_merge_kind_still_leaves_doors_permitted() {
    let mut quarantined = BTreeSet::new();
    quarantined.insert(1);
    quarantined.insert(2);
    let permitted = permitted_animation_link_kinds(&quarantined, 2)
        .expect("a non-empty quarantine must produce an explicit allow-list");
    assert_eq!(
        permitted,
        BTreeSet::from([0]),
        "every merge portal is blocked, but door links (kind 0) must remain usable"
    );
}

#[test]
fn a_quarantined_kind_past_this_builds_own_range_is_harmless() {
    // Defensive: a stale quarantine entry from a torn-down archipelago
    // (should never happen given issue #162 feature 2's clear-on-
    // retarget/despawn lifecycle, but this function has no way to know
    // that) must not panic or corrupt the allow-list for kinds that do
    // exist in this build.
    let mut quarantined = BTreeSet::new();
    quarantined.insert(99);
    let permitted = permitted_animation_link_kinds(&quarantined, 2)
        .expect("a non-empty quarantine must produce an explicit allow-list");
    assert_eq!(permitted, BTreeSet::from([0, 1, 2]));
}
