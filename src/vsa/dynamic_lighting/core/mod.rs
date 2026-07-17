//! Bevy-free DynamicLighting data and deterministic effect evaluation.

mod effects;
mod types;

pub(crate) use effects::intensity_multiplier;
pub(crate) use types::{DEFAULT_BOUNCE_MULTIPLIER, LightEffect, LightEffectState};
