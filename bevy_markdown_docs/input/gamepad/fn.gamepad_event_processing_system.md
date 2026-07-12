[bevy](../../index.html)::[input](../index.html)::[gamepad](index.html)

# Function gamepad\_event\_processing\_system 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1584-1591)

```rust
pub fn gamepad_event_processing_system(
    gamepads: Query<'_, '_, (&mut Gamepad, &GamepadSettings)>,
    raw_events: MessageReader<'_, '_, RawGamepadEvent>,
    processed_events: MessageWriter<'_, GamepadEvent>,
    processed_axis_events: MessageWriter<'_, GamepadAxisChangedEvent>,
    processed_digital_events: MessageWriter<'_, GamepadButtonStateChangedEvent>,
    processed_analog_events: MessageWriter<'_, GamepadButtonChangedEvent>,
)
```

Available on **crate feature `gamepad`** only.

Consumes [`RawGamepadEvent`](enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent") events, filters them using their [`GamepadSettings`](../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings") and if successful, updates the [`Gamepad`](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad") and sends [`GamepadAxisChangedEvent`](struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent"), [`GamepadButtonStateChangedEvent`](struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent"), [`GamepadButtonChangedEvent`](struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent") events.