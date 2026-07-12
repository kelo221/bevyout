[bevy](../index.html)::[text](index.html)

# Function load\_font\_assets\_into\_font\_collection 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font.rs.html#51-56)

```rust
pub fn load_font_assets_into_font_collection(
    fonts: ResMut<'_, Assets<Font>>,
    loaded_fonts: Local<'_, HashSet<AssetId<Font>>>,
    font_cx: ResMut<'_, FontCx>,
    text_font_query: Query<'_, '_, &mut TextFont>,
)
```

Add new font assets to the internal font collection, and set any associated `TextFont`’s changed. If any fonts are removed, the font collection is completely rebuilt, the generic families are remapped, and all `TextFont`s are set changed.

Font asset changes are track locally instead of waiting for asset events. Text layout also builds the atlas images, and waiting for asset events would delay the image updates by a frame.