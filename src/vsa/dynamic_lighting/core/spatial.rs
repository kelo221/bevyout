use core::f32::consts::{PI, TAU};

#[cfg(test)]
use core::f32::consts::FRAC_PI_2;

use super::{DynamicLightSpatialParameters, DynamicLightType};

/// Produces the three general-purpose shader parameters in the same CPU order
/// as the frozen Unity package.
pub(crate) fn spatial_parameters(
    light_type: DynamicLightType,
    elapsed: f32,
    spatial: DynamicLightSpatialParameters,
) -> [f32; 3] {
    match light_type {
        DynamicLightType::Point => [0.0, 0.0, 0.0],
        DynamicLightType::Spot | DynamicLightType::Discoball => [
            spatial.inner_cutoff_degrees.to_radians().cos(),
            spatial.outer_cutoff_degrees.to_radians().cos(),
            0.0,
        ],
        DynamicLightType::Wave => [
            spatial.wave_offset + elapsed * spatial.wave_speed,
            spatial.wave_frequency * TAU,
            0.0,
        ],
        DynamicLightType::Interference => [
            (spatial.wave_offset + elapsed * spatial.wave_speed) * TAU,
            spatial.wave_frequency * PI,
            0.0,
        ],
        DynamicLightType::Rotor => [
            (spatial.wave_offset + elapsed * spatial.wave_speed) * TAU,
            spatial.wave_frequency.round_ties_even(),
            spatial.rotor_center,
        ],
        DynamicLightType::Shock => [
            spatial.wave_offset + elapsed * spatial.wave_speed,
            spatial.wave_frequency,
            0.0,
        ],
        DynamicLightType::Disco => [
            (spatial.wave_offset + elapsed * spatial.wave_speed) * TAU,
            spatial.wave_frequency.round_ties_even(),
            (spatial.wave_offset + elapsed * spatial.disco_vertical_speed) * TAU,
        ],
    }
}

/// CPU reference for the eight upstream `DynamicLighting.cginc` spatial
/// functions. Runtime pixels use the direct WGSL translation; this mirror is
/// deliberately Bevy-free so Unity-exported checkpoints can test parity.
#[cfg(test)]
pub(crate) fn evaluate_spatial(
    light_type: DynamicLightType,
    radius_sqr: f32,
    position: [f32; 3],
    forward: [f32; 3],
    up: [f32; 3],
    world: [f32; 3],
    parameters: [f32; 3],
) -> f32 {
    let light_minus_world = sub(position, world);
    let light_direction = normalize(light_minus_world);
    let [parameter_a, parameter_b, parameter_c] = parameters;

    match light_type {
        DynamicLightType::Point => 1.0,
        DynamicLightType::Spot => {
            let theta = dot(light_direction, forward);
            safe_cutoff(theta, parameter_a, parameter_b)
        }
        DynamicLightType::Discoball => {
            let rotated = to_light_space(light_direction, forward, up);
            let theta = dot(snap_direction(rotated), rotated);
            safe_cutoff(theta, parameter_a, parameter_b)
        }
        DynamicLightType::Wave => {
            0.7 + 0.3 * ((length(light_minus_world) - parameter_a) * parameter_b).sin()
        }
        DynamicLightType::Interference => {
            let local = to_light_space(light_minus_world, forward, up);
            let angle = (local[0] * local[0] + local[2] * local[2])
                .sqrt()
                .atan2(local[1])
                * parameter_b;
            0.5 + 0.5 * (angle - parameter_a).cos()
        }
        DynamicLightType::Rotor => {
            let local = to_light_space(light_minus_world, forward, up);
            let angle = parameter_b * local[0].atan2(local[2]);
            let mut scale = 0.5 + 0.5 * (angle + parameter_a).cos();
            let absolute_center = radius_sqr * parameter_c.abs();
            let mut distance_sqr = local[0] * local[0] + local[2] * local[2];
            if parameter_c < 0.0 {
                if distance_sqr < absolute_center {
                    scale *= (distance_sqr / absolute_center).powf(PI);
                }
            } else {
                distance_sqr *= 1.0 / absolute_center;
                if distance_sqr < 1.0 {
                    scale = 1.0 - distance_sqr + scale * distance_sqr;
                }
            }
            scale.powf(FRAC_PI_2)
        }
        DynamicLightType::Shock => {
            let distance = parameter_b * length(light_minus_world);
            let mut brightness = 0.9 + 0.1 * ((distance * 2.0 - parameter_a) * TAU).sin();
            brightness *= 0.9 + 0.1 * ((distance + parameter_a) * TAU).cos();
            brightness *= 0.9 + 0.1 * ((distance * 0.5 - parameter_a) * TAU).sin();
            brightness
        }
        DynamicLightType::Disco => {
            let local = to_light_space(light_minus_world, forward, up);
            let horizontal = parameter_b * local[0].atan2(local[2]);
            let vertical = parameter_b
                * (local[0] * local[0] + local[2] * local[2])
                    .sqrt()
                    .atan2(local[1]);
            let scale_1 = 0.5 + 0.5 * (horizontal + parameter_a).cos();
            let scale_2 = 0.5 + 0.5 * (vertical - parameter_c).cos();
            let mut scale = scale_1 + scale_2 - scale_1 * scale_2;
            let distance = 0.5 * (local[0] * local[0] + local[2] * local[2]);
            if distance < 1.0 {
                scale *= distance;
            }
            1.0 - scale
        }
    }
}

