[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_2d](index.html)

# Function prepare\_core\_2d\_depth\_textures 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#424-431)

```rust
pub fn prepare_core_2d_depth_textures(
    commands: Commands<'_, '_>,
    texture_cache: ResMut<'_, TextureCache>,
    render_device: Res<'_, RenderDevice>,
    transparent_2d_phases: Res<'_, ViewSortedRenderPhases<Transparent2d>>,
    opaque_2d_phases: Res<'_, ViewBinnedRenderPhases<Opaque2d>>,
    views_2d: Query<'_, '_, (Entity, &ExtractedCamera, &ExtractedView, &Msaa), (With<Camera2d>,)>,
)
```