[bevy](../../index.html)::[input](../index.html)

# Module mouse 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#35)

Available on **crate feature `mouse`** only.

The mouse input functionality.

## Structs

[AccumulatedMouseMotion](struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion")

Tracks how much the mouse has moved every frame.

[AccumulatedMouseScroll](struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll")

Tracks how much the mouse has scrolled every frame.

[MouseButtonInput](struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput")

A mouse button input event.

[MouseMotion](struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion")

An event reporting the change in physical position of a pointing device.

[MouseWheel](struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel")

A mouse wheel event.

## Enums

[MouseButton](enum.MouseButton.html "enum bevy::input::mouse::MouseButton")

A button on a mouse device.

[MouseScrollUnit](enum.MouseScrollUnit.html "enum bevy::input::mouse::MouseScrollUnit")

The scroll unit.

## Functions

[accumulate\_mouse\_motion\_system](fn.accumulate_mouse_motion_system.html "fn bevy::input::mouse::accumulate_mouse_motion_system")

Updates the [`AccumulatedMouseMotion`](struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion") resource using the [`MouseMotion`](struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion") event. The value of [`AccumulatedMouseMotion`](struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion") is reset to zero every frame

[accumulate\_mouse\_scroll\_system](fn.accumulate_mouse_scroll_system.html "fn bevy::input::mouse::accumulate_mouse_scroll_system")

Updates the [`AccumulatedMouseScroll`](struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll") resource using the [`MouseWheel`](struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel") event. The value of [`AccumulatedMouseScroll`](struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll") is reset to zero every frame

[mouse\_button\_input\_system](fn.mouse_button_input_system.html "fn bevy::input::mouse::mouse_button_input_system")

Updates the [`ButtonInput<MouseButton>`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") resource with the latest [`MouseButtonInput`](struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput") events.