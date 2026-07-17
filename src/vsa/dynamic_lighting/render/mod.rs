//! Isolated render-world bridge and HDR lighting pass.

mod extract;
mod gpu;
mod pipeline;

use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::{
        ExtractSchedule, Render, RenderApp, RenderStartup, RenderSystems,
        extract_component::{ExtractComponent, ExtractComponentPlugin},
    },
    shader::load_shader_library,
};

use self::{
    extract::extract_dynamic_lights,
    gpu::{DynamicLightGpuBuffers, prepare_dynamic_light_buffers},
    pipeline::{
        dynamic_lighting_pass, init_dynamic_lighting_pipeline, prepare_dynamic_lighting_pipelines,
    },
};
use super::bevy_bridge::DynamicLightShadowProxy;

/// Opt-in marker for a camera whose opaque HDR target receives custom lights.
#[derive(Clone, Component, Copy, Debug, Default, ExtractComponent)]
pub(crate) struct DynamicLightingView;

pub(crate) struct DynamicLightingRenderPlugin;

impl Plugin for DynamicLightingRenderPlugin {
    fn build(&self, app: &mut App) {
        if app.get_sub_app(RenderApp).is_none() {
            return;
        }

        load_shader_library!(app, "../shaders/dynamic_lighting_types.wgsl");
        load_shader_library!(app, "../shaders/dynamic_lighting_common.wgsl");
        load_shader_library!(app, "../shaders/dynamic_lighting_spatial.wgsl");
        load_shader_library!(app, "../shaders/dynamic_lighting_surface.wgsl");
        embedded_asset!(app, "../shaders/dynamic_lighting_pass.wgsl");
        embedded_asset!(app, "../shaders/dynamic_lighting_volumetric.wgsl");
        app.add_plugins((
            ExtractComponentPlugin::<DynamicLightingView>::default(),
            ExtractComponentPlugin::<DynamicLightShadowProxy>::default(),
        ));

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .init_resource::<DynamicLightGpuBuffers>()
            .init_resource::<bevy::render::render_resource::SpecializedRenderPipelines<
                pipeline::DynamicLightingPipeline,
            >>()
            .init_resource::<bevy::render::render_resource::SpecializedRenderPipelines<
                pipeline::DynamicLightingVolumetricPipeline,
            >>()
            .add_systems(ExtractSchedule, extract_dynamic_lights)
            .add_systems(
                RenderStartup,
                (
                    init_dynamic_lighting_pipeline,
                    pipeline::init_dynamic_lighting_volumetric_pipeline,
                ),
            )
            .add_systems(
                Render,
                (
                    prepare_dynamic_light_buffers.in_set(RenderSystems::PrepareResources),
                    prepare_dynamic_lighting_pipelines.in_set(RenderSystems::Prepare),
                    pipeline::prepare_dynamic_lighting_volumetric_pipelines
                        .in_set(RenderSystems::Prepare),
                ),
            )
            .add_systems(
                bevy::core_pipeline::schedule::Core3d,
                dynamic_lighting_pass
                    .after(bevy::core_pipeline::Core3dSystems::MainPass)
                    .before(bevy::core_pipeline::Core3dSystems::EarlyPostProcess),
            );
        render_app.add_systems(
            bevy::core_pipeline::schedule::Core3d,
            pipeline::dynamic_lighting_volumetric_pass
                .after(dynamic_lighting_pass)
                // Fog belongs in HDR before bloom so emissive volumes can glow,
                // but bloom must never be composited underneath the fog layer.
                .before(bevy::post_process::bloom::bloom)
                .before(bevy::core_pipeline::Core3dSystems::EarlyPostProcess),
        );
    }
}
