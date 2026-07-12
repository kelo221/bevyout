[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module prepass 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#16)

Run a prepass before the main pass to generate depth, normals, and/or motion vectors textures, sometimes called a thin g-buffer. These textures are useful for various screen-space effects and reducing overdraw in the main pass.

The prepass only runs for opaque meshes or meshes with an alpha mask. Transparent meshes are ignored.

To enable the prepass, you need to add a prepass component to a [`bevy_camera::Camera3d`](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d").

[`DepthPrepass`](struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass") [`NormalPrepass`](struct.NormalPrepass.html "struct bevy::core_pipeline::prepass::NormalPrepass") [`MotionVectorPrepass`](struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")

The textures are automatically added to the default mesh view bindings. You can also get the raw textures by querying the [`ViewPrepassTextures`](struct.ViewPrepassTextures.html "struct bevy::core_pipeline::prepass::ViewPrepassTextures") component on any camera with a prepass component.

The depth prepass will always run and generate the depth buffer as a side effect, but it won’t copy it to a separate texture unless the [`DepthPrepass`](struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass") is activated. This means that if any prepass component is present it will always create a depth buffer that will be used by the main pass.

When using the default mesh view bindings you should be able to use `prepass_depth()`, `prepass_normal()`, and `prepass_motion_vector()` to load the related textures. These functions are defined in `bevy_pbr::prepass_utils`. See the `shader_prepass` example that shows how to use them.

The prepass runs for each `Material`. You can control if the prepass should run per-material by setting the `prepass_enabled` flag on the `MaterialPlugin`.

Currently only works for 3D.

## Modules

[background\_motion\_vectors](background_motion_vectors/index.html "mod bevy::core_pipeline::prepass::background_motion_vectors")

Default background motion vector prepass.

[node](node/index.html "mod bevy::core_pipeline::prepass::node")

## Structs

[AlphaMask3dPrepass](struct.AlphaMask3dPrepass.html "struct bevy::core_pipeline::prepass::AlphaMask3dPrepass")

Alpha mask phase of the 3D prepass.

[BackgroundMotionVectorsBindGroup](struct.BackgroundMotionVectorsBindGroup.html "struct bevy::core_pipeline::prepass::BackgroundMotionVectorsBindGroup")

Stores the background motion vectors bind group on the camera entity. Used by the prepass node.

[BackgroundMotionVectorsPipelineId](struct.BackgroundMotionVectorsPipelineId.html "struct bevy::core_pipeline::prepass::BackgroundMotionVectorsPipelineId")

Stores the background motion vectors pipeline ID on the camera entity. Used by the prepass node.

[BackgroundMotionVectorsPlugin](struct.BackgroundMotionVectorsPlugin.html "struct bevy::core_pipeline::prepass::BackgroundMotionVectorsPlugin")

Plugin that writes camera-rotation motion vectors for background pixels on cameras with [`MotionVectorPrepass`](struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass").

[DeferredPrepass](struct.DeferredPrepass.html "struct bevy::core_pipeline::prepass::DeferredPrepass")

If added to a [`bevy_camera::Camera3d`](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d") then deferred materials will be rendered to the deferred gbuffer texture and will be available to subsequent passes. Note the default deferred lighting plugin also requires `DepthPrepass` to work correctly.

[DeferredPrepassDoubleBuffer](struct.DeferredPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer")

Allows querying the previous frame’s [`DeferredPrepass`](struct.DeferredPrepass.html "struct bevy::core_pipeline::prepass::DeferredPrepass").

[DepthPrepass](struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass")

If added to a [`bevy_camera::Camera3d`](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d") then depth values will be copied to a separate texture available to the main pass.

[DepthPrepassDoubleBuffer](struct.DepthPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer")

Allows querying the previous frame’s [`DepthPrepass`](struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass").

[MotionVectorPrepass](struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")

If added to a [`bevy_camera::Camera3d`](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d") then screen space motion vectors will be copied to a separate texture available to the main pass.

[NoBackgroundMotionVectors](struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

When added to a camera with [`MotionVectorPrepass`](struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass"), disables the automatic background motion vector prepass.

[NormalPrepass](struct.NormalPrepass.html "struct bevy::core_pipeline::prepass::NormalPrepass")

If added to a [`bevy_camera::Camera3d`](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d") then vertex world normals will be copied to a separate texture available to the main pass. Normals will have normal map textures already applied.

[Opaque3dPrepass](struct.Opaque3dPrepass.html "struct bevy::core_pipeline::prepass::Opaque3dPrepass")

Opaque phase of the 3D prepass.

[OpaqueNoLightmap3dBatchSetKey](struct.OpaqueNoLightmap3dBatchSetKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBatchSetKey")

Information that must be identical in order to place opaque meshes in the same _batch set_ in the prepass and deferred pass.

[OpaqueNoLightmap3dBinKey](struct.OpaqueNoLightmap3dBinKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBinKey")

The data used to bin each opaque 3D object in the prepass and deferred pass.

[PreviousViewData](struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData")

View matrices from the previous frame.

[PreviousViewUniformOffset](struct.PreviousViewUniformOffset.html "struct bevy::core_pipeline::prepass::PreviousViewUniformOffset")

[PreviousViewUniforms](struct.PreviousViewUniforms.html "struct bevy::core_pipeline::prepass::PreviousViewUniforms")

[ViewPrepassTextures](struct.ViewPrepassTextures.html "struct bevy::core_pipeline::prepass::ViewPrepassTextures")

Textures that are written to by the prepass.

## Constants

[MOTION\_VECTOR\_PREPASS\_FORMAT](constant.MOTION_VECTOR_PREPASS_FORMAT.html "constant bevy::core_pipeline::prepass::MOTION_VECTOR_PREPASS_FORMAT")

[NORMAL\_PREPASS\_FORMAT](constant.NORMAL_PREPASS_FORMAT.html "constant bevy::core_pipeline::prepass::NORMAL_PREPASS_FORMAT")

## Functions

[prepass\_target\_descriptors](fn.prepass_target_descriptors.html "fn bevy::core_pipeline::prepass::prepass_target_descriptors")