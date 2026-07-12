[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[mip\_generation](../../index.html)::[experimental](../index.html)

# Module depth 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/mod.rs.html#5)

Generation of hierarchical Z buffers for occlusion culling.

Currently, this module only supports generation of hierarchical Z buffers for occlusion culling.

## Structs

[DepthPyramidDummyTexture](struct.DepthPyramidDummyTexture.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DepthPyramidDummyTexture")

Stores a placeholder texture that can be bound to a depth pyramid binding if no depth pyramid is needed.

[DownsampleDepthPipeline](struct.DownsampleDepthPipeline.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipeline")

A single depth downsample pipeline.

[DownsampleDepthPipelineKey](struct.DownsampleDepthPipelineKey.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipelineKey")

Uniquely identifies a configuration of the downsample depth shader.

[DownsampleDepthPipelines](struct.DownsampleDepthPipelines.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipelines")

Stores all depth buffer downsampling pipelines.

[ViewDepthPyramid](struct.ViewDepthPyramid.html "struct bevy::core_pipeline::mip_generation::experimental::depth::ViewDepthPyramid")

Stores a hierarchical Z-buffer for a view, which is a series of mipmaps useful for efficient occlusion culling.

[ViewDownsampleDepthBindGroup](struct.ViewDownsampleDepthBindGroup.html "struct bevy::core_pipeline::mip_generation::experimental::depth::ViewDownsampleDepthBindGroup")

The bind group that we use to attach the depth buffer and depth pyramid for a view to the `downsample_depth.wgsl` shader.

## Constants

[DEPTH\_PYRAMID\_MIP\_COUNT](constant.DEPTH_PYRAMID_MIP_COUNT.html "constant bevy::core_pipeline::mip_generation::experimental::depth::DEPTH_PYRAMID_MIP_COUNT")

The maximum number of mip levels that we can produce.

## Functions

[create\_depth\_pyramid\_dummy\_texture](fn.create_depth_pyramid_dummy_texture.html "fn bevy::core_pipeline::mip_generation::experimental::depth::create_depth_pyramid_dummy_texture")

Creates a placeholder texture that can be bound to a depth pyramid binding if no depth pyramid is needed.

[create\_downsample\_depth\_pipelines](fn.create_downsample_depth_pipelines.html "fn bevy::core_pipeline::mip_generation::experimental::depth::create_downsample_depth_pipelines")

Creates the [`DownsampleDepthPipelines`](struct.DownsampleDepthPipelines.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipelines") if downsampling is supported on the current platform.

[early\_downsample\_depth](fn.early_downsample_depth.html "fn bevy::core_pipeline::mip_generation::experimental::depth::early_downsample_depth")

Produces a hierarchical Z-buffer (depth pyramid) for occlusion culling.

[init\_depth\_pyramid\_dummy\_texture](fn.init_depth_pyramid_dummy_texture.html "fn bevy::core_pipeline::mip_generation::experimental::depth::init_depth_pyramid_dummy_texture")

[late\_downsample\_depth](fn.late_downsample_depth.html "fn bevy::core_pipeline::mip_generation::experimental::depth::late_downsample_depth")

Produces a hierarchical Z-buffer (depth pyramid) for occlusion culling.

[prepare\_downsample\_depth\_view\_bind\_groups](fn.prepare_downsample_depth_view_bind_groups.html "fn bevy::core_pipeline::mip_generation::experimental::depth::prepare_downsample_depth_view_bind_groups")

Creates the [`ViewDownsampleDepthBindGroup`](struct.ViewDownsampleDepthBindGroup.html "struct bevy::core_pipeline::mip_generation::experimental::depth::ViewDownsampleDepthBindGroup")s for all views with occlusion culling enabled.

[prepare\_view\_depth\_pyramids](fn.prepare_view_depth_pyramids.html "fn bevy::core_pipeline::mip_generation::experimental::depth::prepare_view_depth_pyramids")

Creates depth pyramids for views that have occlusion culling enabled.