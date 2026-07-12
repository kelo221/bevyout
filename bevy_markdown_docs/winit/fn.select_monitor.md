[bevy](../index.html)::[winit](index.html)

# Function select\_monitor 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/winit_windows.rs.html#506-511)

```rust
pub fn select_monitor(
    monitors: &WinitMonitors,
    primary_monitor: Option<MonitorHandle>,
    current_monitor: Option<MonitorHandle>,
    monitor_selection: &MonitorSelection,
) -> Option<MonitorHandle>
```

Selects a monitor based on the given [`MonitorSelection`](../prelude/enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection").