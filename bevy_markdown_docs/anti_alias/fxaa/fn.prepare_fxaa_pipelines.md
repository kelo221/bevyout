[bevy](../../index.html)::[anti\_alias](../index.html)::[fxaa](index.html)

# Function prepare\_fxaa\_pipelines 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#191-197)

```rust
pub fn prepare_fxaa_pipelines(
    commands: Commands<'_, '_>,
    pipeline_cache: Res<'_, PipelineCache>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<FxaaPipeline>>,
    fxaa_pipeline: Res<'_, FxaaPipeline>,
    cameras: Query<'_, '_, (Entity, &ExtractedView, &Fxaa), With<ExtractedCamera>>,
)
```