[bevy](../../index.html)::[core\_pipeline](../index.html)::[fullscreen\_material](index.html)

# Function fullscreen\_material\_system 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_material.rs.html#280-289)

```rust
pub fn fullscreen_material_system<T>(
    view: ViewQuery<'_, '_, (&ViewTarget, &DynamicUniformIndex<T>, &FullscreenMaterialBindGroup<T>, &FullscreenMaterialPipelineId)>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
)where
    T: FullscreenMaterial,
```