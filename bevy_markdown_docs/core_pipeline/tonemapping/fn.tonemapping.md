[bevy](../../index.html)::[core\_pipeline](../index.html)::[tonemapping](index.html)

# Function tonemapping 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/node.rs.html#26-42)

```rust
pub fn tonemapping(
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ViewUniformOffset, &ViewTarget, &ViewTonemappingPipeline, &Tonemapping)>,
    pipeline_cache: Res<'_, PipelineCache>,
    tonemapping_pipeline: Res<'_, TonemappingPipeline>,
    gpu_images: Res<'_, RenderAssets<GpuImage>>,
    fallback_image: Res<'_, FallbackImage>,
    view_uniforms: Res<'_, ViewUniforms>,
    tonemapping_luts: Res<'_, TonemappingLuts>,
    cache: Local<'_, TonemappingBindGroupCache>,
    ctx: RenderContext<'_, '_>,
)
```