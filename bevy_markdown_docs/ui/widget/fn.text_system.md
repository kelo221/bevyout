[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function text\_system 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#340-353)

```rust
pub fn text_system(
    textures: ResMut<'_, Assets<Image>>,
    font_atlas_set: ResMut<'_, FontAtlasSet>,
    text_pipeline: ResMut<'_, TextPipeline>,
    text_query: Query<'_, '_, (Ref<'_, ComputedNode>, &TextLayout, &mut TextLayoutInfo, &mut TextNodeFlags, &mut ComputedTextBlock, Ref<'_, FontHinting>)>,
    scale_cx: ResMut<'_, ScaleCx>,
)
```

Updates the layout and size information for a UI text node on changes to the size value of its [`Node`](../../prelude/struct.Node.html "struct bevy::prelude::Node") component, or when the `needs_recompute` field of [`TextNodeFlags`](../struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags") is set to true. This information is computed by the [`TextPipeline`](../../text/struct.TextPipeline.html "struct bevy::text::TextPipeline") and then stored in [`TextLayoutInfo`](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo").

### World Resources

[`ResMut<Assets<Image>>`](../../prelude/struct.Assets.html "struct bevy::prelude::Assets") – This system only adds new [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") assets. It does not modify or observe existing ones. The exception is when adding new glyphs to a [`bevy_text::FontAtlas`](../../text/struct.FontAtlas.html "struct bevy::text::FontAtlas").