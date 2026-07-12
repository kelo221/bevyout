[bevy](../../index.html)::[gizmos](../index.html)::[transform\_gizmo](index.html)

# Function intersect\_plane 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#661)

```rust
pub fn intersect_plane(
    ray: Ray3d,
    plane_normal: Vec3,
    plane_origin: Vec3,
) -> Option<Vec3>
```

Intersect a ray with a plane defined by a normal and a point on the plane.