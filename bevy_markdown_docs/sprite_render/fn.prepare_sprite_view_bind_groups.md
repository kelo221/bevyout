[bevy](../index.html)::[sprite\_render](index.html)

# Function prepare\_sprite\_view\_bind\_groups 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#608-618)

```rust
pub fn prepare_sprite_view_bind_groups(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    sprite_pipeline: Res<'_, SpritePipeline>,
    view_uniforms: Res<'_, ViewUniforms>,
    views: Query<'_, '_, (Entity, &Tonemapping), With<ExtractedView>>,
    tonemapping_luts: Res<'_, TonemappingLuts>,
    images: Res<'_, RenderAssets<GpuImage>>,
    fallback_image: Res<'_, FallbackImage>,
)
```