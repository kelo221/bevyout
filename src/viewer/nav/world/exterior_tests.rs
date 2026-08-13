//! Resident NAVM topology observation tests (M6 W3-C, defect 2).

use bevy::prelude::World;
use bevyout_core::manifest::exterior::{
    EXTERIOR_CELL_PACKAGE_REVISION, ExteriorCellLifecycle, ExteriorCellPackage, ExteriorCellState,
    GridCoordinate, PreparedExteriorEnvironment, PreparedExteriorNavigation,
};

use super::{observe_resident_nav_cells, resident_border_portals};
use crate::viewer::nav::landmass_graph::{
    MergeInput, MeshInput, PolygonInput, ResidentNavCellKey, ResidentNavTopology,
};
use crate::viewer::world::exterior::{
    ExteriorStreamState, begin_test_eviction, insert_test_resident_cell,
};

const CELL_A: u32 = 0x0000_0c67;
const CELL_B: u32 = 0x0000_0c68;
const MESH_A: u32 = 0x0000_1001;
const MESH_B: u32 = 0x0000_1002;

fn navigable_package(cell_form_id: u32, grid: GridCoordinate) -> ExteriorCellPackage {
    ExteriorCellPackage {
        revision: EXTERIOR_CELL_PACKAGE_REVISION.into(),
        content_fingerprint: "synthetic".into(),
        cell_form_id,
        worldspace_form_id: 0x0000_003c,
        grid,
        origin: [0.0; 3],
        terrain: None,
        water: None,
        static_objects: Vec::new(),
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        actors: Vec::new(),
        local_lights: Vec::new(),
        navigation: Some(PreparedExteriorNavigation {
            clearance_ready: true,
            graph_asset_path: Some("synthetic/navmesh.ron".into()),
            ..Default::default()
        }),
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    }
}

/// A one-quad mesh whose boundary edge coincides with its neighbour's, so the
/// ordinary merge-link validation resolves a real descriptor across them.
fn mesh(form_id: u32, x_offset: f32) -> MeshInput {
    MeshInput {
        form_id,
        vertices: vec![
            [x_offset, 0.0, 0.0],
            [x_offset + 1.0, 0.0, 0.0],
            [x_offset + 1.0, 0.0, 1.0],
            [x_offset, 0.0, 1.0],
        ],
        polygons: vec![
            PolygonInput {
                index: 0,
                vertex_indices: [0, 1, 2],
                is_water: false,
                is_preferred_pathing: false,
            },
            PolygonInput {
                index: 1,
                vertex_indices: [0, 2, 3],
                is_water: false,
                is_preferred_pathing: false,
            },
        ],
        doors: Vec::new(),
        derived_doors: Vec::new(),
    }
}

fn border_link() -> MergeInput {
    MergeInput {
        mesh_a_form_id: MESH_A,
        triangle_a: 0,
        mesh_b_form_id: MESH_B,
        triangle_b: 1,
        // The two sides sit a traversable step apart, so the ordinary merge
        // validation resolves a real (non-degenerate) link across the border.
        interval_a: [[0.9, 0.0, 0.0], [0.9, 0.0, 1.0]],
        interval_b: [[1.1, 0.0, 0.0], [1.1, 0.0, 1.0]],
    }
}

fn insert_cell(
    world: &mut World,
    cell_form_id: u32,
    grid: GridCoordinate,
    lifecycle: ExteriorCellLifecycle,
) {
    let mut state = world.resource_mut::<ExteriorStreamState>();
    insert_test_resident_cell(
        &mut state,
        ExteriorCellState {
            cell_form_id,
            grid,
            lifecycle,
            generation: 1,
            pinned: false,
            estimated_bytes: 1,
            failed_attempts: 0,
        },
        navigable_package(cell_form_id, grid),
    );
}

#[test]
fn resident_nav_topology_drops_cross_cell_links_when_one_side_evicts() {
    let mut world = World::new();
    world.init_resource::<ExteriorStreamState>();
    let grid_a = GridCoordinate::new(5, -6);
    let grid_b = GridCoordinate::new(4, -6);
    insert_cell(&mut world, CELL_A, grid_a, ExteriorCellLifecycle::Resident);
    insert_cell(&mut world, CELL_B, grid_b, ExteriorCellLifecycle::Ready);

    let meshes = vec![mesh(MESH_A, 0.0), mesh(MESH_B, 1.0)];
    let owners = [
        (
            MESH_A,
            ResidentNavCellKey {
                cell_form_id: CELL_A,
                generation: 1,
            },
        ),
        (
            MESH_B,
            ResidentNavCellKey {
                cell_form_id: CELL_B,
                generation: 1,
            },
        ),
    ]
    .into_iter()
    .collect();
    let portals = resident_border_portals(&[border_link()], &owners);
    assert_eq!(portals.len(), 1);

    let mut topology = ResidentNavTopology::default();
    topology.rebuild(&observe_resident_nav_cells(&world), &meshes, &portals);
    let archipelago = topology
        .archipelago
        .as_ref()
        .expect("both sides are navigation-ready");
    assert_eq!(archipelago.resident_cells.len(), 2);
    assert_eq!(archipelago.merge_links.len(), 1);

    // One side starts evicting (a real lifecycle transition, generation bump
    // included): the rebuild drops that resident cell and every cross-cell
    // link that used it, without disturbing the surviving side.
    let mut state = world.resource_mut::<ExteriorStreamState>();
    begin_test_eviction(&mut state, grid_b);
    topology.rebuild(&observe_resident_nav_cells(&world), &meshes, &portals);
    let archipelago = topology
        .archipelago
        .as_ref()
        .expect("the surviving side stays resident");
    assert_eq!(
        archipelago.resident_cells,
        vec![ResidentNavCellKey {
            cell_form_id: CELL_A,
            generation: 1,
        }]
    );
    assert!(
        archipelago.merge_links.is_empty(),
        "a cross-cell link cannot survive its side's eviction"
    );
}
