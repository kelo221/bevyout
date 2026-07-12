[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Function camera\_system 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#351-361)

```rust
pub fn camera_system(
    window_resized_reader: MessageReader<'_, '_, WindowResized>,
    window_created_reader: MessageReader<'_, '_, WindowCreated>,
    window_scale_factor_changed_reader: MessageReader<'_, '_, WindowScaleFactorChanged>,
    image_asset_event_reader: MessageReader<'_, '_, AssetEvent<Image>>,
    primary_window: Query<'_, '_, Entity, With<PrimaryWindow>>,
    windows: Query<'_, '_, (Entity, &Window)>,
    images: Res<'_, Assets<Image>>,
    manual_texture_views: Res<'_, ManualTextureViews>,
    cameras: Query<'_, '_, (&mut Camera, &RenderTarget, &mut Projection)>,
) -> Result<(), BevyError>
```

System in charge of updating a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") when its window or projection changes.

The system detects window creation, resize, and scale factor change events to update the camera [`Projection`](../../prelude/enum.Projection.html "enum bevy::prelude::Projection") if needed.

### World Resources

[`Res<Assets<Image>>`](../../prelude/struct.Assets.html "struct bevy::prelude::Assets") – For cameras that render to an image, this resource is used to inspect information about the render target. This system will not access any other image assets.