[bevy](../index.html)

# Crate input 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#1-192)

Input functionality for the [Bevy game engine](https://bevy.org/).

## Supported input devices

`bevy` currently supports keyboard, mouse, gamepad, and touch inputs.

## Modules

[common\_conditions](common_conditions/index.html "mod bevy::input::common_conditions")

Common run conditions

[gamepad](gamepad/index.html "mod bevy::input::gamepad")`gamepad`

The gamepad input functionality.

[gestures](gestures/index.html "mod bevy::input::gestures")`gestures`

Gestures functionality, from touchscreens and touchpads.

[keyboard](keyboard/index.html "mod bevy::input::keyboard")`keyboard`

The keyboard input functionality.

[mouse](mouse/index.html "mod bevy::input::mouse")`mouse`

The mouse input functionality.

[prelude](prelude/index.html "mod bevy::input::prelude")

The input prelude.

[touch](touch/index.html "mod bevy::input::touch")`mouse` or `touch`

The touch input functionality.

## Structs

[Axis](struct.Axis.html "struct bevy::input::Axis")

Stores the position data of the input devices of type `T`.

[ButtonInput](struct.ButtonInput.html "struct bevy::input::ButtonInput")

A “press-able” input of type `T`.

[InputPlugin](struct.InputPlugin.html "struct bevy::input::InputPlugin")

Adds input from various sources to an App

[InputSystems](struct.InputSystems.html "struct bevy::input::InputSystems")

Label for systems that update the input data.

## Enums

[ButtonState](enum.ButtonState.html "enum bevy::input::ButtonState")

The current “press” state of an element