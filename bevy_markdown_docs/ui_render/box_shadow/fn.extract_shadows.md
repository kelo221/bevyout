[bevy](../../index.html)::[ui\_render](../index.html)::[box\_shadow](index.html)

# Function extract\_shadows 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#206-223)

```rust
pub fn extract_shadows(
    commands: Commands<'_, '_>,
    extracted_box_shadows: ResMut<'_, ExtractedBoxShadows>,
    box_shadow_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &InheritedVisibility, &BoxShadow, Option<&CalculatedClip>, &ComputedUiTargetCamera, &ComputedUiRenderTargetInfo)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```