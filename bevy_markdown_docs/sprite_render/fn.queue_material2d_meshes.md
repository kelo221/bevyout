[bevy](../index.html)::[sprite\_render](index.html)

# Function queue\_material2d\_meshes 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#864-879)

```rust
pub fn queue_material2d_meshes<M>(
    _: (Res<'_, RenderAssets<RenderMesh>>, Res<'_, RenderAssets<PreparedMaterial2d<M>>>),
    render_mesh_instances: ResMut<'_, RenderMesh2dInstances>,
    render_material_instances: Res<'_, RenderMaterial2dInstances<M>>,
    transparent_render_phases: ResMut<'_, ViewSortedRenderPhases<Transparent2d>>,
    opaque_render_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque2d>>,
    alpha_mask_render_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask2d>>,
    views: Query<'_, '_, (&MainEntity, &ExtractedView, &RenderVisibleEntities)>,
    dirty_specializations: Res<'_, DirtySpecializations>,
    pending_mesh_material2d_queues: ResMut<'_, PendingMeshMaterial2dQueues>,
    specialized_material_pipeline_cache: ResMut<'_, SpecializedMaterial2dPipelineCache<M>>,
)where
    M: Material2d,
    <M as AsBindGroup>::Data: PartialEq + Eq + Hash + Clone,
```