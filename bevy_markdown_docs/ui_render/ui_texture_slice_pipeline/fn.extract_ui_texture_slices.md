[bevy](../../index.html)::[ui\_render](../index.html)::[ui\_texture\_slice\_pipeline](index.html)

# Function extract\_ui\_texture\_slices 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#215-232)

```rust
pub fn extract_ui_texture_slices(
    commands: Commands<'_, '_>,
    extracted_ui_slicers: ResMut<'_, ExtractedUiTextureSlices>,
    texture_atlases: Extract<'_, '_, Res<'_, Assets<TextureAtlasLayout>>>,
    slicers_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &InheritedVisibility, Option<&CalculatedClip>, &ComputedUiTargetCamera, &ImageNode)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)
```