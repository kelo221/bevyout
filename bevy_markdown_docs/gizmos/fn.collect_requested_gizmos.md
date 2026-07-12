[bevy](../index.html)::[gizmos](index.html)

# Function collect\_requested\_gizmos 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#249-254)

```rust
pub fn collect_requested_gizmos<Config, Clear>(
    update: ResMut<'_, GizmoStorage<Config, ()>>,
    context: ResMut<'_, GizmoStorage<Config, Clear>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

Collect the requested gizmos into a specific clear context.