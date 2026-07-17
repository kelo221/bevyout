//! Bevy-free source-compatible volumetric authoring and packing policy.

use super::{DynamicLightVolumetricParameters, DynamicLightVolumetricType};

const MIN_VISIBILITY: f32 = 0.00001;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PackedVolumetricParameters {
    pub(crate) volumetric_type: DynamicLightVolumetricType,
    pub(crate) radius: f32,
    pub(crate) thickness: f32,
    pub(crate) intensity: f32,
    pub(crate) inverse_visibility: f32,
    pub(crate) scale: [f32; 3],
    pub(crate) cone_angle: f32,
}

pub(crate) fn volumetric_is_active(
    parameters: DynamicLightVolumetricParameters,
    runtime_intensity: f32,
) -> bool {
    parameters.volumetric_type != DynamicLightVolumetricType::None
        && parameters.radius > 0.0
        && parameters.intensity * runtime_intensity != 0.0
}

pub(crate) fn pack_volumetric_parameters(
    parameters: DynamicLightVolumetricParameters,
    runtime_intensity: f32,
    transform_scale: [f32; 3],
    outer_cutoff_degrees: f32,
) -> PackedVolumetricParameters {
    // Frozen `DynamicLightManager.PostProcessing`: Unity's Mathf.Lerp clamps
    // its interpolation parameter before applying this empirical 1.0..1.3
    // factor.
    let cone_multiplier = 1.0 + 0.3 * (outer_cutoff_degrees / 90.0).clamp(0.0, 1.0);
    let cone_angle =
        (core::f32::consts::FRAC_PI_2 + outer_cutoff_degrees.to_radians()).cos() * cone_multiplier;
    PackedVolumetricParameters {
        volumetric_type: parameters.volumetric_type,
        radius: parameters.radius,
        thickness: parameters.thickness,
        intensity: parameters.intensity * runtime_intensity,
        inverse_visibility: 1.0 / parameters.visibility.max(MIN_VISIBILITY),
        scale: transform_scale,
        cone_angle,
    }
}

/// CPU mirror of the frozen post-process opacity calculation. The runtime
/// shader is the production path; this remains Bevy-free so Unity-exported
/// rays can exercise the same sphere, box, cone, clipping, visibility, and
/// thickness decisions in ordinary tests.
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn evaluate_volumetric_opacity(
    volumetric_type: DynamicLightVolumetricType,
    ray_start: [f32; 3],
    ray_end: [f32; 3],
    center: [f32; 3],
    direction: [f32; 3],
    scale: [f32; 3],
    radius: f32,
    thickness: f32,
    intensity: f32,
    inverse_visibility: f32,
    cone_angle: f32,
) -> f32 {
    if volumetric_type == DynamicLightVolumetricType::None || radius <= 0.0 || intensity == 0.0 {
        return 0.0;
    }

    let mut opacity = 0.0;
    if volumetric_type == DynamicLightVolumetricType::Sphere {
        let line = sub(ray_end, ray_start);
        let length_sqr = dot(line, line);
        let line_t = if length_sqr <= 0.000_001 {
            0.0
        } else {
            (dot(sub(center, ray_start), line) / length_sqr).clamp(0.0, 1.0)
        };
        let closest = add(ray_start, mul(line, line_t));
        let center_distance = length(sub(closest, center));
        if center_distance < radius + 0.000_01 {
            opacity = (radius - center_distance) / radius;
        }
    } else {
        let ray = sub(ray_end, ray_start);
        let max_depth = length(ray);
        if max_depth > 0.000_001 {
            let ray_direction = mul(ray, max_depth.recip());
            let hit = if volumetric_type == DynamicLightVolumetricType::Box {
                let extent = component_mul([radius; 3], scale);
                ray_box_intersection(
                    ray_start,
                    ray_direction,
                    sub(center, extent),
                    add(center, extent),
                    max_depth,
                )
            } else {
                ray_cone_intersection(
                    ray_start,
                    ray_direction,
                    center,
                    normalize(direction),
                    cone_angle,
                    radius,
                    max_depth,
                )
            };
            if let Some((near, far)) = hit {
                let denominator = if volumetric_type == DynamicLightVolumetricType::Box {
                    length(mul(component_mul([radius; 3], scale), 2.0))
                } else {
                    radius
                };
                opacity = (far - near) / denominator;
            }
        }
    }

    opacity = smoothstep_01(opacity);
    opacity = (opacity * thickness).clamp(0.0, 1.0);
    opacity = opacity.min(length(sub(ray_end, ray_start)) * inverse_visibility);
    opacity * intensity
}

