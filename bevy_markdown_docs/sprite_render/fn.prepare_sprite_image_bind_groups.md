[bevy](../index.html)::[sprite\_render](index.html)

# Function prepare\_sprite\_image\_bind\_groups 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#638-651)

```rust
pub fn prepare_sprite_image_bind_groups(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    pipeline_cache: Res<'_, PipelineCache>,
    sprite_meta: ResMut<'_, SpriteMeta>,
    sprite_pipeline: Res<'_, SpritePipeline>,
    image_bind_groups: ResMut<'_, ImageBindGroups>,
    gpu_images: Res<'_, RenderAssets<GpuImage>>,
    extracted_sprites: Res<'_, ExtractedSprites>,
    extracted_slices: Res<'_, ExtractedSlices>,
    phases: ResMut<'_, ViewSortedRenderPhases<Transparent2d>>,
    events: Res<'_, SpriteAssetEvents>,
    batches: ResMut<'_, SpriteBatches>,
)
```