[bevy](../../index.html)::[gizmos](../index.html)::[transform\_gizmo](index.html)

# Function point\_to\_ring\_screen\_dist 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#682-689)

```rust
pub fn point_to_ring_screen_dist(
    cursor: Vec2,
    camera: &Camera,
    cam_tf: &GlobalTransform,
    center: Vec3,
    normal: Vec3,
    radius: f32,
) -> f32
```

Minimum screen-space distance from a cursor position to a 3D ring projected onto screen.