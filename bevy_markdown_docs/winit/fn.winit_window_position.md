[bevy](../index.html)::[winit](index.html)

# Function winit\_window\_position 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/winit_windows.rs.html#450-456)

```rust
pub fn winit_window_position(
    position: &WindowPosition,
    resolution: &WindowResolution,
    monitors: &WinitMonitors,
    primary_monitor: Option<MonitorHandle>,
    current_monitor: Option<MonitorHandle>,
) -> Option<PhysicalPosition<i32>>
```

Compute the physical window position for a given [`WindowPosition`](../prelude/enum.WindowPosition.html "enum bevy::prelude::WindowPosition").