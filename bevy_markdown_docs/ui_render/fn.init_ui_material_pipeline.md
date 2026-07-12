[bevy](../index.html)::[ui\_render](index.html)

# Function init\_ui\_material\_pipeline 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#176-180)

```rust
pub fn init_ui_material_pipeline<M>(
    commands: Commands<'_, '_>,
    asset_server: Res<'_, AssetServer>,
    render_device: Res<'_, RenderDevice>,
)where
    M: UiMaterial,
```