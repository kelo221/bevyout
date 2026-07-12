[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[mip\_generation](../../index.html)::[experimental](../index.html)::[depth](index.html)

# Function late\_downsample\_depth 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#138-153)

```rust
pub fn late_downsample_depth(
    view: ViewQuery<'_, '_, (&ViewDepthPyramid, &ViewDownsampleDepthBindGroup, &ViewDepthTexture, Option<&OcclusionCullingSubviewEntities>)>,
    shadow_view_query: Query<'_, '_, (&ViewDepthPyramid, &ViewDownsampleDepthBindGroup, &OcclusionCullingSubview)>,
    downsample_depth_pipelines: Option<Res<'_, DownsampleDepthPipelines>>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
)
```

Produces a hierarchical Z-buffer (depth pyramid) for occlusion culling.

This runs the single-pass downsampling (SPD) shader with the _min_ filter in order to generate a series of mipmaps for the Z buffer. The resulting hierarchical Z-buffer can be used for occlusion culling.

The _late_ downsample depth pass runs at the end of the main phase. It prepares the Z-buffer for the occlusion culling that the early mesh preprocessing phase of the _next_ frame will perform.

This system won’t do anything if occlusion culling isn’t on.