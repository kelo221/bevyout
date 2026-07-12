[bevy](../../index.html)::[anti\_alias](../index.html)::[fxaa](index.html)

# Function init\_fxaa\_pipeline 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#121-126)

```rust
pub fn init_fxaa_pipeline(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
)
```