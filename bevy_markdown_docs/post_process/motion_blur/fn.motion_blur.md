[bevy](../../index.html)::[post\_process](../index.html)::[motion\_blur](index.html)

# Function motion\_blur 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#184-198)

```rust
pub fn motion_blur(
    view: ViewQuery<'_, '_, (&ViewTarget, &MotionBlurPipelineId, &ViewPrepassTextures, &ViewDepthTexture, &MotionBlurUniform, &Msaa)>,
    motion_blur_pipeline: Res<'_, MotionBlurPipeline>,
    pipeline_cache: Res<'_, PipelineCache>,
    settings_uniforms: Res<'_, ComponentUniforms<MotionBlurUniform>>,
    globals_buffer: Res<'_, GlobalsBuffer>,
    ctx: RenderContext<'_, '_>,
)
```