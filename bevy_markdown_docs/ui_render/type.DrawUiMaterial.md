[bevy](../index.html)::[ui\_render](index.html)

# Type Alias DrawUiMaterial 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#213)

```rust
pub type DrawUiMaterial<M> = (SetItemPipeline, SetMatUiViewBindGroup<M, 0>, SetUiMaterialBindGroup<M, 1>, DrawUiMaterialNode<M>);
```