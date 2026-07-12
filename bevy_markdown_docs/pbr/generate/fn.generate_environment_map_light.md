[bevy](../../index.html)::[pbr](../index.html)::[generate](index.html)

# Function generate\_environment\_map\_light 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#1018-1022)

```rust
pub fn generate_environment_map_light(
    commands: Commands<'_, '_>,
    images: ResMut<'_, Assets<Image>>,
    query: Query<'_, '_, (Entity, &GeneratedEnvironmentMapLight), Without<EnvironmentMapLight>>,
)
```

System that generates an `EnvironmentMapLight` component based on the `GeneratedEnvironmentMapLight` component