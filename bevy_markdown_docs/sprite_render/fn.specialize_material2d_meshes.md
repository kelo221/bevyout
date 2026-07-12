[bevy](../index.html)::[sprite\_render](index.html)

# Function specialize\_material2d\_meshes 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#743-762)

```rust
pub fn specialize_material2d_meshes<M>(
    material2d_pipeline: Res<'_, Material2dPipeline<M>>,
    pipelines: ResMut<'_, SpecializedMeshPipelines<Material2dPipeline<M>>>,
    pipeline_cache: Res<'_, PipelineCache>,
    _: (Res<'_, RenderAssets<RenderMesh>>, Res<'_, RenderAssets<PreparedMaterial2d<M>>>),
    render_mesh_instances: ResMut<'_, RenderMesh2dInstances>,
    render_material_instances: Res<'_, RenderMaterial2dInstances<M>>,
    transparent_render_phases: Res<'_, ViewSortedRenderPhases<Transparent2d>>,
    opaque_render_phases: Res<'_, ViewBinnedRenderPhases<Opaque2d>>,
    alpha_mask_render_phases: Res<'_, ViewBinnedRenderPhases<AlphaMask2d>>,
    views: Query<'_, '_, (&MainEntity, &ExtractedView, &RenderVisibleEntities)>,
    view_key_cache: Res<'_, ViewKeyCache>,
    dirty_specializations: Res<'_, DirtySpecializations>,
    pending_mesh_material2d_queues: ResMut<'_, PendingMeshMaterial2dQueues>,
    specialized_material_pipeline_cache: ResMut<'_, SpecializedMaterial2dPipelineCache<M>>,
)where
    M: Material2d,
    <M as AsBindGroup>::Data: PartialEq + Eq + Hash + Clone,
```