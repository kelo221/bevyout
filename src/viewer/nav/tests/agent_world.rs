use std::collections::HashSet;

use super::*;
use crate::console::{ConsoleError, ConsoleInvocation, ConsoleSessionId};
use crate::viewer::nav::world::links::*;
use crate::viewer::nav::world::portals::*;
use crate::vsa::{PreparedNavGraph, PreparedNavMesh, PreparedNavPolygon};
use bevy::ecs::system::SystemState;
use bevy_boxddd::boxddd::{BodyDef, BodyType, BoxHull, Filter, ShapeDef};
use bevy_landmass::prelude::*;
use bevyout_core::manifest::exterior::ExteriorBorderPortal;

use super::tests_support::*;

#[test]
fn exterior_portal_link_endpoints_are_inset_into_the_owning_cell() {
    let interval = [[10.0, 20.0, 30.0], [10.0, 21.0, 32.0]];

    let min_x = inset_exterior_portal_interval(interval, 1);
    assert_eq!(min_x[0][0], 10.0 + EXTERIOR_PORTAL_LINK_INSET_METRES);
    assert_eq!(min_x[0][1], interval[0][1]);
    assert_eq!(min_x[0][2], interval[0][2]);

    let max_x = inset_exterior_portal_interval(interval, 0);
    assert_eq!(max_x[0][0], 10.0 - EXTERIOR_PORTAL_LINK_INSET_METRES);

    let min_z = inset_exterior_portal_interval(interval, 2);
    assert_eq!(min_z[0][2], 30.0 + EXTERIOR_PORTAL_LINK_INSET_METRES);

    let max_z = inset_exterior_portal_interval(interval, 3);
    assert_eq!(max_z[0][2], 30.0 - EXTERIOR_PORTAL_LINK_INSET_METRES);
}

#[test]
fn exterior_portal_side_selection_is_lowest_residual_and_deterministic() {
    let graph = PreparedNavGraph {
        meshes: vec![PreparedNavMesh {
            form_id: 0x20,
            cell_form_id: Some(0x10),
            vertices: vec![
                [0.6, 0.0, 0.0],
                [0.6, 0.0, 2.0],
                [0.6, 1.0, 1.0],
                [0.02, 0.0, 0.0],
                [0.02, 0.0, 2.0],
                [0.02, 1.0, 1.0],
            ],
            polygons: vec![
                PreparedNavPolygon {
                    index: 7,
                    vertex_indices: [0, 1, 2],
                    ..default()
                },
                PreparedNavPolygon {
                    index: 3,
                    vertex_indices: [3, 4, 5],
                    ..default()
                },
            ],
            ..default()
        }],
        ..default()
    };
    let portal = ExteriorBorderPortal {
        edge: 1,
        start: [0.0, 0.0, 0.0],
        end: [0.0, 0.0, 2.0],
        tolerance: 0.75,
    };

    let side = find_exterior_portal_side(&graph, &portal, 0x10).expect("matching side");
    assert_eq!(side.triangle_index, 3);
    assert_eq!(side.matched_edge, 0);
    assert!(side.residual < 0.1);
}

#[test]
fn exterior_portal_points_stay_inside_selected_triangles_and_source_segments() {
    let left = ExteriorPortalSide {
        mesh_form_id: 1,
        triangle_index: 1,
        interval: [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0]],
        triangle: [[0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [0.6, 0.0, 1.0]],
        matched_edge: 0,
        residual: 0.0,
        border_plane_residual: 0.0,
    };
    let right = ExteriorPortalSide {
        mesh_form_id: 2,
        triangle_index: 2,
        interval: [[1.0, 0.0, 0.0], [1.0, 0.0, 2.0]],
        triangle: [[1.0, 0.0, 0.0], [1.0, 0.0, 2.0], [0.4, 0.0, 1.0]],
        matched_edge: 0,
        residual: 0.0,
        border_plane_residual: 0.0,
    };

    let (left_point, right_point) =
        select_exterior_portal_points(&left, 1, &right, 0).expect("safe portal points");
    assert!(point_in_triangle_xz(left_point, left.triangle));
    assert!(point_in_triangle_xz(right_point, right.triangle));
    assert!(source_segment_inside_triangle(
        left_point,
        right_point,
        left.triangle
    ));
    assert!(source_segment_inside_triangle(
        right_point,
        left_point,
        right.triangle
    ));
}

#[test]
fn an_early_setlock_lock_survives_the_first_archipelago_build() {
    let mut world = archipelago_build_world();
    let manifest = manifest_with_nav_graph_and_door(0xBEEF, 0x99, None);
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    set_door_lock_level(&mut world, 0x99, Some(50));

    ensure_archipelago(&mut world).expect("archipelago builds");

    assert_eq!(
        door_lock_level_for_test(&world, 0x99),
        Some(50),
        "the runtime lock recorded before the archipelago existed must survive the build, winning over the authored (unlocked) baseline"
    );
}

