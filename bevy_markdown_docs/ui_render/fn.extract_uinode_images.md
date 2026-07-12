[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_uinode\_images 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#488-506)

```rust
pub fn extract_uinode_images(
    commands: Commands<'_, '_>,
    extracted_uinodes: ResMut<'_, ExtractedUiNodes>,
    texture_atlases: Extract<'_, '_, Res<'_, Assets<TextureAtlasLayout>>>,
    uinode_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &InheritedVisibility, Option<&CalculatedClip>, &ComputedUiTargetCamera, &ImageNode, &ImageNodeSize)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```