//! Shared lighting units and deterministic bake inputs.
//!
//! The viewer and bake paths must interpret the same prepared light values.
//! Keep this module Bevy-free so the transport contract can be tested without
//! starting a renderer.

/// Runtime lighting scale used by the viewer's neutral presentation profile.
pub const DEFAULT_LIGHTING_SCALE: f32 = 128.0;

/// Historical prepared-light reference scale. Prepared fallback intensities
/// are expressed relative to this value and then converted to the active
/// runtime/bake lighting scale.
pub const AUTHORED_LIGHTING_SCALE: f32 = 8_192.0;

/// Brightness multiplier used by the current cell ambient presentation.
pub const AMBIENT_BRIGHTNESS: f32 = 25.0;

/// Neutral ambient presentation multiplier used by the current viewer.
pub const DEFAULT_AMBIENT_SCALE: f32 = 0.05;

/// Cell directional-light illuminance when the prepared directional color is
/// non-zero and finite.
pub const CELL_DIRECTIONAL_ILLUMINANCE: f32 = 10_000.0;

/// Physical contribution of an emissive material in the current bake scale.
pub const EMISSION_SCALE: f32 = 0.01;

/// Convert a prepared point-light value into the intensity consumed by both
/// the Bevy runtime and the CPU transport integrator.
pub fn point_light_intensity(radius: f32, intensity_lumens: f32, lighting_scale: f32) -> f32 {
    let radius = if radius.is_finite() {
        radius.max(0.01)
    } else {
        0.01
    };
    let authored_intensity = if intensity_lumens.is_finite() && intensity_lumens > 0.0 {
        intensity_lumens
    } else {
        radius * radius * 2.0 * AUTHORED_LIGHTING_SCALE
    };
    (authored_intensity * lighting_scale / AUTHORED_LIGHTING_SCALE).max(0.0)
}

/// Convert a prepared sRGB cell color to linear RGB.
pub fn srgb_to_linear_rgb(color: [f32; 3]) -> [f32; 3] {
    color.map(srgb_to_linear)
}

/// Convert prepared cell ambient into irradiance units used by the transport
/// integrator. The integrator divides this by PI when it needs incident
/// radiance for an escaped sample.
pub fn ambient_irradiance(
    ambient_rgba: [f32; 4],
    lighting_scale: f32,
    ambient_scale: f32,
) -> [f32; 3] {
    let color = srgb_to_linear_rgb([ambient_rgba[0], ambient_rgba[1], ambient_rgba[2]]);
    color.map(|channel| (channel * AMBIENT_BRIGHTNESS * lighting_scale * ambient_scale).max(0.0))
}

/// Resolve the prepared cell directional-light flag without allowing invalid
/// authored values to reach either bake or runtime.
pub fn cell_directional_illuminance(directional_rgba: [f32; 4]) -> f32 {
    let luminance = directional_rgba[0] + directional_rgba[1] + directional_rgba[2];
    if luminance.is_finite() && luminance > f32::EPSILON {
        CELL_DIRECTIONAL_ILLUMINANCE
    } else {
        0.0
    }
}

fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
#[path = "tests/lighting.rs"]
mod tests;
