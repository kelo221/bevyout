[bevy](../index.html)::[pbr](index.html)

# Function build\_dummy\_white\_gpu\_image 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2738-2742)

```rust
pub fn build_dummy_white_gpu_image(
    render_device: Res<'_, RenderDevice>,
    default_sampler: Res<'_, DefaultImageSampler>,
    render_queue: Res<'_, RenderQueue>,
) -> GpuImage
```

A 1x1x1 ‘all 1.0’ texture to use as a dummy texture in place of optional [`crate::pbr_material::StandardMaterial`](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial") textures