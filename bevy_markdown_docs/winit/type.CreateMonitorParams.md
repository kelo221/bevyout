[bevy](../index.html)::[winit](index.html)

# Type Alias CreateMonitorParams 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#248)

```rust
pub type CreateMonitorParams<'w, 's> = (Commands<'w, 's>, ResMut<'w, WinitMonitors>);
```

The parameters of the [`create_monitors`](fn.create_monitors.html "fn bevy::winit::create_monitors") system.