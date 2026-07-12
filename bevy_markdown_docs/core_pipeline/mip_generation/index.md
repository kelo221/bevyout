[bevy](../../index.html)::[core\_pipeline](../index.html)

# Module mip\_generation 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#14)

Downsampling of textures to produce mipmap levels.

This module implements variations on the [AMD FidelityFX single-pass downsampling](https://gpuopen.com/fidelityfx-spd/) shader. It’s used for generating mipmaps for textures ([`MipGenerationJobs`](struct.MipGenerationJobs.html "struct bevy::core_pipeline::mip_generation::MipGenerationJobs")) and for creating hierarchical Z-buffers (the [`experimental::depth`](experimental/depth/index.html "mod bevy::core_pipeline::mip_generation::experimental::depth") module).

See the documentation for [`MipGenerationJobs`](struct.MipGenerationJobs.html "struct bevy::core_pipeline::mip_generation::MipGenerationJobs") and [`experimental::depth`](experimental/depth/index.html "mod bevy::core_pipeline::mip_generation::experimental::depth") for more information.

## Modules

[experimental](experimental/index.html "mod bevy::core_pipeline::mip_generation::experimental")

Experimental functionality related to mipmap generation.

## Structs

[DownsampleShaders](struct.DownsampleShaders.html "struct bevy::core_pipeline::mip_generation::DownsampleShaders")

A resource that stores the shaders that perform downsampling.

[DownsamplingConstants](struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants")

Constants for the single-pass downsampling shader generated on the CPU and read on the GPU.

[MipGenerationJobs](struct.MipGenerationJobs.html "struct bevy::core_pipeline::mip_generation::MipGenerationJobs")

A render-world resource that stores a list of [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image")s that will have mipmaps generated for them.

[MipGenerationPhase](struct.MipGenerationPhase.html "struct bevy::core_pipeline::mip_generation::MipGenerationPhase")

The list of [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image")s that will have mipmaps generated for them during a specific phase.

[MipGenerationPhaseId](struct.MipGenerationPhaseId.html "struct bevy::core_pipeline::mip_generation::MipGenerationPhaseId")

Identifies a _phase_ during which mipmaps will be generated for an image.

[MipGenerationPipelines](struct.MipGenerationPipelines.html "struct bevy::core_pipeline::mip_generation::MipGenerationPipelines")

Stores all render pipelines and bind groups associated with the mipmap generation shader.

[MipGenerationPlugin](struct.MipGenerationPlugin.html "struct bevy::core_pipeline::mip_generation::MipGenerationPlugin")

A plugin that allows Bevy to repeatedly downsample textures to create mipmaps.

## Functions

[can\_combine\_downsampling\_bind\_groups](fn.can_combine_downsampling_bind_groups.html "fn bevy::core_pipeline::mip_generation::can_combine_downsampling_bind_groups")

Returns true if the current platform can use a single bind group for single-pass downsampling.

[generate\_mips\_for\_phase](fn.generate_mips_for_phase.html "fn bevy::core_pipeline::mip_generation::generate_mips_for_phase")

Generates mipmaps for all images in a [`MipGenerationPhaseId`](struct.MipGenerationPhaseId.html "struct bevy::core_pipeline::mip_generation::MipGenerationPhaseId").