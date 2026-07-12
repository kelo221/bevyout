[bevy](../index.html)::[pbr](index.html)

# Function queue\_material\_meshes 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1112-1130)

```rust
pub fn queue_material_meshes(
    render_materials: Res<'_, ErasedRenderAssets<PreparedMaterial>>,
    render_mesh_instances: Res<'_, RenderMeshInstances>,
    render_material_instances: Res<'_, RenderMaterialInstances>,
    mesh_assets: Res<'_, RenderAssets<RenderMesh>>,
    mesh_allocator: Res<'_, MeshAllocator>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
    maybe_batched_instance_buffers: Option<Res<'_, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>>,
    opaque_render_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque3d>>,
    alpha_mask_render_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask3d>>,
    transmissive_render_phases: ResMut<'_, ViewSortedRenderPhases<Transmissive3d>>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<Transparent3d>>,
    pending_mesh_material_queues: ResMut<'_, PendingMeshMaterialQueues>,
    views: Query<'_, '_, (&ExtractedView, &RenderVisibleEntities)>,
    specialized_material_pipeline_cache: ResMut<'_, SpecializedMaterialPipelineCache>,
    dirty_specializations: Res<'_, DirtySpecializations>,
)
```

For each view, iterates over all the meshes visible from that view and adds them to [`BinnedRenderPhase`](../render/render_phase/struct.BinnedRenderPhase.html "struct bevy::render::render_phase::BinnedRenderPhase")s or [`SortedRenderPhase`](../render/render_phase/struct.SortedRenderPhase.html "struct bevy::render::render_phase::SortedRenderPhase")s as appropriate.