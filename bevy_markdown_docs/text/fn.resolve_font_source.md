[bevy](../index.html)::[text](index.html)

# Function resolve\_font\_source 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#415-418)

```rust
pub fn resolve_font_source<'a>(
    text_font: &'a TextFont,
    fonts: &'a Assets<Font>,
) -> Result<FontFamily<'a>, TextError>
```

Resolve a [`TextFont`](../prelude/struct.TextFont.html "struct bevy::prelude::TextFont")’s [`FontSource`](../prelude/enum.FontSource.html "enum bevy::prelude::FontSource") to a font family.