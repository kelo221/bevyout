[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function init\_wireframe\_3d\_pipeline 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#703-708)

```rust
pub fn init_wireframe_3d_pipeline(
    commands: Commands<'_, '_>,
    mesh_pipeline: Res<'_, MeshPipeline>,
    asset_server: Res<'_, AssetServer>,
    render_device: Res<'_, RenderDevice>,
)
```