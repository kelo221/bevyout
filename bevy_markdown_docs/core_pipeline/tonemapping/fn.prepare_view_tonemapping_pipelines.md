[bevy](../../index.html)::[core\_pipeline](../index.html)::[tonemapping](index.html)

# Function prepare\_view\_tonemapping\_pipelines 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#331-345)

```rust
pub fn prepare_view_tonemapping_pipelines(
    commands: Commands<'_, '_>,
    pipeline_cache: Res<'_, PipelineCache>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<TonemappingPipeline>>,
    upscaling_pipeline: Res<'_, TonemappingPipeline>,
    view_targets: Query<'_, '_, (Entity, &ExtractedView, Option<&Tonemapping>, Option<&DebandDither>), With<ViewTarget>>,
)
```