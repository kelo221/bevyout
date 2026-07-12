[bevy](../../../index.html)::[core\_pipeline](../../index.html)::[oit](../index.html)

# Module resolve 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#31)

Module that defines the necessary systems to resolve the OIT buffer and render it to the screen.

## Modules

[node](node/index.html "mod bevy::core_pipeline::oit::resolve::node")

Contains the render node used to run the resolve pass.

## Structs

[OitResolveBindGroup](struct.OitResolveBindGroup.html "struct bevy::core_pipeline::oit::resolve::OitResolveBindGroup")

Bind group for the OIT resolve pass.

[OitResolvePipeline](struct.OitResolvePipeline.html "struct bevy::core_pipeline::oit::resolve::OitResolvePipeline")

Bind group layouts used for the OIT resolve pass.

[OitResolvePipelineId](struct.OitResolvePipelineId.html "struct bevy::core_pipeline::oit::resolve::OitResolvePipelineId")

[OitResolvePipelineKey](struct.OitResolvePipelineKey.html "struct bevy::core_pipeline::oit::resolve::OitResolvePipelineKey")

This key is used to cache the pipeline id and to specialize the render pipeline descriptor.

[OitResolvePlugin](struct.OitResolvePlugin.html "struct bevy::core_pipeline::oit::resolve::OitResolvePlugin")

Plugin needed to resolve the Order Independent Transparency (OIT) buffer to the screen.

## Constants

[OIT\_REQUIRED\_STORAGE\_BUFFERS](constant.OIT_REQUIRED_STORAGE_BUFFERS.html "constant bevy::core_pipeline::oit::resolve::OIT_REQUIRED_STORAGE_BUFFERS")

Minimum required value of `wgpu::Limits::max_storage_buffers_per_shader_stage`.

## Functions

[is\_oit\_supported](fn.is_oit_supported.html "fn bevy::core_pipeline::oit::resolve::is_oit_supported")

[prepare\_oit\_resolve\_bind\_group](fn.prepare_oit_resolve_bind_group.html "fn bevy::core_pipeline::oit::resolve::prepare_oit_resolve_bind_group")

[queue\_oit\_resolve\_pipeline](fn.queue_oit_resolve_pipeline.html "fn bevy::core_pipeline::oit::resolve::queue_oit_resolve_pipeline")