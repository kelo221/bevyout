[bevy](../index.html)::[ui\_render](index.html)

# Function queue\_ui\_material\_nodes 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#594-605)

```rust
pub fn queue_ui_material_nodes<M>(
    extracted_uinodes: Res<'_, ExtractedUiMaterialNodes<M>>,
    draw_functions: Res<'_, DrawFunctions<TransparentUi>>,
    ui_material_pipeline: Res<'_, UiMaterialPipeline<M>>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<UiMaterialPipeline<M>>>,
    pipeline_cache: Res<'_, PipelineCache>,
    render_materials: Res<'_, RenderAssets<PreparedUiMaterial<M>>>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    render_views: Query<'_, '_, &UiCameraView, With<ExtractedView>>,
    camera_views: Query<'_, '_, &ExtractedView>,
)where
    M: UiMaterial,
    <M as AsBindGroup>::Data: PartialEq + Eq + Hash + Clone,
```