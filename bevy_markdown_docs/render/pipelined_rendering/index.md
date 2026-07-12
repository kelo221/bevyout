[bevy](../../index.html)::[render](../index.html)

# Module pipelined\_rendering 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#55)

Available on **non-WebAssembly** only.

## Structs

[PipelinedRenderingPlugin](struct.PipelinedRenderingPlugin.html "struct bevy::render::pipelined_rendering::PipelinedRenderingPlugin")

The [`PipelinedRenderingPlugin`](struct.PipelinedRenderingPlugin.html "struct bevy::render::pipelined_rendering::PipelinedRenderingPlugin") can be added to your application to enable pipelined rendering.

[RenderAppChannels](struct.RenderAppChannels.html "struct bevy::render::pipelined_rendering::RenderAppChannels")

Channels used by the main app to send and receive the render app.

[RenderExtractApp](struct.RenderExtractApp.html "struct bevy::render::pipelined_rendering::RenderExtractApp")

A Label for the sub app that runs the parts of pipelined rendering that need to run on the main thread.