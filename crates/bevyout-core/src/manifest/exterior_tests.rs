use super::*;

#[test]
fn shared_weather_catalog_uses_cell_local_timings() {
    let environment = PreparedExteriorEnvironment {
        timings: crate::time_of_day::DayNightTimings {
            sunrise_begin_hour: 4.0,
            sunrise_end_hour: 8.0,
            sunset_begin_hour: 17.0,
            sunset_end_hour: 21.0,
        },
        ..Default::default()
    };
    let mut ambient = crate::time_of_day::ColorKeyframes::default();
    ambient.day = [0.75; 4];
    let catalog = vec![PreparedWeatherCatalogEntry {
        form_id: 0x50,
        editor_id: Some("WastelandClear".into()),
        sky_upper: Default::default(),
        sky_lower: Default::default(),
        ambient,
        sunlight: Default::default(),
    }];

    let profile = resolve_prepared_weather_profile(&environment, &catalog, 0x50).unwrap();
    assert_eq!(profile.timings, environment.timings);
    assert_eq!(profile.ambient.day, [0.75; 4]);
}

#[test]
fn empty_legacy_weather_profiles_are_omitted_from_ron() {
    let environment = PreparedExteriorEnvironment::default();
    let encoded = ron::ser::to_string(&environment).unwrap();
    assert!(!encoded.contains("weather_profiles"));
}
use crate::manifest::{PreparedDoorDestination, PreparedSemantic};

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
fn residency_cancels_queued_and_loading_cells_outside_the_ring() {
    let indexed = BTreeMap::from([
        (GridCoordinate::new(0, 0), 1),
        (GridCoordinate::new(1, 0), 2),
        (GridCoordinate::new(2, 0), 3),
    ]);
    let states = vec![
        ExteriorCellState {
            cell_form_id: 2,
            grid: GridCoordinate::new(1, 0),
            lifecycle: ExteriorCellLifecycle::Queued,
            generation: 4,
            pinned: false,
            estimated_bytes: 0,
            failed_attempts: 0,
        },
        ExteriorCellState {
            cell_form_id: 3,
            grid: GridCoordinate::new(2, 0),
            lifecycle: ExteriorCellLifecycle::Loading,
            generation: 7,
            pinned: false,
            estimated_bytes: 0,
            failed_attempts: 0,
        },
    ];

    let plan = plan_residency(
        ExteriorResidencyInput {
            current_grid: GridCoordinate::new(0, 0),
            velocity_grid: (0, 0),
            resident_budget: 1,
            byte_budget: 0,
            near_radius: 0,
            prefetch_radius: 0,
            distant_radius: None,
        },
        &indexed,
        &states,
    );

    assert_eq!(
        plan.actions
            .iter()
            .filter(|action| action.action == ExteriorLoadAction::Cancel)
            .map(|action| (action.grid, action.generation))
            .collect::<Vec<_>>(),
        vec![
            (GridCoordinate::new(1, 0), 4),
            (GridCoordinate::new(2, 0), 7),
        ]
    );
}

#[test]
fn residency_reverses_an_eviction_when_the_cell_returns_to_the_ring() {
    let indexed = BTreeMap::from([(GridCoordinate::new(1, 0), 2)]);
    let states = vec![ExteriorCellState {
        cell_form_id: 2,
        grid: GridCoordinate::new(1, 0),
        lifecycle: ExteriorCellLifecycle::Evicting,
        generation: 9,
        pinned: false,
        estimated_bytes: 32,
        failed_attempts: 0,
    }];

    let plan = plan_residency(
        ExteriorResidencyInput {
            current_grid: GridCoordinate::new(1, 0),
            velocity_grid: (0, 0),
            resident_budget: 1,
            byte_budget: 32,
            near_radius: 0,
            prefetch_radius: 0,
            distant_radius: None,
        },
        &indexed,
        &states,
    );

    assert_eq!(
        plan.actions,
        vec![ExteriorResidencyAction {
            form_id: 2,
            grid: GridCoordinate::new(1, 0),
            action: ExteriorLoadAction::Cancel,
            generation: 9,
        }]
    );
}

