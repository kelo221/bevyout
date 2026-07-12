[bevy](../../index.html)::[gizmos](../index.html)::[transform\_gizmo](index.html)

# Function translation\_plane\_normal 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#651)

```rust
pub fn translation_plane_normal(ray: Ray3d, axis: Vec3) -> Vec3
```

Construct the constraint plane normal for axis translation/scale.

The plane contains the drag axis and is oriented to face the camera as much as possible, matching the approach from `bevy_transform_gizmo`.