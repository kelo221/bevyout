[bevy](../../index.html)::[input](../index.html)

# Module touch 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#39)

Available on **crate features `mouse` or `touch`** only.

The touch input functionality.

## Structs

[Touch](struct.Touch.html "struct bevy::input::touch::Touch")

A touch input.

[TouchInput](struct.TouchInput.html "struct bevy::input::touch::TouchInput")

A touch input event.

[Touches](struct.Touches.html "struct bevy::input::touch::Touches")

A collection of [`Touch`](struct.Touch.html "struct bevy::input::touch::Touch")es.

## Enums

[ForceTouch](enum.ForceTouch.html "enum bevy::input::touch::ForceTouch")

A force description of a [`Touch`](struct.Touch.html "struct bevy::input::touch::Touch") input.

[TouchPhase](enum.TouchPhase.html "enum bevy::input::touch::TouchPhase")

A phase of a [`TouchInput`](../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput").

## Functions

[touch\_screen\_input\_system](fn.touch_screen_input_system.html "fn bevy::input::touch::touch_screen_input_system")

Updates the [`Touches`](../../prelude/struct.Touches.html "struct bevy::prelude::Touches") resource with the latest [`TouchInput`](../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput") events.