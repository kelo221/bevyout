[bevy](../index.html)::[text](index.html)

# Function get\_glyph\_atlas\_info 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_atlas.rs.html#253-256)

```rust
pub fn get_glyph_atlas_info(
    font_atlases: &mut [FontAtlas],
    cache_key: GlyphCacheKey,
) -> Option<GlyphAtlasInfo>
```

Generates the [`GlyphAtlasInfo`](struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo") for the given subpixel-offset glyph.