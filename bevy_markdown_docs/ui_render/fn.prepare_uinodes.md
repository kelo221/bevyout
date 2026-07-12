[bevy](../index.html)::[ui\_render](index.html)

# Function prepare\_uinodes 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#1575-1589)

```rust
pub fn prepare_uinodes(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    pipeline_cache: Res<'_, PipelineCache>,
    ui_meta: ResMut<'_, UiMeta>,
    extracted_uinodes: ResMut<'_, ExtractedUiNodes>,
    view_uniforms: Res<'_, ViewUniforms>,
    ui_pipeline: Res<'_, UiPipeline>,
    image_bind_groups: ResMut<'_, ImageNodeBindGroups>,
    gpu_images: Res<'_, RenderAssets<GpuImage>>,
    phases: ResMut<'_, ViewSortedRenderPhases<TransparentUi>>,
    events: Res<'_, SpriteAssetEvents>,
    previous_len: Local<'_, usize>,
)
```