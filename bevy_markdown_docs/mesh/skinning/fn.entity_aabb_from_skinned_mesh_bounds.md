[bevy](../../index.html)::[mesh](../index.html)::[skinning](index.html)

# Function entity\_aabb\_from\_skinned\_mesh\_bounds 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#191-197)

```rust
pub fn entity_aabb_from_skinned_mesh_bounds(
    joint_entities: &Query<'_, '_, &GlobalTransform>,
    mesh: &Mesh,
    skinned_mesh: &SkinnedMesh,
    skinned_mesh_inverse_bindposes: &SkinnedMeshInverseBindposes,
    world_from_entity: Option<&GlobalTransform>,
) -> Result<Aabb3d, EntityAabbFromSkinnedMeshBoundsError>
```

Given the components of a skinned mesh entity, return an `Aabb3d` that encloses the skinned vertices of the mesh.