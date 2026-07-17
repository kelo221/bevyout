use core::num::NonZeroU64;

use bevy::pbr::{ShadowSamplers, ViewShadowBindings};
use bevy::{
    asset::load_embedded_asset,
    core_pipeline::{FullscreenShader, prepass::ViewPrepassTextures},
    ecs::error::BevyError,
    prelude::*,
    render::{
        render_resource::{
            BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
            CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, Operations,
            PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            SpecializedRenderPipeline, SpecializedRenderPipelines, TextureFormat,
            TextureSampleType,
            binding_types::{
                sampler, storage_buffer_read_only_sized, texture_2d, texture_cube_array,
                texture_depth_2d, uniform_buffer,
            },
        },
        renderer::{RenderContext, RenderDevice, ViewQuery},
        view::{ExtractedView, ViewTarget, ViewUniform, ViewUniformOffset, ViewUniforms},
    },
};

use super::{
    DynamicLightingView,
    gpu::{DynamicLightGpuBuffers, GPU_DYNAMIC_LIGHT_SIZE, GpuDynamicLightMeta},
};

#[derive(Resource)]
pub(super) struct DynamicLightingPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    shader: Handle<bevy::shader::Shader>,
    fullscreen_shader: FullscreenShader,
}

#[derive(Resource)]
pub(super) struct DynamicLightingVolumetricPipeline {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    shader: Handle<bevy::shader::Shader>,
    fullscreen_shader: FullscreenShader,
}

impl SpecializedRenderPipeline for DynamicLightingPipeline {
    type Key = TextureFormat;

    fn specialize(&self, target_format: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("dynamic_lighting_hdr_pipeline".into()),
            layout: vec![self.layout.clone()],
            vertex: self.fullscreen_shader.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                ..default()
            }),
            ..default()
        }
    }
}

impl SpecializedRenderPipeline for DynamicLightingVolumetricPipeline {
    type Key = TextureFormat;

    fn specialize(&self, target_format: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("dynamic_lighting_volumetric_pipeline".into()),
            layout: vec![self.layout.clone()],
            vertex: self.fullscreen_shader.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                ..default()
            }),
            ..default()
        }
    }
}

#[derive(Component)]
pub(super) struct DynamicLightingPipelineId(CachedRenderPipelineId);

#[derive(Component)]
pub(super) struct DynamicLightingVolumetricPipelineId(CachedRenderPipelineId);

pub(super) fn init_dynamic_lighting_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "dynamic_lighting_hdr_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                texture_depth_2d(),
                texture_2d(TextureSampleType::Uint),
                uniform_buffer::<ViewUniform>(true),
                storage_buffer_read_only_sized(
                    false,
                    NonZeroU64::new(GPU_DYNAMIC_LIGHT_SIZE as u64),
                ),
                uniform_buffer::<GpuDynamicLightMeta>(false),
                texture_cube_array(TextureSampleType::Depth),
                sampler(SamplerBindingType::Comparison),
            ),
        ),
    );
    commands.insert_resource(DynamicLightingPipeline {
        layout,
        sampler: render_device.create_sampler(&SamplerDescriptor::default()),
        shader: load_embedded_asset!(
            asset_server.as_ref(),
            "../shaders/dynamic_lighting_pass.wgsl"
        ),
        fullscreen_shader: fullscreen_shader.clone(),
    });
}

pub(super) fn init_dynamic_lighting_volumetric_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "dynamic_lighting_volumetric_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                texture_depth_2d(),
                uniform_buffer::<ViewUniform>(true),
                storage_buffer_read_only_sized(
                    false,
                    NonZeroU64::new(GPU_DYNAMIC_LIGHT_SIZE as u64),
                ),
                uniform_buffer::<GpuDynamicLightMeta>(false),
            ),
        ),
    );
    commands.insert_resource(DynamicLightingVolumetricPipeline {
        layout,
        sampler: render_device.create_sampler(&SamplerDescriptor::default()),
        shader: load_embedded_asset!(
            asset_server.as_ref(),
            "../shaders/dynamic_lighting_volumetric.wgsl"
        ),
        fullscreen_shader: fullscreen_shader.clone(),
    });
}

pub(super) fn prepare_dynamic_lighting_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<DynamicLightingPipeline>,
    mut specialized: ResMut<SpecializedRenderPipelines<DynamicLightingPipeline>>,
    views: Query<(Entity, &ExtractedView), With<DynamicLightingView>>,
) {
    for (entity, view) in &views {
        let id = specialized.specialize(&pipeline_cache, &pipeline, view.target_format);
        commands
            .entity(entity)
            .insert(DynamicLightingPipelineId(id));
    }
}

