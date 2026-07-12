[bevy](../../index.html)::[pbr](../index.html)::[wireframe](index.html)

# Function check\_wireframe\_entities\_needing\_specialization 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#1310-1325)

```rust
pub fn check_wireframe_entities_needing_specialization(
    needs_specialization: Query<'_, '_, Entity, Or<(Changed<Mesh3d>, AssetChanged<Mesh3d>, Changed<Mesh3dWireframe>, AssetChanged<Mesh3dWireframe>, Changed<WireframeLineWidth>, Changed<WireframeTopology>)>>,
    entities_needing_specialization: ResMut<'_, WireframeEntitiesNeedingSpecialization>,
    removed_mesh_3d_components: RemovedComponents<'_, '_, Mesh3d>,
    removed_mesh_3d_wireframe_components: RemovedComponents<'_, '_, Mesh3dWireframe>,
)
```

Finds 3D wireframe entities that have changed in such a way as to potentially require specialization and adds them to the [`WireframeEntitiesNeedingSpecialization`](struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::pbr::wireframe::WireframeEntitiesNeedingSpecialization") list.