#[test]
fn residency_requests_a_new_generation_after_a_cancelled_load() {
    let indexed = BTreeMap::from([(GridCoordinate::new(0, 0), 1)]);
    let states = vec![ExteriorCellState {
        cell_form_id: 1,
        grid: GridCoordinate::new(0, 0),
        lifecycle: ExteriorCellLifecycle::Unloaded,
        generation: 12,
        pinned: false,
        estimated_bytes: 0,
        failed_attempts: 0,
    }];

    let plan = plan_residency(
        ExteriorResidencyInput {
            current_grid: GridCoordinate::new(0, 0),
            velocity_grid: (0, 0),
            resident_budget: 1,
            byte_budget: 0,
            near_radius: 0,
            prefetch_radius: 0,
            distant_radius: None,
        },
        &indexed,
        &states,
    );

    assert_eq!(
        plan.actions,
        vec![ExteriorResidencyAction {
            form_id: 1,
            grid: GridCoordinate::new(0, 0),
            action: ExteriorLoadAction::Request,
            generation: 13,
        }]
    );
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
fn terrain_lod_selection_covers_base_bands_and_hysteresis_boundaries() {
    let cases = [
        (None, 50.0, TerrainLod::Near),
        (None, 51.0, TerrainLod::Middle),
        (None, 150.0, TerrainLod::Middle),
        (None, 151.0, TerrainLod::Distant),
        (Some(TerrainLod::Near), 140.0, TerrainLod::Near),
        (Some(TerrainLod::Near), 141.0, TerrainLod::Middle),
        (Some(TerrainLod::Middle), 40.0, TerrainLod::Near),
        (Some(TerrainLod::Middle), 41.0, TerrainLod::Middle),
        (Some(TerrainLod::Middle), 160.0, TerrainLod::Middle),
        (Some(TerrainLod::Middle), 161.0, TerrainLod::Distant),
        (Some(TerrainLod::Distant), 140.0, TerrainLod::Middle),
        (Some(TerrainLod::Distant), 141.0, TerrainLod::Distant),
        (Some(TerrainLod::Near), 50.0, TerrainLod::Near),
    ];

    for (previous, distance, expected) in cases {
        assert_eq!(
            select_terrain_lod(distance, previous, 50.0, 150.0, 10.0),
            expected,
            "distance={distance}, previous={previous:?}"
        );
    }

    assert_eq!(
        select_terrain_lod(50.0, Some(TerrainLod::Near), 50.0, 150.0, -10.0),
        TerrainLod::Near,
        "negative hysteresis must be treated as zero"
    );
}

#[test]
fn clamp_lod_delta_preserves_adjacent_pairs_and_clamps_both_directions() {
    let lods = [TerrainLod::Near, TerrainLod::Middle, TerrainLod::Distant];
    let rank = |lod| -> i8 {
        match lod {
            TerrainLod::Near => 0,
            TerrainLod::Middle => 1,
            TerrainLod::Distant => 2,
        }
    };

    for left in lods {
        for right in lods {
            let result = clamp_lod_delta(left, right);
            let difference = (rank(result.0) - rank(result.1)).abs();
            assert!(difference <= 1, "result={result:?}");
            if (rank(left) - rank(right)).abs() <= 1 {
                assert_eq!(result, (left, right));
            }
        }
    }

    assert_eq!(
        clamp_lod_delta(TerrainLod::Distant, TerrainLod::Near),
        (TerrainLod::Middle, TerrainLod::Near)
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
fn deeper_water_never_turns_a_submerged_player_dry() {
    let water = PreparedWater {
        form_id: None,
        height: 2.0,
        water_type_form_id: None,
        swim_depth: 1.0,
    };
    assert_eq!(resolve_water_contact(Some(&water), 1.5).unwrap().depth, 0.5);
    assert!(resolve_water_contact(Some(&water), 1.5).unwrap().submerged);
    assert!(resolve_water_contact(Some(&water), 0.5).unwrap().submerged);
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

#[test]
fn world_location_variants_keep_distinct_identity_and_exact_authored_pose() {
    let exterior = WorldLocation::Exterior(WorldLocationExterior {
        worldspace_form_id: 0x0001_51e3,
        position: [12.345678, 3.500001, -8.125004],
        rotation_xyzw: [0.1234567, -0.7070001, 0.2222222, 0.6543211],
    });
    let interior = WorldLocation::Interior(WorldLocationInterior {
        cell_form_id: 0x0002_0001,
        position: [-4.125003, 7.750002, 0.000007],
        rotation_xyzw: [-0.3333333, 0.4444444, -0.5555555, 0.6666666],
    });

    assert_eq!(exterior.cell_key(), 0x0001_51e3);
    assert_eq!(interior.cell_key(), 0x0002_0001);
    assert!(matches!(exterior, WorldLocation::Exterior(_)));
    assert!(matches!(interior, WorldLocation::Interior(_)));
    assert!(exterior.is_well_formed());
    assert!(interior.is_well_formed());
}

#[test]
fn malformed_world_location_is_not_well_formed() {
    assert!(
        !WorldLocation::Exterior(WorldLocationExterior {
            worldspace_form_id: 0,
            position: [0.0, 0.0, 0.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        })
        .is_well_formed()
    );
    assert!(
        !WorldLocation::Interior(WorldLocationInterior {
            cell_form_id: 1,
            position: [f32::INFINITY, 0.0, 0.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        })
        .is_well_formed()
    );
    assert!(
        !WorldLocation::Interior(WorldLocationInterior {
            cell_form_id: 1,
            position: [0.0, 0.0, 0.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 0.0],
        })
        .is_well_formed()
    );
}

#[test]
fn prepared_door_destinations_preserve_authored_arrival_pose() {
    let interior_destination = PreparedDoorDestination {
        door_reference_form_id: 0x10,
        cell_form_id: 0x20,
        translation: [12.345678, 3.500001, -8.125004],
        rotation_xyzw: [0.1234567, -0.7070001, 0.2222222, 0.6543211],
    };
    let exterior_destination = PreparedExteriorDoorDestination {
        door_reference_form_id: 0x30,
        cell_form_id: 0x40,
        position: [-4.125003, 7.750002, 0.000007],
        rotation_xyzw: [-0.3333333, 0.4444444, -0.5555555, 0.6666666],
    };

    let placement = PreparedSemantic::Door(crate::manifest::PreparedDoor {
        lock_level: None,
        key_form_id: None,
        trapped: false,
        destination: Some(interior_destination.clone()),
    });
    let decoded: PreparedSemantic =
        ron::de::from_str(&ron::ser::to_string(&placement).unwrap()).unwrap();
    assert_eq!(decoded, placement);
    assert_eq!(
        interior_destination.translation,
        [12.345678, 3.500001, -8.125004]
    );
    assert_eq!(
        interior_destination.rotation_xyzw,
        [0.1234567, -0.7070001, 0.2222222, 0.6543211]
    );
    assert_eq!(
        exterior_destination.position,
        [-4.125003, 7.750002, 0.000007]
    );
    assert_eq!(
        exterior_destination.rotation_xyzw,
        [-0.3333333, 0.4444444, -0.5555555, 0.6666666]
    );
}
