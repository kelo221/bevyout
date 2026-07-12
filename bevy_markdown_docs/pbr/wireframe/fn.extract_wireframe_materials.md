[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function extract\_wireframe\_materials 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#999-1009)

```rust
pub fn extract_wireframe_materials(
    material_instances: ResMut<'_, RenderWireframeInstances>,
    changed_meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &Mesh3dWireframe), Or<(Changed<ViewVisibility>, Changed<Mesh3dWireframe>)>>>,
    removed_visibilities_query: Extract<'_, '_, RemovedComponents<'_, '_, ViewVisibility>>,
    removed_materials_query: Extract<'_, '_, RemovedComponents<'_, '_, Mesh3dWireframe>>,
)
```