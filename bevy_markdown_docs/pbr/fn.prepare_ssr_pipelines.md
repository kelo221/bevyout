[bevy](../index.html)::[pbr](index.html)

# Function prepare\_ssr\_pipelines 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#389-403)

```rust
pub fn prepare_ssr_pipelines(
    commands: Commands<'_, '_>,
    pipeline_cache: Res<'_, PipelineCache>,
    view_key_cache: Res<'_, ViewKeyCache>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<ScreenSpaceReflectionsPipeline>>,
    ssr_pipeline: Res<'_, ScreenSpaceReflectionsPipeline>,
    views: Query<'_, '_, (Entity, &ExtractedView), (With<ScreenSpaceReflectionsUniform>, With<DepthPrepass>, With<DeferredPrepass>)>,
)
```

Sets up screen space reflection pipelines for each applicable view.