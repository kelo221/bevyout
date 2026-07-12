[bevy](../index.html)::[pbr](index.html)

# Function prepare\_mesh\_bind\_groups 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3847-3864)

```rust
pub fn prepare_mesh_bind_groups(
    commands: Commands<'_, '_>,
    meshes: Res<'_, RenderAssets<RenderMesh>>,
    mesh_pipeline: Res<'_, MeshPipeline>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    cpu_batched_instance_buffer: Option<Res<'_, BatchedInstanceBuffer<MeshUniform>>>,
    gpu_batched_instance_buffers: Option<Res<'_, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>>,
    skins_uniform: Res<'_, SkinUniforms>,
    weights_uniform: Res<'_, MorphUniforms>,
    mesh_allocator: Res<'_, MeshAllocator>,
    render_morph_target_allocator: Res<'_, RenderMorphTargetAllocator>,
    render_lightmaps: ResMut<'_, RenderLightmaps>,
)
```

Creates the per-mesh bind groups for each type of mesh and each phase.