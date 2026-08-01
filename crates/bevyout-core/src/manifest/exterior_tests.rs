use super::*;

#[test]
fn positive_and_negative_grid_origins_round_trip() {
    let policy = ExteriorCoordinatePolicy::default();
    for grid in [
        GridCoordinate::new(0, 0),
        GridCoordinate::new(2, -3),
        GridCoordinate::new(-4, 5),
    ] {
        let origin = policy.grid_origin(grid);
        assert_eq!(policy.grid_for_bevy(origin), grid);
    }
}

#[test]
fn adjacent_cell_origins_use_one_fixed_span() {
    let policy = ExteriorCoordinatePolicy::default();
    let left = policy.grid_origin(GridCoordinate::new(0, 0));
    let right = policy.grid_origin(GridCoordinate::new(1, 0));
    assert!((right[0] - left[0] - policy.cell_span_metres()).abs() < 1e-9);
}

#[test]
fn border_epsilon_is_deterministic() {
    let policy = ExteriorCoordinatePolicy::default();
    let span = policy.cell_span_metres();
    assert_eq!(
        policy.grid_for_bevy([span, 0.0, 0.0]),
        GridCoordinate::new(1, 0)
    );
    assert_eq!(
        policy.grid_for_bevy([span - 0.0002, 0.0, 0.0]),
        GridCoordinate::new(0, 0)
    );
}

#[test]
fn residency_prefers_current_and_cancels_stale_loads() {
    let mut indexed = BTreeMap::new();
    indexed.insert(GridCoordinate::new(0, 0), 1);
    indexed.insert(GridCoordinate::new(1, 0), 2);
    indexed.insert(GridCoordinate::new(-1, 0), 3);
    let states = vec![ExteriorCellState {
        cell_form_id: 2,
        grid: GridCoordinate::new(1, 0),
        lifecycle: ExteriorCellLifecycle::Loading,
        generation: 7,
        pinned: false,
        estimated_bytes: 1,
        failed_attempts: 0,
    }];
    let plan = plan_residency(
        ExteriorResidencyInput {
            current_grid: GridCoordinate::new(0, 0),
            velocity_grid: (-1, 0),
            resident_budget: 2,
            byte_budget: 1024,
            near_radius: 1,
            prefetch_radius: 1,
            distant_radius: None,
        },
        &indexed,
        &states,
    );
    assert_eq!(plan.desired[0], GridCoordinate::new(0, 0));
    assert!(plan.actions.iter().any(|action| {
        action.grid == GridCoordinate::new(1, 0) && action.action == ExteriorLoadAction::Cancel
    }));
}

#[test]
fn pinned_cells_are_not_evicted_when_the_player_grid_changes() {
    let mut indexed = BTreeMap::new();
    indexed.insert(GridCoordinate::new(0, 0), 1);
    indexed.insert(GridCoordinate::new(5, 5), 2);
    let states = vec![ExteriorCellState {
        cell_form_id: 2,
        grid: GridCoordinate::new(5, 5),
        lifecycle: ExteriorCellLifecycle::Resident,
        generation: 1,
        pinned: true,
        estimated_bytes: 1,
        failed_attempts: 0,
    }];
    let plan = plan_residency(
        ExteriorResidencyInput {
            current_grid: GridCoordinate::new(0, 0),
            velocity_grid: (0, 0),
            resident_budget: 1,
            byte_budget: 1,
            near_radius: 0,
            prefetch_radius: 0,
            distant_radius: None,
        },
        &indexed,
        &states,
    );
    assert!(!plan.actions.iter().any(|action| {
        action.grid == GridCoordinate::new(5, 5) && action.action == ExteriorLoadAction::Evict
    }));
}

#[test]
fn terrain_lod_hysteresis_and_neighbor_clamp_are_bounded() {
    assert_eq!(
        select_terrain_lod(100.0, Some(TerrainLod::Near), 50.0, 150.0, 10.0),
        TerrainLod::Near
    );
    assert_eq!(
        clamp_lod_delta(TerrainLod::Near, TerrainLod::Distant),
        (TerrainLod::Near, TerrainLod::Middle)
    );
}

#[test]
fn environment_prefers_interior_image_space_and_wraps_time() {
    let result = resolve_environment(EnvironmentInput {
        interior: true,
        behave_like_exterior: false,
        worldspace_form_id: Some(1),
        climate_form_id: Some(2),
        weather: WeatherBlend {
            source_weather_form_id: Some(3),
            target_weather_form_id: Some(4),
            progress: 0.5,
        },
        image_space_form_id: Some(5),
        time_hours: 25.0,
    });
    assert_eq!(result.source, "interior-cell");
    assert!(!result.dynamic_lighting_allowed);
    assert_eq!(result.time_hours, 1.0);
}

#[test]
fn water_contact_is_bounded_by_authored_swim_depth() {
    let water = PreparedWater {
        form_id: None,
        height: 2.0,
        water_type_form_id: None,
        swim_depth: 1.0,
    };
    assert_eq!(resolve_water_contact(Some(&water), 1.5).unwrap().depth, 0.5);
    assert!(resolve_water_contact(Some(&water), 1.5).unwrap().submerged);
    assert!(!resolve_water_contact(Some(&water), 0.5).unwrap().submerged);
}

#[test]
fn portal_matching_is_symmetric_and_deterministic() {
    let policy = ExteriorCoordinatePolicy::default();
    let left_grid = GridCoordinate::new(4, -5);
    let right_grid = GridCoordinate::new(5, -5);
    let left_origin = policy.grid_origin(left_grid);
    let boundary_x = left_origin[0] + policy.cell_span_metres();
    let left = [ExteriorBorderPortal {
        edge: 0,
        start: [boundary_x as f32, 2.0, 1.0],
        end: [boundary_x as f32, 2.0, 3.0],
        tolerance: 0.01,
    }];
    let right = [ExteriorBorderPortal {
        edge: 1,
        start: [boundary_x as f32, 2.0, 3.0],
        end: [boundary_x as f32, 2.0, 1.0],
        tolerance: 0.01,
    }];
    assert_eq!(
        matching_portals(left_grid, &left, right_grid, &right),
        vec![(0, 0)]
    );

    let lower_grid = GridCoordinate::new(-4, 5);
    let upper_grid = GridCoordinate::new(-4, 6);
    let lower_origin = policy.grid_origin(lower_grid);
    let boundary_z = lower_origin[2] - policy.cell_span_metres();
    let lower = [ExteriorBorderPortal {
        edge: 2,
        start: [(lower_origin[0] + 1.0) as f32, 2.0, boundary_z as f32],
        end: [(lower_origin[0] + 3.0) as f32, 2.0, boundary_z as f32],
        tolerance: 0.01,
    }];
    let upper = [ExteriorBorderPortal {
        edge: 3,
        start: [(lower_origin[0] + 3.0) as f32, 2.0, boundary_z as f32],
        end: [(lower_origin[0] + 1.0) as f32, 2.0, boundary_z as f32],
        tolerance: 0.01,
    }];
    assert_eq!(
        matching_portals(lower_grid, &lower, upper_grid, &upper),
        vec![(0, 0)]
    );
}
