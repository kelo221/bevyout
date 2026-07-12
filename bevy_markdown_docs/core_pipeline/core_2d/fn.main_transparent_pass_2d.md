[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_2d](index.html)

# Function main\_transparent\_pass\_2d 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/main_transparent_pass_2d_node.rs.html#15-25)

```rust
pub fn main_transparent_pass_2d(
    world: &World,
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ExtractedView, &ViewTarget, &ViewDepthTexture)>,
    transparent_phases: Res<'_, ViewSortedRenderPhases<Transparent2d>>,
    ctx: RenderContext<'_, '_>,
)
```