[bevy](../index.html)::[ui\_render](index.html)

# Function queue\_uinodes 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#1511-1520)

```rust
pub fn queue_uinodes(
    extracted_uinodes: Res<'_, ExtractedUiNodes>,
    ui_pipeline: Res<'_, UiPipeline>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<UiPipeline>>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    render_views: Query<'_, '_, (&UiCameraView, Option<&UiAntiAlias>), With<ExtractedView>>,
    camera_views: Query<'_, '_, &ExtractedView>,
    pipeline_cache: Res<'_, PipelineCache>,
    draw_functions: Res<'_, DrawFunctions<TransparentUi>>,
)
```