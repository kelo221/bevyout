[bevy](../index.html)::[light](index.html)

# Function update\_point\_light\_frusta 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#212-226)

```rust
pub fn update_point_light_frusta(
    views: Query<'_, '_, (&GlobalTransform, &PointLight, &mut CubemapFrusta, &ViewVisibility), Or<(Changed<GlobalTransform>, Changed<PointLight>, Changed<ViewVisibility>)>>,
)
```

Updates the frusta for all visible shadow mapped [`PointLight`](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")s.