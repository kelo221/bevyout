[bevy](../index.html)::[pbr](index.html)

# Function extract\_meshes\_for\_gpu\_building 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#1901-1963)

```rust
pub fn extract_meshes_for_gpu_building(
    render_mesh_instances: ResMut<'_, RenderMeshInstances>,
    render_visibility_ranges: Res<'_, RenderVisibilityRanges>,
    render_mesh_instance_queues: ResMut<'_, RenderMeshInstanceGpuQueues>,
    changed_meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &'static ViewVisibility, &'static GlobalTransform, Option<&'static PreviousGlobalTransform>, Option<&'static Lightmap>, Option<&'static Aabb>, &'static Mesh3d, Option<&'static MeshTag>, (Has<NoFrustumCulling>, Has<NotShadowReceiver>, Has<TransmittedShadowReceiver>, Has<NotShadowCaster>, Has<NoAutomaticBatching>, Has<NoCpuCulling>), Option<&'static VisibilityRange>, Option<&'static RenderLayers>), Or<(Changed<ViewVisibility>, Changed<GlobalTransform>, Changed<PreviousGlobalTransform>, Changed<Lightmap>, Changed<Aabb>, Changed<Mesh3d>, Changed<MeshTag>, Or<(Changed<NoFrustumCulling>, Changed<NotShadowReceiver>, Changed<TransmittedShadowReceiver>, Changed<NotShadowCaster>, Changed<NoAutomaticBatching>, Changed<NoCpuCulling>)>, Changed<VisibilityRange>, Changed<SkinnedMesh>)>>>,
    _: (Extract<'_, '_, RemovedComponents<'_, '_, PreviousGlobalTransform>>, Extract<'_, '_, RemovedComponents<'_, '_, Lightmap>>, Extract<'_, '_, RemovedComponents<'_, '_, Aabb>>, Extract<'_, '_, RemovedComponents<'_, '_, MeshTag>>, Extract<'_, '_, RemovedComponents<'_, '_, NoFrustumCulling>>, Extract<'_, '_, RemovedComponents<'_, '_, NotShadowReceiver>>, Extract<'_, '_, RemovedComponents<'_, '_, TransmittedShadowReceiver>>, Extract<'_, '_, RemovedComponents<'_, '_, NotShadowCaster>>, Extract<'_, '_, RemovedComponents<'_, '_, NoAutomaticBatching>>, Extract<'_, '_, RemovedComponents<'_, '_, NoCpuCulling>>, Extract<'_, '_, RemovedComponents<'_, '_, VisibilityRange>>, Extract<'_, '_, RemovedComponents<'_, '_, SkinnedMesh>>),
    all_meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &'static ViewVisibility, &'static GlobalTransform, Option<&'static PreviousGlobalTransform>, Option<&'static Lightmap>, Option<&'static Aabb>, &'static Mesh3d, Option<&'static MeshTag>, (Has<NoFrustumCulling>, Has<NotShadowReceiver>, Has<TransmittedShadowReceiver>, Has<NotShadowCaster>, Has<NoAutomaticBatching>, Has<NoCpuCulling>), Option<&'static VisibilityRange>, Option<&'static RenderLayers>)>>,
    removed_meshes_query: Extract<'_, '_, RemovedComponents<'_, '_, Mesh3d>>,
    gpu_culling_query: Extract<'_, '_, Query<'_, '_, (), (With<Camera>, Without<NoIndirectDrawing>)>>,
    meshes_to_reextract_next_frame: ResMut<'_, MeshesToReextractNextFrame>,
    reextract_entities: Local<'_, EntityHashSet>,
    potential_reextraction_set: Local<'_, IndexSet<Entity, EntityHash>>,
    potential_reextraction_bitfield: Local<'_, Vec<Atomic<u64>>>,
)
```

Extracts meshes from the main world to thread-local buffers in the render world.

This is optimized to only look at entities that have changed since the last frame.

This is the variant of the system that runs when we’re using GPU [`MeshUniform`](struct.MeshUniform.html "struct bevy::pbr::MeshUniform") building.