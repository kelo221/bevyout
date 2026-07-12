[bevy](../index.html)::[pbr](index.html)

# Function queue\_prepass\_material\_meshes 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1176-1190)

```rust
pub fn queue_prepass_material_meshes(
    render_mesh_instances: Res<'_, RenderMeshInstances>,
    render_materials: Res<'_, ErasedRenderAssets<PreparedMaterial>>,
    render_material_instances: Res<'_, RenderMaterialInstances>,
    mesh_allocator: Res<'_, MeshAllocator>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
    opaque_prepass_render_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque3dPrepass>>,
    alpha_mask_prepass_render_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask3dPrepass>>,
    opaque_deferred_render_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque3dDeferred>>,
    alpha_mask_deferred_render_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask3dDeferred>>,
    views: Query<'_, '_, (&ExtractedView, &RenderVisibleEntities)>,
    specialized_material_pipeline_cache: Res<'_, SpecializedPrepassMaterialPipelineCache>,
    pending_prepass_mesh_material_queues: ResMut<'_, PendingPrepassMeshMaterialQueues>,
    dirty_specializations: Res<'_, DirtySpecializations>,
)
```