[bevy](../index.html)::[gizmos](index.html)

# Macro resolve\_gizmo\_camera 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#253)

```rust
macro_rules! resolve_gizmo_camera {
    ($marked:expr, $all:expr) => { ... };
}
```

Resolves which camera the gizmo should use.

Prefers cameras marked with [`TransformGizmoCamera`](../prelude/struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera"). Falls back to the sole camera in the world when no marker is present, and warns when ambiguous.