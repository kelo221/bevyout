[bevy](../../index.html)::[input](../index.html)::[mouse](index.html)

# Function accumulate\_mouse\_motion\_system 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#259-262)

```rust
pub fn accumulate_mouse_motion_system(
    mouse_motion_event: MessageReader<'_, '_, MouseMotion>,
    accumulated_mouse_motion: ResMut<'_, AccumulatedMouseMotion>,
)
```

Available on **crate feature `mouse`** only.

Updates the [`AccumulatedMouseMotion`](struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion") resource using the [`MouseMotion`](struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion") event. The value of [`AccumulatedMouseMotion`](struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion") is reset to zero every frame