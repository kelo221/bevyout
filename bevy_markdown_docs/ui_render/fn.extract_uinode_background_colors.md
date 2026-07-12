[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_uinode\_background\_colors 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#389-406)

```rust
pub fn extract_uinode_background_colors(
    commands: Commands<'_, '_>,
    extracted_uinodes: ResMut<'_, ExtractedUiNodes>,
    uinode_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &InheritedVisibility, Option<&CalculatedClip>, &ComputedUiTargetCamera, &BackgroundColor, Option<&OuterColor>)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```