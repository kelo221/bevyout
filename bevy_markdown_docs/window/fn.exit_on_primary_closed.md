[bevy](../index.html)::[window](index.html)

# Function exit\_on\_primary\_closed 

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/system.rs.html#33-36)

```rust
pub fn exit_on_primary_closed(
    app_exit_writer: MessageWriter<'_, AppExit>,
    windows: Query<'_, '_, (), (With<Window>, With<PrimaryWindow>)>,
)
```

Exit the application when the primary window has been closed

This system is added by the [`WindowPlugin`](../prelude/struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin")