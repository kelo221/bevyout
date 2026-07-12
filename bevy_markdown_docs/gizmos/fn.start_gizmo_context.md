[bevy](../index.html)::[gizmos](index.html)

# Function start\_gizmo\_context 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#222-227)

```rust
pub fn start_gizmo_context<Config, Clear>(
    swap: ResMut<'_, GizmoStorage<Config, Swap<Clear>>>,
    default: ResMut<'_, GizmoStorage<Config, ()>>,
)where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
```

Start a new gizmo clearing context.

Internally this pushes the parent default context into a swap buffer. Gizmo contexts should be handled like a stack, so if you push a new context, you must pop the context before the parent context ends.