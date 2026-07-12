[bevy](../index.html)::[gizmos](index.html)

# Function propagate\_gizmos 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#272-277)

```rust
pub fn propagate_gizmos<Config, Clear>(
    update_storage: ResMut<'_, GizmoStorage<Config, ()>>,
    contextual_storage: Res<'_, GizmoStorage<Config, Clear>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

Propagate the contextual gizmo into the `Update` storage for rendering.

This should be before [`GizmoMeshSystems`](struct.GizmoMeshSystems.html "struct bevy::gizmos::GizmoMeshSystems").