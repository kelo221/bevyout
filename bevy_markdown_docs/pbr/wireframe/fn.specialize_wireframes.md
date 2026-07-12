[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function specialize\_wireframes 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#1353-1369)

```rust
pub fn specialize_wireframes(
    render_meshes: Res<'_, RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<'_, RenderMeshInstances>,
    render_wireframe_instances: Res<'_, RenderWireframeInstances>,
    render_wireframe_assets: Res<'_, RenderAssets<RenderWireframeMaterial>>,
    render_visibility_ranges: Res<'_, RenderVisibilityRanges>,
    wireframe_phases: Res<'_, ViewBinnedRenderPhases<Wireframe3d>>,
    views: Query<'_, '_, (&ExtractedView, &RenderVisibleEntities)>,
    view_key_cache: Res<'_, ViewKeyCache>,
    dirty_wireframe_specializations: Res<'_, DirtyWireframeSpecializations>,
    specialized_material_pipeline_cache: ResMut<'_, SpecializedWireframePipelineCache>,
    pipelines: ResMut<'_, SpecializedMeshPipelines<Wireframe3dPipeline>>,
    pending_wireframe_queues: ResMut<'_, PendingWireframeQueues>,
    pipeline: Res<'_, Wireframe3dPipeline>,
    pipeline_cache: Res<'_, PipelineCache>,
    render_lightmaps: Res<'_, RenderLightmaps>,
)
```