[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module tonemapping 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#19)

## Structs

[TonemappingLuts](struct.TonemappingLuts.html "struct bevy::core_pipeline::tonemapping::TonemappingLuts")

3D LUT (look up table) textures used for tonemapping

[TonemappingPipeline](struct.TonemappingPipeline.html "struct bevy::core_pipeline::tonemapping::TonemappingPipeline")

[TonemappingPipelineKey](struct.TonemappingPipelineKey.html "struct bevy::core_pipeline::tonemapping::TonemappingPipelineKey")

[TonemappingPipelineKeyFlags](struct.TonemappingPipelineKeyFlags.html "struct bevy::core_pipeline::tonemapping::TonemappingPipelineKeyFlags")

Various flags describing what tonemapping needs to do.

[TonemappingPlugin](struct.TonemappingPlugin.html "struct bevy::core_pipeline::tonemapping::TonemappingPlugin")

[ViewTonemappingPipeline](struct.ViewTonemappingPipeline.html "struct bevy::core_pipeline::tonemapping::ViewTonemappingPipeline")

## Enums

[DebandDither](enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

Enables a debanding shader that applies dithering to mitigate color banding in the final image for a given [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") entity.

[Tonemapping](enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

Optionally enables a tonemapping shader that attempts to map linear input stimulus into a perceptually uniform image for a given [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera") entity.

## Functions

[get\_lut\_bind\_group\_layout\_entries](fn.get_lut_bind_group_layout_entries.html "fn bevy::core_pipeline::tonemapping::get_lut_bind_group_layout_entries")

[get\_lut\_bindings](fn.get_lut_bindings.html "fn bevy::core_pipeline::tonemapping::get_lut_bindings")

[init\_tonemapping\_pipeline](fn.init_tonemapping_pipeline.html "fn bevy::core_pipeline::tonemapping::init_tonemapping_pipeline")

[lut\_placeholder](fn.lut_placeholder.html "fn bevy::core_pipeline::tonemapping::lut_placeholder")

[prepare\_view\_tonemapping\_pipelines](fn.prepare_view_tonemapping_pipelines.html "fn bevy::core_pipeline::tonemapping::prepare_view_tonemapping_pipelines")

[tonemapping](fn.tonemapping.html "fn bevy::core_pipeline::tonemapping::tonemapping")