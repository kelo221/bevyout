[bevy](../index.html)::[pbr](index.html)

# Function update\_mesh\_previous\_global\_transforms 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#222-230)

```rust
pub fn update_mesh_previous_global_transforms(
    commands: Commands<'_, '_>,
    views: Query<'_, '_, &Camera, Or<(With<Camera3d>, With<ShadowView>)>>,
    new_meshes: Query<'_, '_, (Entity, &GlobalTransform), (Or<(With<Mesh3d>, With<MeshletMesh3d>)>, Without<PreviousGlobalTransform>)>,
    meshes: Query<'_, '_, (Ref<'_, GlobalTransform>, &mut PreviousGlobalTransform), Or<(With<Mesh3d>, With<MeshletMesh3d>)>>,
)
```