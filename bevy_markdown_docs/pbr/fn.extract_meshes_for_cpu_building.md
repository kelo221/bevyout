[bevy](../index.html)::[pbr](index.html)

# Function extract\_meshes\_for\_cpu\_building 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#1758-1781)

```rust
pub fn extract_meshes_for_cpu_building(
    render_mesh_instances: ResMut<'_, RenderMeshInstances>,
    mesh_material_ids: Res<'_, RenderMaterialInstances>,
    render_material_bindings: Res<'_, RenderMaterialBindings>,
    render_visibility_ranges: Res<'_, RenderVisibilityRanges>,
    render_mesh_instance_queues: Local<'_, Parallel<Vec<(Entity, RenderMeshInstanceCpu)>>>,
    meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &GlobalTransform, Option<&PreviousGlobalTransform>, &Mesh3d, Option<&MeshTag>, Has<NoFrustumCulling>, Has<NotShadowReceiver>, Has<TransmittedShadowReceiver>, Has<NotShadowCaster>, Has<NoAutomaticBatching>, Option<&VisibilityRange>, Option<&RenderLayers>)>>,
)
```

Extracts meshes from the main world into the render world, populating the [`RenderMeshInstances`](enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances").

This is the variant of the system that runs when we’re _not_ using GPU [`MeshUniform`](struct.MeshUniform.html "struct bevy::pbr::MeshUniform") building.