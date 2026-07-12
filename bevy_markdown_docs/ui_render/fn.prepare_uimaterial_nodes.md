[bevy](../index.html)::[ui\_render](index.html)

# Function prepare\_uimaterial\_nodes 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#384-396)

```rust
pub fn prepare_uimaterial_nodes<M>(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    pipeline_cache: Res<'_, PipelineCache>,
    ui_meta: ResMut<'_, UiMaterialMeta<M>>,
    extracted_uinodes: ResMut<'_, ExtractedUiMaterialNodes<M>>,
    view_uniforms: Res<'_, ViewUniforms>,
    globals_buffer: Res<'_, GlobalsBuffer>,
    ui_material_pipeline: Res<'_, UiMaterialPipeline<M>>,
    phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    previous_len: Local<'_, usize>,
)where
    M: UiMaterial,
```