[bevy](../../index.html)::[dev\_tools](../index.html)

# Module picking\_debug 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/lib.rs.html#21)

Text and on-screen debugging tools

## Structs

[DebugPickingPlugin](struct.DebugPickingPlugin.html "struct bevy::dev_tools::picking_debug::DebugPickingPlugin")

Logs events for debugging

[PointerDebug](struct.PointerDebug.html "struct bevy::dev_tools::picking_debug::PointerDebug")

Storage for per-pointer debug information.

## Enums

[DebugPickingMode](enum.DebugPickingMode.html "enum bevy::dev_tools::picking_debug::DebugPickingMode")

This resource determines the runtime behavior of the debug plugin.

## Functions

[add\_pointer\_debug](fn.add_pointer_debug.html "fn bevy::dev_tools::picking_debug::add_pointer_debug")

Adds [`PointerDebug`](struct.PointerDebug.html "struct bevy::dev_tools::picking_debug::PointerDebug") to pointers automatically.

[debug\_draw](fn.debug_draw.html "fn bevy::dev_tools::picking_debug::debug_draw")

Draw text on each cursor with debug info

[log\_message\_debug](fn.log_message_debug.html "fn bevy::dev_tools::picking_debug::log_message_debug")

Listen for any message and logs it at the debug level

[log\_pointer\_event\_trace](fn.log_pointer_event_trace.html "fn bevy::dev_tools::picking_debug::log_pointer_event_trace")

Listens for pointer events of type `E` and logs them at “trace” level

[log\_pointer\_message\_debug](fn.log_pointer_message_debug.html "fn bevy::dev_tools::picking_debug::log_pointer_message_debug")

Listens for pointer events of type `E` and logs them at “debug” level

[pointer\_debug\_visibility](fn.pointer_debug_visibility.html "fn bevy::dev_tools::picking_debug::pointer_debug_visibility")

Hide text from pointers.

[update\_debug\_data](fn.update_debug_data.html "fn bevy::dev_tools::picking_debug::update_debug_data")

Update typed debug data used to draw overlays