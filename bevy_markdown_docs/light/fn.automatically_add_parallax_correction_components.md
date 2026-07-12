[bevy](../index.html)::[light](index.html)

# Function automatically\_add\_parallax\_correction\_components 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#446-456)

```rust
pub fn automatically_add_parallax_correction_components(
    commands: Commands<'_, '_>,
    query: Query<'_, '_, Entity, (With<EnvironmentMapLight>, With<LightProbe>, Without<ParallaxCorrection>)>,
)
```

A system that automatically adds a [`ParallaxCorrection::Auto`](enum.ParallaxCorrection.html#variant.Auto "variant bevy::light::ParallaxCorrection::Auto") component to any reflection probe that doesn’t already have a [`ParallaxCorrection`](enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection") component.

A reflection probe is any entity with both an [`EnvironmentMapLight`](../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight") and a [`LightProbe`](../prelude/struct.LightProbe.html "struct bevy::prelude::LightProbe") component.