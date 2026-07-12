[bevy](../index.html)::[ui\_render](index.html)

# Function extract\_ui\_material\_nodes 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#321-338)

```rust
pub fn extract_ui_material_nodes<M>(
    commands: Commands<'_, '_>,
    extracted_uinodes: ResMut<'_, ExtractedUiMaterialNodes<M>>,
    materials: Extract<'_, '_, Res<'_, Assets<M>>>,
    uinode_query: Extract<'_, '_, Query<'_, '_, (Entity, &ComputedNode, &ComputedStackIndex, &UiGlobalTransform, &MaterialNode<M>, &InheritedVisibility, Option<&CalculatedClip>, &ComputedUiTargetCamera)>>,
    camera_map: Extract<'_, '_, UiCameraMap<'_, '_>>,
)where
    M: UiMaterial,
```