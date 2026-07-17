//! Isolated port boundary for Henry de Jongh's DynamicLighting concepts.
//!
//! The pure effect model stays independent from Bevy. The small bridge module
//! is the only part that owns ECS components and runtime systems. Static point
//! shadow preparation and the existing irradiance baker remain separate
//! systems until their contracts are deliberately migrated here.

mod bevy_bridge;
pub(crate) mod core;

pub(crate) use bevy_bridge::{DynamicLight, update_dynamic_lights};
pub(crate) use core::{DEFAULT_BOUNCE_MULTIPLIER, LightEffect};
