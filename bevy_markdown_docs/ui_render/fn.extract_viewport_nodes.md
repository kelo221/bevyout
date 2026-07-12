[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_viewport\_nodes 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#880-897)

```rust
pub fn extract_viewport_nodes(
    commands: Commands<'_, '_>,
    extracted_uinodes: ResMut<'_, ExtractedUiNodes>,
    camera_query: Extract<'_, '_, Query<'_, '_, (&Camera, &RenderTarget)>>,
    uinode_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &InheritedVisibility, Option<&CalculatedClip>, &ComputedUiTargetCamera, &ViewportNode)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```