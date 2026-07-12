[bevy](../index.html)::[pbr](index.html)

# Function write\_morph\_buffers 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#192-196)

```rust
pub fn write_morph_buffers(
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    uniform: ResMut<'_, MorphUniforms>,
)
```

A system that writes the buffers inside [`MorphUniforms`](struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms") to the GPU.