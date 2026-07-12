[bevy](../../index.html)::[post\_process](../index.html)

# Module msaa\_writeback 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/lib.rs.html#14)

## Structs

[MsaaWritebackBlitPipeline](struct.MsaaWritebackBlitPipeline.html "struct bevy::post_process::msaa_writeback::MsaaWritebackBlitPipeline")

[MsaaWritebackPlugin](struct.MsaaWritebackPlugin.html "struct bevy::post_process::msaa_writeback::MsaaWritebackPlugin")

This enables “msaa writeback” support for the `core_2d` and `core_3d` pipelines, which can be enabled on cameras using [`bevy_camera::Camera::msaa_writeback`](../../prelude/struct.Camera.html#structfield.msaa_writeback "field bevy::prelude::Camera::msaa_writeback"). See the docs on that field for more information.