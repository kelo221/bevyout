[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_wireframe\_materials 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#511-521)

```rust
pub fn extract_wireframe_materials(
    material_instances: ResMut<'_, RenderWireframeInstances>,
    changed_meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &Mesh2dWireframe), Or<(Changed<ViewVisibility>, Changed<Mesh2dWireframe>)>>>,
    removed_visibilities_query: Extract<'_, '_, RemovedComponents<'_, '_, ViewVisibility>>,
    removed_materials_query: Extract<'_, '_, RemovedComponents<'_, '_, Mesh2dWireframe>>,
)
```