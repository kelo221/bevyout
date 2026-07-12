[bevy](../index.html)::[pbr](index.html)

# Function collect\_meshes\_for\_gpu\_building 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2407-2423)

```rust
pub fn collect_meshes_for_gpu_building(
    render_mesh_instances: ResMut<'_, RenderMeshInstances>,
    batched_instance_buffers: ResMut<'_, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    mesh_culling_data_buffer: ResMut<'_, MeshCullingDataBuffer>,
    render_mesh_instance_queues: ResMut<'_, RenderMeshInstanceGpuQueues>,
    render_gpu_culled_entities: ResMut<'_, RenderGpuCulledEntities>,
    mesh_allocator: Res<'_, MeshAllocator>,
    mesh_material_ids: Res<'_, RenderMaterialInstances>,
    render_material_bindings: Res<'_, RenderMaterialBindings>,
    render_lightmaps: Res<'_, RenderLightmaps>,
    skin_uniforms: Res<'_, SkinUniforms>,
    morph_indices: Res<'_, MorphIndices>,
    frame_count: Res<'_, FrameCount>,
    meshes_to_reextract_next_frame: ResMut<'_, MeshesToReextractNextFrame>,
)
```

Creates the [`RenderMeshInstanceGpu`](struct.RenderMeshInstanceGpu.html "struct bevy::pbr::RenderMeshInstanceGpu")s and [`MeshInputUniform`](struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")s when GPU preprocessing is in use.