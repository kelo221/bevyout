[bevy](../../index.html)::[render](../index.html)

# Module camera 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#40)

## Structs

[CameraMainPassTextureFormats](struct.CameraMainPassTextureFormats.html "struct bevy::render::camera::CameraMainPassTextureFormats")

Main-pass color [`TextureFormat`](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") keyed by camera render entity.

[CameraPlugin](struct.CameraPlugin.html "struct bevy::render::camera::CameraPlugin")

[CameraRenderGraph](struct.CameraRenderGraph.html "struct bevy::render::camera::CameraRenderGraph")

Configures the render schedule to be run for a given [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") entity.

[DirtySpecializations](struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations")

Stores information about all entities that have changed in such a way as to potentially require their pipelines to be re-specialized.

[DirtyWireframeSpecializations](struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations")

Stores information about all entities that have changed in such a way as to potentially require their wireframe pipelines to be re-specialized.

[ExtractedCamera](struct.ExtractedCamera.html "struct bevy::render::camera::ExtractedCamera")

Describes a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") in the render world.

[MipBias](struct.MipBias.html "struct bevy::render::camera::MipBias")

Camera component specifying a mip bias to apply when sampling from material textures.

[PendingQueues](struct.PendingQueues.html "struct bevy::render::camera::PendingQueues")

Holds all entities that couldn’t be specialized and/or queued because their materials or other dependent resources hadn’t loaded yet.

[SortedCamera](struct.SortedCamera.html "struct bevy::render::camera::SortedCamera")

[SortedCameras](struct.SortedCameras.html "struct bevy::render::camera::SortedCameras")

Cameras sorted by their order field. This is updated in the [`sort_cameras`](fn.sort_cameras.html "fn bevy::render::camera::sort_cameras") system.

[TemporalJitter](struct.TemporalJitter.html "struct bevy::render::camera::TemporalJitter")

A subpixel offset to jitter a perspective camera’s frustum by.

[ViewPendingQueues](struct.ViewPendingQueues.html "struct bevy::render::camera::ViewPendingQueues")

Holds all entities that couldn’t be specialized and/or queued because their materials and/or other dependent resources hadn’t loaded yet for a single view.

## Enums

[DirtySpecializationSystems](enum.DirtySpecializationSystems.html "enum bevy::render::camera::DirtySpecializationSystems")

A [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") that contains all systems that mutate the [`DirtySpecializations`](struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations") resource and other resources that wrap that type.

[MissingRenderTargetInfoError](enum.MissingRenderTargetInfoError.html "enum bevy::render::camera::MissingRenderTargetInfoError")

## Traits

[NormalizedRenderTargetExt](trait.NormalizedRenderTargetExt.html "trait bevy::render::camera::NormalizedRenderTargetExt")

## Functions

[camera\_system](fn.camera_system.html "fn bevy::render::camera::camera_system")

System in charge of updating a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") when its window or projection changes.

[clear\_dirty\_specializations](fn.clear_dirty_specializations.html "fn bevy::render::camera::clear_dirty_specializations")

Clears out the [`DirtySpecializations`](struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations") resource in preparation for a new frame.

[clear\_dirty\_wireframe\_specializations](fn.clear_dirty_wireframe_specializations.html "fn bevy::render::camera::clear_dirty_wireframe_specializations")

Clears out the [`DirtyWireframeSpecializations`](struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations") resource in preparation for a new frame.

[expire\_specializations\_for\_views](fn.expire_specializations_for_views.html "fn bevy::render::camera::expire_specializations_for_views")

A system that removes views that don’t exist any longer from [`DirtySpecializations`](struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations").

[expire\_wireframe\_specializations\_for\_views](fn.expire_wireframe_specializations_for_views.html "fn bevy::render::camera::expire_wireframe_specializations_for_views")

A system that removes views that don’t exist any longer from [`DirtyWireframeSpecializations`](struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations").

[extract\_cameras](fn.extract_cameras.html "fn bevy::render::camera::extract_cameras")

[sort\_cameras](fn.sort_cameras.html "fn bevy::render::camera::sort_cameras")