#[cfg(test)]
fn safe_cutoff(theta: f32, inner: f32, outer: f32) -> f32 {
    let epsilon = inner - outer;
    if epsilon.abs() <= 1.0e-6 {
        if theta >= inner { 1.0 } else { 0.0 }
    } else {
        ((theta - outer) / epsilon).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
fn to_light_space(value: [f32; 3], forward: [f32; 3], up: [f32; 3]) -> [f32; 3] {
    let right = cross(forward, up);
    [dot(value, right), dot(value, up), dot(value, forward)]
}

#[cfg(test)]
fn snap_direction(mut value: [f32; 3]) -> [f32; 3] {
    let divisor = value[0].abs().max(value[1].abs()).max(value[2].abs());
    for component in &mut value {
        *component /= divisor;
        *component = if component.abs() < (PI / 8.0).tan() {
            0.0
        } else {
            component.signum()
        };
    }
    normalize(value)
}

#[cfg(test)]
fn sub(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

#[cfg(test)]
fn dot(left: [f32; 3], right: [f32; 3]) -> f32 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

#[cfg(test)]
fn cross(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

#[cfg(test)]
fn length(value: [f32; 3]) -> f32 {
    dot(value, value).sqrt()
}

#[cfg(test)]
fn normalize(value: [f32; 3]) -> [f32; 3] {
    let reciprocal = length(value).recip();
    [
        value[0] * reciprocal,
        value[1] * reciprocal,
        value[2] * reciprocal,
    ]
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct SpatialFixture {
        samples: Vec<SpatialSample>,
    }

    #[derive(Deserialize)]
    struct SpatialSample {
        discriminant: usize,
        time: f32,
        world: [f32; 3],
        value: f32,
    }

    #[test]
    fn all_spatial_types_match_unity_6000_3_goldens() {
        let fixture: SpatialFixture =
            serde_json::from_str(include_str!("../tests/golden/unity_spatial_v1.json")).unwrap();
        let spatial = DynamicLightSpatialParameters::default();
        for sample in fixture.samples {
            let light_type = DynamicLightType::ALL[sample.discriminant];
            let actual = evaluate_spatial(
                light_type,
                16.0,
                [0.0; 3],
                [0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0],
                sample.world,
                spatial_parameters(light_type, sample.time, spatial),
            );
            assert!(
                (actual - sample.value).abs() <= 2.0e-5,
                "{light_type:?} t={} world={:?}: Rust {actual} != Unity {}",
                sample.time,
                sample.world,
                sample.value,
            );
        }
    }
}
