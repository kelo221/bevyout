use serde::{Deserialize, Serialize};

use super::types::{DynamicLightEffect, DynamicLightType};

pub(crate) const DEFAULT_BOUNCE_MULTIPLIER: f32 = 1.0;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct DynamicLightEffectParameters {
    pub(crate) pulse_speed: f32,
    pub(crate) pulse_modifier: f32,
    pub(crate) pulse_offset: f32,
    pub(crate) timestep_seconds: f32,
}

impl Default for DynamicLightEffectParameters {
    fn default() -> Self {
        Self {
            pulse_speed: 1.0,
            pulse_modifier: 0.25,
            pulse_offset: 0.0,
            timestep_seconds: 1.0 / 30.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct DynamicLightSpatialParameters {
    pub(crate) inner_cutoff_degrees: f32,
    pub(crate) outer_cutoff_degrees: f32,
    pub(crate) wave_speed: f32,
    pub(crate) wave_frequency: f32,
    pub(crate) wave_offset: f32,
    pub(crate) rotor_center: f32,
    pub(crate) disco_vertical_speed: f32,
}

impl Default for DynamicLightSpatialParameters {
    fn default() -> Self {
        Self {
            inner_cutoff_degrees: 26.0,
            outer_cutoff_degrees: 30.0,
            wave_speed: 1.0,
            wave_frequency: 1.0,
            wave_offset: 0.0,
            rotor_center: 0.1,
            disco_vertical_speed: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct DynamicLightBounceParameters {
    pub(crate) color_rgba: [f32; 4],
    pub(crate) modifier: f32,
    pub(crate) intensity: f32,
    /// User-requested port policy: a single diffuse bounce is enabled by default.
    pub(crate) enabled: bool,
}

impl Default for DynamicLightBounceParameters {
    fn default() -> Self {
        Self {
            color_rgba: [1.0, 1.0, 1.0, 0.0],
            modifier: 1.0,
            intensity: DEFAULT_BOUNCE_MULTIPLIER,
            enabled: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct DynamicLightConfig {
    pub(crate) color: [f32; 3],
    pub(crate) intensity: f32,
    pub(crate) radius: f32,
    pub(crate) falloff: f32,
    pub(crate) light_type: DynamicLightType,
    pub(crate) effect: DynamicLightEffect,
    pub(crate) effect_parameters: DynamicLightEffectParameters,
    pub(crate) spatial: DynamicLightSpatialParameters,
    pub(crate) bounce: DynamicLightBounceParameters,
    pub(crate) view_mask: u32,
    pub(crate) shadow_enabled: bool,
    pub(crate) cookie_index: Option<u32>,
}

impl Default for DynamicLightConfig {
    fn default() -> Self {
        Self {
            color: [1.0; 3],
            intensity: 2.0,
            radius: 4.0,
            falloff: 0.0,
            light_type: DynamicLightType::Point,
            effect: DynamicLightEffect::Steady,
            effect_parameters: DynamicLightEffectParameters::default(),
            spatial: DynamicLightSpatialParameters::default(),
            bounce: DynamicLightBounceParameters::default(),
            view_mask: u32::MAX,
            shadow_enabled: false,
            cookie_index: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_upstream_and_default_bounce_policy() {
        let config = DynamicLightConfig::default();
        assert_eq!(config.color, [1.0; 3]);
        assert_eq!(config.intensity, 2.0);
        assert_eq!(config.radius, 4.0);
        assert_eq!(config.falloff, 0.0);
        assert_eq!(config.spatial.inner_cutoff_degrees, 26.0);
        assert_eq!(config.spatial.outer_cutoff_degrees, 30.0);
        assert_eq!(config.effect_parameters.pulse_modifier, 0.25);
        assert_eq!(config.effect_parameters.timestep_seconds, 1.0 / 30.0);
        assert_eq!(config.bounce.intensity, DEFAULT_BOUNCE_MULTIPLIER);
        assert!(config.bounce.enabled);
    }

    #[test]
    fn serde_round_trip_preserves_every_parameter_meaning() {
        let mut config = DynamicLightConfig {
            color: [0.2, 0.4, 0.8],
            intensity: 7.25,
            radius: 13.0,
            falloff: 0.75,
            light_type: DynamicLightType::Disco,
            effect: DynamicLightEffect::FluorescentRandom,
            view_mask: 0x1234,
            shadow_enabled: true,
            cookie_index: Some(9),
            ..Default::default()
        };
        config.effect_parameters.pulse_speed = 2.5;
        config.spatial.wave_frequency = 4.0;
        config.bounce.color_rgba = [0.7, 0.3, 0.2, 0.6];

        let json = serde_json::to_string(&config).unwrap();
        let decoded: DynamicLightConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, config);
    }
}
