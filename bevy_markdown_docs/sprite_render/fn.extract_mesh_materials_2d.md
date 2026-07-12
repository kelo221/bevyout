[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_mesh\_materials\_2d 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#340-350)

```rust
pub fn extract_mesh_materials_2d<M>(
    material_instances: ResMut<'_, RenderMaterial2dInstances<M>>,
    render_material_2d_ids: ResMut<'_, RenderMaterial2dIds>,
    changed_meshes_query: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &MeshMaterial2d<M>), Or<(Changed<ViewVisibility>, Changed<MeshMaterial2d<M>>)>>>,
    removed_materials_query: Extract<'_, '_, RemovedComponents<'_, '_, MeshMaterial2d<M>>>,
)where
    M: Material2d,
```