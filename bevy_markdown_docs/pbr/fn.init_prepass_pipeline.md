[bevy](../index.html)::[pbr](index.html)

# Function init\_prepass\_pipeline 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#266-273)

```rust
pub fn init_prepass_pipeline(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_adapter: Res<'_, RenderAdapter>,
    mesh_pipeline: Res<'_, MeshPipeline>,
    material_pipeline: Res<'_, MaterialPipeline>,
    asset_server: Res<'_, AssetServer>,
)
```