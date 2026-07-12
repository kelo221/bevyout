[bevy](../index.html)::[sprite\_render](index.html)

# Function prepare\_mesh2d\_view\_bind\_groups 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#774-785)

```rust
pub fn prepare_mesh2d_view_bind_groups(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    mesh2d_pipeline: Res<'_, Mesh2dPipeline>,
    view_uniforms: Res<'_, ViewUniforms>,
    views: Query<'_, '_, (Entity, &Tonemapping), (With<ExtractedView>, With<Camera2d>)>,
    globals_buffer: Res<'_, GlobalsBuffer>,
    tonemapping_luts: Res<'_, TonemappingLuts>,
    images: Res<'_, RenderAssets<GpuImage>>,
    fallback_image: Res<'_, FallbackImage>,
)
```