pub(super) fn prepare_dynamic_lighting_volumetric_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<DynamicLightingVolumetricPipeline>,
    mut specialized: ResMut<SpecializedRenderPipelines<DynamicLightingVolumetricPipeline>>,
    views: Query<(Entity, &ExtractedView), With<DynamicLightingView>>,
) {
    for (entity, view) in &views {
        let id = specialized.specialize(&pipeline_cache, &pipeline, view.target_format);
        commands
            .entity(entity)
            .insert(DynamicLightingVolumetricPipelineId(id));
    }
}

pub(super) fn dynamic_lighting_pass(
    view: ViewQuery<(
        &ViewTarget,
        &ViewPrepassTextures,
        &ViewUniformOffset,
        &DynamicLightingPipelineId,
        &DynamicLightingView,
        &ViewShadowBindings,
    )>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<DynamicLightingPipeline>,
    buffers: Res<DynamicLightGpuBuffers>,
    view_uniforms: Res<ViewUniforms>,
    shadow_samplers: Res<ShadowSamplers>,
    mut ctx: RenderContext,
) -> Result<(), BevyError> {
    let (view_target, prepass, view_offset, pipeline_id, _, shadow_bindings) = view.into_inner();
    let Some(render_pipeline) = pipeline_cache.get_render_pipeline(pipeline_id.0) else {
        return Ok(());
    };
    let (Some(depth), Some(deferred), Some(view_binding), Some(light_binding), Some(meta_binding)) = (
        prepass.depth_view(),
        prepass.deferred_view(),
        view_uniforms.uniforms.binding(),
        buffers.lights.binding(),
        buffers.meta.binding(),
    ) else {
        return Ok(());
    };

    let post_process = view_target.post_process_write();
    let bind_group = ctx.render_device().create_bind_group(
        "dynamic_lighting_hdr_bind_group",
        &pipeline_cache.get_bind_group_layout(&pipeline.layout),
        &BindGroupEntries::sequential((
            post_process.source,
            &pipeline.sampler,
            depth,
            deferred,
            view_binding,
            light_binding,
            meta_binding,
            &shadow_bindings.point_light_depth_texture_view,
            &shadow_samplers.point_light_comparison_sampler,
        )),
    );

    let mut pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
        label: Some("dynamic_lighting_hdr_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: post_process.destination,
            depth_slice: None,
            resolve_target: None,
            ops: Operations::default(),
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_render_pipeline(render_pipeline);
    pass.set_bind_group(0, &bind_group, &[view_offset.offset]);
    pass.draw(0..3, 0..1);
    Ok(())
}

pub(super) fn dynamic_lighting_volumetric_pass(
    view: ViewQuery<(
        &ViewTarget,
        &ViewPrepassTextures,
        &ViewUniformOffset,
        &DynamicLightingVolumetricPipelineId,
        &DynamicLightingView,
    )>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<DynamicLightingVolumetricPipeline>,
    buffers: Res<DynamicLightGpuBuffers>,
    view_uniforms: Res<ViewUniforms>,
    mut ctx: RenderContext,
) -> Result<(), BevyError> {
    if !buffers.volumetric_enabled || buffers.volumetric_count == 0 {
        return Ok(());
    }
    let (view_target, prepass, view_offset, pipeline_id, _) = view.into_inner();
    let Some(render_pipeline) = pipeline_cache.get_render_pipeline(pipeline_id.0) else {
        return Ok(());
    };
    let (Some(depth), Some(view_binding), Some(light_binding), Some(meta_binding)) = (
        prepass.depth_view(),
        view_uniforms.uniforms.binding(),
        buffers.volumetric_lights.binding(),
        buffers.volumetric_meta.binding(),
    ) else {
        return Ok(());
    };

    let post_process = view_target.post_process_write();
    let bind_group = ctx.render_device().create_bind_group(
        "dynamic_lighting_volumetric_bind_group",
        &pipeline_cache.get_bind_group_layout(&pipeline.layout),
        &BindGroupEntries::sequential((
            post_process.source,
            &pipeline.sampler,
            depth,
            view_binding,
            light_binding,
            meta_binding,
        )),
    );
    let mut pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
        label: Some("dynamic_lighting_volumetric_pass"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: post_process.destination,
            depth_slice: None,
            resolve_target: None,
            ops: Operations::default(),
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_render_pipeline(render_pipeline);
    pass.set_bind_group(0, &bind_group, &[view_offset.offset]);
    pass.draw(0..3, 0..1);
    Ok(())
}
