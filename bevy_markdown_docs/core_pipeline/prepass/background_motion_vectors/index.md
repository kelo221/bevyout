[bevy](../../../index.html)::[core\_pipeline](../../index.html)::[prepass](../index.html)

# Module background\_motion\_vectors 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#28)

Default background motion vector prepass.

When a camera has [`MotionVectorPrepass`](../struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass") but no [`NoBackgroundMotionVectors`](../struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors"), this module writes motion vectors for background pixels (depth == 0 in reversed-Z) based on camera rotation, so that effects like TAA and motion blur work correctly on the background.

This is a general solution that works for any background: skyboxes, atmospheric sky, solid color backgrounds, etc.

## Structs

[BackgroundMotionVectorsBindGroup](struct.BackgroundMotionVectorsBindGroup.html "struct bevy::core_pipeline::prepass::background_motion_vectors::BackgroundMotionVectorsBindGroup")

Stores the background motion vectors bind group on the camera entity. Used by the prepass node.

[BackgroundMotionVectorsPipelineId](struct.BackgroundMotionVectorsPipelineId.html "struct bevy::core_pipeline::prepass::background_motion_vectors::BackgroundMotionVectorsPipelineId")

Stores the background motion vectors pipeline ID on the camera entity. Used by the prepass node.

[BackgroundMotionVectorsPlugin](struct.BackgroundMotionVectorsPlugin.html "struct bevy::core_pipeline::prepass::background_motion_vectors::BackgroundMotionVectorsPlugin")

Plugin that writes camera-rotation motion vectors for background pixels on cameras with [`MotionVectorPrepass`](../struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass").

[NoBackgroundMotionVectors](struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::background_motion_vectors::NoBackgroundMotionVectors")

When added to a camera with [`MotionVectorPrepass`](../struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass"), disables the automatic background motion vector prepass.