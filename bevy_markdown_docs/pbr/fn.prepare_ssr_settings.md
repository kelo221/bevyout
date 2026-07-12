[bevy](../index.html)::[pbr](index.html)

# Function prepare\_ssr\_settings 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#427-433)

```rust
pub fn prepare_ssr_settings(
    commands: Commands<'_, '_>,
    views: Query<'_, '_, (Entity, &ScreenSpaceReflectionsUniform), With<ExtractedView>>,
    ssr_settings_buffer: ResMut<'_, ScreenSpaceReflectionsBuffer>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
)
```

Gathers up screen space reflection settings for each applicable view and writes them into a GPU buffer.