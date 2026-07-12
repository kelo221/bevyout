[bevy](../index.html)::[text](index.html)

# Function add\_glyph\_to\_atlas 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_atlas.rs.html#132-138)

```rust
pub fn add_glyph_to_atlas(
    font_atlases: &mut Vec<FontAtlas>,
    textures: &mut Assets<Image>,
    scaler: &mut Scaler<'_>,
    font_smoothing: FontSmoothing,
    glyph_id: u16,
) -> Result<GlyphAtlasInfo, TextError>
```

Adds the given subpixel-offset glyph to the given font atlases