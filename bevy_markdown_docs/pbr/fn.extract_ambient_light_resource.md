[bevy](../index.html)::[pbr](index.html)

# Function extract\_ambient\_light\_resource 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#288-292)

```rust
pub fn extract_ambient_light_resource(
    commands: Commands<'_, '_>,
    main_resource: Extract<'_, '_, Option<Res<'_, GlobalAmbientLight>>>,
    target_resource: Option<ResMut<'_, GlobalAmbientLight>>,
)
```