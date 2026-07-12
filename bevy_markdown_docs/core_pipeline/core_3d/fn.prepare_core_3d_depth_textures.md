[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Function prepare\_core\_3d\_depth\_textures 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#662-673)

```rust
pub fn prepare_core_3d_depth_textures(
    commands: Commands<'_, '_>,
    texture_cache: ResMut<'_, TextureCache>,
    render_device: Res<'_, RenderDevice>,
    views_3d: Query<'_, '_, (Entity, &ExtractedCamera, Option<&DepthPrepass>, &Camera3d, &Msaa)>,
)
```