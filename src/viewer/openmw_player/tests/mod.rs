use super::*;
use bevyout_core::manifest::exterior::PreparedWater;

#[test]
fn default_step_height_matches_openmw_thirty_four_units() {
    assert!((DEFAULT_STEP_HEIGHT - 0.485_775).abs() < 0.000_001);
}

#[test]
fn directional_jump_keeps_full_vertical_arc() {
    let (height, direction) = jump_profile(Vec3::new(3.0, 0.0, 4.0));
    assert_eq!(height, STATIONARY_JUMP_HEIGHT);
    assert_eq!(height, DIRECTIONAL_JUMP_HEIGHT);
    assert_eq!(direction, Some(Vec3::new(0.6, 0.0, 0.8)));
}

#[test]
fn stationary_jump_keeps_full_height() {
    let (height, direction) = jump_profile(Vec3::ZERO);
    assert_eq!(height, STATIONARY_JUMP_HEIGHT);
    assert_eq!(direction, None);
}

#[test]
fn air_control_is_halved_only_while_airborne() {
    let input = Vec3::new(0.0, 0.0, -1.0);
    assert_eq!(air_control_motion(input, false), input);
    assert_eq!(air_control_motion(input, true), input * AIR_CONTROL_FACTOR);
}

#[test]
fn landing_accumulates_descent_and_marks_hard_impacts() {
    let mut state = LocomotionState::default();
    state.update(Vec3::ZERO, 0.0, false);
    state.update(Vec3::new(0.0, 1.0, 0.0), 4.0, true);
    assert_eq!(state.phase, LocomotionPhase::Rising);
    state.update(Vec3::new(0.0, 0.5, 0.0), -2.0, true);
    assert_eq!(state.phase, LocomotionPhase::Falling);
    let impact = state
        .update(Vec3::ZERO, 0.0, false)
        .expect("airborne-to-ground transition should land");
    assert!((impact.distance - 0.5).abs() < f32::EPSILON);
    assert!(!impact.hard);
    assert_eq!(impact.variant, 0);
}

#[test]
fn hard_landing_threshold_matches_openmw_fall_distance() {
    let mut state = LocomotionState::default();
    state.update(Vec3::ZERO, 0.0, false);
    state.update(Vec3::new(0.0, HARD_LANDING_DISTANCE + 1.0, 0.0), 1.0, true);
    state.update(Vec3::ZERO, -8.0, true);
    let impact = state
        .update(Vec3::ZERO, 0.0, false)
        .expect("the hard fall should produce a landing impact");
    assert!(impact.hard);
}

#[test]
fn jump_landing_sound_survives_a_ceiling_stop_without_downward_travel() {
    let mut state = LocomotionState::default();
    state.update(Vec3::ZERO, 0.0, false);
    state.mark_jump_started();
    assert!(state.update(Vec3::ZERO, 4.0, false).is_none());
    state.update(Vec3::ZERO, 0.0, true);
    let impact = state
        .update(Vec3::ZERO, 0.0, false)
        .expect("a jump that hits a ceiling must still land audibly");
    assert_eq!(impact.distance, 0.0);
    assert!(!impact.hard);
}

#[test]
fn tiny_airborne_transitions_are_silent() {
    let mut state = LocomotionState::default();
    state.update(Vec3::ZERO, 0.0, false);
    state.update(Vec3::new(0.0, 0.02, 0.0), 1.0, true);
    assert!(state.update(Vec3::ZERO, 0.0, false).is_none());
}

#[test]
fn water_policy_distinguishes_dry_surface_entry_exit_and_deep_contact() {
    let water = PreparedWater {
        form_id: None,
        height: 2.0,
        water_type_form_id: None,
        swim_depth: 1.0,
    };

    let dry = resolve_water_policy(false, None, 1.0);
    assert_eq!(dry.contact, None);
    assert_eq!(dry.phase, WaterPhase::Dry);
    assert_eq!(dry.transition, WaterTransition::None);

    let surface = resolve_water_policy(false, Some(&water), 2.0);
    assert_eq!(surface.contact.expect("surface contact").depth, 0.0);
    assert_eq!(surface.phase, WaterPhase::Surface);
    assert_eq!(surface.transition, WaterTransition::None);

    let entered = resolve_water_policy(false, Some(&water), 1.5);
    assert_eq!(entered.contact.expect("entry contact").depth, 0.5);
    assert_eq!(entered.phase, WaterPhase::Submerged);
    assert_eq!(entered.transition, WaterTransition::Entered);

    let deep = resolve_water_policy(true, Some(&water), 0.5);
    assert_eq!(deep.contact.expect("deep contact").depth, 1.5);
    assert_eq!(deep.phase, WaterPhase::Submerged);
    assert_eq!(deep.transition, WaterTransition::None);

    let exited = resolve_water_policy(true, Some(&water), 2.0);
    assert_eq!(exited.phase, WaterPhase::Surface);
    assert_eq!(exited.transition, WaterTransition::Exited);
}

