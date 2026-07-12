[bevy](../../index.html)::[core\_pipeline](../index.html)::[blit](index.html)

# Function init\_blit\_pipeline 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/blit/mod.rs.html#43-48)

```rust
pub fn init_blit_pipeline(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
)
```