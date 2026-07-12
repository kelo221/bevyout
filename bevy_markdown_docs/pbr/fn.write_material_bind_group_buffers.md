[bevy](../index.html)::[pbr](index.html)

# Function write\_material\_bind\_group\_buffers 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1798-1802)

```rust
pub fn write_material_bind_group_buffers(
    allocators: ResMut<'_, MaterialBindGroupAllocators>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
)
```

Uploads the contents of all buffers that the [`MaterialBindGroupAllocator`](enum.MaterialBindGroupAllocator.html "enum bevy::pbr::MaterialBindGroupAllocator") manages to the GPU.

Non-bindless allocators don’t currently manage any buffers, so this method only has an effect for bindless allocators.