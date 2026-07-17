use std::collections::HashMap;

use bevy::pbr::GlobalClusterableObjectMeta;
use bevy::{
    math::Vec3,
    prelude::{GlobalTransform, Resource},
    render::{
        render_resource::{BufferUsages, RawBufferVec, ShaderType, UniformBuffer},
        renderer::{RenderDevice, RenderQueue},
    },
};
use bytemuck::{Pod, Zeroable};

use super::super::{
    bevy_bridge::{
        DynamicLight, DynamicLightPreparedShadow, DynamicLightRuntime, DynamicLightShadowProxy,
    },
    core::{
        DynamicLightType, DynamicLightVolumetricType, pack_volumetric_parameters, source_is_valid,
        spatial_parameters, volumetric_is_active,
    },
};

pub(super) const MAX_DYNAMIC_LIGHTS: usize = 1024;
pub(super) const GPU_DYNAMIC_LIGHT_SIZE: usize = 112;
pub(super) const GPU_DYNAMIC_SHADOW_SIZE: usize = 16;

/// Exact seven-block mirror of upstream `ShaderDynamicLight`.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(super) struct GpuDynamicLight {
    pub(super) position: [f32; 3],
    pub(super) radius_sqr: f32,
    pub(super) channel: u32,
    pub(super) intensity: f32,
    pub(super) gp_float_1: f32,
    pub(super) gp_float_2: f32,
    pub(super) color: [f32; 3],
    pub(super) gp_float_3: f32,
    pub(super) up: [f32; 3],
    pub(super) shimmer_scale: f32,
    pub(super) forward: [f32; 3],
    pub(super) shimmer_modifier: f32,
    pub(super) volumetric_intensity: f32,
    pub(super) volumetric_visibility: f32,
    pub(super) cookie_index: u32,
    pub(super) shadow_cubemap_index: u32,
    pub(super) falloff: f32,
    pub(super) bounce_color: [f32; 3],
}

/// Per-light runtime shadow data copied from Bevy after shadow allocation.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(super) struct GpuDynamicShadow {
    pub(super) cubemap_index: u32,
    pub(super) depth_bias: f32,
    pub(super) normal_bias: f32,
    pub(super) near_z: f32,
}

impl Default for GpuDynamicShadow {
    fn default() -> Self {
        Self {
            cubemap_index: u32::MAX,
            depth_bias: 0.0,
            normal_bias: 0.0,
            near_z: 0.0,
        }
    }
}

impl From<DynamicLightPreparedShadow> for GpuDynamicShadow {
    fn from(shadow: DynamicLightPreparedShadow) -> Self {
        Self {
            cubemap_index: shadow.cubemap_index,
            depth_bias: shadow.depth_bias,
            normal_bias: shadow.normal_bias,
            near_z: shadow.near_z,
        }
    }
}

impl GpuDynamicLight {
    pub(super) fn from_main_world(
        light: &DynamicLight,
        runtime: &DynamicLightRuntime,
        transform: &GlobalTransform,
        prepared_source: bool,
    ) -> Self {
        let config = light.config;
        let mut channel = (config.light_type as u32) << 6;
        if config.shadow_enabled {
            channel |= 32_768;
        } else {
            channel |= 32;
        }
        if prepared_source {
            channel |= 16_384;
        }

        let [gp_float_1, gp_float_2, gp_float_3] = spatial_parameters(
            config.light_type,
            runtime.state.animation_time_seconds,
            config.spatial,
        );
        let color = Vec3::from_array(config.color);
        let authored_bounce = Vec3::new(
            config.bounce.color_rgba[0],
            config.bounce.color_rgba[1],
            config.bounce.color_rgba[2],
        );
        let bounce = if config.bounce.enabled {
            color.lerp(authored_bounce, config.bounce.color_rgba[3])
                * config.bounce.modifier
                * config.bounce.intensity
        } else {
            Vec3::ZERO
        };
        let source_forward = match config.light_type {
            DynamicLightType::Discoball
            | DynamicLightType::Interference
            | DynamicLightType::Rotor
            | DynamicLightType::Disco => transform.back(),
            DynamicLightType::Point
            | DynamicLightType::Spot
            | DynamicLightType::Wave
            | DynamicLightType::Shock => transform.forward(),
        };
        Self {
            position: transform.translation().to_array(),
            radius_sqr: if !source_is_valid(&config) {
                -1.0
            } else {
                config.radius * config.radius
            },
            channel,
            intensity: config.intensity * runtime.state.intensity,
            gp_float_1,
            gp_float_2,
            color: config.color,
            gp_float_3,
            up: transform.up().as_vec3().to_array(),
            shimmer_scale: 0.0,
            forward: source_forward.as_vec3().to_array(),
            shimmer_modifier: 0.0,
            volumetric_intensity: 0.0,
            volumetric_visibility: 0.0,
            // Cookie textures are not exposed until the texture-array path is
            // implemented. Preserve the upstream ABI sentinel meanwhile.
            cookie_index: u32::MAX,
            shadow_cubemap_index: u32::MAX,
            falloff: config.radius * config.falloff * config.falloff,
            bounce_color: bounce.to_array(),
        }
    }

