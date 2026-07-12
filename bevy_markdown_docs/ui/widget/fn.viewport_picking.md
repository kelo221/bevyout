[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function viewport\_picking 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/viewport.rs.html#69-83)

```rust
pub fn viewport_picking(
    commands: Commands<'_, '_>,
    viewport_query: Query<'_, '_, (Entity, &mut ViewportNode, &PointerId, &mut PointerLocation, &ComputedNode, &UiGlobalTransform)>,
    camera_query: Query<'_, '_, (&Camera, &RenderTarget)>,
    hover_map: Res<'_, HoverMap>,
    pointer_state: Res<'_, PointerState>,
    pointer_inputs: MessageReader<'_, '_, PointerInput>,
)
```

Available on **crate feature `bevy_picking`** only.

Handles viewport picking logic.

Viewport entities that are being hovered or dragged will have all pointer inputs sent to them.