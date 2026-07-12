[bevy](../index.html)::[pbr](index.html)

# Function extract\_shadow\_filtering\_method 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#273-277)

```rust
pub fn extract_shadow_filtering_method(
    commands: Commands<'_, '_>,
    previous_len: Local<'_, usize>,
    query: Extract<'_, '_, Query<'_, '_, (RenderEntity, &ShadowFilteringMethod)>>,
)
```