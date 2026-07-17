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
    // Frozen `DynamicLightManager.PostProcessing`: this deliberately preserves
    // the upstream cone expression, including its empirical 1.0..1.3 factor.
    let cone_multiplier = 1.0 + 0.3 * (outer_cutoff_degrees / 90.0);
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

#[cfg(test)]
fn volumetric_bounding_radius(
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
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct VolumetricSample {
        #[serde(rename = "type")]
        type_name: String,
        discriminant: u32,
        scale: [f32; 3],
        radius: f32,
        thickness: f32,
        intensity: f32,
        visibility: f32,
        temporal_multiplier: f32,
        cone_angle: f32,
        opacity: f32,
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
    fn volumetric_packing_matches_unity_6000_3_fixture() {
        let fixture: VolumetricFixture =
            serde_json::from_str(include_str!("../tests/golden/unity_volumetric_v1.json")).unwrap();
        assert_eq!(
            fixture.upstream_commit,
            "dd7c195cba2599a20bf1b662fa0f69366e0f74b5"
        );
        assert_eq!(fixture.unity_version, "6000.3.17f1");
        let mut covered = [false; 5];
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
                30.0,
            );
            assert_eq!(packed.radius, sample.radius);
            assert_eq!(packed.thickness, sample.thickness);
            assert!(
                (packed.intensity - sample.intensity * sample.temporal_multiplier).abs() < 1e-6
            );
            assert!((packed.inverse_visibility - 1.0 / sample.visibility).abs() < 1e-6);
            assert!((packed.cone_angle - sample.cone_angle).abs() < 1e-6);
            assert!(sample.opacity.is_finite());
            assert!(sample.output_rgb.into_iter().all(f32::is_finite));
        }
        assert!(covered.into_iter().all(|value| value));
    }
}
