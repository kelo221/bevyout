[bevy](../index.html)::[sprite\_render](index.html)

# Function check\_entities\_needing\_specialization 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#677-695)

```rust
pub fn check_entities_needing_specialization<M>(
    needs_specialization: Query<'_, '_, Entity, (Or<(Changed<Mesh2d>, AssetChanged<Mesh2d>, Changed<MeshMaterial2d<M>>, AssetChanged<MeshMaterial2d<M>>)>, With<MeshMaterial2d<M>>)>,
    par_local: Local<'_, Parallel<Vec<Entity>>>,
    entities_needing_specialization: ResMut<'_, EntitiesNeedingSpecialization<M>>,
    removed_mesh_2d_components: RemovedComponents<'_, '_, Mesh2d>,
    removed_mesh_material_2d_components: RemovedComponents<'_, '_, MeshMaterial2d<M>>,
)where
    M: Material2d,
```

Finds 2D entities that have changed in such a way as to potentially require specialization and adds them to the [`EntitiesNeedingSpecialization`](struct.EntitiesNeedingSpecialization.html "struct bevy::sprite_render::EntitiesNeedingSpecialization") list.