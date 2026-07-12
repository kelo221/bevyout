[bevy](../../index.html)::[anti\_alias](../index.html)::[contrast\_adaptive\_sharpening](index.html)

# Function init\_cas\_pipeline 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#142-147)

```rust
pub fn init_cas_pipeline(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
)
```