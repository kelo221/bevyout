[bevy](../../index.html)::[pbr](../index.html)::[deferred](index.html)

# Function deferred\_lighting 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#125-136)

```rust
pub fn deferred_lighting(
    view: ViewQuery<'_, '_, (&MeshViewBindGroup, &ViewTarget, &DeferredLightingIdDepthTexture, &DeferredLightingPipeline)>,
    pipeline_cache: Res<'_, PipelineCache>,
    deferred_lighting_layout: Res<'_, DeferredLightingLayout>,
    deferred_lighting_pass_id: Res<'_, ComponentUniforms<PbrDeferredLightingDepthId>>,
    ctx: RenderContext<'_, '_>,
)
```