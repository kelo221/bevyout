[bevy](../../index.html)::[core\_pipeline](../index.html)::[tonemapping](index.html)

# Function init\_tonemapping\_pipeline 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#295-300)

```rust
pub fn init_tonemapping_pipeline(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
)
```