    pub(super) fn from_volumetric_main_world(
        light: &DynamicLight,
        runtime: &DynamicLightRuntime,
        transform: &GlobalTransform,
    ) -> Option<Self> {
        let config = light.config;
        if !source_is_valid(&config)
            || !volumetric_is_active(config.volumetric, runtime.state.intensity)
        {
            return None;
        }
        let (scale, _, _) = transform.to_scale_rotation_translation();
        let packed = pack_volumetric_parameters(
            config.volumetric,
            runtime.state.intensity,
            scale.to_array(),
            config.spatial.outer_cutoff_degrees,
        );
        let forward = match packed.volumetric_type {
            DynamicLightVolumetricType::ConeY => transform.up().as_vec3(),
            DynamicLightVolumetricType::ConeZ => transform.back().as_vec3(),
            DynamicLightVolumetricType::None
            | DynamicLightVolumetricType::Sphere
            | DynamicLightVolumetricType::Box => transform.forward().as_vec3(),
        };
        let (gp_float_2, gp_float_3, shimmer_scale) = match packed.volumetric_type {
            DynamicLightVolumetricType::Box => (packed.scale[0], packed.scale[1], packed.scale[2]),
            DynamicLightVolumetricType::ConeZ | DynamicLightVolumetricType::ConeY => {
                (packed.cone_angle, 0.0, 0.0)
            }
            DynamicLightVolumetricType::None | DynamicLightVolumetricType::Sphere => {
                (0.0, 0.0, 0.0)
            }
        };
        Some(Self {
            position: transform.translation().to_array(),
            // The upstream post-process deliberately recycles this direct-light
            // slot to store an unsquared volumetric radius.
            radius_sqr: packed.radius,
            channel: packed.volumetric_type as u32,
            intensity: config.intensity * runtime.state.intensity,
            gp_float_1: packed.thickness,
            gp_float_2,
            color: config.color,
            gp_float_3,
            up: transform.up().as_vec3().to_array(),
            shimmer_scale,
            forward: forward.to_array(),
            shimmer_modifier: 0.0,
            volumetric_intensity: packed.intensity,
            volumetric_visibility: packed.inverse_visibility,
            cookie_index: u32::MAX,
            shadow_cubemap_index: u32::MAX,
            falloff: 0.0,
            bounce_color: [0.0; 3],
        })
    }
}

#[derive(Resource)]
pub(super) struct ExtractedDynamicLights {
    pub(super) values: Vec<ExtractedDynamicLight>,
    pub(super) enabled: bool,
    pub(super) volumetric_values: Vec<GpuDynamicLight>,
    pub(super) volumetric_enabled: bool,
}

pub(super) struct ExtractedDynamicLight {
    pub(super) main_entity: bevy::prelude::Entity,
    pub(super) light: GpuDynamicLight,
    pub(super) prepared_shadow: GpuDynamicShadow,
}

#[derive(Clone, Copy, Debug, Default, ShaderType)]
pub(super) struct GpuDynamicLightMeta {
    pub(super) count: u32,
    pub(super) enabled: u32,
    pub(super) padding_a: f32,
    pub(super) padding_b: f32,
}

