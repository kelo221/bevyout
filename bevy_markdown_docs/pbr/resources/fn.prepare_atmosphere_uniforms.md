[bevy](../../index.html)::[pbr](../index.html)::[resources](index.html)

# Function prepare\_atmosphere\_uniforms 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#485-488)

```rust
pub fn prepare_atmosphere_uniforms(
    commands: Commands<'_, '_>,
    atmospheres: Query<'_, '_, (Entity, &ExtractedAtmosphere)>,
) -> Result<(), BevyError>
```