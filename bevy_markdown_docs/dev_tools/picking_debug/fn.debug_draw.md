[bevy](../../index.html)::[dev\_tools](../index.html)::[picking\_debug](index.html)

# Function debug\_draw 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#244-250)

```rust
pub fn debug_draw(
    commands: Commands<'_, '_>,
    camera_query: Query<'_, '_, (Entity, &Camera, &RenderTarget)>,
    primary_window: Query<'_, '_, Entity, With<PrimaryWindow>>,
    pointers: Query<'_, '_, (Entity, &PointerId, &PointerDebug)>,
    scale: Res<'_, UiScale>,
)
```

Draw text on each cursor with debug info