[bevy](../../index.html)::[picking](../index.html)::[input](index.html)

# Function mouse\_pick\_events 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#121-129)

```rust
pub fn mouse_pick_events(
    window_events: MessageReader<'_, '_, WindowEvent>,
    primary_window: Query<'_, '_, Entity, With<PrimaryWindow>>,
    cursor_last: Local<'_, Vec2>,
    pointer_inputs: MessageWriter<'_, PointerInput>,
)
```

Sends mouse pointer events to be processed by the core plugin