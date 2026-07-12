[bevy](../index.html)::[sprite\_render](index.html)

# Function check\_wireframe\_entities\_needing\_specialization 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#701-714)

```rust
pub fn check_wireframe_entities_needing_specialization(
    needs_specialization: Query<'_, '_, Entity, Or<(Changed<Mesh2d>, AssetChanged<Mesh2d>, Changed<Mesh2dWireframe>, AssetChanged<Mesh2dWireframe>)>>,
    entities_needing_specialization: ResMut<'_, WireframeEntitiesNeedingSpecialization>,
    removed_mesh_2d_components: RemovedComponents<'_, '_, Mesh2d>,
    removed_mesh_2d_wireframe_components: RemovedComponents<'_, '_, Mesh2dWireframe>,
)
```

Finds 2D wireframe entities that have changed in such a way as to potentially require specialization and adds them to the [`WireframeEntitiesNeedingSpecialization`](struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::sprite_render::WireframeEntitiesNeedingSpecialization") list.