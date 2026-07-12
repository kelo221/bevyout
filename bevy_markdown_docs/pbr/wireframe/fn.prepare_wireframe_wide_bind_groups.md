[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function prepare\_wireframe\_wide\_bind\_groups 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#413-423)

```rust
pub fn prepare_wireframe_wide_bind_groups(
    render_mesh_instances: Res<'_, RenderMeshInstances>,
    render_meshes: Res<'_, RenderAssets<RenderMesh>>,
    render_wireframe_instances: Res<'_, RenderWireframeInstances>,
    render_wireframe_assets: Res<'_, RenderAssets<RenderWireframeMaterial>>,
    mesh_allocator: Res<'_, MeshAllocator>,
    pipeline: Res<'_, Wireframe3dPipeline>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    wide_bind_groups: ResMut<'_, WireframeWideBindGroups>,
)
```