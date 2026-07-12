[bevy](../../index.html)::[input](../index.html)::[keyboard](index.html)

# Function keyboard\_input\_system 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#170-175)

```rust
pub fn keyboard_input_system(
    keycode_input: ResMut<'_, ButtonInput<KeyCode>>,
    key_input: ResMut<'_, ButtonInput<Key>>,
    keyboard_input_reader: MessageReader<'_, '_, KeyboardInput>,
    keyboard_focus_lost_reader: MessageReader<'_, '_, KeyboardFocusLost>,
)
```

Available on **crate feature `keyboard`** only.

Updates the [`ButtonInput<KeyCode>`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") and [`ButtonInput<Key>`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") resources with the latest [`KeyboardInput`](struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput") events.

### Differences

The main difference between the [`KeyboardInput`](struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput") event and the [`ButtonInput`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") resources are that the latter has convenient functions such as [`ButtonInput::pressed`](../../prelude/struct.ButtonInput.html#method.pressed "method bevy::prelude::ButtonInput::pressed"), [`ButtonInput::just_pressed`](../../prelude/struct.ButtonInput.html#method.just_pressed "method bevy::prelude::ButtonInput::just_pressed") and [`ButtonInput::just_released`](../../prelude/struct.ButtonInput.html#method.just_released "method bevy::prelude::ButtonInput::just_released") and is window id agnostic.

There is a [`ButtonInput`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") for both [`KeyCode`](../../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode") and [`Key`](enum.Key.html "enum bevy::input::keyboard::Key") as they are both useful in different situations, see their documentation for the details.