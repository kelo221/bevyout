[bevy](../../index.html)::[ui\_render](../index.html)::[box\_shadow](index.html)

# Function queue\_shadows 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#300-309)

```rust
pub fn queue_shadows(
    extracted_box_shadows: ResMut<'_, ExtractedBoxShadows>,
    box_shadow_pipeline: Res<'_, BoxShadowPipeline>,
    pipelines: ResMut<'_, SpecializedRenderPipelines<BoxShadowPipeline>>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    render_views: Query<'_, '_, (&UiCameraView, Option<&BoxShadowSamples>), With<ExtractedView>>,
    camera_views: Query<'_, '_, &ExtractedView>,
    pipeline_cache: Res<'_, PipelineCache>,
    draw_functions: Res<'_, DrawFunctions<TransparentUi>>,
)
```