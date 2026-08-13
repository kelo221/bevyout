use crate::local_light_policy::{
    ExteriorLocalLightCandidate, ExteriorLocalLightOwner, candidates_for_package,
    rank_exterior_local_lights, select_exterior_local_lights,
    select_exterior_local_lights_for_owner,
};
use crate::manifest::exterior::{
    ExteriorCellPackage, GridCoordinate, PreparedExteriorEnvironment, PreparedExteriorLight,
};

fn light(reference_form_id: u32, position: [f32; 3]) -> PreparedExteriorLight {
    PreparedExteriorLight {
        reference_form_id,
        position,
        color_rgba: [1.0, 0.8, 0.4, 1.0],
        range: 10.0,
    }
}

fn owner(
    worldspace_form_id: u32,
    cell_form_id: u32,
    grid: GridCoordinate,
) -> ExteriorLocalLightOwner {
    ExteriorLocalLightOwner::new(worldspace_form_id, cell_form_id, grid)
}

fn candidate<'a>(
    owner: ExteriorLocalLightOwner,
    light: &'a PreparedExteriorLight,
) -> ExteriorLocalLightCandidate<'a> {
    ExteriorLocalLightCandidate::new(owner, light)
}

fn package(
    owner: ExteriorLocalLightOwner,
    local_lights: Vec<PreparedExteriorLight>,
) -> ExteriorCellPackage {
    ExteriorCellPackage {
        revision: "fixture".into(),
        content_fingerprint: "fixture".into(),
        cell_form_id: owner.cell_form_id,
        worldspace_form_id: owner.worldspace_form_id,
        grid: owner.grid,
        origin: [0.0; 3],
        terrain: None,
        water: None,
        static_objects: Vec::new(),
        dynamic_objects: Vec::new(),
        distant_objects: Vec::new(),
        actors: Vec::new(),
        local_lights,
        navigation: None,
        environment: PreparedExteriorEnvironment::default(),
        diagnostics: Vec::new(),
    }
}

#[test]
fn ranking_is_distance_then_form_id_then_owner() {
    let owner_a = owner(0x10, 0x20, GridCoordinate::new(0, 0));
    let owner_b = owner(0x10, 0x21, GridCoordinate::new(0, 0));
    let nearest_high_id = light(20, [1.0, 0.0, 0.0]);
    let nearest_low_id = light(10, [0.0, 1.0, 0.0]);
    let farther_low_id = light(2, [2.0, 0.0, 0.0]);
    let same_id_owner_b = light(7, [0.0, 3.0, 0.0]);
    let same_id_owner_a = light(7, [3.0, 0.0, 0.0]);

    let ranked = rank_exterior_local_lights(
        [0.0, 0.0, 0.0],
        [
            candidate(owner_a, &farther_low_id),
            candidate(owner_b, &same_id_owner_b),
            candidate(owner_a, &nearest_high_id),
            candidate(owner_a, &same_id_owner_a),
            candidate(owner_a, &nearest_low_id),
        ],
    );

    assert_eq!(
        ranked
            .iter()
            .map(|entry| (
                entry.candidate.light.reference_form_id,
                entry.candidate.owner
            ))
            .collect::<Vec<_>>(),
        vec![
            (10, owner_a),
            (20, owner_a),
            (2, owner_a),
            (7, owner_a),
            (7, owner_b),
        ]
    );
    assert_eq!(ranked[0].distance_squared, 1.0);
    assert_eq!(ranked[2].distance_squared, 4.0);
}

#[test]
fn zero_budget_selects_nothing() {
    let cell_owner = owner(0x10, 0x20, GridCoordinate::new(0, 0));
    let first = light(1, [1.0, 0.0, 0.0]);
    let second = light(2, [2.0, 0.0, 0.0]);

    let selected = select_exterior_local_lights(
        [0.0, 0.0, 0.0],
        0,
        [
            candidate(cell_owner, &first),
            candidate(cell_owner, &second),
        ],
    );

    assert!(selected.is_empty());
}

