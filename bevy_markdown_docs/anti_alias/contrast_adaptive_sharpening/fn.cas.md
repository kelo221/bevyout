[bevy](../../index.html)::[anti\_alias](../index.html)::[contrast\_adaptive\_sharpening](index.html)

# Function cas 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/node.rs.html#16-30)

```rust
pub fn cas(
    view: ViewQuery<'_, '_, (&ViewTarget, &ViewCasPipeline, &DynamicUniformIndex<CasUniform>), With<ExtractedView>>,
    sharpening_pipeline: Res<'_, CasPipeline>,
    pipeline_cache: Res<'_, PipelineCache>,
    uniforms: Res<'_, ComponentUniforms<CasUniform>>,
    ctx: RenderContext<'_, '_>,
    cached_bind_group: Local<'_, Option<(BufferId, TextureViewId, BindGroup)>>,
)
```