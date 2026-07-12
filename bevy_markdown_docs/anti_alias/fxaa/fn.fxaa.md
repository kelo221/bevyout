[bevy](../../index.html)::[anti\_alias](../index.html)::[fxaa](index.html)

# Function fxaa 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/node.rs.html#13-19)

```rust
pub fn fxaa(
    view: ViewQuery<'_, '_, (&ViewTarget, &CameraFxaaPipeline, &Fxaa)>,
    fxaa_pipeline: Res<'_, FxaaPipeline>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
    cached_bind_group: Local<'_, Option<(TextureViewId, BindGroup)>>,
)
```