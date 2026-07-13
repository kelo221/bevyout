use super::*;

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
