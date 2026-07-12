[bevy](../../index.html)::[input](../index.html)::[gamepad](index.html)

# Function gamepad\_connection\_system 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1506-1509)

```rust
pub fn gamepad_connection_system(
    commands: Commands<'_, '_>,
    connection_events: MessageReader<'_, '_, GamepadConnectionEvent>,
)
```

Available on **crate feature `gamepad`** only.

Handles [`GamepadConnectionEvent`](struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")s events.

On connection, adds the components representing a [`Gamepad`](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad") to the entity. On disconnection, removes the [`Gamepad`](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad") and other related components. Entities are left alive and might leave components like [`GamepadSettings`](../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings") to preserve state in the case of a reconnection.

### Note

Whenever a [`Gamepad`](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad") connects or disconnects, an information gets printed to the console using the [`info!`](https://docs.rs/log/0.4.32/x86_64-unknown-linux-gnu/log/macro.info.html "macro log::info") macro.