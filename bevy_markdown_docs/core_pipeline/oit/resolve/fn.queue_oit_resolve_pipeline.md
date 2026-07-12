[bevy](../../../index.html)::[core\_pipeline](../../index.html)::[oit](../index.html)::[resolve](index.html)

# Function queue\_oit\_resolve\_pipeline 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/resolve/mod.rs.html#150-171)

```rust
pub fn queue_oit_resolve_pipeline(
    commands: Commands<'_, '_>,
    pipeline_cache: Res<'_, PipelineCache>,
    resolve_pipeline: Res<'_, OitResolvePipeline>,
    cameras: Query<'_, '_, (Entity, &ExtractedView, &OrderIndependentTransparencySettings, Has<DepthPrepass>), (With<OrderIndependentTransparencySettings>, With<ExtractedCamera>)>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
    cached_pipeline_id: Local<'_, EntityHashMap<(OitResolvePipelineKey, CachedRenderPipelineId)>>,
)
```