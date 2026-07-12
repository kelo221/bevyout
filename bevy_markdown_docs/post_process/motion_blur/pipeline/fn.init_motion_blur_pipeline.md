[bevy](../../../index.html)::[post\_process](../../index.html)::[motion\_blur](../index.html)::[pipeline](index.html)

# Function init\_motion\_blur\_pipeline 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/pipeline.rs.html#97-102)

```rust
pub fn init_motion_blur_pipeline(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    fullscreen_shader: Res<'_, FullscreenShader>,
    asset_server: Res<'_, AssetServer>,
)
```