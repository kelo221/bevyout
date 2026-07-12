[bevy](../index.html)::[light](index.html)

# Type Alias WithLight 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#248)

```rust
pub type WithLight = Or<(With<PointLight>, With<SpotLight>, With<DirectionalLight>, With<RectLight>)>;
```

A convenient alias for `Or<(With<PointLight>, With<SpotLight>, With<DirectionalLight>, With<RectLight>)>`, for use with [`bevy_camera::visibility::VisibleEntities`](../camera/visibility/struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities").

## Aliased Type

```rust
pub struct WithLight(/* private fields */);
```