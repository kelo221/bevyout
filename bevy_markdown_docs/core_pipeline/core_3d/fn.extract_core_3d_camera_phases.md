[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Function extract\_core\_3d\_camera\_phases 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#496-503)

```rust
pub fn extract_core_3d_camera_phases(
    opaque_3d_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque3d>>,
    alpha_mask_3d_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask3d>>,
    transparent_3d_phases: ResMut<'_, ViewSortedRenderPhases<Transparent3d>>,
    cameras_3d: Extract<'_, '_, Query<'_, '_, (Entity, &Camera, Has<NoIndirectDrawing>), With<Camera3d>>>,
    live_entities: Local<'_, HashSet<RetainedViewEntity>>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
)
```