[bevy](../index.html)::[pbr](index.html)

# Function prepare\_fog 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/fog.rs.html#50-56)

```rust
pub fn prepare_fog(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    fog_meta: ResMut<'_, FogMeta>,
    views: Query<'_, '_, (Entity, &DistanceFog), With<ExtractedView>>,
)
```

Prepares fog metadata and writes the fog-related uniform buffers to the GPU