#[test]
fn invalid_water_contact_is_dry_and_can_emit_exit() {
    let invalid = PreparedWater {
        form_id: None,
        height: f32::NAN,
        water_type_form_id: None,
        swim_depth: 1.0,
    };
    let result = resolve_water_policy(true, Some(&invalid), 1.0);
    assert_eq!(result.contact, None);
    assert_eq!(result.phase, WaterPhase::Dry);
    assert_eq!(result.transition, WaterTransition::Exited);

    let invalid_player = resolve_water_policy(
        false,
        Some(&PreparedWater {
            height: 2.0,
            ..invalid
        }),
        f32::INFINITY,
    );
    assert_eq!(invalid_player.contact, None);
    assert_eq!(invalid_player.phase, WaterPhase::Dry);
}

#[test]
fn breath_policy_clamps_recovery_and_exposes_exhaustion() {
    let drained = advance_breath(5.0, 20.0, true, 2.0);
    assert_eq!(drained.remaining_seconds, 3.0);
    assert_eq!(drained.consequence, BreathConsequence::None);

    let exhausted = advance_breath(1.0, 20.0, true, 2.0);
    assert_eq!(exhausted.remaining_seconds, 0.0);
    assert_eq!(exhausted.consequence, BreathConsequence::Exhausted);

    let recovered = advance_breath(0.0, 20.0, false, 2.0);
    assert_eq!(recovered.remaining_seconds, 3.0);
    assert_eq!(recovered.consequence, BreathConsequence::None);

    let capped = advance_breath(19.5, 20.0, false, 1.0);
    assert_eq!(capped.remaining_seconds, 20.0);
    assert_eq!(capped.consequence, BreathConsequence::None);
}

#[test]
fn breath_policy_is_stable_when_elapsed_time_is_split_into_chunks() {
    fn simulate(initial: f32, submerged: bool, chunks: &[f32]) -> BreathUpdate {
        chunks.iter().fold(
            BreathUpdate {
                remaining_seconds: initial,
                consequence: BreathConsequence::None,
            },
            |state, elapsed| advance_breath(state.remaining_seconds, 20.0, submerged, *elapsed),
        )
    }

    let direct_drain = advance_breath(10.0, 20.0, true, 1.0);
    let split_drain = simulate(10.0, true, &[0.25, 0.25, 0.25, 0.25]);
    assert_eq!(split_drain, direct_drain);

    let direct_recovery = advance_breath(2.0, 20.0, false, 1.0);
    let split_recovery = simulate(2.0, false, &[0.25, 0.25, 0.25, 0.25]);
    assert_eq!(split_recovery, direct_recovery);
}

#[test]
fn landing_policy_keeps_tiny_and_hard_thresholds_exact() {
    assert_eq!(landing_impact(false, false, HARD_LANDING_DISTANCE, 0), None);
    assert_eq!(
        landing_impact(true, false, MIN_LANDING_SOUND_DISTANCE - 0.0001, 1,),
        None
    );

    let ordinary = landing_impact(true, false, MIN_LANDING_SOUND_DISTANCE, 2)
        .expect("the exact ordinary landing threshold should emit");
    assert_eq!(ordinary.distance, MIN_LANDING_SOUND_DISTANCE);
    assert!(!ordinary.hard);
    assert_eq!(ordinary.variant, 2);

    assert!(
        !landing_impact(true, false, HARD_LANDING_DISTANCE - 0.0001, 3)
            .expect("the ordinary landing should still emit")
            .hard
    );
    assert!(
        landing_impact(true, false, HARD_LANDING_DISTANCE, 4)
            .expect("the exact hard threshold should emit")
            .hard
    );
}
