[bevy](../index.html)::[pbr](index.html)

# Function extract\_skins 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/skin.rs.html#272-289)

```rust
pub fn extract_skins(
    skin_uniforms: ResMut<'_, SkinUniforms>,
    skinned_meshes: Extract<'_, '_, Query<'_, '_, (Entity, &SkinnedMesh)>>,
    changed_skinned_meshes: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &SkinnedMesh), Or<(Changed<ViewVisibility>, Changed<SkinnedMesh>, AssetChanged<SkinnedMesh>)>>>,
    skinned_mesh_inverse_bindposes: Extract<'_, '_, Res<'_, Assets<SkinnedMeshInverseBindposes>>>,
    changed_transforms: Extract<'_, '_, Query<'_, '_, (Entity, &GlobalTransform), Changed<GlobalTransform>>>,
    joints: Extract<'_, '_, Query<'_, '_, &GlobalTransform>>,
    removed_skinned_meshes_query: Extract<'_, '_, RemovedComponents<'_, '_, SkinnedMesh>>,
)
```