[bevy](../../index.html)::[light](../index.html)::[cascade](index.html)

# Function build\_directional\_light\_cascades 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#195-204)

```rust
pub fn build_directional_light_cascades(
    directional_light_shadow_map: Res<'_, DirectionalLightShadowMap>,
    views: Query<'_, '_, (Entity, &GlobalTransform, &Projection, &Camera)>,
    lights: Query<'_, '_, (&GlobalTransform, &DirectionalLight, &CascadeShadowConfig, &mut Cascades)>,
)
```

Sets up [`Cascades`](../struct.Cascades.html "struct bevy::light::Cascades") for all shadow mapped [`DirectionalLight`](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")s.