[bevy](../../index.html)::[render](../index.html)

# Module error\_handler 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#43)

## Structs

[RenderError](struct.RenderError.html "struct bevy::render::error_handler::RenderError")

An error encountered during rendering. These are errors reported by wgpu validation layers, and typically indicate problems in the way it is being used.

[RenderErrorHandler](struct.RenderErrorHandler.html "struct bevy::render::error_handler::RenderErrorHandler")

Determines what [`RenderErrorPolicy`](enum.RenderErrorPolicy.html "enum bevy::render::error_handler::RenderErrorPolicy") should be used to respond to a given [`RenderError`](struct.RenderError.html "struct bevy::render::error_handler::RenderError").

## Enums

[ErrorType](enum.ErrorType.html "enum bevy::render::error_handler::ErrorType")

A classification of WebGPU error for implementers of the WebGPU API to use in their own error layer(s).

[RenderErrorPolicy](enum.RenderErrorPolicy.html "enum bevy::render::error_handler::RenderErrorPolicy")

Resource to indicate renderer behavior upon error.