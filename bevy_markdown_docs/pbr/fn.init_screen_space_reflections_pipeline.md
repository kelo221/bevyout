[bevy](../index.html)::[pbr](index.html)

# Function init\_screen\_space\_reflections\_pipeline 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#322-329)

```rust
pub fn init_screen_space_reflections_pipeline(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_adapter: Res<'_, RenderAdapter>,
    mesh_view_layouts: Res<'_, MeshPipelineViewLayouts>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
)
```