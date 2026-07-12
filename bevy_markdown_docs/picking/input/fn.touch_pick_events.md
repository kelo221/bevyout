[bevy](../../index.html)::[picking](../index.html)::[input](index.html)

# Function touch\_pick\_events 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#204-213)

```rust
pub fn touch_pick_events(
    window_events: MessageReader<'_, '_, WindowEvent>,
    primary_window: Query<'_, '_, Entity, With<PrimaryWindow>>,
    touch_cache: Local<'_, HashMap<u64, TouchInput>>,
    commands: Commands<'_, '_>,
    pointer_inputs: MessageWriter<'_, PointerInput>,
)
```

Sends touch pointer events to be consumed by the core plugin