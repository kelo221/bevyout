[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Function extract\_cameras 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#473-509)

```rust
pub fn extract_cameras(
    commands: Commands<'_, '_>,
    main_pass_formats: ResMut<'_, CameraMainPassTextureFormats>,
    query: Extract<'_, '_, Query<'_, '_, (Entity, RenderEntity, &Camera, &RenderTarget, &CameraRenderGraph, &GlobalTransform, &VisibleEntities, &Frustum, (Has<Hdr>, Option<&CompositingSpace>, Option<&ColorGrading>, Option<&Exposure>, Option<&TemporalJitter>, Option<&MipBias>, Option<&RenderLayers>, Option<&Projection>, Has<NoIndirectDrawing>))>>,
    primary_window: Extract<'_, '_, Query<'_, '_, Entity, With<PrimaryWindow>>>,
    extracted_windows: Res<'_, ExtractedWindows>,
    manual_texture_views: Res<'_, ManualTextureViews>,
    images: Res<'_, RenderAssets<GpuImage>>,
    existing_render_visible_entities_cpu_culling: Query<'_, '_, &mut RenderExtractedVisibleEntities, With<RenderVisibleEntities>>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
    visibility_extraction_system_param: VisibilityExtractionSystemParam<'_, '_>,
)
```