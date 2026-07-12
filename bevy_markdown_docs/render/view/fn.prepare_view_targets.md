[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function prepare\_view\_targets 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#1199-1213)

```rust
pub fn prepare_view_targets(
    commands: Commands<'_, '_>,
    clear_color_global: Res<'_, ClearColor>,
    render_device: Res<'_, RenderDevice>,
    texture_cache: ResMut<'_, TextureCache>,
    cameras: Query<'_, '_, (Entity, &ExtractedCamera, &ExtractedView, &CameraMainTextureUsages, &Msaa)>,
    view_target_attachments: Res<'_, ViewTargetAttachments>,
    main_texture_atomics: Local<'_, HashMap<(Option<NormalizedRenderTarget>, TextureUsages, TextureFormat, Msaa), Weak<Atomic<usize>>>>,
)
```