/// CPU mirror of the shader's order-independent screen accumulation followed
/// by its maximum-opacity source blend.
#[cfg(test)]
pub(crate) fn compose_volumetric(source: [f32; 3], volumes: &[([f32; 3], f32)]) -> [f32; 3] {
    let mut fog = [0.0; 3];
    let mut maximum_opacity = 0.0_f32;
    for &(color, opacity) in volumes {
        fog = screen_color(fog, mul(color, opacity));
        maximum_opacity = maximum_opacity.max(opacity);
    }
    let screened = screen_color(fog, source);
    lerp(screened, fog, maximum_opacity.clamp(0.0, 1.0))
}

#[cfg(test)]
fn ray_box_intersection(
    origin: [f32; 3],
    direction: [f32; 3],
    minimum: [f32; 3],
    maximum: [f32; 3],
    max_depth: f32,
) -> Option<(f32, f32)> {
    let inverse = direction.map(f32::recip);
    let first = component_mul(sub(minimum, origin), inverse);
    let second = component_mul(sub(maximum, origin), inverse);
    let near = [
        first[0].min(second[0]),
        first[1].min(second[1]),
        first[2].min(second[2]),
    ];
    let far = [
        first[0].max(second[0]),
        first[1].max(second[1]),
        first[2].max(second[2]),
    ];
    let minimum_t = 0.0_f32.max(near[0].max(near[1].max(near[2])));
    let maximum_t = max_depth.min(far[0].min(far[1].min(far[2])));
    (maximum_t >= minimum_t).then_some((minimum_t, maximum_t))
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn ray_cone_intersection(
    origin: [f32; 3],
    direction: [f32; 3],
    tip: [f32; 3],
    cone_direction: [f32; 3],
    cone_angle: f32,
    cone_distance: f32,
    max_depth: f32,
) -> Option<(f32, f32)> {
    let w = sub(origin, tip);
    let cosine = cone_angle.cos();
    let cosine_sqr = cosine * cosine;
    let sine_sqr = 1.0 - cosine_sqr;
    let direction_dot_axis = dot(direction, cone_direction);
    let w_dot_axis = dot(w, cone_direction);
    let direction_dot_w = dot(direction, w);
    let w_dot_w = dot(w, w);
    let a = direction_dot_axis * direction_dot_axis - cosine_sqr;
    let b = 2.0 * (direction_dot_axis * w_dot_axis - cosine_sqr * direction_dot_w);
    let c = w_dot_axis * w_dot_axis - cosine_sqr * w_dot_w;
    let discriminant = b * b - 4.0 * a * c;
    let inside = w_dot_axis >= 0.0
        && w_dot_axis <= cone_distance
        && w_dot_w - w_dot_axis * w_dot_axis
            <= w_dot_axis * w_dot_axis * sine_sqr / cosine_sqr.max(0.000_001);
    let mut hits = [0.0; 3];
    let mut hit_count = 0;
    if discriminant >= 0.0 && a.abs() > 0.000_001 {
        let root = discriminant.sqrt();
        for hit in [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)] {
            let height = w_dot_axis + hit * direction_dot_axis;
            if hit >= 0.0 && height >= 0.0 && height <= cone_distance {
                hits[hit_count] = hit;
                hit_count += 1;
            }
        }
    }
    let cap_denominator = dot(direction, cone_direction);
    if cap_denominator.abs() > 0.000_1 {
        let cap = (cone_distance - w_dot_axis) / cap_denominator;
        if cap >= 0.0 {
            let point = add(w, mul(direction, cap));
            let height = w_dot_axis + cap * direction_dot_axis;
            let radial_sqr = dot(point, point) - height * height;
            let cap_radius_sqr =
                cone_distance * cone_distance * sine_sqr / cosine_sqr.max(0.000_001);
            if radial_sqr <= cap_radius_sqr {
                hits[hit_count] = cap;
                hit_count += 1;
            }
        }
    }
    if !inside && hit_count == 0 {
        return None;
    }
    hits[..hit_count].sort_by(f32::total_cmp);
    let minimum_t = if inside { 0.0 } else { hits[0] };
    let maximum_t = if hit_count == 0 {
        0.0
    } else {
        hits[hit_count - 1].min(max_depth)
    };
    (maximum_t >= minimum_t).then_some((minimum_t, maximum_t))
}

#[cfg(test)]
fn smoothstep_01(value: f32) -> f32 {
    let value = value.clamp(0.0, 1.0);
    value * value * (3.0 - 2.0 * value)
}

