//! Thin Bevy bridge for the isolated DynamicLighting core.

use bevy::light::PointLight;
use bevy::prelude::*;

use super::core::{LightEffect, LightEffectState};

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct DynamicLight {
    pub(crate) base_intensity: f32,
    pub(crate) effect: LightEffectState,
    pub(crate) elapsed_seconds: f32,
}

impl DynamicLight {
    pub(crate) fn with_effect(base_intensity: f32, effect: LightEffect) -> Self {
        let state = LightEffectState {
            effect,
            ..Default::default()
        };
        Self {
            base_intensity,
            effect: state,
            elapsed_seconds: 0.0,
        }
    }

    pub(crate) fn strobe(base_intensity: f32, frequency_hz: f32) -> Self {
        Self {
            base_intensity,
            effect: LightEffectState::strobe(frequency_hz),
            elapsed_seconds: 0.0,
        }
    }
}

pub(crate) fn update_dynamic_lights(
    time: Res<Time>,
    mut lights: Query<(&mut PointLight, &mut DynamicLight)>,
) {
    let delta = time.delta_secs().max(0.0);
    for (mut light, mut dynamic) in &mut lights {
        dynamic.elapsed_seconds += delta;
        light.intensity =
            dynamic.base_intensity * dynamic.effect.intensity_multiplier(dynamic.elapsed_seconds);
    }
}
