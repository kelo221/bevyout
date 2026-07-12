[bevy](../../index.html)::[dev\_tools](../index.html)

# Module render\_debug 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/lib.rs.html#30)

Renderer debugging overlay

## Structs

[GlobalRenderDebugOverlay](struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

Configure the render debug overlay globally. Can be overwritten by using a [`RenderDebugOverlay`](struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay") component.

[RenderDebugOverlay](struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

Configure the render debug overlay. Overwrites the default [`GlobalRenderDebugOverlay`](struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay") resource.

[RenderDebugOverlayPlugin](struct.RenderDebugOverlayPlugin.html "struct bevy::dev_tools::render_debug::RenderDebugOverlayPlugin")

Adds a rendering debug overlay to visualize various renderer buffers.

## Enums

[RenderDebugMode](enum.RenderDebugMode.html "enum bevy::dev_tools::render_debug::RenderDebugMode")

The kind of renderer data to visualize.

[RenderDebugOverlayEvent](enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent")

Configure the render debug overlay.

## Functions

[handle\_input](fn.handle_input.html "fn bevy::dev_tools::render_debug::handle_input")

Automatically attach keybinds to make render debug overlays available to users without code changes when the feature is enabled.

[update\_overlay](fn.update_overlay.html "fn bevy::dev_tools::render_debug::update_overlay")

Listen to messages to update the debug overlay configuration.