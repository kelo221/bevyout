[bevy](../index.html)::[pbr](index.html)

# Function prepare\_skins 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/skin.rs.html#176-180)

```rust
pub fn prepare_skins(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    uniform: ResMut<'_, SkinUniforms>,
)
```

Uploads the buffers containing the joints to the GPU.