[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[mip\_generation](../../index.html)::[experimental](../index.html)::[depth](index.html)

# Function prepare\_view\_depth\_pyramids 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#694-701)

```rust
pub fn prepare_view_depth_pyramids(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    texture_cache: ResMut<'_, TextureCache>,
    depth_pyramid_dummy_texture: Res<'_, DepthPyramidDummyTexture>,
    views: Query<'_, '_, (Entity, &ExtractedView), (With<OcclusionCulling>, Without<NoIndirectDrawing>)>,
    stale_views: Query<'_, '_, Entity, (With<ViewDepthPyramid>, Without<OcclusionCulling>)>,
)
```

Creates depth pyramids for views that have occlusion culling enabled.