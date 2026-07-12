[bevy](../index.html)::[gizmos](index.html)

# Function end\_gizmo\_context 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#237-242)

```rust
pub fn end_gizmo_context<Config, Clear>(
    swap: ResMut<'_, GizmoStorage<Config, Swap<Clear>>>,
    default: ResMut<'_, GizmoStorage<Config, ()>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

End this gizmo clearing context.

Pop the default gizmos context out of the [`Swap<Clear>`](gizmos/struct.Swap.html "struct bevy::gizmos::gizmos::Swap") gizmo storage.

This must be called before [`GizmoMeshSystems`](struct.GizmoMeshSystems.html "struct bevy::gizmos::GizmoMeshSystems") in the [`Last`](../prelude/struct.Last.html "struct bevy::prelude::Last") schedule.