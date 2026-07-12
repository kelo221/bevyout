[bevy](../../index.html)::[winit](../index.html)

# Module accessibility 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#44)

Helpers for mapping window entities to accessibility types

## Structs

[AccessKitAdapters](struct.AccessKitAdapters.html "struct bevy::winit::accessibility::AccessKitAdapters")

Maps window entities to their `AccessKit` [`Adapter`](https://docs.rs/accesskit_winit/0.32.2/x86_64-unknown-linux-gnu/accesskit_winit/struct.Adapter.html "struct accesskit_winit::Adapter")s.

[AccessKitPlugin](struct.AccessKitPlugin.html "struct bevy::winit::accessibility::AccessKitPlugin")

Implements winit-specific `AccessKit` functionality.

[WinitActionRequestHandler](struct.WinitActionRequestHandler.html "struct bevy::winit::accessibility::WinitActionRequestHandler")

Forwards `AccessKit` [`ActionRequest`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.ActionRequest.html "struct accesskit::ActionRequest")s from winit to an event channel.

[WinitActionRequestHandlers](struct.WinitActionRequestHandlers.html "struct bevy::winit::accessibility::WinitActionRequestHandlers")

Maps window entities to their respective [`ActionRequest`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.ActionRequest.html "struct accesskit::ActionRequest")s.

## Constants

[ACCESS\_KIT\_ADAPTERS](constant.ACCESS_KIT_ADAPTERS.html "constant bevy::winit::accessibility::ACCESS_KIT_ADAPTERS")

Temporary storage of access kit adapter data to replace usage of `!Send` resources. This will be replaced with proper storage of `!Send` data after issue #17667 is complete.