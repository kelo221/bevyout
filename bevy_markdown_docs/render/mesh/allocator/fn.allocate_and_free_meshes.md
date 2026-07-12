[bevy](../../../index.html)::[render](../../index.html)::[mesh](../index.html)::[allocator](index.html)

# Function allocate\_and\_free\_meshes 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#243-250)

```rust
pub fn allocate_and_free_meshes(
    mesh_allocator: ResMut<'_, MeshAllocator>,
    mesh_allocator_settings: Res<'_, MeshAllocatorSettings>,
    extracted_meshes: Res<'_, ExtractedAssets<RenderMesh>>,
    mesh_vertex_buffer_layouts: ResMut<'_, MeshVertexBufferLayouts>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
)
```

A system that processes newly-extracted or newly-removed meshes and writes their data into buffers or frees their data as appropriate.