[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_text\_shadows 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#1098-1118)

```rust
pub fn extract_text_shadows(
    commands: Commands<'_, '_>,
    extracted_uinodes: ResMut<'_, ExtractedUiNodes>,
    uinode_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &ComputedUiTargetCamera, &InheritedVisibility, Option<&CalculatedClip>, &TextLayoutInfo, &TextShadow, &ComputedTextBlock, Option<&TextScroll>)>>,
    text_decoration_query: Extract<'_, '_, Query<'_, '_, (Has<Strikethrough>, Has<Underline>)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```