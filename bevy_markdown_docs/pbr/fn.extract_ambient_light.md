[bevy](../index.html)::[pbr](index.html)

# Function extract\_ambient\_light 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#306-310)

```rust
pub fn extract_ambient_light(
    commands: Commands<'_, '_>,
    previous_len: Local<'_, usize>,
    query: Extract<'_, '_, Query<'_, '_, (RenderEntity, &AmbientLight)>>,
)
```