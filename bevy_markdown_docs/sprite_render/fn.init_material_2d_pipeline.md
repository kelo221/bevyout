[bevy](../index.html)::[sprite\_render](index.html)

# Function init\_material\_2d\_pipeline 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#497-502)

```rust
pub fn init_material_2d_pipeline<M>(
    commands: Commands<'_, '_>,
    asset_server: Res<'_, AssetServer>,
    render_device: Res<'_, RenderDevice>,
    mesh_2d_pipeline: Res<'_, Mesh2dPipeline>,
)where
    M: Material2d,
```