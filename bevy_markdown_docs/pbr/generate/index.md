[bevy](../../index.html)::[pbr](../index.html)

# Module generate 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#45)

Like [`EnvironmentMapLight`](../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight"), but filtered in realtime from a cubemap.

An environment map needs to be processed to be able to support uses beyond a simple skybox, such as reflections, and ambient light contribution. This process is called filtering, and can either be done ahead of time (prefiltering), or in realtime, although at a reduced quality. Prefiltering is preferred, but not always possible: sometimes you only gain access to an environment map at runtime, for whatever reason. Typically this is from realtime reflection probes, but can also be from other sources.

In any case, Bevy supports both modes of filtering. This module provides realtime filtering via [`bevy_light::GeneratedEnvironmentMapLight`](../../prelude/struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight"). For prefiltered environment maps, see [`bevy_light::EnvironmentMapLight`](../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight"). These components are intended to be added to a camera.

## Structs

[DownsamplingConfig](struct.DownsamplingConfig.html "struct bevy::pbr::generate::DownsamplingConfig")

Configuration for downsampling strategy based on device limits

[EnvironmentMapGenerationPlugin](struct.EnvironmentMapGenerationPlugin.html "struct bevy::pbr::generate::EnvironmentMapGenerationPlugin")

[FilteringConstants](struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants")

Constants for filtering

[GeneratorBindGroupLayouts](struct.GeneratorBindGroupLayouts.html "struct bevy::pbr::generate::GeneratorBindGroupLayouts")

Stores the bind group layouts for the environment map generation pipelines

[GeneratorBindGroups](struct.GeneratorBindGroups.html "struct bevy::pbr::generate::GeneratorBindGroups")

Stores bind groups for the environment map generation pipelines

[GeneratorPipelines](struct.GeneratorPipelines.html "struct bevy::pbr::generate::GeneratorPipelines")

Pipelines for the environment map generation pipelines

[GeneratorSamplers](struct.GeneratorSamplers.html "struct bevy::pbr::generate::GeneratorSamplers")

Samplers for the environment map generation pipelines

[IntermediateTextures](struct.IntermediateTextures.html "struct bevy::pbr::generate::IntermediateTextures")

[RenderEnvironmentMap](struct.RenderEnvironmentMap.html "struct bevy::pbr::generate::RenderEnvironmentMap")

## Functions

[downsampling\_system](fn.downsampling_system.html "fn bevy::pbr::generate::downsampling_system")

[extract\_generated\_environment\_map\_entities](fn.extract_generated_environment_map_entities.html "fn bevy::pbr::generate::extract_generated_environment_map_entities")

[filtering\_system](fn.filtering_system.html "fn bevy::pbr::generate::filtering_system")

[generate\_environment\_map\_light](fn.generate_environment_map_light.html "fn bevy::pbr::generate::generate_environment_map_light")

System that generates an `EnvironmentMapLight` component based on the `GeneratedEnvironmentMapLight` component

[initialize\_generated\_environment\_map\_resources](fn.initialize_generated_environment_map_resources.html "fn bevy::pbr::generate::initialize_generated_environment_map_resources")

Initializes all render-world resources used by the environment-map generator once on [`bevy_render::RenderStartup`](../../render/struct.RenderStartup.html "struct bevy::render::RenderStartup").

[prepare\_generated\_environment\_map\_bind\_groups](fn.prepare_generated_environment_map_bind_groups.html "fn bevy::pbr::generate::prepare_generated_environment_map_bind_groups")

Prepares bind groups for environment map generation pipelines

[prepare\_generated\_environment\_map\_intermediate\_textures](fn.prepare_generated_environment_map_intermediate_textures.html "fn bevy::pbr::generate::prepare_generated_environment_map_intermediate_textures")

Prepares textures needed for single pass downsampling