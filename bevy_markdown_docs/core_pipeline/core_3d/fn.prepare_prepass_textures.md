[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Function prepare\_prepass\_textures 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#762-783)

```rust
pub fn prepare_prepass_textures(
    commands: Commands<'_, '_>,
    texture_cache: ResMut<'_, TextureCache>,
    render_device: Res<'_, RenderDevice>,
    frame_count: Res<'_, FrameCount>,
    opaque_3d_prepass_phases: Res<'_, ViewBinnedRenderPhases<Opaque3dPrepass>>,
    alpha_mask_3d_prepass_phases: Res<'_, ViewBinnedRenderPhases<AlphaMask3dPrepass>>,
    opaque_3d_deferred_phases: Res<'_, ViewBinnedRenderPhases<Opaque3dDeferred>>,
    alpha_mask_3d_deferred_phases: Res<'_, ViewBinnedRenderPhases<AlphaMask3dDeferred>>,
    views_3d: Query<'_, '_, (Entity, &ExtractedCamera, &ExtractedView, &Msaa, Has<DepthPrepass>, Has<NormalPrepass>, Has<MotionVectorPrepass>, Has<DeferredPrepass>, Has<DepthPrepassDoubleBuffer>, Has<DeferredPrepassDoubleBuffer>)>,
)
```