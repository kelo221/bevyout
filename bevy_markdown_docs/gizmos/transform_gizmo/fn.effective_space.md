[bevy](../../index.html)::[gizmos](../index.html)::[transform\_gizmo](index.html)

# Function effective\_space 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#731)

```rust
pub fn effective_space(
    settings: &TransformGizmoSettings,
) -> &TransformGizmoSpace
```

Return the effective space for the gizmo: scale always uses local space.