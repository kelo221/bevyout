use serde::{Deserialize, Serialize};

/// Default indirect-light contribution for a prepared light.
pub(crate) const DEFAULT_BOUNCE_MULTIPLIER: f32 = 1.0;

/// Intensity effects implemented by Henry's DynamicLighting manager.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) enum LightEffect {
    Steady,
    Pulse,
    Random,
    Strobe,
    Flicker,
    FluorescentStarter,
    FluorescentClicker,
    FluorescentRandom,
    Candle,
    Pulsar,
    Fire,
    Generator,
    Lightning,
    Cloudy,
    Overcast,
}

impl LightEffect {
    // Kept as a complete source-package catalog for import validation and
    // future editor/manifest adapters. Runtime dispatch matches on the enum
    // directly, so the catalog is intentionally not needed in every build.
    #[allow(dead_code)]
    pub(crate) const ALL: [Self; 15] = [
        Self::Steady,
        Self::Pulse,
        Self::Random,
        Self::Strobe,
        Self::Flicker,
        Self::FluorescentStarter,
        Self::FluorescentClicker,
        Self::FluorescentRandom,
        Self::Candle,
        Self::Pulsar,
        Self::Fire,
        Self::Generator,
        Self::Lightning,
        Self::Cloudy,
        Self::Overcast,
    ];
}

/// Spatial light distributions from the source package. These are separate
/// from intensity effects: they need shader-side projection/spot data rather
/// than only a scalar point-light multiplier.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) enum DynamicLightType {
    Point,
    Spot,
    Discoball,
    Wave,
    Interference,
    Rotor,
    Shock,
    Disco,
}

impl DynamicLightType {
    #[allow(dead_code)]
    pub(crate) const ALL: [Self; 8] = [
        Self::Point,
        Self::Spot,
        Self::Discoball,
        Self::Wave,
        Self::Interference,
        Self::Rotor,
        Self::Shock,
        Self::Disco,
    ];
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct LightEffectState {
    pub(crate) effect: LightEffect,
    /// Frequency used by square-wave strobe effects.
    pub(crate) frequency_hz: f32,
    /// Minimum intensity for effects that dim instead of switching fully off.
    pub(crate) duty_cycle: f32,
    pub(crate) pulse_speed: f32,
    pub(crate) pulse_modifier: f32,
    pub(crate) pulse_offset: f32,
    /// Fixed update interval used by random/flicker effects.
    pub(crate) timestep_seconds: f32,
    pub(crate) lightning_layers: u8,
    pub(crate) random_seed: u32,
    pub(crate) bounce_multiplier: f32,
}

impl Default for LightEffectState {
    fn default() -> Self {
        Self {
            effect: LightEffect::Steady,
            frequency_hz: 1.0,
            duty_cycle: 0.5,
            pulse_speed: 1.0,
            pulse_modifier: 0.25,
            pulse_offset: 0.0,
            timestep_seconds: 1.0 / 30.0,
            lightning_layers: 1,
            random_seed: 0,
            bounce_multiplier: DEFAULT_BOUNCE_MULTIPLIER,
        }
    }
}

impl LightEffectState {
    pub(crate) const fn strobe(frequency_hz: f32) -> Self {
        Self {
            effect: LightEffect::Strobe,
            frequency_hz,
            duty_cycle: 0.5,
            pulse_speed: 1.0,
            pulse_modifier: 0.25,
            pulse_offset: 0.0,
            timestep_seconds: 1.0 / 30.0,
            lightning_layers: 1,
            random_seed: 0,
            bounce_multiplier: DEFAULT_BOUNCE_MULTIPLIER,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_effect(mut self, effect: LightEffect) -> Self {
        self.effect = effect;
        self
    }

    pub(crate) fn intensity_multiplier(self, elapsed_seconds: f32) -> f32 {
        super::intensity_multiplier(self, elapsed_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_source_intensity_effect_is_represented() {
        assert_eq!(LightEffect::ALL.len(), 15);
    }

    #[test]
    fn every_source_light_type_is_represented() {
        assert_eq!(DynamicLightType::ALL.len(), 8);
    }

    #[test]
    fn bounce_is_enabled_by_default() {
        assert_eq!(LightEffectState::default().bounce_multiplier, 1.0);
        assert_eq!(DEFAULT_BOUNCE_MULTIPLIER, 1.0);
    }

    #[test]
    fn strobe_is_deterministic_and_has_a_half_duty_cycle() {
        let strobe = LightEffectState::strobe(2.0);
        assert_eq!(strobe.intensity_multiplier(0.0), 1.0);
        assert_eq!(strobe.intensity_multiplier(0.24), 1.0);
        assert_eq!(strobe.intensity_multiplier(0.26), 0.25);
        assert_eq!(strobe.intensity_multiplier(0.51), 1.0);
        assert_eq!(strobe.intensity_multiplier(1.01), 1.0);
    }

    #[test]
    fn invalid_strobe_frequency_falls_back_to_steady() {
        let mut strobe = LightEffectState::strobe(f32::NAN);
        assert_eq!(strobe.intensity_multiplier(0.2), 1.0);
        strobe.frequency_hz = -1.0;
        assert_eq!(strobe.intensity_multiplier(0.2), 1.0);
    }
}
