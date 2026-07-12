[bevy](../index.html)::[pbr](index.html)

# Function queue\_shadows 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2505-2517)

```rust
pub fn queue_shadows(
    render_mesh_instances: Res<'_, RenderMeshInstances>,
    render_materials: Res<'_, ErasedRenderAssets<PreparedMaterial>>,
    render_material_instances: Res<'_, RenderMaterialInstances>,
    shadow_render_phases: ResMut<'_, ViewBinnedRenderPhases<Shadow>>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
    mesh_allocator: Res<'_, MeshAllocator>,
    view_light_entities: Query<'_, '_, (&LightEntity, &ExtractedView, Option<&RenderLayers>)>,
    shadow_map_visible_entities_query: Query<'_, '_, &RenderShadowMapVisibleEntities>,
    specialized_material_pipeline_cache: Res<'_, SpecializedShadowMaterialPipelineCache>,
    pending_shadow_queues: ResMut<'_, PendingShadowQueues>,
    dirty_specializations: Res<'_, DirtySpecializations>,
)
```

For each shadow cascade, iterates over all the meshes “visible” from it and adds them to [`BinnedRenderPhase`](../render/render_phase/struct.BinnedRenderPhase.html "struct bevy::render::render_phase::BinnedRenderPhase")s or [`SortedRenderPhase`](../render/render_phase/struct.SortedRenderPhase.html "struct bevy::render::render_phase::SortedRenderPhase")s as appropriate.