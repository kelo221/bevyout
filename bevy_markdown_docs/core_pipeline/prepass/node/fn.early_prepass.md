[bevy](../../../index.html)::[core\_pipeline](../../index.html)::[prepass](../index.html)::[node](index.html)

# Function early\_prepass 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/node.rs.html#44-51)

```rust
pub fn early_prepass(
    world: &World,
    view: ViewQuery<'_, '_, ((&'static ExtractedCamera, &'static ExtractedView, &'static ViewDepthTexture, &'static ViewPrepassTextures, &'static ViewUniformOffset), (Option<&'static DeferredPrepass>, Option<&'static BackgroundMotionVectorsPipelineId>, Option<&'static BackgroundMotionVectorsBindGroup>, Option<&'static PreviousViewUniformOffset>, Option<&'static MainPassResolutionOverride>), (Has<OcclusionCulling>, Has<NoIndirectDrawing>))>,
    opaque_prepass_phases: Res<'_, ViewBinnedRenderPhases<Opaque3dPrepass>>,
    alpha_mask_prepass_phases: Res<'_, ViewBinnedRenderPhases<AlphaMask3dPrepass>>,
    pipeline_cache: Res<'_, PipelineCache>,
    ctx: RenderContext<'_, '_>,
)
```