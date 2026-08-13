use super::*;
use bevyout_core::manifest::exterior::{
    ExteriorCellIndexEntry, ExteriorCoordinatePolicy, ExteriorWorldspaceIndex,
};

fn index() -> ExteriorWorldspaceIndex {
    let policy = ExteriorCoordinatePolicy::default();
    ExteriorWorldspaceIndex {
        revision: "test".into(),
        content_fingerprint: "fp".into(),
        worldspace_form_id: 1,
        editor_id: None,
        name: None,
        climate_form_id: None,
        coordinate_policy: policy.clone(),
        weather_profiles: Vec::new(),
        cells: [-1i32, 0, 1]
            .into_iter()
            .map(|x| ExteriorCellIndexEntry {
                cell_form_id: x.unsigned_abs() + 1,
                grid: GridCoordinate::new(x, 0),
                origin: policy
                    .grid_origin(GridCoordinate::new(x, 0))
                    .map(|v| v as f32),
                package_path: String::new(),
                land_form_id: None,
                road_count: 0,
                navm_count: 0,
                persistent_reference_count: 0,
                distant_reference_count: 0,
            })
            .collect(),
        persistent_references: Vec::new(),
        worldspace_lod: Vec::new(),
        diagnostics: Vec::new(),
    }
}

#[test]
fn policy_prefers_current_grid_and_keeps_negative_coordinates() {
    let index = index();
    let plan = desired_plan(
        &index,
        GridCoordinate::new(-1, 0),
        (-1, 0),
        &[],
        4,
        1_000_000,
    );
    assert_eq!(plan.desired[0], GridCoordinate::new(-1, 0));
    assert!(
        plan.actions
            .iter()
            .any(|action| action.grid == GridCoordinate::new(0, 0))
    );
}

#[test]
fn duplicate_grid_entries_choose_the_lowest_cell_form_id() {
    let mut index = index();
    index.cells.push(ExteriorCellIndexEntry {
        cell_form_id: 99,
        grid: GridCoordinate::new(0, 0),
        origin: [0.0; 3],
        package_path: String::new(),
        land_form_id: None,
        road_count: 0,
        navm_count: 0,
        persistent_reference_count: 0,
        distant_reference_count: 0,
    });

    assert_eq!(index_cells(&index)[&GridCoordinate::new(0, 0)], 1);
}

#[test]
fn collision_handoff_pins_old_cell_while_target_is_requested() {
    let index = index();
    let states = vec![ExteriorCellState {
        cell_form_id: 1,
        grid: GridCoordinate::new(0, 0),
        lifecycle: bevyout_core::manifest::exterior::ExteriorCellLifecycle::Resident,
        generation: 1,
        pinned: true,
        estimated_bytes: 1,
        failed_attempts: 0,
    }];
    let plan = plan_residency(
        ExteriorResidencyInput {
            current_grid: GridCoordinate::new(1, 0),
            velocity_grid: (1, 0),
            resident_budget: 2,
            byte_budget: 1024,
            near_radius: 1,
            prefetch_radius: 1,
            distant_radius: Some(2),
        },
        &index_cells(&index),
        &states,
    );
    assert!(plan.actions.iter().any(|action| {
        action.grid == GridCoordinate::new(1, 0)
            && action.action == bevyout_core::manifest::exterior::ExteriorLoadAction::Request
    }));
    assert!(!plan.actions.iter().any(|action| {
        action.grid == GridCoordinate::new(0, 0)
            && action.action == bevyout_core::manifest::exterior::ExteriorLoadAction::Evict
    }));
}

#[test]
fn desired_plan_cancels_eviction_when_a_target_reverses() {
    let index = index();
    let grid = GridCoordinate::new(1, 0);
    let states = vec![ExteriorCellState {
        cell_form_id: 2,
        grid,
        lifecycle: bevyout_core::manifest::exterior::ExteriorCellLifecycle::Evicting,
        generation: 6,
        pinned: false,
        estimated_bytes: 1,
        failed_attempts: 0,
    }];

    let plan = desired_plan(&index, grid, (0, 0), &states, 1, 1024);

    assert_eq!(
        plan.actions,
        vec![bevyout_core::manifest::exterior::ExteriorResidencyAction {
            form_id: 2,
            grid,
            action: bevyout_core::manifest::exterior::ExteriorLoadAction::Cancel,
            generation: 6,
        }]
    );
}
