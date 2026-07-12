[bevy](../index.html)::[text](index.html)

# Function get\_outlined\_glyph\_texture 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_atlas.rs.html#185-189)

```rust
pub fn get_outlined_glyph_texture(
    scaler: &mut Scaler<'_>,
    glyph_id: u16,
    font_smoothing: FontSmoothing,
) -> Result<(Image, Vec2, bool), TextError>
```

Get the texture of the glyph as a rendered image, and its offset