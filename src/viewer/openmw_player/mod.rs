//! OpenMW-derived locomotion rules for the FPS viewer.
//!
//! This module keeps the small, portable part of OpenMW's actor controller
//! separate from the Bevy presentation code: jump profile selection, reduced
//! air control, and fall/landing state transitions.  The provenance and
//! adaptation notes live beside this file in `README.md` and `NOTICE.md`.

use bevy::prelude::*;

pub(crate) const GRAVITY: f32 = 8.96;
pub(crate) const AIR_CONTROL_FACTOR: f32 = 0.5;
pub(crate) const STATIONARY_JUMP_HEIGHT: f32 = 1.2;
// Direction changes the launch direction, not the vertical arc. Keeping the
// same height avoids the forward-jump dip that was especially noticeable in
// the native BoxDDD controller.
pub(crate) const DIRECTIONAL_JUMP_HEIGHT: f32 = STATIONARY_JUMP_HEIGHT;
pub(crate) const DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE: f32 = STATIONARY_JUMP_HEIGHT;
pub(crate) const MIN_LANDING_SOUND_DISTANCE: f32 = 0.1;
pub(crate) const HARD_LANDING_DISTANCE: f32 = 400.0 / 70.0;

#[derive(Component, Debug, Default)]
pub(crate) struct LocomotionState {
    last_position: Option<Vec3>,
    was_airborne: bool,
    jump_active: bool,
    phase: LocomotionPhase,
    fall_distance: f32,
    landing_index: usize,
    jump_pressed: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum LocomotionPhase {
    #[default]
    Grounded,
    Rising,
    Falling,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LandingImpact {
    pub(crate) distance: f32,
    pub(crate) hard: bool,
    pub(crate) variant: usize,
}

impl LocomotionState {
    pub(crate) fn reset(&mut self, position: Vec3) {
        self.last_position = Some(position);
        self.was_airborne = false;
        self.jump_active = false;
        self.phase = LocomotionPhase::Grounded;
        self.fall_distance = 0.0;
        self.jump_pressed = false;
    }

    pub(crate) fn jump_was_pressed(&self) -> bool {
        self.jump_pressed
    }

    pub(crate) fn set_jump_pressed(&mut self, pressed: bool) {
        self.jump_pressed = pressed;
    }

    pub(crate) fn mark_jump_started(&mut self) {
        self.jump_active = true;
        self.phase = LocomotionPhase::Rising;
    }

    pub(crate) fn next_landing_variant(&mut self) -> usize {
        let variant = self.landing_index;
        self.landing_index = self.landing_index.wrapping_add(1);
        variant
    }

    pub(crate) fn update(
        &mut self,
        position: Vec3,
        vertical_velocity: f32,
        airborne: bool,
    ) -> Option<LandingImpact> {
        let Some(last_position) = self.last_position.replace(position) else {
            self.was_airborne |= airborne;
            return None;
        };

        if airborne {
            if position.y < last_position.y {
                self.fall_distance += last_position.y - position.y;
            }
            self.was_airborne = true;
            self.phase = if vertical_velocity > 0.0 {
                LocomotionPhase::Rising
            } else {
                LocomotionPhase::Falling
            };
            return None;
        }

        let landing_distance = self.fall_distance;
        let should_emit_landing = self.was_airborne
            && (self.jump_active || landing_distance >= MIN_LANDING_SOUND_DISTANCE);
        let impact = should_emit_landing.then_some(LandingImpact {
            distance: landing_distance,
            hard: landing_distance >= HARD_LANDING_DISTANCE,
            variant: self.next_landing_variant(),
        });
        if self.was_airborne {
            self.jump_active = false;
        }
        self.was_airborne = false;
        self.phase = LocomotionPhase::Grounded;
        self.fall_distance = 0.0;
        impact
    }
}

pub(crate) fn has_directional_input(input: Vec3) -> bool {
    input.length_squared() > f32::EPSILON
}

pub(crate) fn jump_profile(input: Vec3) -> (f32, Option<Vec3>) {
    if has_directional_input(input) {
        (DIRECTIONAL_JUMP_HEIGHT, Some(input.normalize_or_zero()))
    } else {
        (STATIONARY_JUMP_HEIGHT, None)
    }
}

pub(crate) fn air_control_motion(input: Vec3, airborne: bool) -> Vec3 {
    let motion = input.normalize_or_zero();
    if airborne {
        motion * AIR_CONTROL_FACTOR
    } else {
        motion
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
