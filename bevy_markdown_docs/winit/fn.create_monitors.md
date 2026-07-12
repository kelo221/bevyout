[bevy](../index.html)::[winit](index.html)

# Function create\_monitors 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/system.rs.html#177-180)

```rust
pub fn create_monitors(
    event_loop: &ActiveEventLoop,
    _: <(Commands<'_, '_>, ResMut<'_, WinitMonitors>) as SystemParam>::Item<'_, '_>,
)
```

Synchronize available monitors as reported by [`winit`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/index.html "mod winit") with [`Monitor`](../window/struct.Monitor.html "struct bevy::window::Monitor") entities in the world.