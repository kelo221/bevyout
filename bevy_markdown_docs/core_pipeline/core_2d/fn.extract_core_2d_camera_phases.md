[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_2d](index.html)

# Function extract\_core\_2d\_camera\_phases 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#393-399)

```rust
pub fn extract_core_2d_camera_phases(
    transparent_2d_phases: ResMut<'_, ViewSortedRenderPhases<Transparent2d>>,
    opaque_2d_phases: ResMut<'_, ViewBinnedRenderPhases<Opaque2d>>,
    alpha_mask_2d_phases: ResMut<'_, ViewBinnedRenderPhases<AlphaMask2d>>,
    cameras_2d: Extract<'_, '_, Query<'_, '_, (Entity, &Camera), With<Camera2d>>>,
    live_entities: Local<'_, HashSet<RetainedViewEntity>>,
)
```