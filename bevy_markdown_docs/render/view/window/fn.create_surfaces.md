[bevy](../../../index.html)::[render](../../index.html)::[view](../index.html)::[window](index.html)

# Function create\_surfaces 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/mod.rs.html#362-371)

```rust
pub fn create_surfaces(
    windows: ResMut<'_, ExtractedWindows>,
    window_surfaces: ResMut<'_, WindowSurfaces>,
    render_instance: Res<'_, RenderInstance>,
    render_adapter: Res<'_, RenderAdapter>,
    render_device: Res<'_, RenderDevice>,
)
```

Creates window surfaces.