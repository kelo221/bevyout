[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_mesh2d 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#268-282)

```rust
pub fn extract_mesh2d(
    render_mesh_instances: ResMut<'_, RenderMesh2dInstances>,
    render_material_2d_bind_group_ids: Res<'_, RenderMaterial2dBindGroupIds>,
    render_material_instances: Res<'_, RenderMaterial2dIds>,
    query: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &GlobalTransform, &Mesh2d, Option<&MeshTag>, Has<NoAutomaticBatching>)>>,
)
```