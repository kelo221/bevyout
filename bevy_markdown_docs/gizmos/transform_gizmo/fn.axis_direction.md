[bevy](../../index.html)::[gizmos](../index.html)::[transform\_gizmo](index.html)

# Function axis\_direction 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#638)

```rust
pub fn axis_direction(
    axis: TransformGizmoAxis,
    rotation: Quat,
    cam_tf: &GlobalTransform,
) -> Vec3
```

Get the world-space direction for a given axis.