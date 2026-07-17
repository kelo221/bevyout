//! Isolated port boundary for Henry de Jongh's DynamicLighting.

mod bevy_bridge;
pub(crate) mod core;
mod render;

pub(crate) use bevy_bridge::{
    DynamicLight, DynamicLightShadowProxy, DynamicLightingDiagnostics, DynamicLightingPlugin,
    DynamicLightingSettings,
};
pub(crate) use core::{DEFAULT_BOUNCE_MULTIPLIER, DynamicLightEffect, DynamicLightType};
pub(crate) use render::DynamicLightingView;