#[test]
fn a_late_setlock_change_still_applies_without_a_rebuild() {
    let mut world = archipelago_build_world();
    let manifest = manifest_with_nav_graph_and_door(0xBEEF, 0x99, Some(25));
    world.insert_resource(crate::viewer::LoadedSceneManifest(manifest));

    ensure_archipelago(&mut world).expect("archipelago builds");
    assert_eq!(door_lock_level_for_test(&world, 0x99), Some(25));

    set_door_lock_level(&mut world, 0x99, None);
    assert_eq!(
        door_lock_level_for_test(&world, 0x99),
        None,
        "a lock change after the archipelago exists must apply immediately, no rebuild needed"
    );
}

#[test]
fn derived_gate_and_blocking_associations_take_distinct_type_indices() {
    // Issue #177 feature 2: the two classes must never share an index,
    // or opening a door would clear the wrong override.
    let mesh = landmass_graph::MeshInput {
        form_id: 0x10,
        vertices: Vec::new(),
        polygons: Vec::new(),
        doors: Vec::new(),
        derived_doors: vec![
            landmass_graph::DerivedDoorInput {
                triangle_index: 1,
                door_reference_form_id: 0x99,
                blocks_when_closed: false,
                openable: true,
            },
            landmass_graph::DerivedDoorInput {
                triangle_index: 2,
                door_reference_form_id: 0x99,
                blocks_when_closed: true,
                openable: true,
            },
        ],
    };
    let meshes = [mesh];
    let door_indices = landmass_graph::door_type_indices(&meshes);
    let closed_indices = landmass_graph::closed_door_type_indices(&meshes, &door_indices);
    assert_eq!(door_indices.get(&0x99), Some(&1));
    assert_eq!(closed_indices.get(&0x99), Some(&2));
    assert_eq!(
        landmass_graph::preferred_pathing_type_index(&door_indices, &closed_indices),
        3
    );
}

#[test]
fn a_preferred_corridor_is_chosen_over_an_equal_length_ordinary_one() {
    let mesh = preferred_path_mesh();
    let door_type_indices = BTreeMap::new();
    let preferred_index =
        landmass_graph::preferred_pathing_type_index(&door_type_indices, &BTreeMap::new());
    let build_result =
        landmass_graph::build_navigation_mesh(&mesh, &[], &door_type_indices, &BTreeMap::new());
    let valid = build_result.nav_mesh.unwrap_or_else(|| {
        panic!(
            "preferred_path_mesh always validates: {:?}",
            build_result.diagnostics
        )
    });

    let mut app = App::new();
    app.add_plugins((
        bevy::MinimalPlugins,
        bevy::asset::AssetPlugin::default(),
        Landmass3dPlugin::default(),
    ));
    let nav_mesh_handle = app
        .world_mut()
        .resource_mut::<Assets<NavMesh3d>>()
        .add(NavMesh3d {
            nav_mesh: Arc::new(valid),
        });
    let mut archipelago_component = Archipelago3d::new(archipelago_options());
    // The exact production call under test (issue #168,
    // `apply_preferred_pathing_base_cost`).
    archipelago_component
        .set_type_index_cost(preferred_index, PREFERRED_PATHING_TYPE_INDEX_COST)
        .expect("PREFERRED_PATHING_TYPE_INDEX_COST is a positive finite documented constant");
    let archipelago = app.world_mut().spawn(archipelago_component).id();
    app.world_mut().spawn(Island3dBundle {
        island: Island,
        archipelago_ref: ArchipelagoRef3d::new(archipelago),
        nav_mesh: NavMeshHandle::<ThreeD>(nav_mesh_handle),
    });
    let agent = app
        .world_mut()
        .spawn((
            Agent3dBundle {
                agent: default(),
                settings: AgentSettings {
                    radius: AGENT_RADIUS,
                    desired_speed: AGENT_DESIRED_SPEED,
                    max_speed: AGENT_MAX_SPEED,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago),
            },
            Transform::from_translation(PREFERRED_PATH_START),
            AgentTarget3d::Point(PREFERRED_PATH_TARGET),
        ))
        .id();

    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);
    app.world_mut().run_schedule(bevy::app::FixedPreUpdate);

    assert_ne!(
        app.world().get::<AgentState>(agent).copied(),
        Some(AgentState::NoPath),
        "both corridors are open; a path must be found"
    );
    let desired = app.world().get::<AgentDesiredVelocity3d>(agent).unwrap();
    assert!(
        desired.velocity().z > 0.0,
        "the cheaper preferred (north, +Z) corridor must be chosen over the \
             equal-length ordinary (south, -Z) one, got desired velocity {:?}",
        desired.velocity()
    );
}
