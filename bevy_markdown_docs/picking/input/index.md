[bevy](../../index.html)::[picking](../index.html)

# Module input 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#162)

This module provides unsurprising default inputs to `bevy_picking` through [`PointerInput`](../pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput"). The included systems are responsible for sending mouse and touch inputs to their respective `Pointer`s.

Because this has it’s own plugin, it’s easy to omit it, and provide your own inputs as needed. Because `Pointer`s aren’t coupled to the underlying input hardware, you can easily mock inputs, and allow users full accessibility to map whatever inputs they need to pointer input.

If, for example, you wanted to add support for VR input, all you need to do is spawn a pointer entity with a custom [`PointerId`](../pointer/enum.PointerId.html "enum bevy::picking::pointer::PointerId"), and write a system that updates its position. If you want this to work properly with the existing interaction events, you need to be sure that you also write a [`PointerInput`](../pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") event stream.

## Modules

[prelude](prelude/index.html "mod bevy::picking::input::prelude")

The picking input prelude.

## Structs

[PointerInputPlugin](struct.PointerInputPlugin.html "struct bevy::picking::input::PointerInputPlugin")

Adds mouse and touch inputs for picking pointers to your app. This is a default input plugin, that you can replace with your own plugin as needed.

[PointerInputSettings](struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings")

Settings for enabling and disabling updating mouse and touch inputs for picking

## Functions

[deactivate\_touch\_pointers](fn.deactivate_touch_pointers.html "fn bevy::picking::input::deactivate_touch_pointers")

Deactivates unused touch pointers.

[mouse\_pick\_events](fn.mouse_pick_events.html "fn bevy::picking::input::mouse_pick_events")

Sends mouse pointer events to be processed by the core plugin

[spawn\_mouse\_pointer](fn.spawn_mouse_pointer.html "fn bevy::picking::input::spawn_mouse_pointer")

Spawns the default mouse pointer.

[touch\_pick\_events](fn.touch_pick_events.html "fn bevy::picking::input::touch_pick_events")

Sends touch pointer events to be consumed by the core plugin