#[cfg(test)]
fn screen_color(first: [f32; 3], second: [f32; 3]) -> [f32; 3] {
    [
        1.0 - (1.0 - first[0]) * (1.0 - second[0]),
        1.0 - (1.0 - first[1]) * (1.0 - second[1]),
        1.0 - (1.0 - first[2]) * (1.0 - second[2]),
    ]
}

#[cfg(test)]
fn lerp(first: [f32; 3], second: [f32; 3], amount: f32) -> [f32; 3] {
    add(first, mul(sub(second, first), amount))
}

#[cfg(test)]
fn add(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] + right[0], left[1] + right[1], left[2] + right[2]]
}

#[cfg(test)]
fn sub(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

#[cfg(test)]
fn mul(value: [f32; 3], scalar: f32) -> [f32; 3] {
    [value[0] * scalar, value[1] * scalar, value[2] * scalar]
}

#[cfg(test)]
fn component_mul(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] * right[0], left[1] * right[1], left[2] * right[2]]
}

#[cfg(test)]
fn dot(left: [f32; 3], right: [f32; 3]) -> f32 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

#[cfg(test)]
fn length(value: [f32; 3]) -> f32 {
    dot(value, value).sqrt()
}

#[cfg(test)]
fn normalize(value: [f32; 3]) -> [f32; 3] {
    mul(value, length(value).recip())
}