#[derive(Resource)]
pub(super) struct DynamicLightGpuBuffers {
    pub(super) lights: RawBufferVec<GpuDynamicLight>,
    pub(super) realtime_shadows: RawBufferVec<GpuDynamicShadow>,
    pub(super) prepared_shadows: RawBufferVec<GpuDynamicShadow>,
    pub(super) meta: UniformBuffer<GpuDynamicLightMeta>,
    pub(super) volumetric_lights: RawBufferVec<GpuDynamicLight>,
    pub(super) volumetric_meta: UniformBuffer<GpuDynamicLightMeta>,
    pub(super) volumetric_count: u32,
    pub(super) volumetric_enabled: bool,
}

impl Default for DynamicLightGpuBuffers {
    fn default() -> Self {
        let mut lights = RawBufferVec::new(BufferUsages::STORAGE);
        lights.set_label(Some("dynamic_lighting_gpu_lights"));
        let mut realtime_shadows = RawBufferVec::new(BufferUsages::STORAGE);
        realtime_shadows.set_label(Some("dynamic_lighting_gpu_realtime_shadows"));
        let mut prepared_shadows = RawBufferVec::new(BufferUsages::STORAGE);
        prepared_shadows.set_label(Some("dynamic_lighting_gpu_prepared_shadows"));
        let mut volumetric_lights = RawBufferVec::new(BufferUsages::STORAGE);
        volumetric_lights.set_label(Some("dynamic_lighting_gpu_volumetric_lights"));
        Self {
            lights,
            realtime_shadows,
            prepared_shadows,
            meta: UniformBuffer::from(GpuDynamicLightMeta::default()),
            volumetric_lights,
            volumetric_meta: UniformBuffer::from(GpuDynamicLightMeta::default()),
            volumetric_count: 0,
            volumetric_enabled: false,
        }
    }
}

