[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_sprites 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#360-375)

```rust
pub fn extract_sprites(
    extracted_sprites: ResMut<'_, ExtractedSprites>,
    extracted_slices: ResMut<'_, ExtractedSlices>,
    texture_atlases: Extract<'_, '_, Res<'_, Assets<TextureAtlasLayout>>>,
    sprite_query: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &ViewVisibility, &Sprite, &GlobalTransform, &Anchor, Option<&ComputedTextureSlices>)>>,
)
```