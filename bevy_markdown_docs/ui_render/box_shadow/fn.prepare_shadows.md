[bevy](../../index.html)::[ui\_render](../index.html)::[box\_shadow](index.html)

# Function prepare\_shadows 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#351-362)

```rust
pub fn prepare_shadows(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    pipeline_cache: Res<'_, PipelineCache>,
    ui_meta: ResMut<'_, BoxShadowMeta>,
    extracted_shadows: ResMut<'_, ExtractedBoxShadows>,
    view_uniforms: Res<'_, ViewUniforms>,
    box_shadow_pipeline: Res<'_, BoxShadowPipeline>,
    phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    previous_len: Local<'_, usize>,
)
```