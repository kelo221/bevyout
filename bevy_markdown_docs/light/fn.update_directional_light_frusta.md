[bevy](../index.html)::[light](index.html)

# Function update\_directional\_light\_frusta 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#217-230)

```rust
pub fn update_directional_light_frusta(
    views: Query<'_, '_, (&Cascades, &DirectionalLight, &ViewVisibility, &mut CascadesFrusta), (Without<Camera>,)>,
)
```

Updates the frusta for all visible shadow mapped [`DirectionalLight`](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s.