[bevy](../../index.html)::[camera](../index.html)::[visibility](index.html)

# Function update\_frusta 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#627-632)

```rust
pub fn update_frusta(
    views: Query<'_, '_, (&GlobalTransform, &Projection, &mut Frustum), Or<(Changed<GlobalTransform>, Changed<Projection>)>>,
)
```

Updates [`Frustum`](../primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum").

This system is used in [`CameraProjectionPlugin`](../struct.CameraProjectionPlugin.html "struct bevy::camera::CameraProjectionPlugin").