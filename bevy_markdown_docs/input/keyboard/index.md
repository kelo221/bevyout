[bevy](../../index.html)::[input](../index.html)

# Module keyboard 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#32)

Available on **crate feature `keyboard`** only.

The keyboard input functionality.

## Structs

[KeyboardFocusLost](struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost")

Gets generated from `bevy_winit::winit_runner`

[KeyboardInput](struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

A keyboard input event.

## Enums

[Key](enum.Key.html "enum bevy::input::keyboard::Key")

The logical key code of a [`KeyboardInput`](struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput").

[KeyCode](enum.KeyCode.html "enum bevy::input::keyboard::KeyCode")

The key code of a [`KeyboardInput`](struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput").

[NativeKey](enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")

Contains the platform-native logical key identifier, known as keysym.

[NativeKeyCode](enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")

Contains the platform-native physical key identifier

## Functions

[keyboard\_input\_system](fn.keyboard_input_system.html "fn bevy::input::keyboard::keyboard_input_system")

Updates the [`ButtonInput<KeyCode>`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") and [`ButtonInput<Key>`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") resources with the latest [`KeyboardInput`](struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput") events.