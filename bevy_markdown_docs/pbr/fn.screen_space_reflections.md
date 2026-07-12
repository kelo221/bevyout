[bevy](../index.html)::[pbr](index.html)

# Function screen\_space\_reflections 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#245-256)

```rust
pub fn screen_space_reflections(
    view: ViewQuery<'_, '_, (&ViewTarget, &MeshViewBindGroup, &ScreenSpaceReflectionsPipelineId)>,
    pipeline_cache: Res<'_, PipelineCache>,
    ssr_pipeline: Res<'_, ScreenSpaceReflectionsPipeline>,
    bluenoise: Res<'_, Bluenoise>,
    render_images: Res<'_, RenderAssets<GpuImage>>,
    ctx: RenderContext<'_, '_>,
)
```