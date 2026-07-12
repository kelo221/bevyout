[bevy](../../index.html)::[post\_process](../index.html)

# Module dof 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/lib.rs.html#11)

Depth of field, a postprocessing effect that simulates camera focus.

By default, Bevy renders all objects in full focus: regardless of depth, all objects are rendered perfectly sharp (up to output resolution). Real lenses, however, can only focus on objects at a specific distance. The distance between the nearest and furthest objects that are in focus is known as [depth of field](https://en.wikipedia.org/wiki/Depth_of_field), and this term is used more generally in computer graphics to refer to the effect that simulates focus of lenses.

Attaching [`DepthOfField`](struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField") to a camera causes Bevy to simulate the focus of a camera lens. Generally, Bevy’s implementation of depth of field is optimized for speed instead of physical accuracy. Nevertheless, the depth of field effect in Bevy is based on physical parameters.

## Structs

[AuxiliaryDepthOfFieldTexture](struct.AuxiliaryDepthOfFieldTexture.html "struct bevy::post_process::dof::AuxiliaryDepthOfFieldTexture")

The extra texture used as the second render target for the hexagonal bokeh blur.

[DepthOfField](struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField")

A component that enables a [depth of field](https://en.wikipedia.org/wiki/Depth_of_field) postprocessing effect when attached to a [`Camera3d`](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d"), simulating the focus of a camera lens.

[DepthOfFieldGlobalBindGroup](struct.DepthOfFieldGlobalBindGroup.html "struct bevy::post_process::dof::DepthOfFieldGlobalBindGroup")

The bind group shared among all invocations of the depth of field shader, regardless of view.

[DepthOfFieldGlobalBindGroupLayout](struct.DepthOfFieldGlobalBindGroupLayout.html "struct bevy::post_process::dof::DepthOfFieldGlobalBindGroupLayout")

The layout for the bind group shared among all invocations of the depth of field shader.

[DepthOfFieldPipeline](struct.DepthOfFieldPipeline.html "struct bevy::post_process::dof::DepthOfFieldPipeline")

Information needed to specialize the pipeline corresponding to a pass of the depth of field shader.

[DepthOfFieldPipelineKey](struct.DepthOfFieldPipelineKey.html "struct bevy::post_process::dof::DepthOfFieldPipelineKey")

A key that uniquely identifies depth of field pipelines.

[DepthOfFieldPlugin](struct.DepthOfFieldPlugin.html "struct bevy::post_process::dof::DepthOfFieldPlugin")

A plugin that adds support for the depth of field effect to Bevy.

[DepthOfFieldUniform](struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform")

Data about the depth of field effect that’s uploaded to the GPU.

[ViewDepthOfFieldBindGroupLayouts](struct.ViewDepthOfFieldBindGroupLayouts.html "struct bevy::post_process::dof::ViewDepthOfFieldBindGroupLayouts")

Bind group layouts for depth of field specific to a single view.

## Enums

[DepthOfFieldMode](enum.DepthOfFieldMode.html "enum bevy::post_process::dof::DepthOfFieldMode")

Controls the appearance of the effect.

[DepthOfFieldPipelines](enum.DepthOfFieldPipelines.html "enum bevy::post_process::dof::DepthOfFieldPipelines")

## Functions

[calculate\_focal\_length](fn.calculate_focal_length.html "fn bevy::post_process::dof::calculate_focal_length")

Given the sensor height and the FOV, returns the focal length.

[configure\_depth\_of\_field\_view\_targets](fn.configure_depth_of_field_view_targets.html "fn bevy::post_process::dof::configure_depth_of_field_view_targets")

Configures depth textures so that the depth of field shader can read from them.

[init\_dof\_global\_bind\_group\_layout](fn.init_dof_global_bind_group_layout.html "fn bevy::post_process::dof::init_dof_global_bind_group_layout")

[prepare\_auxiliary\_depth\_of\_field\_textures](fn.prepare_auxiliary_depth_of_field_textures.html "fn bevy::post_process::dof::prepare_auxiliary_depth_of_field_textures")

Creates the second render target texture that the first pass of the bokeh effect needs.

[prepare\_depth\_of\_field\_global\_bind\_group](fn.prepare_depth_of_field_global_bind_group.html "fn bevy::post_process::dof::prepare_depth_of_field_global_bind_group")

Creates depth of field bind group 1, which is shared among all instances of the depth of field shader.

[prepare\_depth\_of\_field\_pipelines](fn.prepare_depth_of_field_pipelines.html "fn bevy::post_process::dof::prepare_depth_of_field_pipelines")

Specializes the depth of field pipelines specific to a view.

[prepare\_depth\_of\_field\_view\_bind\_group\_layouts](fn.prepare_depth_of_field_view_bind_group_layouts.html "fn bevy::post_process::dof::prepare_depth_of_field_view_bind_group_layouts")

Creates the bind group layouts for the depth of field effect that are specific to each view.