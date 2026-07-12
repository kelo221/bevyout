[bevy](../index.html)::[light](index.html)

# Function update\_spot\_light\_frusta 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#232-241)

```rust
pub fn update_spot_light_frusta(
    views: Query<'_, '_, (&GlobalTransform, &SpotLight, &mut Frustum, &ViewVisibility), Or<(Changed<GlobalTransform>, Changed<SpotLight>, Changed<ViewVisibility>)>>,
)
```

Updates the frusta for all visible shadow mapped [`SpotLight`](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")s.