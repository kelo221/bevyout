[bevy](../../index.html)::[ui\_render](../index.html)::[ui\_texture\_slice\_pipeline](index.html)

# Function prepare\_ui\_slices 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#362-376)

```rust
pub fn prepare_ui_slices(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    pipeline_cache: Res<'_, PipelineCache>,
    ui_meta: ResMut<'_, UiTextureSliceMeta>,
    extracted_slices: ResMut<'_, ExtractedUiTextureSlices>,
    view_uniforms: Res<'_, ViewUniforms>,
    texture_slicer_pipeline: Res<'_, UiTextureSlicePipeline>,
    image_bind_groups: ResMut<'_, UiTextureSliceImageBindGroups>,
    gpu_images: Res<'_, RenderAssets<GpuImage>>,
    phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    events: Res<'_, SpriteAssetEvents>,
    previous_len: Local<'_, usize>,
)
```