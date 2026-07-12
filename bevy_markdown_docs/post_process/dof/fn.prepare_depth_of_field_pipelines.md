[bevy](../../index.html)::[post\_process](../index.html)::[dof](index.html)

# Function prepare\_depth\_of\_field\_pipelines 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#502-519)

```rust
pub fn prepare_depth_of_field_pipelines(
    commands: Commands<'_, '_>,
    pipeline_cache: Res<'_, PipelineCache>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<DepthOfFieldPipeline>>,
    global_bind_group_layout: Res<'_, DepthOfFieldGlobalBindGroupLayout>,
    view_targets: Query<'_, '_, (Entity, &ExtractedView, &DepthOfField, &ViewDepthOfFieldBindGroupLayouts, &Msaa), With<ExtractedCamera>>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
)
```

Specializes the depth of field pipelines specific to a view.