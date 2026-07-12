[bevy](../../../index.html)::[core\_pipeline](../../index.html)::[deferred](../index.html)::[node](index.html)

# Function late\_deferred\_prepass 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/node.rs.html#67-73)

```rust
pub fn late_deferred_prepass(
    world: &World,
    view: ViewQuery<'_, '_, (&'static ExtractedCamera, &'static ExtractedView, &'static ViewDepthTexture, &'static ViewPrepassTextures, Option<&'static MainPassResolutionOverride>, Has<OcclusionCulling>, Has<NoIndirectDrawing>)>,
    opaque_deferred_phases: Res<'_, ViewBinnedRenderPhases<Opaque3dDeferred>>,
    alpha_mask_deferred_phases: Res<'_, ViewBinnedRenderPhases<AlphaMask3dDeferred>>,
    ctx: RenderContext<'_, '_>,
)
```