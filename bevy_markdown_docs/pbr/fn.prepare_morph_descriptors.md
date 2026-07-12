[bevy](../index.html)::[pbr](index.html)

# Function prepare\_morph\_descriptors 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#350-356)

```rust
pub fn prepare_morph_descriptors(
    morph_indices: ResMut<'_, MorphIndices>,
    morph_uniforms: ResMut<'_, MorphUniforms>,
    render_mesh_instances: Res<'_, RenderMeshInstances>,
    meshes: Res<'_, RenderAssets<RenderMesh>>,
    mesh_allocator: Res<'_, MeshAllocator>,
)
```

A system that writes [`GpuMorphDescriptor`](struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor") values to the [`MorphUniforms`](struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms") for each mesh instance with morph targets.

As morph descriptors are only used when the platform supports storage buffers, if the platform doesn’t support storage buffers, this system does nothing.