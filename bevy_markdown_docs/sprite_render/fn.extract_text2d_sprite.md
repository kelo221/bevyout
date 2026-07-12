[bevy](../index.html)::[sprite\_render](index.html)

# Function extract\_text2d\_sprite 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/text2d/mod.rs.html#24-51)

```rust
pub fn extract_text2d_sprite(
    commands: Commands<'_, '_>,
    extracted_sprites: ResMut<'_, ExtractedSprites>,
    extracted_slices: ResMut<'_, ExtractedSlices>,
    text2d_query: Extract<'_, '_, Query<'_, '_, (Entity, &ViewVisibility, &ComputedTextBlock, &TextLayoutInfo, &TextBounds, &Anchor, Option<&Text2dShadow>, &GlobalTransform)>>,
    text_colors: Extract<'_, '_, Query<'_, '_, &TextColor>>,
    text_background_colors_query: Extract<'_, '_, Query<'_, '_, &TextBackgroundColor>>,
    decoration_query: Extract<'_, '_, Query<'_, '_, (&TextColor, Has<Strikethrough>, Has<Underline>, Option<&StrikethroughColor>, Option<&UnderlineColor>)>>,
)
```

This system extracts the sprites from the 2D text components and adds them to the “render world”.