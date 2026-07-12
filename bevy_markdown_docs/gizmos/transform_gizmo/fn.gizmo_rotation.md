[bevy](../../index.html)::[gizmos](../index.html)::[transform\_gizmo](index.html)

# Function gizmo\_rotation 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#740)

```rust
pub fn gizmo_rotation(
    global_tf: &GlobalTransform,
    space: &TransformGizmoSpace,
) -> Quat
```

Compute the gizmo rotation based on the space setting.