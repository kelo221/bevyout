[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Function main\_transparent\_pass\_3d 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/main_transparent_pass_3d_node.rs.html#19-32)

```rust
pub fn main_transparent_pass_3d(
    world: &World,
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ExtractedView, &ViewTarget, &ViewDepthTexture, Option<&MainPassResolutionOverride>, Has<OrderIndependentTransparencySettings>, Option<&OitResolvePipelineId>)>,
    transparent_phases: Res<'_, ViewSortedRenderPhases<Transparent3d>>,
    ctx: RenderContext<'_, '_>,
)
```