//! Bevy-free DynamicLighting authoring data and source-compatible runtime.

mod config;
mod fixed_timestep;
mod runtime;
mod spatial;
mod types;
mod unity_math;
mod unity_random;

pub(crate) use config::{
    DEFAULT_BOUNCE_MULTIPLIER, DynamicLightConfig, DynamicLightSpatialParameters,
};
pub(crate) use runtime::{LightEffectRuntime, advance_effect};
pub(crate) use spatial::spatial_parameters;
pub(crate) use types::{DynamicLightEffect, DynamicLightType};
pub(crate) use unity_random::UnityRandom;
