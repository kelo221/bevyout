[bevy](../index.html)::[window](index.html)

# Function close\_when\_requested 

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/system.rs.html#50-54)

```rust
pub fn close_when_requested(
    commands: Commands<'_, '_>,
    closed: MessageReader<'_, '_, WindowCloseRequested>,
    closing: Query<'_, '_, Entity, With<ClosingWindow>>,
)
```

Close windows in response to [`WindowCloseRequested`](struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested") (e.g. when the close button is pressed).

This system is added by the [`WindowPlugin`](../prelude/struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin") in the default configuration. To disable this behavior, set `close_when_requested` (on the [`WindowPlugin`](../prelude/struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin")) to `false`. Ensure that you read the caveats documented on that field if doing so.