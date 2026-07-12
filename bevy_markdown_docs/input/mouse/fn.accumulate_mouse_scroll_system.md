[bevy](../../index.html)::[input](../index.html)::[mouse](index.html)

# Function accumulate\_mouse\_scroll\_system 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#272-275)

```rust
pub fn accumulate_mouse_scroll_system(
    mouse_scroll_event: MessageReader<'_, '_, MouseWheel>,
    accumulated_mouse_scroll: ResMut<'_, AccumulatedMouseScroll>,
)
```

Available on **crate feature `mouse`** only.

Updates the [`AccumulatedMouseScroll`](struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll") resource using the [`MouseWheel`](struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel") event. The value of [`AccumulatedMouseScroll`](struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll") is reset to zero every frame