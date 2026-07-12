[bevy](../index.html)::[gizmos](index.html)

# Function clear\_gizmo\_context 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#261-264)

```rust
pub fn clear_gizmo_context<Config, Clear>(
    context: ResMut<'_, GizmoStorage<Config, Clear>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

Clear out the contextual gizmos.