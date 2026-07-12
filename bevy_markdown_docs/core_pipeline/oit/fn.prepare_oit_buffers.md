[bevy](../../index.html)::[core\_pipeline](../index.html)::[oit](index.html)

# Function prepare\_oit\_buffers 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#222-238)

```rust
pub fn prepare_oit_buffers(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    cameras: Query<'_, '_, (&ExtractedCamera, &OrderIndependentTransparencySettings), (Changed<ExtractedCamera>, Changed<OrderIndependentTransparencySettings>)>,
    camera_oit_uniforms: Query<'_, '_, (Entity, &OrderIndependentTransparencySettings), With<ExtractedCamera>>,
    buffers: ResMut<'_, OitBuffers>,
)
```

This creates or resizes the oit buffers for each camera. It will always create one big buffer that’s as big as the biggest buffer needed. Cameras with smaller viewports or less layers will simply use the big buffer and ignore the rest.