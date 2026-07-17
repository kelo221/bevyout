use serde::{Deserialize, Serialize};

/// Numeric values are part of the upstream CPU/GPU contract.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum DynamicLightEffect {
    #[default]
    Steady = 0,
    Pulse = 1,
    Random = 2,
    Strobe = 3,
    Flicker = 4,
    FluorescentStarter = 5,
    FluorescentClicker = 6,
    FluorescentRandom = 7,
    Candle = 8,
    Pulsar = 9,
    Fire = 10,
    Generator = 11,
    Lightning = 12,
    Cloudy = 13,
    Overcast = 14,
}

impl DynamicLightEffect {
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

/// Numeric values are shifted left by six in the upstream shader channel.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum DynamicLightType {
    #[default]
    Point = 0,
    Spot = 1,
    Discoball = 2,
    Wave = 3,
    Interference = 4,
    Rotor = 5,
    Shock = 6,
    Disco = 7,
}

impl DynamicLightType {
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

/// Numeric values are consumed directly by Henry's volumetric post-process.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum DynamicLightVolumetricType {
    #[default]
    None = 0,
    Sphere = 1,
    Box = 2,
    ConeZ = 3,
    ConeY = 4,
}

/// How a light's visibility is obtained.  The numeric values intentionally
/// match Henry's DynamicLighting authoring data so prepared catalogs can be
/// exchanged without a translation table.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum DynamicLightShadowMode {
    #[default]
    RaytracedShadows = 0,
    RealtimeShadows = 1,
    Disabled = 2,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum DynamicLightIlluminationMode {
    #[default]
    DirectIllumination = 0,
    SingleBounce = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum DynamicLightTransparencyMode {
    #[default]
    Disabled = 0,
    AlphaTest = 1,
    AlphaBlend = 2,
}

/// Quantization used for the per-texel single-bounce payload.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum DynamicBounceCompression {
    #[default]
    Bits8,
    Bits6,
    Bits5,
    Bits4,
}

impl DynamicBounceCompression {
    pub(crate) const fn bits(self) -> u8 {
        match self {
            Self::Bits8 => 8,
            Self::Bits6 => 6,
            Self::Bits5 => 5,
            Self::Bits4 => 4,
        }
    }
}

impl DynamicLightVolumetricType {
    #[cfg(test)]
    pub(crate) const ALL: [Self; 5] = [
        Self::None,
        Self::Sphere,
        Self::Box,
        Self::ConeZ,
        Self::ConeY,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants_match_upstream() {
        assert!(
            DynamicLightEffect::ALL
                .into_iter()
                .map(|value| value as u32)
                .eq(0..15)
        );
        assert!(
            DynamicLightType::ALL
                .into_iter()
                .map(|value| value as u32)
                .eq(0..8)
        );
        assert!(
            DynamicLightVolumetricType::ALL
                .into_iter()
                .map(|value| value as u32)
                .eq(0..5)
        );
    }
}
