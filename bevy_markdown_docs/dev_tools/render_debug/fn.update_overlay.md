[bevy](../../index.html)::[dev\_tools](../index.html)::[render\_debug](index.html)

# Function update\_overlay 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#116-133)

```rust
pub fn update_overlay(
    commands: Commands<'_, '_>,
    events: MessageReader<'_, '_, RenderDebugOverlayEvent>,
    config_res: ResMut<'_, GlobalRenderDebugOverlay>,
    cameras: Query<'_, '_, (Entity, Option<&RenderDebugOverlay>, Has<DepthPrepass>, Has<NormalPrepass>, Has<MotionVectorPrepass>, Has<DeferredPrepass>, Has<OcclusionCulling>, Has<ScreenSpaceReflections>), With<Camera>>,
)
```

Listen to messages to update the debug overlay configuration.