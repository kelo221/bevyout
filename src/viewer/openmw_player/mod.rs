//! OpenMW-derived locomotion rules for the FPS viewer.
//!
//! This module keeps the small, portable part of OpenMW's actor controller
//! separate from the Bevy presentation code: jump profile selection, reduced
//! air control, and fall/landing state transitions.  The provenance and
//! adaptation notes live beside this file in `README.md` and `NOTICE.md`.

use bevy::prelude::*;

// These policy modules are deliberately wired before their W4-C runtime
// adapter exists; keep their deferred adapter surface warning-free for the
// current wave.
#[allow(dead_code)]
mod breath;
#[allow(dead_code)]
mod water;

#[allow(unused_imports)]
pub(crate) use breath::{
    BREATH_DRAIN_PER_SECOND, BREATH_RECOVERY_PER_SECOND, BreathConsequence, BreathUpdate,
    advance_breath,
};
#[allow(unused_imports)]
pub(crate) use water::{WaterPhase, WaterPolicyResult, WaterTransition, resolve_water_policy};

pub(crate) const GRAVITY: f32 = 8.96;
pub(crate) const AIR_CONTROL_FACTOR: f32 = 0.5;
pub(crate) const STATIONARY_JUMP_HEIGHT: f32 = 1.2;
// Direction changes the launch direction, not the vertical arc. Keeping the
// same height avoids the forward-jump dip that was especially noticeable in
// the native BoxDDD controller.
pub(crate) const DIRECTIONAL_JUMP_HEIGHT: f32 = STATIONARY_JUMP_HEIGHT;
pub(crate) const DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE: f32 = STATIONARY_JUMP_HEIGHT;
/// OpenMW's default 34-unit upward step converted with its metric scale.
pub(crate) const DEFAULT_STEP_HEIGHT: f32 = 34.0 / 69.991_25;
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
        let impact = landing_impact(
            self.was_airborne,
            self.jump_active,
            landing_distance,
            self.landing_index,
        );
        if impact.is_some() {
            self.landing_index = self.landing_index.wrapping_add(1);
        }
        if self.was_airborne {
            self.jump_active = false;
        }
        self.was_airborne = false;
        self.phase = LocomotionPhase::Grounded;
        self.fall_distance = 0.0;
        impact
    }
}

pub(crate) fn landing_impact(
    was_airborne: bool,
    jump_active: bool,
    fall_distance: f32,
    variant: usize,
) -> Option<LandingImpact> {
    let distance = if fall_distance.is_finite() {
        fall_distance.max(0.0)
    } else {
        0.0
    };
    if !was_airborne || (!jump_active && distance < MIN_LANDING_SOUND_DISTANCE) {
        return None;
    }
    Some(LandingImpact {
        distance,
        hard: distance >= HARD_LANDING_DISTANCE,
        variant,
    })
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
#[path = "tests/mod.rs"]
mod tests;
