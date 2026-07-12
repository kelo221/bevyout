[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[mip\_generation](../../index.html)::[experimental](../index.html)::[depth](index.html)

# Function prepare\_downsample\_depth\_view\_bind\_groups 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#730-744)

```rust
pub fn prepare_downsample_depth_view_bind_groups(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    downsample_depth_pipelines: Res<'_, DownsampleDepthPipelines>,
    pipeline_cache: Res<'_, PipelineCache>,
    view_depth_textures: Query<'_, '_, (Entity, &ViewDepthPyramid, Option<&ViewDepthTexture>, Option<&OcclusionCullingSubview>), Or<(With<ViewDepthTexture>, With<OcclusionCullingSubview>)>>,
)
```

Creates the [`ViewDownsampleDepthBindGroup`](struct.ViewDownsampleDepthBindGroup.html "struct bevy::core_pipeline::mip_generation::experimental::depth::ViewDownsampleDepthBindGroup")s for all views with occlusion culling enabled.