[bevy](../index.html)::[pbr](index.html)

# Function check\_entities\_needing\_specialization 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#835-853)

```rust
pub fn check_entities_needing_specialization<M>(
    needs_specialization: Query<'_, '_, Entity, (Or<(Changed<Mesh3d>, AssetChanged<Mesh3d>, Changed<MeshMaterial3d<M>>, AssetChanged<MeshMaterial3d<M>>)>, With<MeshMaterial3d<M>>)>,
    par_local: Local<'_, Parallel<Vec<Entity>>>,
    entities_needing_specialization: ResMut<'_, EntitiesNeedingSpecialization<M>>,
    removed_mesh_3d_components: RemovedComponents<'_, '_, Mesh3d>,
    removed_mesh_material_3d_components: RemovedComponents<'_, '_, MeshMaterial3d<M>>,
)where
    M: Material,
```

Finds 3D entities that have changed in such a way as to potentially require specialization and adds them to the [`EntitiesNeedingSpecialization`](struct.EntitiesNeedingSpecialization.html "struct bevy::pbr::EntitiesNeedingSpecialization") list.