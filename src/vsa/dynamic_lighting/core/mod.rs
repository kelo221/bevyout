//! Bevy-free DynamicLighting authoring data and source-compatible runtime.

mod config;
mod fixed_timestep;
mod runtime;
mod spatial;
mod types;
mod unity_math;
mod unity_random;
mod volumetric;

#[cfg(test)]
pub(crate) use config::DynamicLightBounceParameters;
pub(crate) use config::{
    DEFAULT_BOUNCE_MULTIPLIER, DynamicLightConfig, DynamicLightEffectParameters,
    DynamicLightSpatialParameters, DynamicLightVolumetricParameters, source_is_valid,
};
pub(crate) use runtime::{LightEffectRuntime, advance_effect};
pub(crate) use spatial::spatial_parameters;
pub(crate) use types::{DynamicLightEffect, DynamicLightType, DynamicLightVolumetricType};
pub(crate) use unity_random::UnityRandom;
pub(crate) use volumetric::{
    pack_volumetric_parameters, volumetric_bounding_radius, volumetric_is_active,
};
