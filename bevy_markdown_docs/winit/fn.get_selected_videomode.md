[bevy](../index.html)::[winit](index.html)

# Function get\_selected\_videomode 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/winit_windows.rs.html#372-375)

```rust
pub fn get_selected_videomode(
    monitor: &MonitorHandle,
    selection: &VideoModeSelection,
) -> Option<VideoModeHandle>
```

Returns some [`winit::monitor::VideoModeHandle`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/monitor/struct.VideoModeHandle.html "struct winit::monitor::VideoModeHandle") given a [`MonitorHandle`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/monitor/struct.MonitorHandle.html "struct winit::monitor::MonitorHandle") and a [`VideoModeSelection`](../prelude/enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection") or None if no valid matching video mode was found.