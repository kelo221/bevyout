[bevy](../../index.html)::[core\_pipeline](../index.html)::[schedule](index.html)

# Function camera\_driver 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#133)

```rust
pub fn camera_driver(world: &mut World)
```

The default entry point for camera driven rendering added to the root [`bevy_render::renderer::RenderGraph`](../../prelude/struct.RenderGraph.html "struct bevy::prelude::RenderGraph") schedule. This system iterates over all cameras in the world, executing their associated rendering schedules defined by the [`bevy_render::camera::CameraRenderGraph`](../../render/camera/struct.CameraRenderGraph.html "struct bevy::render::camera::CameraRenderGraph") component.

After executing all camera schedules, it submits any pending command buffers to the GPU and clears any swap chains that were not covered by a camera. Users can order any additional operations (e.g. one-off compute passes) before or after this system in the root render graph schedule.