pub(crate) fn volumetric_bounding_radius(
    parameters: DynamicLightVolumetricParameters,
    transform_scale: [f32; 3],
    outer_cutoff_degrees: f32,
) -> f32 {
    match parameters.volumetric_type {
        DynamicLightVolumetricType::Box => {
            let size = transform_scale.map(|axis| parameters.radius * axis.abs());
            (size[0] * size[0] + size[1] * size[1] + size[2] * size[2]).sqrt()
        }
        DynamicLightVolumetricType::ConeZ | DynamicLightVolumetricType::ConeY => {
            let mut angle = outer_cutoff_degrees.clamp(0.0, 75.0);
            if outer_cutoff_degrees > 90.0 {
                let inverse_lerp = ((outer_cutoff_degrees - 115.0) / 65.0).clamp(0.0, 1.0);
                angle = (1.0 - inverse_lerp) * 75.0;
            }
            parameters.radius / angle.to_radians().cos()
        }
        DynamicLightVolumetricType::None | DynamicLightVolumetricType::Sphere => parameters.radius,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct VolumetricFixture {
        upstream_commit: String,
        unity_version: String,
        samples: Vec<VolumetricSample>,
        composition_samples: Vec<VolumetricCompositionSample>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct VolumetricSample {
        #[serde(rename = "type")]
        type_name: String,
        discriminant: u32,
        camera: [f32; 3],
        world: [f32; 3],
        center: [f32; 3],
        direction: [f32; 3],
        scale: [f32; 3],
        radius: f32,
        thickness: f32,
        intensity: f32,
        visibility: f32,
        temporal_multiplier: f32,
        outer_cutoff_degrees: f32,
        cone_angle: f32,
        opacity: f32,
        source_color: [f32; 3],
        fog_color: [f32; 3],
        output_rgb: [f32; 3],
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct VolumetricCompositionSample {
        source_color: [f32; 3],
        fog_color_a: [f32; 3],
        opacity_a: f32,
        fog_color_b: [f32; 3],
        opacity_b: f32,
        output_rgb: [f32; 3],
    }

    #[test]
    fn inactive_sources_match_upstream_filtering() {
        let defaults = DynamicLightVolumetricParameters::default();
        assert!(!volumetric_is_active(defaults, 1.0));
        assert!(!volumetric_is_active(
            DynamicLightVolumetricParameters {
                volumetric_type: DynamicLightVolumetricType::Sphere,
                radius: 0.0,
                ..defaults
            },
            1.0,
        ));
        assert!(!volumetric_is_active(
            DynamicLightVolumetricParameters {
                volumetric_type: DynamicLightVolumetricType::Sphere,
                intensity: 0.0,
                ..defaults
            },
            1.0,
        ));
    }

    #[test]
    fn packing_preserves_temporal_visibility_box_and_cone_parameters() {
        let packed = pack_volumetric_parameters(
            DynamicLightVolumetricParameters {
                volumetric_type: DynamicLightVolumetricType::Box,
                visibility: 2.0,
                ..Default::default()
            },
            0.25,
            [1.0, 2.0, 3.0],
            30.0,
        );
        assert_eq!(packed.intensity, 0.1875);
        assert_eq!(packed.inverse_visibility, 0.5);
        assert_eq!(packed.scale, [1.0, 2.0, 3.0]);
        assert!((packed.cone_angle - -0.55).abs() < 0.000001);
        assert!(
            (volumetric_bounding_radius(
                DynamicLightVolumetricParameters {
                    volumetric_type: DynamicLightVolumetricType::Box,
                    radius: 4.0,
                    ..Default::default()
                },
                [1.0, 2.0, 3.0],
                30.0,
            ) - (224.0_f32).sqrt())
            .abs()
                < 0.000001
        );
    }

    #[test]
    fn cone_multiplier_clamps_like_unity_mathf_lerp() {
        let parameters = DynamicLightVolumetricParameters {
            volumetric_type: DynamicLightVolumetricType::ConeZ,
            ..Default::default()
        };
        for cutoff in [-30.0, 0.0, 30.0, 90.0, 120.0, 180.0] {
            let packed = pack_volumetric_parameters(parameters, 1.0, [1.0; 3], cutoff);
            let lerp_t = (cutoff / 90.0).clamp(0.0, 1.0);
            let expected_multiplier = 1.0 + 0.3 * lerp_t;
            let expected =
                (core::f32::consts::FRAC_PI_2 + cutoff.to_radians()).cos() * expected_multiplier;
            assert!((packed.cone_angle - expected).abs() < 1.0e-6, "{cutoff}");
        }
    }

    #[test]
    fn volumetric_packing_matches_unity_6000_3_fixture() {
        let fixture: VolumetricFixture =
            serde_json::from_str(include_str!("../tests/golden/unity_volumetric_v1.json")).unwrap();
        assert_eq!(
            fixture.upstream_commit,
            "dd7c195cba2599a20bf1b662fa0f69366e0f74b5"
        );
        assert_eq!(fixture.unity_version, "6000.3.17f1");
        let mut covered = [false; 5];
        assert!(
            fixture.samples.len() >= 18,
            "expanded ray coverage regressed"
        );
        for sample in fixture.samples {
            let volumetric_type = DynamicLightVolumetricType::ALL[sample.discriminant as usize];
            assert_eq!(format!("{volumetric_type:?}"), sample.type_name);
            covered[sample.discriminant as usize] = true;
            let packed = pack_volumetric_parameters(
                DynamicLightVolumetricParameters {
                    volumetric_type,
                    radius: sample.radius,
                    thickness: sample.thickness,
                    intensity: sample.intensity,
                    visibility: sample.visibility,
                },
                sample.temporal_multiplier,
                sample.scale,
                sample.outer_cutoff_degrees,
            );
            assert_eq!(packed.radius, sample.radius);
            assert_eq!(packed.thickness, sample.thickness);
            assert!(
                (packed.intensity - sample.intensity * sample.temporal_multiplier).abs() < 1e-6
            );
            assert!((packed.inverse_visibility - 1.0 / sample.visibility).abs() < 1e-6);
            assert!((packed.cone_angle - sample.cone_angle).abs() < 1e-6);
            let actual_opacity = evaluate_volumetric_opacity(
                volumetric_type,
                sample.camera,
                sample.world,
                sample.center,
                sample.direction,
                sample.scale,
                sample.radius,
                sample.thickness,
                sample.intensity * sample.temporal_multiplier,
                packed.inverse_visibility,
                packed.cone_angle,
            );
            assert!(
                (actual_opacity - sample.opacity).abs() <= 2.0e-5,
                "{} opacity: Rust {actual_opacity} != Unity {}",
                sample.type_name,
                sample.opacity,
            );
            let actual_output =
                compose_volumetric(sample.source_color, &[(sample.fog_color, actual_opacity)]);
            assert_vec3_close(
                actual_output,
                sample.output_rgb,
                &format!("{} output", sample.type_name),
            );
        }
        assert!(covered.into_iter().all(|value| value));
        assert!(!fixture.composition_samples.is_empty());
        for sample in fixture.composition_samples {
            let actual = compose_volumetric(
                sample.source_color,
                &[
                    (sample.fog_color_a, sample.opacity_a),
                    (sample.fog_color_b, sample.opacity_b),
                ],
            );
            assert_vec3_close(actual, sample.output_rgb, "overlap output");
        }
    }

    fn assert_vec3_close(actual: [f32; 3], expected: [f32; 3], context: &str) {
        for axis in 0..3 {
            assert!(
                (actual[axis] - expected[axis]).abs() <= 2.0e-5,
                "{context} axis {axis}: Rust {} != Unity {}",
                actual[axis],
                expected[axis],
            );
        }
    }
}
