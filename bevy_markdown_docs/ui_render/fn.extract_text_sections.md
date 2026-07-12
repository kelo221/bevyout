[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_text\_sections 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#955-976)

```rust
pub fn extract_text_sections(
    commands: Commands<'_, '_>,
    extracted_uinodes: ResMut<'_, ExtractedUiNodes>,
    uinode_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &InheritedVisibility, Option<&CalculatedClip>, &ComputedUiTargetCamera, &ComputedTextBlock, &TextColor, &TextLayoutInfo, Option<&TextScroll>, Option<&TextCursorStyle>)>>,
    text_styles: Extract<'_, '_, Query<'_, '_, &TextColor>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```