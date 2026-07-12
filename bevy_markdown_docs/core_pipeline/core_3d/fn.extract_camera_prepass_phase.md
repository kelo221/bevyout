[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Function extract\_camera\_prepass\_phase 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#536-561)

```rust
pub fn extract_camera_prepass_phase(
    commands: Commands<'_, '_>,
    opaque_3d_prepass_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque3dPrepass>>,
    alpha_mask_3d_prepass_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask3dPrepass>>,
    opaque_3d_deferred_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque3dDeferred>>,
    alpha_mask_3d_deferred_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask3dDeferred>>,
    cameras_3d: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &Camera, Has<NoIndirectDrawing>, Has<DepthPrepass>, Has<NormalPrepass>, Has<MotionVectorPrepass>, Has<DeferredPrepass>, Has<DepthPrepassDoubleBuffer>, Has<DeferredPrepassDoubleBuffer>), With<Camera3d>>>,
    live_entities: Local<'_, HashSet<RetainedViewEntity>>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
)
```