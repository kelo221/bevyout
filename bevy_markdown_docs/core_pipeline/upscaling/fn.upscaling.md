[bevy](../../index.html)::[core\_pipeline](../index.html)::[upscaling](index.html)

# Function upscaling 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/upscaling/node.rs.html#17-28)

```rust
pub fn upscaling(
    view: ViewQuery<'_, '_, (&ViewTarget, &ViewUpscalingPipeline, Option<&ExtractedCamera>)>,
    pipeline_cache: Res<'_, PipelineCache>,
    blit_pipeline: Res<'_, BlitPipeline>,
    clear_color_global: Res<'_, ClearColor>,
    cache: Local<'_, UpscalingBindGroupCache>,
    ctx: RenderContext<'_, '_>,
)
```