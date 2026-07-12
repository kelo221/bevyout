[bevy](../../index.html)::[gizmos\_render](../index.html)

# Module transform\_gizmo\_render 

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#22)

Available on **crate feature `bevy_pbr`** only.

Mesh-based rendering for the transform gizmo.

Uses [`StandardMaterial`](../../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial") with `unlit: true` and a dedicated overlay camera on a separate [`RenderLayers`](../../camera/visibility/struct.RenderLayers.html "struct bevy::camera::visibility::RenderLayers") to render gizmo meshes always-on-top.

## Structs

[TransformGizmoRenderPlugin](struct.TransformGizmoRenderPlugin.html "struct bevy::gizmos_render::transform_gizmo_render::TransformGizmoRenderPlugin")

Plugin that adds mesh-based rendering for the transform gizmo.