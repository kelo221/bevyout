[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_2d](index.html)

# Function main\_opaque\_pass\_2d 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/main_opaque_pass_2d_node.rs.html#17-28)

```rust
pub fn main_opaque_pass_2d(
    world: &World,
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ExtractedView, &ViewTarget, &ViewDepthTexture)>,
    opaque_phases: Res<'_, ViewBinnedRenderPhases<Opaque2d>>,
    alpha_mask_phases: Res<'_, ViewBinnedRenderPhases<AlphaMask2d>>,
    ctx: RenderContext<'_, '_>,
)
```