#[test]
fn fewer_than_budget_keeps_all_valid_candidates() {
    let cell_owner = owner(0x10, 0x20, GridCoordinate::new(0, 0));
    let first = light(1, [1.0, 0.0, 0.0]);
    let second = light(2, [2.0, 0.0, 0.0]);

    let selected = select_exterior_local_lights(
        [0.0, 0.0, 0.0],
        64,
        [
            candidate(cell_owner, &first),
            candidate(cell_owner, &second),
        ],
    );

    assert_eq!(selected.len(), 2);
    assert_eq!(
        selected
            .iter()
            .map(|entry| entry.candidate.light.reference_form_id)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn over_budget_keeps_the_nearest_fixed_number() {
    let cell_owner = owner(0x10, 0x20, GridCoordinate::new(0, 0));
    let nearest = light(1, [1.0, 0.0, 0.0]);
    let middle = light(2, [2.0, 0.0, 0.0]);
    let farthest = light(3, [3.0, 0.0, 0.0]);

    let selected = select_exterior_local_lights(
        [0.0, 0.0, 0.0],
        2,
        [
            candidate(cell_owner, &farthest),
            candidate(cell_owner, &nearest),
            candidate(cell_owner, &middle),
        ],
    );

    assert_eq!(selected.len(), 2);
    assert_eq!(
        selected
            .iter()
            .map(|entry| entry.candidate.light.reference_form_id)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
}

#[test]
fn invalid_candidates_and_observer_positions_are_rejected_deterministically() {
    let valid_owner = owner(0x10, 0x20, GridCoordinate::new(0, 0));
    let valid = light(1, [1.0, 0.0, 0.0]);
    let no_reference = light(0, [1.0, 0.0, 0.0]);
    let non_finite_position = light(2, [f32::NAN, 0.0, 0.0]);
    let mut non_finite_range = light(3, [1.0, 0.0, 0.0]);
    non_finite_range.range = f32::INFINITY;
    let mut zero_range = light(4, [1.0, 0.0, 0.0]);
    zero_range.range = 0.0;
    let mut negative_range = light(5, [1.0, 0.0, 0.0]);
    negative_range.range = -1.0;
    let mut non_finite_color = light(6, [1.0, 0.0, 0.0]);
    non_finite_color.color_rgba[0] = f32::NEG_INFINITY;

    let invalid_owner = owner(0, valid_owner.cell_form_id, valid_owner.grid);
    let ranked = rank_exterior_local_lights(
        [0.0, 0.0, 0.0],
        [
            candidate(valid_owner, &valid),
            candidate(valid_owner, &no_reference),
            candidate(valid_owner, &non_finite_position),
            candidate(valid_owner, &non_finite_range),
            candidate(valid_owner, &zero_range),
            candidate(valid_owner, &negative_range),
            candidate(valid_owner, &non_finite_color),
            candidate(invalid_owner, &valid),
        ],
    );

    assert_eq!(
        ranked
            .iter()
            .map(|entry| entry.candidate.light.reference_form_id)
            .collect::<Vec<_>>(),
        vec![1]
    );
    assert!(
        rank_exterior_local_lights([f32::NAN, 0.0, 0.0], [candidate(valid_owner, &valid)],)
            .is_empty()
    );
}

#[test]
fn package_owner_is_preserved_at_position_boundaries_and_owner_filter_is_exact() {
    let package_owner = owner(0x10, 0x20, GridCoordinate::new(4, -3));
    let foreign_owner = owner(0x10, 0x21, GridCoordinate::new(5, -3));
    let package_light = light(10, [0.0, 0.0, 0.0]);
    let foreign_light = light(11, [0.1, 0.0, 0.0]);
    let package = package(package_owner, vec![package_light]);
    let package_candidates = candidates_for_package(&package).collect::<Vec<_>>();

    assert_eq!(package_candidates.len(), 1);
    assert_eq!(package_candidates[0].owner, package_owner);

    let selected_for_package = select_exterior_local_lights_for_owner(
        package_owner,
        [0.0, 0.0, 0.0],
        1,
        [
            package_candidates[0],
            candidate(foreign_owner, &foreign_light),
        ],
    );

    assert_eq!(selected_for_package.len(), 1);
    assert_eq!(
        selected_for_package[0].candidate.light.reference_form_id,
        10
    );
    assert_eq!(selected_for_package[0].candidate.owner, package_owner);
}
