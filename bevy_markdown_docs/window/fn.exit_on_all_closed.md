[bevy](../index.html)::[window](index.html)

# Function exit\_on\_all\_closed 

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/system.rs.html#18-21)

```rust
pub fn exit_on_all_closed(
    app_exit_writer: MessageWriter<'_, AppExit>,
    windows: Query<'_, '_, (), With<Window>>,
)
```

Exit the application when there are no open windows.

This system is added by the [`WindowPlugin`](../prelude/struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin") in the default configuration. To disable this behavior, set `close_when_requested` (on the [`WindowPlugin`](../prelude/struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin")) to `false`. Ensure that you read the caveats documented on that field if doing so.