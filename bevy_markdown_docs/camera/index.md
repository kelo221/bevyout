[bevy](../index.html)

# Crate camera 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/lib.rs.html#1-44)

## Modules

[prelude](prelude/index.html "mod bevy::camera::prelude")

The camera prelude.

[primitives](primitives/index.html "mod bevy::camera::primitives")

[visibility](visibility/index.html "mod bevy::camera::visibility")

Components that control the visibility of entities.

## Structs

[Camera](struct.Camera.html "struct bevy::camera::Camera")

The defining [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") for camera entities, storing information about how and what to render through this camera.

[Camera2d](struct.Camera2d.html "struct bevy::camera::Camera2d")

A 2D camera component. Enables the 2D render graph for a [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera").

[Camera3d](struct.Camera3d.html "struct bevy::camera::Camera3d")

A 3D camera component. Enables the main 3D render graph for a [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera").

[Camera3dDepthTextureUsage](struct.Camera3dDepthTextureUsage.html "struct bevy::camera::Camera3dDepthTextureUsage")

[CameraMainTextureUsages](struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

This component lets you control the [`TextureUsages`](../render/render_resource/struct.TextureUsages.html "struct bevy::render::render_resource::TextureUsages") field of the main texture generated for the camera

[CameraPlugin](struct.CameraPlugin.html "struct bevy::camera::CameraPlugin")

[CameraProjectionPlugin](struct.CameraProjectionPlugin.html "struct bevy::camera::CameraProjectionPlugin")

Adds [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera") driver systems for a given projection type.

[CameraUpdateSystems](struct.CameraUpdateSystems.html "struct bevy::camera::CameraUpdateSystems")

Label for `camera_system<T>`, shared across all `T`.

[ClearColor](struct.ClearColor.html "struct bevy::camera::ClearColor")

A [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") that stores the default color that cameras use to clear the screen between frames.

[ComputedCameraValues](struct.ComputedCameraValues.html "struct bevy::camera::ComputedCameraValues")

Holds internally computed [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera") values.

[CustomProjection](struct.CustomProjection.html "struct bevy::camera::CustomProjection")

Holds a dynamic [`CameraProjection`](trait.CameraProjection.html "trait bevy::camera::CameraProjection") trait object. Use [`Projection::custom()`](../prelude/enum.Projection.html#method.custom "associated function bevy::prelude::Projection::custom") to construct a custom projection.

[Exposure](struct.Exposure.html "struct bevy::camera::Exposure")

How much energy a [`Camera3d`](../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d") absorbs from incoming light.

[Hdr](struct.Hdr.html "struct bevy::camera::Hdr")

If this component is added to a camera, the camera will use an intermediate “high dynamic range” render texture. This allows rendering with a wider range of lighting values. However, this does _not_ affect whether the camera will render with hdr display output (which bevy does not support currently) and only affects the intermediate render texture.

[ImageRenderTarget](struct.ImageRenderTarget.html "struct bevy::camera::ImageRenderTarget")

A render target that renders to an [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image").

[MainPassResolutionOverride](struct.MainPassResolutionOverride.html "struct bevy::camera::MainPassResolutionOverride")

Override the resolution a 3d camera’s main pass is rendered at.

[ManualTextureViewHandle](struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

A unique id that corresponds to a specific `ManualTextureView` in the `ManualTextureViews` collection.

[ManualTextureViewHandleTemplate](struct.ManualTextureViewHandleTemplate.html "struct bevy::camera::ManualTextureViewHandleTemplate")

[OrthographicProjection](struct.OrthographicProjection.html "struct bevy::camera::OrthographicProjection")

Project a 3D space onto a 2D surface using parallel lines, i.e., unlike [`PerspectiveProjection`](../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection"), the size of objects remains the same regardless of their distance to the camera.

[PerspectiveProjection](struct.PerspectiveProjection.html "struct bevy::camera::PerspectiveProjection")

A 3D camera projection in which distant objects appear smaller than close objects.

[PhysicalCameraParameters](struct.PhysicalCameraParameters.html "struct bevy::camera::PhysicalCameraParameters")

Parameters based on physical camera characteristics for calculating EV100 values for use with [`Exposure`](struct.Exposure.html "struct bevy::camera::Exposure"). This is also used for depth of field.

[RenderTargetInfo](struct.RenderTargetInfo.html "struct bevy::camera::RenderTargetInfo")

Information about the current [`RenderTarget`](enum.RenderTarget.html "enum bevy::camera::RenderTarget").

[ShadowLodOrigin](struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin")

The entity that Bevy uses to resolve visibility ranges when no specific camera is applicable.

[SubCameraView](struct.SubCameraView.html "struct bevy::camera::SubCameraView")

Settings to define a camera sub view.

[Viewport](struct.Viewport.html "struct bevy::camera::Viewport")

Render viewport configuration for the [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera") component.

## Enums

[Camera3dDepthLoadOp](enum.Camera3dDepthLoadOp.html "enum bevy::camera::Camera3dDepthLoadOp")

The depth clear operation to perform for the main 3d pass.

[CameraOutputMode](enum.CameraOutputMode.html "enum bevy::camera::CameraOutputMode")

Control how this [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera") outputs once rendering is completed.

[ClearColorConfig](enum.ClearColorConfig.html "enum bevy::camera::ClearColorConfig")

For a camera, specifies the color used to clear the viewport [before rendering](../prelude/struct.Camera.html#structfield.clear_color "field bevy::prelude::Camera::clear_color") or when [writing to the final render target texture](../prelude/struct.Camera.html#structfield.output_mode "field bevy::prelude::Camera::output_mode").

[CompositingSpace](enum.CompositingSpace.html "enum bevy::camera::CompositingSpace")

Color space for alpha compositing. Affects how overlapping semi-transparent layers blend.

[MsaaWriteback](enum.MsaaWriteback.html "enum bevy::camera::MsaaWriteback")

Controls when MSAA writeback occurs for a camera.

[NormalizedRenderTarget](enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget")

Normalized version of the render target.

[Projection](enum.Projection.html "enum bevy::camera::Projection")

Component that defines how to compute a [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera")’s projection matrix.

[RenderTarget](enum.RenderTarget.html "enum bevy::camera::RenderTarget")

The “target” that a [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera") will render to. For example, this could be a `Window` swapchain or an [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image").

[ScalingMode](enum.ScalingMode.html "enum bevy::camera::ScalingMode")

Scaling mode for [`OrthographicProjection`](../prelude/struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection").

[ViewportConversionError](enum.ViewportConversionError.html "enum bevy::camera::ViewportConversionError")

Error returned when a conversion between world-space and viewport-space coordinates fails.

## Traits

[CameraProjection](trait.CameraProjection.html "trait bevy::camera::CameraProjection")

Describes a type that can generate a projection matrix, allowing it to be added to a [`Camera`](../prelude/struct.Camera.html "struct bevy::prelude::Camera")’s [`Projection`](../prelude/enum.Projection.html "enum bevy::prelude::Projection") component.