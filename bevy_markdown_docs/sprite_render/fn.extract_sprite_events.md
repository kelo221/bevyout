[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_sprite\_events 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#348-351)

```rust
pub fn extract_sprite_events(
    events: ResMut<'_, SpriteAssetEvents>,
    image_events: Extract<'_, '_, MessageReader<'_, '_, AssetEvent<Image>>>,
)
```