[bevy](../../index.html)::[picking](../index.html)

# Module pointer 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#165)

Types and systems for pointer inputs, such as position and buttons.

The picking system is built around the concept of a ‘Pointer’, which is an abstract representation of a user input with a specific screen location. The cursor and touch input is provided under [`input`](../input/index.html "mod bevy::picking::input"), but you can also implement your own custom pointers by supplying a unique ID.

The purpose of this module is primarily to provide a common interface that can be driven by lower-level input devices and consumed by higher-level interaction systems.

## Structs

[Location](struct.Location.html "struct bevy::picking::pointer::Location")

The location of a pointer, including the current [`NormalizedRenderTarget`](../../camera/enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget"), and the x/y position of the pointer on this render target.

[PointerInput](struct.PointerInput.html "struct bevy::picking::pointer::PointerInput")

An input event effecting a pointer.

[PointerInteraction](struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction")

Holds a list of entities this pointer is currently interacting with, sorted from nearest to farthest.

[PointerLocation](struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation")

Component that tracks a pointer’s current [`Location`](struct.Location.html "struct bevy::picking::pointer::Location").

[PointerMap](struct.PointerMap.html "struct bevy::picking::pointer::PointerMap")

A resource that maps each [`PointerId`](enum.PointerId.html "enum bevy::picking::pointer::PointerId") to their [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") for easy lookups.

[PointerPress](struct.PointerPress.html "struct bevy::picking::pointer::PointerPress")

Tracks the state of the pointer’s buttons in response to [`PointerInput`](struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") events.

## Enums

[PointerAction](enum.PointerAction.html "enum bevy::picking::pointer::PointerAction")

Event sent to drive a pointer.

[PointerButton](enum.PointerButton.html "enum bevy::picking::pointer::PointerButton")

The button that was just pressed or released

[PointerId](enum.PointerId.html "enum bevy::picking::pointer::PointerId")

Identifies a unique pointer entity. `Mouse` and `Touch` pointers are automatically spawned.

[PressDirection](enum.PressDirection.html "enum bevy::picking::pointer::PressDirection")

The stage of the pointer button press event

## Functions

[update\_pointer\_map](fn.update_pointer_map.html "fn bevy::picking::pointer::update_pointer_map")

Update the [`PointerMap`](struct.PointerMap.html "struct bevy::picking::pointer::PointerMap") resource with the current frame’s data.