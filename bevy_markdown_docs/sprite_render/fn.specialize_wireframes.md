[bevy](../index.html)::[sprite\_render](index.html)

# Function specialize\_wireframes 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#739-753)

```rust
pub fn specialize_wireframes(
    render_meshes: Res<'_, RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<'_, RenderMesh2dInstances>,
    render_wireframe_instances: Res<'_, RenderWireframeInstances>,
    wireframe_phases: Res<'_, ViewBinnedRenderPhases<Wireframe2dPhaseItem>>,
    views: Query<'_, '_, (&ExtractedView, &RenderVisibleEntities)>,
    view_key_cache: Res<'_, ViewKeyCache>,
    dirty_wireframe_specializations: Res<'_, DirtyWireframeSpecializations>,
    specialized_material_pipeline_cache: ResMut<'_, SpecializedWireframePipelineCache>,
    pending_wireframe2d_queues: ResMut<'_, PendingWireframe2dQueues>,
    pipelines: ResMut<'_, SpecializedMeshPipelines<Wireframe2dPipeline>>,
    pipeline: Res<'_, Wireframe2dPipeline>,
    pipeline_cache: Res<'_, PipelineCache>,
    ticks: SystemChangeTick,
)
```