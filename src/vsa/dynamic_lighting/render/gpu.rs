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
    bevy_bridge::{DynamicLight, DynamicLightRuntime, DynamicLightShadowProxy},
    core::spatial_parameters,
};

pub(super) const MAX_DYNAMIC_LIGHTS: usize = 1024;
pub(super) const GPU_DYNAMIC_LIGHT_SIZE: usize = 112;

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

impl GpuDynamicLight {
    pub(super) fn from_main_world(
        light: &DynamicLight,
        runtime: &DynamicLightRuntime,
        transform: &GlobalTransform,
    ) -> Self {
        let config = light.config;
        let mut channel = (config.light_type as u32) << 6;
        if config.shadow_enabled {
            channel |= 32_768;
        } else {
            channel |= 32;
        }
        if config.cookie_index.is_some() {
            channel |= 65_536;
        }

        let [gp_float_1, gp_float_2, gp_float_3] = spatial_parameters(
            config.light_type,
            runtime.state.elapsed_seconds,
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

        Self {
            position: transform.translation().to_array(),
            radius_sqr: config.radius * config.radius,
            channel,
            intensity: config.intensity * runtime.state.intensity,
            gp_float_1,
            gp_float_2,
            color: config.color,
            gp_float_3,
            up: transform.up().as_vec3().to_array(),
            shimmer_scale: 0.0,
            forward: transform.forward().as_vec3().to_array(),
            shimmer_modifier: 0.0,
            volumetric_intensity: 0.0,
            volumetric_visibility: 0.0,
            cookie_index: config.cookie_index.unwrap_or(u32::MAX),
            shadow_cubemap_index: u32::MAX,
            falloff: config.radius * config.falloff * config.falloff,
            bounce_color: bounce.to_array(),
        }
    }
}

#[derive(Resource)]
pub(super) struct ExtractedDynamicLights {
    pub(super) values: Vec<ExtractedDynamicLight>,
    pub(super) enabled: bool,
    pub(super) shadow_texel_size: f32,
}

pub(super) struct ExtractedDynamicLight {
    pub(super) main_entity: bevy::prelude::Entity,
    pub(super) light: GpuDynamicLight,
}

#[derive(Clone, Copy, Debug, Default, ShaderType)]
pub(super) struct GpuDynamicLightMeta {
    pub(super) count: u32,
    pub(super) enabled: u32,
    pub(super) shadow_texel_size: f32,
    pub(super) shadow_near_z: f32,
}

#[derive(Resource)]
pub(super) struct DynamicLightGpuBuffers {
    pub(super) lights: RawBufferVec<GpuDynamicLight>,
    pub(super) meta: UniformBuffer<GpuDynamicLightMeta>,
}

impl Default for DynamicLightGpuBuffers {
    fn default() -> Self {
        let mut lights = RawBufferVec::new(BufferUsages::STORAGE);
        lights.set_label(Some("dynamic_lighting_gpu_lights"));
        Self {
            lights,
            meta: UniformBuffer::from(GpuDynamicLightMeta::default()),
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
    let proxy_indices = proxies
        .iter()
        .filter_map(|(proxy_entity, proxy)| {
            clusterable_objects
                .entity_to_index
                .get(&proxy_entity)
                .map(|index| (proxy.dynamic_light, *index as u32))
        })
        .collect::<HashMap<_, _>>();
    let (count, enabled, shadow_texel_size) = if let Some(extracted) = extracted {
        for extracted_light in &extracted.values {
            let mut light = extracted_light.light;
            light.shadow_cubemap_index = proxy_indices
                .get(&extracted_light.main_entity)
                .copied()
                .unwrap_or(u32::MAX);
            buffers.lights.push(light);
        }
        (
            extracted.values.len() as u32,
            extracted.enabled as u32,
            extracted.shadow_texel_size,
        )
    } else {
        (0, 0, 0.0)
    };
    if buffers.lights.is_empty() {
        buffers.lights.push(GpuDynamicLight::zeroed());
    }
    buffers.lights.write_buffer(&render_device, &render_queue);
    buffers.meta.set(GpuDynamicLightMeta {
        count,
        enabled,
        shadow_texel_size,
        shadow_near_z: bevy::light::PointLight::DEFAULT_SHADOW_MAP_NEAR_Z,
    });
    buffers.meta.write_buffer(&render_device, &render_queue);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{math::Quat, prelude::Transform};

    use crate::vsa::dynamic_lighting::core::{
        DynamicLightConfig, DynamicLightEffect, DynamicLightType, LightEffectRuntime,
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
    fn extraction_maps_source_fields_and_feature_bits_exactly() {
        let config = DynamicLightConfig {
            color: [0.2, 0.4, 0.8],
            intensity: 3.0,
            radius: 7.0,
            falloff: 0.5,
            light_type: DynamicLightType::Rotor,
            effect: DynamicLightEffect::Pulse,
            shadow_enabled: true,
            cookie_index: Some(7),
            ..Default::default()
        };
        let light = DynamicLight { config };
        let runtime = DynamicLightRuntime {
            state: LightEffectRuntime {
                intensity: 0.4,
                elapsed_seconds: 0.75,
                ..Default::default()
            },
        };
        let transform = GlobalTransform::from(
            Transform::from_xyz(1.0, 2.0, 3.0).with_rotation(Quat::from_rotation_y(0.6)),
        );
        let gpu = GpuDynamicLight::from_main_world(&light, &runtime, &transform);

        assert_eq!(gpu.position, [1.0, 2.0, 3.0]);
        assert_eq!(gpu.radius_sqr, 49.0);
        assert_eq!(gpu.channel, (5 << 6) | 32_768 | 65_536);
        assert!((gpu.intensity - 1.2).abs() <= f32::EPSILON);
        assert_eq!(gpu.color, config.color);
        assert_eq!(gpu.cookie_index, 7);
        assert_eq!(gpu.shadow_cubemap_index, u32::MAX);
        assert_eq!(gpu.falloff, 7.0 * 0.5 * 0.5);
        assert_eq!(gpu.forward, transform.forward().as_vec3().to_array());
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
                    elapsed_seconds: 0.625,
                    ..Default::default()
                },
            };
            let gpu =
                GpuDynamicLight::from_main_world(&light, &runtime, &GlobalTransform::IDENTITY);
            let expected = spatial_parameters(light_type, 0.625, spatial);
            assert_eq!(
                [gpu.gp_float_1, gpu.gp_float_2, gpu.gp_float_3],
                expected,
                "{light_type:?}",
            );
            assert_eq!(gpu.channel & (15 << 6), (light_type as u32) << 6);
        }
    }
}
