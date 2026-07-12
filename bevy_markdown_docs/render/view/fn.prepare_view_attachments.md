[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function prepare\_view\_attachments 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#1139-1145)

```rust
pub fn prepare_view_attachments(
    windows: Res<'_, ExtractedWindows>,
    images: Res<'_, RenderAssets<GpuImage>>,
    manual_texture_views: Res<'_, ManualTextureViews>,
    cameras: Query<'_, '_, &ExtractedCamera>,
    view_target_attachments: ResMut<'_, ViewTargetAttachments>,
)
```

Prepares the view target [`OutputColorAttachment`](../texture/struct.OutputColorAttachment.html "struct bevy::render::texture::OutputColorAttachment") for each view in the current frame.