[bevy](../../index.html)::[input](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#47)

The input prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Axis](struct.Axis.html "struct bevy::input::prelude::Axis")

Stores the position data of the input devices of type `T`.

[ButtonInput](struct.ButtonInput.html "struct bevy::input::prelude::ButtonInput")

A “press-able” input of type `T`.

[Gamepad](struct.Gamepad.html "struct bevy::input::prelude::Gamepad")

Stores a connected gamepad’s metadata such as the name and its [`GamepadButton`](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton") and [`GamepadAxis`](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis").

[GamepadSettings](struct.GamepadSettings.html "struct bevy::input::prelude::GamepadSettings")

Gamepad settings component.

[TouchInput](struct.TouchInput.html "struct bevy::input::prelude::TouchInput")

A touch input event.

[Touches](struct.Touches.html "struct bevy::input::prelude::Touches")

A collection of [`Touch`](../touch/struct.Touch.html "struct bevy::input::touch::Touch")es.

## Enums

[GamepadAxis](enum.GamepadAxis.html "enum bevy::input::prelude::GamepadAxis")

Represents gamepad input types that are mapped in the range \[-1.0, 1.0\].

[GamepadButton](enum.GamepadButton.html "enum bevy::input::prelude::GamepadButton")

Represents gamepad input types that are mapped in the range \[0.0, 1.0\].

[KeyCode](enum.KeyCode.html "enum bevy::input::prelude::KeyCode")

The key code of a [`KeyboardInput`](../keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput").

[MouseButton](enum.MouseButton.html "enum bevy::input::prelude::MouseButton")

A button on a mouse device.