pub(super) fn prepare_dynamic_light_buffers(
    extracted: Option<bevy::prelude::Res<ExtractedDynamicLights>>,
    proxies: bevy::prelude::Query<(bevy::prelude::Entity, &DynamicLightShadowProxy)>,
    clusterable_objects: bevy::prelude::Res<GlobalClusterableObjectMeta>,
    mut buffers: bevy::prelude::ResMut<DynamicLightGpuBuffers>,
    render_device: bevy::prelude::Res<RenderDevice>,
    render_queue: bevy::prelude::Res<RenderQueue>,
) {
    buffers.lights.clear();
    buffers.realtime_shadows.clear();
    buffers.prepared_shadows.clear();
    buffers.volumetric_lights.clear();
    let proxy_shadows = proxies
        .iter()
        .filter_map(|(proxy_entity, proxy)| {
            clusterable_objects
                .point_shadow_metadata(proxy_entity)
                .map(|metadata| (proxy.dynamic_light, metadata))
        })
        .collect::<HashMap<_, _>>();
    let (count, enabled, volumetric_count, volumetric_enabled) = if let Some(extracted) = extracted
    {
        for extracted_light in &extracted.values {
            let mut light = extracted_light.light;
            let shadow = proxy_shadows.get(&extracted_light.main_entity).map_or_else(
                GpuDynamicShadow::default,
                |metadata| GpuDynamicShadow {
                    cubemap_index: metadata.cubemap_index,
                    depth_bias: metadata.depth_bias,
                    normal_bias: metadata.normal_bias,
                    near_z: metadata.near_z,
                },
            );
            light.shadow_cubemap_index = shadow.cubemap_index;
            buffers.lights.push(light);
            buffers.realtime_shadows.push(shadow);
            buffers
                .prepared_shadows
                .push(extracted_light.prepared_shadow);
        }
        for light in &extracted.volumetric_values {
            buffers.volumetric_lights.push(*light);
        }
        (
            extracted.values.len() as u32,
            extracted.enabled as u32,
            extracted.volumetric_values.len() as u32,
            extracted.volumetric_enabled,
        )
    } else {
        (0, 0, 0, false)
    };
    if buffers.lights.is_empty() {
        buffers.lights.push(GpuDynamicLight::zeroed());
    }
    if buffers.realtime_shadows.is_empty() {
        buffers.realtime_shadows.push(GpuDynamicShadow::default());
    }
    if buffers.prepared_shadows.is_empty() {
        buffers.prepared_shadows.push(GpuDynamicShadow::default());
    }
    if buffers.volumetric_lights.is_empty() {
        buffers.volumetric_lights.push(GpuDynamicLight::zeroed());
    }
    buffers.lights.write_buffer(&render_device, &render_queue);
    buffers
        .realtime_shadows
        .write_buffer(&render_device, &render_queue);
    buffers
        .prepared_shadows
        .write_buffer(&render_device, &render_queue);
    buffers
        .volumetric_lights
        .write_buffer(&render_device, &render_queue);
    buffers.meta.set(GpuDynamicLightMeta {
        count,
        enabled,
        padding_a: 0.0,
        padding_b: 0.0,
    });
    buffers.meta.write_buffer(&render_device, &render_queue);
    buffers.volumetric_meta.set(GpuDynamicLightMeta {
        count: volumetric_count,
        enabled: volumetric_enabled as u32,
        padding_a: 0.0,
        padding_b: 0.0,
    });
    buffers
        .volumetric_meta
        .write_buffer(&render_device, &render_queue);
    buffers.volumetric_count = volumetric_count;
    buffers.volumetric_enabled = volumetric_enabled;
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{
        math::{EulerRot, Quat},
        prelude::Transform,
    };

    use crate::vsa::dynamic_lighting::core::{
        DynamicLightBounceParameters, DynamicLightConfig, DynamicLightEffect,
        DynamicLightSpatialParameters, DynamicLightType, DynamicLightVolumetricParameters,
        DynamicLightVolumetricType, LightEffectRuntime,
    };

    #[test]
    fn gpu_abi_is_seven_exact_sixteen_byte_blocks() {
        assert_eq!(size_of::<GpuDynamicLight>(), GPU_DYNAMIC_LIGHT_SIZE);
        assert_eq!(align_of::<GpuDynamicLight>(), 16);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, position), 0);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, radius_sqr), 12);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, channel), 16);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, intensity), 20);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, gp_float_1), 24);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, gp_float_2), 28);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, color), 32);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, gp_float_3), 44);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, up), 48);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, shimmer_scale), 60);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, forward), 64);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, shimmer_modifier), 76);
        assert_eq!(
            core::mem::offset_of!(GpuDynamicLight, volumetric_intensity),
            80
        );
        assert_eq!(
            core::mem::offset_of!(GpuDynamicLight, volumetric_visibility),
            84
        );
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, cookie_index), 88);
        assert_eq!(
            core::mem::offset_of!(GpuDynamicLight, shadow_cubemap_index),
            92
        );
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, falloff), 96);
        assert_eq!(core::mem::offset_of!(GpuDynamicLight, bounce_color), 100);
    }

    #[test]
    fn gpu_shadow_metadata_is_one_exact_block_with_an_invalid_default() {
        let shadow = GpuDynamicShadow::default();

        assert_eq!(size_of::<GpuDynamicShadow>(), GPU_DYNAMIC_SHADOW_SIZE);
        assert_eq!(align_of::<GpuDynamicShadow>(), 16);
        assert_eq!(shadow.cubemap_index, u32::MAX);
        assert_eq!(shadow.depth_bias, 0.0);
        assert_eq!(shadow.normal_bias, 0.0);
        assert_eq!(shadow.near_z, 0.0);
    }

    #[test]
    fn extraction_maps_source_fields_and_feature_bits_exactly() {
        let config = DynamicLightConfig {
            color: [0.2, 0.4, 0.8],
            intensity: 3.0,
            radius: 7.0,
            falloff: 0.5,
            light_type: DynamicLightType::Rotor,
            effect: DynamicLightEffect::Pulse,
            shadow_enabled: true,
            bounce: DynamicLightBounceParameters {
                enabled: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let light = DynamicLight { config };
        let runtime = DynamicLightRuntime {
            state: LightEffectRuntime {
                intensity: 0.4,
                animation_time_seconds: 0.75,
                ..Default::default()
            },
            ..Default::default()
        };
        let transform = GlobalTransform::from(
            Transform::from_xyz(1.0, 2.0, 3.0).with_rotation(Quat::from_rotation_y(0.6)),
        );
        let gpu = GpuDynamicLight::from_main_world(&light, &runtime, &transform, false);

        assert_eq!(gpu.position, [1.0, 2.0, 3.0]);
        assert_eq!(gpu.radius_sqr, 49.0);
        assert_eq!(gpu.channel, (5 << 6) | 32_768);
        assert!((gpu.intensity - 1.2).abs() <= f32::EPSILON);
        assert_eq!(gpu.color, config.color);
        assert_eq!(gpu.cookie_index, u32::MAX);
        assert_eq!(gpu.shadow_cubemap_index, u32::MAX);
        assert_eq!(gpu.falloff, 7.0 * 0.5 * 0.5);
        assert_eq!(gpu.forward, transform.back().as_vec3().to_array());
        assert_eq!(gpu.up, transform.up().as_vec3().to_array());
        assert_eq!(gpu.bounce_color, config.color);
    }

    #[test]
    fn every_spatial_type_maps_general_purpose_parameters() {
        let spatial = DynamicLightConfig::default().spatial;
        for light_type in DynamicLightType::ALL {
            let light = DynamicLight {
                config: DynamicLightConfig {
                    light_type,
                    spatial,
                    ..Default::default()
                },
            };
            let runtime = DynamicLightRuntime {
                state: LightEffectRuntime {
                    intensity: 1.0,
                    animation_time_seconds: 0.625,
                    ..Default::default()
                },
                ..Default::default()
            };
            let gpu = GpuDynamicLight::from_main_world(
                &light,
                &runtime,
                &GlobalTransform::IDENTITY,
                false,
            );
            let expected = spatial_parameters(light_type, 0.625, spatial);
            assert_eq!(
                [gpu.gp_float_1, gpu.gp_float_2, gpu.gp_float_3],
                expected,
                "{light_type:?}",
            );
            assert_eq!(gpu.channel & (15 << 6), (light_type as u32) << 6);
        }
    }

    #[test]
    fn rotated_source_axes_match_unity_per_light_type() {
        let transform = GlobalTransform::from(Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            0.61,
            -0.37,
            0.22,
        )));
        for light_type in DynamicLightType::ALL {
            let light = DynamicLight {
                config: DynamicLightConfig {
                    light_type,
                    ..Default::default()
                },
            };
            let gpu = GpuDynamicLight::from_main_world(
                &light,
                &DynamicLightRuntime::default(),
                &transform,
                false,
            );
            let expected = match light_type {
                DynamicLightType::Discoball
                | DynamicLightType::Interference
                | DynamicLightType::Rotor
                | DynamicLightType::Disco => transform.back(),
                _ => transform.forward(),
            };
            assert_eq!(gpu.forward, expected.as_vec3().to_array(), "{light_type:?}");
        }
    }

    #[test]
    fn invalid_spot_and_discoball_cutoffs_disable_the_gpu_light() {
        for light_type in [DynamicLightType::Spot, DynamicLightType::Discoball] {
            for (inner, outer) in [(26.0, 20.0), (0.0, 0.0)] {
                let light = DynamicLight {
                    config: DynamicLightConfig {
                        light_type,
                        spatial: DynamicLightSpatialParameters {
                            inner_cutoff_degrees: inner,
                            outer_cutoff_degrees: outer,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                };
                let gpu = GpuDynamicLight::from_main_world(
                    &light,
                    &DynamicLightRuntime::default(),
                    &GlobalTransform::IDENTITY,
                    false,
                );
                assert_eq!(gpu.radius_sqr, -1.0, "{light_type:?} {inner}/{outer}");
            }
        }
    }

    #[test]
    fn invalid_spot_and_discoball_cutoffs_disable_every_volumetric_shape() {
        for light_type in [DynamicLightType::Spot, DynamicLightType::Discoball] {
            for volumetric_type in [
                DynamicLightVolumetricType::Sphere,
                DynamicLightVolumetricType::Box,
                DynamicLightVolumetricType::ConeZ,
                DynamicLightVolumetricType::ConeY,
            ] {
                for (inner, outer) in [(26.0, 20.0), (0.0, 0.0)] {
                    let mut config = DynamicLightConfig {
                        light_type,
                        spatial: DynamicLightSpatialParameters {
                            inner_cutoff_degrees: inner,
                            outer_cutoff_degrees: outer,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    config.volumetric.volumetric_type = volumetric_type;
                    let runtime = DynamicLightRuntime {
                        state: LightEffectRuntime {
                            intensity: 1.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    assert!(
                        GpuDynamicLight::from_volumetric_main_world(
                            &DynamicLight { config },
                            &runtime,
                            &GlobalTransform::IDENTITY,
                        )
                        .is_none(),
                        "{light_type:?} {volumetric_type:?} {inner}/{outer}",
                    );
                }
            }
        }
    }

    #[test]
    fn volumetric_extraction_recycles_the_upstream_abi_exactly() {
        let light = DynamicLight {
            config: DynamicLightConfig {
                color: [0.2, 0.6, 0.9],
                spatial: crate::vsa::dynamic_lighting::core::DynamicLightSpatialParameters {
                    outer_cutoff_degrees: 30.0,
                    ..Default::default()
                },
                volumetric: DynamicLightVolumetricParameters {
                    volumetric_type: DynamicLightVolumetricType::Box,
                    radius: 5.0,
                    thickness: 2.5,
                    intensity: 0.6,
                    visibility: 4.0,
                },
                ..Default::default()
            },
        };
        let runtime = DynamicLightRuntime {
            state: LightEffectRuntime {
                intensity: 0.25,
                ..Default::default()
            },
            ..Default::default()
        };
        let transform = GlobalTransform::from(
            Transform::from_xyz(1.0, 2.0, 3.0).with_scale(Vec3::new(1.0, 2.0, 3.0)),
        );
        let gpu = GpuDynamicLight::from_volumetric_main_world(&light, &runtime, &transform)
            .expect("active box volume");
        assert_eq!(gpu.position, [1.0, 2.0, 3.0]);
        assert_eq!(gpu.radius_sqr, 5.0);
        assert_eq!(gpu.channel, DynamicLightVolumetricType::Box as u32);
        assert_eq!(gpu.gp_float_1, 2.5);
        assert_eq!(
            [gpu.gp_float_2, gpu.gp_float_3, gpu.shimmer_scale],
            [1.0, 2.0, 3.0]
        );
        assert_eq!(gpu.volumetric_intensity, 0.15);
        assert_eq!(gpu.volumetric_visibility, 0.25);
        assert_eq!(gpu.color, light.config.color);
    }

    #[test]
    fn cone_y_uses_the_transformed_up_axis_and_none_is_filtered() {
        let mut config = DynamicLightConfig::default();
        config.volumetric.volumetric_type = DynamicLightVolumetricType::ConeY;
        let light = DynamicLight { config };
        let runtime = DynamicLightRuntime {
            state: LightEffectRuntime {
                intensity: 1.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let transform =
            GlobalTransform::from(Transform::IDENTITY.with_rotation(Quat::from_rotation_z(0.4)));
        let gpu = GpuDynamicLight::from_volumetric_main_world(&light, &runtime, &transform)
            .expect("active cone volume");
        assert_eq!(gpu.channel, DynamicLightVolumetricType::ConeY as u32);
        assert_eq!(gpu.forward, transform.up().as_vec3().to_array());
        assert!((gpu.gp_float_2 - -0.55).abs() < 0.000001);

        config.volumetric.volumetric_type = DynamicLightVolumetricType::None;
        assert!(
            GpuDynamicLight::from_volumetric_main_world(
                &DynamicLight { config },
                &runtime,
                &transform,
            )
            .is_none()
        );
    }

    #[test]
    fn cone_z_uses_unitys_positive_local_z_axis() {
        let mut config = DynamicLightConfig::default();
        config.volumetric.volumetric_type = DynamicLightVolumetricType::ConeZ;
        let transform =
            GlobalTransform::from(Transform::IDENTITY.with_rotation(Quat::from_rotation_y(0.73)));
        let gpu = GpuDynamicLight::from_volumetric_main_world(
            &DynamicLight { config },
            &DynamicLightRuntime {
                state: LightEffectRuntime {
                    intensity: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            &transform,
        )
        .expect("active cone Z volume");
        assert_eq!(gpu.forward, transform.back().as_vec3().to_array());
    }
}
