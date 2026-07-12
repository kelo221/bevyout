[bevy](../../index.html)::[pbr](../index.html)::[generate](index.html)

# Function prepare\_generated\_environment\_map\_intermediate\_textures 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#502-507)

```rust
pub fn prepare_generated_environment_map_intermediate_textures(
    light_probes: Query<'_, '_, (Entity, &RenderEnvironmentMap)>,
    render_device: Res<'_, RenderDevice>,
    texture_cache: ResMut<'_, TextureCache>,
    commands: Commands<'_, '_>,
)
```

Prepares textures needed for single pass downsampling