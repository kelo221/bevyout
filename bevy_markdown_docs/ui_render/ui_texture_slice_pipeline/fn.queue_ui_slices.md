[bevy](../../index.html)::[ui\_render](../index.html)::[ui\_texture\_slice\_pipeline](index.html)

# Function queue\_ui\_slices 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#314-323)

```rust
pub fn queue_ui_slices(
    extracted_ui_slicers: ResMut<'_, ExtractedUiTextureSlices>,
    ui_slicer_pipeline: Res<'_, UiTextureSlicePipeline>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<UiTextureSlicePipeline>>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    render_views: Query<'_, '_, &UiCameraView, With<ExtractedView>>,
    camera_views: Query<'_, '_, &ExtractedView>,
    pipeline_cache: Res<'_, PipelineCache>,
    draw_functions: Res<'_, DrawFunctions<TransparentUi>>,
)
```