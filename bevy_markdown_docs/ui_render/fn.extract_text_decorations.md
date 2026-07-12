[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_text\_decorations 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#1266-1292)

```rust
pub fn extract_text_decorations(
    commands: Commands<'_, '_>,
    extracted_uinodes: ResMut<'_, ExtractedUiNodes>,
    uinode_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &ComputedTextBlock, &UiGlobalTransform, &InheritedVisibility, Option<&CalculatedClip>, &ComputedUiTargetCamera, &TextLayoutInfo, Option<&TextScroll>)>>,
    text_background_colors_query: Extract<'_, '_, Query<'_, '_, (AnyOf<(&TextBackgroundColor, &Strikethrough, &Underline)>, &TextColor, Option<&StrikethroughColor>, Option<&UnderlineColor>)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```