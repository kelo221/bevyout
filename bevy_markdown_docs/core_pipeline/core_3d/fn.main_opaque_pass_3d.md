[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Function main\_opaque\_pass\_3d 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/main_opaque_pass_3d_node.rs.html#21-37)

```rust
pub fn main_opaque_pass_3d(
    world: &World,
    view: ViewQuery<'_, '_, (&ExtractedCamera, &ExtractedView, &ViewTarget, &ViewDepthTexture, Option<&SkyboxPipelineId>, Option<&SkyboxBindGroup>, &ViewUniformOffset, Option<&MainPassResolutionOverride>)>,
    opaque_phases: Res<'_, ViewBinnedRenderPhases<Opaque3d>>,
    alpha_mask_phases: Res<'_, ViewBinnedRenderPhases<AlphaMask3d>>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
)
```