[bevy](../index.html)

# Crate pbr 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#1-545)

## Modules

[contact\_shadows](contact_shadows/index.html "mod bevy::pbr::contact_shadows")

Contact shadows implemented via screenspace raymarching.

[decal](decal/index.html "mod bevy::pbr::decal")

Decal rendering.

[deferred](deferred/index.html "mod bevy::pbr::deferred")

[diagnostic](diagnostic/index.html "mod bevy::pbr::diagnostic")

[environment\_map](environment_map/index.html "mod bevy::pbr::environment_map")

Environment maps and reflection probes.

[experimental](experimental/index.html "mod bevy::pbr::experimental")`meshlet`

Experimental features that are not yet finished. Please report any issues you encounter!

[generate](generate/index.html "mod bevy::pbr::generate")

Like [`EnvironmentMapLight`](../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight"), but filtered in realtime from a cubemap.

[irradiance\_volume](irradiance_volume/index.html "mod bevy::pbr::irradiance_volume")

Irradiance volumes, also known as voxel global illumination.

[prelude](prelude/index.html "mod bevy::pbr::prelude")

The PBR prelude.

[resources](resources/index.html "mod bevy::pbr::resources")

[wireframe](wireframe/index.html "mod bevy::pbr::wireframe")

## Structs

[AreaLightLuts](struct.AreaLightLuts.html "struct bevy::pbr::AreaLightLuts")

LTC (Linearly Transformed Cosines) LUT textures for area light shading.

[AtmosphereSettings](struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings")

This component controls the resolution of the atmosphere LUTs, and how many samples are used when computing them.

[BinUnpackingBindGroups](struct.BinUnpackingBindGroups.html "struct bevy::pbr::BinUnpackingBindGroups")

A resource, part of the render world, that stores all the bind groups for the bin unpacking shader.

[BinUnpackingPipeline](struct.BinUnpackingPipeline.html "struct bevy::pbr::BinUnpackingPipeline")

The pipeline for the `unpack_bins` compute shader.

[Bluenoise](struct.Bluenoise.html "struct bevy::pbr::Bluenoise")

A resource that stores the spatio-temporal blue noise texture.

[BuildIndirectParametersBindGroups](struct.BuildIndirectParametersBindGroups.html "struct bevy::pbr::BuildIndirectParametersBindGroups")

The bind groups for the compute shaders that reset indirect draw counts and build indirect parameters.

[BuildIndirectParametersPipeline](struct.BuildIndirectParametersPipeline.html "struct bevy::pbr::BuildIndirectParametersPipeline")

The pipeline for the indirect parameter building shader.

[BuildIndirectParametersPipelineKey](struct.BuildIndirectParametersPipelineKey.html "struct bevy::pbr::BuildIndirectParametersPipelineKey")

Specifies variants of the indirect parameter building shader.

[ClusteredDecalPlugin](struct.ClusteredDecalPlugin.html "struct bevy::pbr::ClusteredDecalPlugin")

A plugin that adds support for clustered decals.

[ContactShadows](struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

Add this component to a camera to enable contact shadows.

[ContactShadowsBuffer](struct.ContactShadowsBuffer.html "struct bevy::pbr::ContactShadowsBuffer")

A GPU buffer that stores the contact shadow settings for each view.

[ContactShadowsPlugin](struct.ContactShadowsPlugin.html "struct bevy::pbr::ContactShadowsPlugin")

Enables contact shadows for a camera.

[ContactShadowsUniform](struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform")

A version of [`ContactShadows`](struct.ContactShadows.html "struct bevy::pbr::ContactShadows") for upload to the GPU.

[DefaultOpaqueRendererMethod](struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

Default render method used for opaque materials.

[DeferredAlphaMaskDrawFunction](struct.DeferredAlphaMaskDrawFunction.html "struct bevy::pbr::DeferredAlphaMaskDrawFunction")

[DeferredFragmentShader](struct.DeferredFragmentShader.html "struct bevy::pbr::DeferredFragmentShader")

[DeferredOpaqueDrawFunction](struct.DeferredOpaqueDrawFunction.html "struct bevy::pbr::DeferredOpaqueDrawFunction")

[DeferredVertexShader](struct.DeferredVertexShader.html "struct bevy::pbr::DeferredVertexShader")

[DfgLut](struct.DfgLut.html "struct bevy::pbr::DfgLut")

The split-sum approximation LUT (`F_AB`) indexed by (`NdotV`, `perceptual_roughness`).

[DirectionalLightViewEntities](struct.DirectionalLightViewEntities.html "struct bevy::pbr::DirectionalLightViewEntities")

Component automatically attached to a light entity to track light-view entities for each view.

[DistanceFog](struct.DistanceFog.html "struct bevy::pbr::DistanceFog")

Configures the “classic” computer graphics [distance fog](https://en.wikipedia.org/wiki/Distance_fog) effect, in which objects appear progressively more covered in atmospheric haze the further away they are from the camera. Affects meshes rendered via the PBR [`StandardMaterial`](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial").

[DrawMesh](struct.DrawMesh.html "struct bevy::pbr::DrawMesh")

[EntitiesNeedingSpecialization](struct.EntitiesNeedingSpecialization.html "struct bevy::pbr::EntitiesNeedingSpecialization")

Temporarily stores entities that were determined to either need their specialized pipelines updated or to have their specialized pipelines removed.

[ExtendedMaterial](struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")

A material that extends a base [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material") with additional shaders and data.

[ExtractedAtmosphere](struct.ExtractedAtmosphere.html "struct bevy::pbr::ExtractedAtmosphere")

The render-world representation of an `Atmosphere`, but which hasn’t been converted into shader uniforms yet.

[ExtractedClusterConfig](struct.ExtractedClusterConfig.html "struct bevy::pbr::ExtractedClusterConfig")

[ExtractedClusterableObjects](struct.ExtractedClusterableObjects.html "struct bevy::pbr::ExtractedClusterableObjects")

[ExtractedDirectionalLight](struct.ExtractedDirectionalLight.html "struct bevy::pbr::ExtractedDirectionalLight")

[ExtractedPointLight](struct.ExtractedPointLight.html "struct bevy::pbr::ExtractedPointLight")

[ExtractedRectLight](struct.ExtractedRectLight.html "struct bevy::pbr::ExtractedRectLight")

[FallbackBindlessResources](struct.FallbackBindlessResources.html "struct bevy::pbr::FallbackBindlessResources")

Dummy instances of various resources that we fill unused slots in binding arrays with.

[FogMeta](struct.FogMeta.html "struct bevy::pbr::FogMeta")

Metadata for fog

[FogPlugin](struct.FogPlugin.html "struct bevy::pbr::FogPlugin")

A plugin that consolidates fog extraction, preparation and related resources/assets

[GlobalClusterableObjectMeta](struct.GlobalClusterableObjectMeta.html "struct bevy::pbr::GlobalClusterableObjectMeta")

Contains information about clusterable objects in the scene that’s global: i.e. not specific to any view.

[GpuAtmosphereSettings](struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

[GpuClusteredLight](struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight")

The GPU-side structure that stores information about a clustered light (point or spot).

[GpuClusteredLights](struct.GpuClusteredLights.html "struct bevy::pbr::GpuClusteredLights")

GPU buffers that hold data about the clustered lights.

[GpuDirectionalCascade](struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade")

[GpuDirectionalLight](struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight")

[GpuFog](struct.GpuFog.html "struct bevy::pbr::GpuFog")

The GPU-side representation of the fog configuration that’s sent as a uniform to the shader

[GpuLights](struct.GpuLights.html "struct bevy::pbr::GpuLights")

[GpuMeshPreprocessPlugin](struct.GpuMeshPreprocessPlugin.html "struct bevy::pbr::GpuMeshPreprocessPlugin")

A plugin that builds mesh uniforms on GPU.

[GpuMorphDescriptor](struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor")

Information that the GPU needs about a single mesh instance that uses morph targets.

[GpuRectLight](struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight")

[GpuScatteringMedium](struct.GpuScatteringMedium.html "struct bevy::pbr::GpuScatteringMedium")

The GPU representation of a [`ScatteringMedium`](../light/atmosphere/struct.ScatteringMedium.html "struct bevy::light::atmosphere::ScatteringMedium").

[LightKeyCache](struct.LightKeyCache.html "struct bevy::pbr::LightKeyCache")

[LightMeta](struct.LightMeta.html "struct bevy::pbr::LightMeta")

[LightProbePlugin](struct.LightProbePlugin.html "struct bevy::pbr::LightProbePlugin")

Adds support for light probes: cuboid bounding regions that apply global illumination to objects within them.

[LightProbesBuffer](struct.LightProbesBuffer.html "struct bevy::pbr::LightProbesBuffer")

A GPU buffer that stores information about all light probes.

[LightProbesUniform](struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform")

A per-view shader uniform that specifies all the light probes that the view takes into account.

[Lightmap](struct.Lightmap.html "struct bevy::pbr::Lightmap")

A component that applies baked indirect diffuse global illumination from a lightmap.

[LightmapPlugin](struct.LightmapPlugin.html "struct bevy::pbr::LightmapPlugin")

A plugin that provides an implementation of lightmaps.

[LightmapSlab](struct.LightmapSlab.html "struct bevy::pbr::LightmapSlab")

A binding array that contains lightmaps.

[LightmapSlabIndex](struct.LightmapSlabIndex.html "struct bevy::pbr::LightmapSlabIndex")

The index of the slab (binding array) in which a lightmap is located.

[LightmapSlotIndex](struct.LightmapSlotIndex.html "struct bevy::pbr::LightmapSlotIndex")

The index of the slot (element within the binding array) in the slab in which a lightmap is located.

[LightmapTemplate](struct.LightmapTemplate.html "struct bevy::pbr::LightmapTemplate")

[MainPassAlphaMaskDrawFunction](struct.MainPassAlphaMaskDrawFunction.html "struct bevy::pbr::MainPassAlphaMaskDrawFunction")

[MainPassOpaqueDrawFunction](struct.MainPassOpaqueDrawFunction.html "struct bevy::pbr::MainPassOpaqueDrawFunction")

[MainPassTransmissiveDrawFunction](struct.MainPassTransmissiveDrawFunction.html "struct bevy::pbr::MainPassTransmissiveDrawFunction")

[MainPassTransparentDrawFunction](struct.MainPassTransparentDrawFunction.html "struct bevy::pbr::MainPassTransparentDrawFunction")

[MaterialBindGroupAllocators](struct.MaterialBindGroupAllocators.html "struct bevy::pbr::MaterialBindGroupAllocators")

[MaterialBindGroupBindlessAllocator](struct.MaterialBindGroupBindlessAllocator.html "struct bevy::pbr::MaterialBindGroupBindlessAllocator")

The allocator that places bindless materials into bind groups and tracks their resources.

[MaterialBindGroupIndex](struct.MaterialBindGroupIndex.html "struct bevy::pbr::MaterialBindGroupIndex")

The index of each material bind group.

[MaterialBindGroupNonBindlessAllocator](struct.MaterialBindGroupNonBindlessAllocator.html "struct bevy::pbr::MaterialBindGroupNonBindlessAllocator")

The allocator that stores bind groups for non-bindless materials.

[MaterialBindGroupSlot](struct.MaterialBindGroupSlot.html "struct bevy::pbr::MaterialBindGroupSlot")

The index of the slot containing material data within each material bind group.

[MaterialBindingId](struct.MaterialBindingId.html "struct bevy::pbr::MaterialBindingId")

The location of a material (either bindless or non-bindless) within the slabs.

[MaterialBindlessSlab](struct.MaterialBindlessSlab.html "struct bevy::pbr::MaterialBindlessSlab")

A single bind group and the bookkeeping necessary to allocate into it.

[MaterialExtensionBindGroupData](struct.MaterialExtensionBindGroupData.html "struct bevy::pbr::MaterialExtensionBindGroupData")

[MaterialExtensionKey](struct.MaterialExtensionKey.html "struct bevy::pbr::MaterialExtensionKey")

[MaterialExtensionPipeline](struct.MaterialExtensionPipeline.html "struct bevy::pbr::MaterialExtensionPipeline")

[MaterialExtractionSystems](struct.MaterialExtractionSystems.html "struct bevy::pbr::MaterialExtractionSystems")

A [`SystemSet`](../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") that contains all `extract_mesh_materials` systems.

[MaterialFragmentShader](struct.MaterialFragmentShader.html "struct bevy::pbr::MaterialFragmentShader")

[MaterialPipeline](struct.MaterialPipeline.html "struct bevy::pbr::MaterialPipeline")

Render pipeline data for a given [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material").

[MaterialPipelineKey](struct.MaterialPipelineKey.html "struct bevy::pbr::MaterialPipelineKey")

A key uniquely identifying a specialized [`MaterialPipeline`](struct.MaterialPipeline.html "struct bevy::pbr::MaterialPipeline").

[MaterialPipelineSpecializer](struct.MaterialPipelineSpecializer.html "struct bevy::pbr::MaterialPipelineSpecializer")

[MaterialPlugin](struct.MaterialPlugin.html "struct bevy::pbr::MaterialPlugin")

Adds the necessary ECS resources and render logic to enable rendering entities using the given [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material") asset type.

[MaterialSlab](struct.MaterialSlab.html "struct bevy::pbr::MaterialSlab")

The public interface to a slab, which represents a single bind group.

[MaterialVertexShader](struct.MaterialVertexShader.html "struct bevy::pbr::MaterialVertexShader")

[MaterialsPlugin](struct.MaterialsPlugin.html "struct bevy::pbr::MaterialsPlugin")

[MeshBatchSetCompareData](struct.MeshBatchSetCompareData.html "struct bevy::pbr::MeshBatchSetCompareData")

Data that must be identical for meshes to be multi-drawn together.

[MeshBindGroupPair](struct.MeshBindGroupPair.html "struct bevy::pbr::MeshBindGroupPair")

[MeshCullingData](struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData")

Information about each mesh instance needed to cull it on GPU.

[MeshCullingDataBlob](struct.MeshCullingDataBlob.html "struct bevy::pbr::MeshCullingDataBlob")

[MeshCullingDataBuffer](struct.MeshCullingDataBuffer.html "struct bevy::pbr::MeshCullingDataBuffer")

A GPU buffer that holds the information needed to cull meshes on GPU.

[MeshExtractionSystems](struct.MeshExtractionSystems.html "struct bevy::pbr::MeshExtractionSystems")

A [`SystemSet`](../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") that encompasses both [`extract_meshes_for_cpu_building`](fn.extract_meshes_for_cpu_building.html "fn bevy::pbr::extract_meshes_for_cpu_building") and [`extract_meshes_for_gpu_building`](fn.extract_meshes_for_gpu_building.html "fn bevy::pbr::extract_meshes_for_gpu_building").

[MeshFlags](struct.MeshFlags.html "struct bevy::pbr::MeshFlags")

Various flags and tightly-packed values on a mesh.

[MeshInputUniform](struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")

Information that has to be transferred from CPU to GPU in order to produce the full [`MeshUniform`](struct.MeshUniform.html "struct bevy::pbr::MeshUniform").

[MeshInputUniformBlob](struct.MeshInputUniformBlob.html "struct bevy::pbr::MeshInputUniformBlob")

[MeshLayouts](struct.MeshLayouts.html "struct bevy::pbr::MeshLayouts")

All possible [`BindGroupLayout`](../render/render_resource/struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout")s in bevy’s default mesh shader (`mesh.wgsl`).

[MeshMaterial3d](struct.MeshMaterial3d.html "struct bevy::pbr::MeshMaterial3d")

A [material](../prelude/trait.Material.html "trait bevy::prelude::Material") used for rendering a [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d").

[MeshMaterial3dTemplate](struct.MeshMaterial3dTemplate.html "struct bevy::pbr::MeshMaterial3dTemplate")

[MeshMorphTargetStorageBindGroups](struct.MeshMorphTargetStorageBindGroups.html "struct bevy::pbr::MeshMorphTargetStorageBindGroups")

The bind groups associated with a single morph displacements slab.

[MeshPhaseBindGroups](struct.MeshPhaseBindGroups.html "struct bevy::pbr::MeshPhaseBindGroups")

The bind groups for meshes currently loaded.

[MeshPipeline](struct.MeshPipeline.html "struct bevy::pbr::MeshPipeline")

All data needed to construct a pipeline for rendering 3D meshes.

[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[MeshPipelineSystems](struct.MeshPipelineSystems.html "struct bevy::pbr::MeshPipelineSystems")

[MeshPipelineViewLayout](struct.MeshPipelineViewLayout.html "struct bevy::pbr::MeshPipelineViewLayout")

[MeshPipelineViewLayoutKey](struct.MeshPipelineViewLayoutKey.html "struct bevy::pbr::MeshPipelineViewLayoutKey")

A key that uniquely identifies a [`MeshPipelineViewLayout`](struct.MeshPipelineViewLayout.html "struct bevy::pbr::MeshPipelineViewLayout").

[MeshPipelineViewLayouts](struct.MeshPipelineViewLayouts.html "struct bevy::pbr::MeshPipelineViewLayouts")

Stores the view layouts entries for creating bind group layouts of pipeline keys.

[MeshRenderPlugin](struct.MeshRenderPlugin.html "struct bevy::pbr::MeshRenderPlugin")

Provides support for rendering 3D meshes.

[MeshTransforms](struct.MeshTransforms.html "struct bevy::pbr::MeshTransforms")

[MeshUniform](struct.MeshUniform.html "struct bevy::pbr::MeshUniform")

[MeshViewBindGroup](struct.MeshViewBindGroup.html "struct bevy::pbr::MeshViewBindGroup")

[MeshesToReextractNextFrame](struct.MeshesToReextractNextFrame.html "struct bevy::pbr::MeshesToReextractNextFrame")

Holds a list of meshes that couldn’t be extracted this frame because their materials weren’t prepared yet.

[MeshletDeferredFragmentShader](struct.MeshletDeferredFragmentShader.html "struct bevy::pbr::MeshletDeferredFragmentShader")

[MeshletFragmentShader](struct.MeshletFragmentShader.html "struct bevy::pbr::MeshletFragmentShader")

[MeshletPrepassFragmentShader](struct.MeshletPrepassFragmentShader.html "struct bevy::pbr::MeshletPrepassFragmentShader")

[MorphDescriptorIndex](struct.MorphDescriptorIndex.html "struct bevy::pbr::MorphDescriptorIndex")

The index of the [`GpuMorphDescriptor`](struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor") in the `morph_descriptors` buffer.

[MorphIndex](struct.MorphIndex.html "struct bevy::pbr::MorphIndex")

[MorphUniforms](struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms")

The GPU buffers containing morph weights for all meshes with morph targets.

[MorphWeightsInfo](struct.MorphWeightsInfo.html "struct bevy::pbr::MorphWeightsInfo")

Information that the CPU needs about each morh target for the purposes of weight calculation.

[PbrPlugin](struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")

Sets up the entire PBR infrastructure of bevy.

[PendingMeshMaterialQueues](struct.PendingMeshMaterialQueues.html "struct bevy::pbr::PendingMeshMaterialQueues")

Holds all entities with mesh materials that couldn’t be specialized and/or queued because their materials hadn’t loaded yet.

[PendingPrepassMeshMaterialQueues](struct.PendingPrepassMeshMaterialQueues.html "struct bevy::pbr::PendingPrepassMeshMaterialQueues")

Holds all entities with mesh materials for which the prepass couldn’t be specialized and/or queued because their materials hadn’t loaded yet.

[PendingShadowQueues](struct.PendingShadowQueues.html "struct bevy::pbr::PendingShadowQueues")

Holds all entities with mesh materials for which the shadow pass couldn’t be specialized and/or queued because their materials hadn’t loaded yet.

[PhaseBuildIndirectParametersBindGroups](struct.PhaseBuildIndirectParametersBindGroups.html "struct bevy::pbr::PhaseBuildIndirectParametersBindGroups")

The per-phase set of bind groups for the compute shaders that reset indirect draw counts and build indirect parameters.

[PointAndSpotLightViewEntities](struct.PointAndSpotLightViewEntities.html "struct bevy::pbr::PointAndSpotLightViewEntities")

A component that stores the shadow maps associated with a point or spot light.

[PreparedMaterial](struct.PreparedMaterial.html "struct bevy::pbr::PreparedMaterial")

Data prepared for a [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material") instance.

[PrepassAlphaMaskDrawFunction](struct.PrepassAlphaMaskDrawFunction.html "struct bevy::pbr::PrepassAlphaMaskDrawFunction")

[PrepassFragmentShader](struct.PrepassFragmentShader.html "struct bevy::pbr::PrepassFragmentShader")

[PrepassOpaqueDepthOnlyDrawFunction](struct.PrepassOpaqueDepthOnlyDrawFunction.html "struct bevy::pbr::PrepassOpaqueDepthOnlyDrawFunction")

[PrepassOpaqueDrawFunction](struct.PrepassOpaqueDrawFunction.html "struct bevy::pbr::PrepassOpaqueDrawFunction")

[PrepassPipeline](struct.PrepassPipeline.html "struct bevy::pbr::PrepassPipeline")

[PrepassPipelinePlugin](struct.PrepassPipelinePlugin.html "struct bevy::pbr::PrepassPipelinePlugin")

Sets up everything required to use the prepass pipeline.

[PrepassPipelineSpecializer](struct.PrepassPipelineSpecializer.html "struct bevy::pbr::PrepassPipelineSpecializer")

[PrepassPlugin](struct.PrepassPlugin.html "struct bevy::pbr::PrepassPlugin")

Sets up the prepasses for a material.

[PrepassVertexShader](struct.PrepassVertexShader.html "struct bevy::pbr::PrepassVertexShader")

[PrepassViewBindGroup](struct.PrepassViewBindGroup.html "struct bevy::pbr::PrepassViewBindGroup")

[PreprocessBindGroups](struct.PreprocessBindGroups.html "struct bevy::pbr::PreprocessBindGroups")

The compute shader bind group for the mesh preprocessing pass for each render phase.

[PreprocessPhasePipelines](struct.PreprocessPhasePipelines.html "struct bevy::pbr::PreprocessPhasePipelines")

Compute shader pipelines for a specific phase: early, late, or main.

[PreprocessPipeline](struct.PreprocessPipeline.html "struct bevy::pbr::PreprocessPipeline")

The pipeline for the GPU mesh preprocessing shader.

[PreprocessPipelineKey](struct.PreprocessPipelineKey.html "struct bevy::pbr::PreprocessPipelineKey")

Specifies variants of the mesh preprocessing shader.

[PreprocessPipelines](struct.PreprocessPipelines.html "struct bevy::pbr::PreprocessPipelines")

The compute shader pipelines for the GPU mesh preprocessing and indirect parameter building passes.

[PreviousGlobalTransform](struct.PreviousGlobalTransform.html "struct bevy::pbr::PreviousGlobalTransform")

[RenderGpuCulledEntities](struct.RenderGpuCulledEntities.html "struct bevy::pbr::RenderGpuCulledEntities")

A resource, part of the render world, that stores all entities that are potentially-visible and have [`NoCpuCulling`](../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling") components.

[RenderLightProbeFlags](struct.RenderLightProbeFlags.html "struct bevy::pbr::RenderLightProbeFlags")

Various flags that can be associated with light probes.

[RenderLightmaps](struct.RenderLightmaps.html "struct bevy::pbr::RenderLightmaps")

Stores data for all lightmaps in the render world.

[RenderMaterialBindings](struct.RenderMaterialBindings.html "struct bevy::pbr::RenderMaterialBindings")

A resource that maps each untyped material ID to its binding.

[RenderMaterialInstance](struct.RenderMaterialInstance.html "struct bevy::pbr::RenderMaterialInstance")

The material associated with a single mesh instance in the main world.

[RenderMaterialInstances](struct.RenderMaterialInstances.html "struct bevy::pbr::RenderMaterialInstances")

Stores all extracted instances of all [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material")s in the render world.

[RenderMeshInstanceCpu](struct.RenderMeshInstanceCpu.html "struct bevy::pbr::RenderMeshInstanceCpu")

CPU data that the render world keeps for each entity, when _not_ using GPU mesh uniform building.

[RenderMeshInstanceFlags](struct.RenderMeshInstanceFlags.html "struct bevy::pbr::RenderMeshInstanceFlags")

Various useful flags for \[`RenderMeshInstance`\]s.

[RenderMeshInstanceGpu](struct.RenderMeshInstanceGpu.html "struct bevy::pbr::RenderMeshInstanceGpu")

CPU data that the render world needs to keep for each entity that contains a mesh when using GPU mesh uniform building.

[RenderMeshInstanceGpuBuilder](struct.RenderMeshInstanceGpuBuilder.html "struct bevy::pbr::RenderMeshInstanceGpuBuilder")

Information that is gathered during the parallel portion of mesh extraction when GPU mesh uniform building is enabled.

[RenderMeshInstanceGpuFlat](struct.RenderMeshInstanceGpuFlat.html "struct bevy::pbr::RenderMeshInstanceGpuFlat")

Data in [`RenderMeshInstanceGpu`](struct.RenderMeshInstanceGpu.html "struct bevy::pbr::RenderMeshInstanceGpu") that’s both specific to the GPU preprocessing path and POD.

[RenderMeshInstanceGpuFlatBlob](struct.RenderMeshInstanceGpuFlatBlob.html "struct bevy::pbr::RenderMeshInstanceGpuFlatBlob")

[RenderMeshInstanceGpuPrepared](struct.RenderMeshInstanceGpuPrepared.html "struct bevy::pbr::RenderMeshInstanceGpuPrepared")

Data needed to construct the [`RenderMeshInstanceGpu`](struct.RenderMeshInstanceGpu.html "struct bevy::pbr::RenderMeshInstanceGpu") for a mesh instance.

[RenderMeshInstanceGpuQueues](struct.RenderMeshInstanceGpuQueues.html "struct bevy::pbr::RenderMeshInstanceGpuQueues")

The per-thread queues containing mesh instances, populated during the extract phase.

[RenderMeshInstanceShared](struct.RenderMeshInstanceShared.html "struct bevy::pbr::RenderMeshInstanceShared")

CPU data that the render world needs to keep about each entity that contains a mesh.

[RenderMeshInstanceSharedFlat](struct.RenderMeshInstanceSharedFlat.html "struct bevy::pbr::RenderMeshInstanceSharedFlat")

The thread-safe POD that’s stored for each mesh, common to both the CPU and GPU preprocessing paths.

[RenderMeshInstanceSharedFlatBlob](struct.RenderMeshInstanceSharedFlatBlob.html "struct bevy::pbr::RenderMeshInstanceSharedFlatBlob")

[RenderMeshInstancesCpu](struct.RenderMeshInstancesCpu.html "struct bevy::pbr::RenderMeshInstancesCpu")

Information that the render world keeps about each entity that contains a mesh, when using CPU mesh instance data building.

[RenderMeshInstancesGpu](struct.RenderMeshInstancesGpu.html "struct bevy::pbr::RenderMeshInstancesGpu")

Information that the render world keeps about each entity that contains a mesh, when using GPU mesh instance data building.

[RenderMeshQueueData](struct.RenderMeshQueueData.html "struct bevy::pbr::RenderMeshQueueData")

Data that [`crate::material::queue_material_meshes`](fn.queue_material_meshes.html "fn bevy::pbr::queue_material_meshes") and similar systems need in order to place entities that contain meshes in the right batch.

[RenderViewLightProbes](struct.RenderViewLightProbes.html "struct bevy::pbr::RenderViewLightProbes")

A component, part of the render world, that stores the mapping from asset ID or IDs to the texture index in the appropriate binding arrays.

[ResetIndirectBatchSetsPipeline](struct.ResetIndirectBatchSetsPipeline.html "struct bevy::pbr::ResetIndirectBatchSetsPipeline")

The pipeline for the batch set count reset shader.

[ScatteringMediumSampler](struct.ScatteringMediumSampler.html "struct bevy::pbr::ScatteringMediumSampler")

The default sampler for all scattering media LUTs.

[ScreenSpaceAmbientOcclusion](struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

Component to apply screen space ambient occlusion to a 3d camera.

[ScreenSpaceAmbientOcclusionPlugin](struct.ScreenSpaceAmbientOcclusionPlugin.html "struct bevy::pbr::ScreenSpaceAmbientOcclusionPlugin")

Plugin for screen space ambient occlusion.

[ScreenSpaceAmbientOcclusionResources](struct.ScreenSpaceAmbientOcclusionResources.html "struct bevy::pbr::ScreenSpaceAmbientOcclusionResources")

[ScreenSpaceReflections](struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

Add this component to a camera to enable _screen-space reflections_ (SSR).

[ScreenSpaceReflectionsBuffer](struct.ScreenSpaceReflectionsBuffer.html "struct bevy::pbr::ScreenSpaceReflectionsBuffer")

A GPU buffer that stores the screen space reflection settings for each view.

[ScreenSpaceReflectionsPipeline](struct.ScreenSpaceReflectionsPipeline.html "struct bevy::pbr::ScreenSpaceReflectionsPipeline")

Information relating to the render pipeline for the screen space reflections shader.

[ScreenSpaceReflectionsPipelineId](struct.ScreenSpaceReflectionsPipelineId.html "struct bevy::pbr::ScreenSpaceReflectionsPipelineId")

Identifies which screen space reflections render pipeline a view needs.

[ScreenSpaceReflectionsPipelineKey](struct.ScreenSpaceReflectionsPipelineKey.html "struct bevy::pbr::ScreenSpaceReflectionsPipelineKey")

Identifies a specific configuration of the SSR pipeline shader.

[ScreenSpaceReflectionsPlugin](struct.ScreenSpaceReflectionsPlugin.html "struct bevy::pbr::ScreenSpaceReflectionsPlugin")

Enables screen-space reflections for a camera.

[ScreenSpaceReflectionsUniform](struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform")

A version of [`ScreenSpaceReflections`](struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections") for upload to the GPU.

[ScreenSpaceTransmission](struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

Configures transmission behavior, offering a trade-off between performance and visual fidelity.

[ScreenSpaceTransmissionPlugin](struct.ScreenSpaceTransmissionPlugin.html "struct bevy::pbr::ScreenSpaceTransmissionPlugin")

Enables screen-space transmission for cameras.

[SetMaterialBindGroup](struct.SetMaterialBindGroup.html "struct bevy::pbr::SetMaterialBindGroup")

Sets the bind group for a given [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material") at the configured `I` index.

[SetMeshBindGroup](struct.SetMeshBindGroup.html "struct bevy::pbr::SetMeshBindGroup")

[SetMeshViewBindGroup](struct.SetMeshViewBindGroup.html "struct bevy::pbr::SetMeshViewBindGroup")

[SetMeshViewBindingArrayBindGroup](struct.SetMeshViewBindingArrayBindGroup.html "struct bevy::pbr::SetMeshViewBindingArrayBindGroup")

[SetMeshViewEmptyBindGroup](struct.SetMeshViewEmptyBindGroup.html "struct bevy::pbr::SetMeshViewEmptyBindGroup")

[SetPrepassEmptyMaterialBindGroup](struct.SetPrepassEmptyMaterialBindGroup.html "struct bevy::pbr::SetPrepassEmptyMaterialBindGroup")

[SetPrepassViewBindGroup](struct.SetPrepassViewBindGroup.html "struct bevy::pbr::SetPrepassViewBindGroup")

[SetPrepassViewEmptyBindGroup](struct.SetPrepassViewEmptyBindGroup.html "struct bevy::pbr::SetPrepassViewEmptyBindGroup")

[Shadow](struct.Shadow.html "struct bevy::pbr::Shadow")

[ShadowBatchSetKey](struct.ShadowBatchSetKey.html "struct bevy::pbr::ShadowBatchSetKey")

Information that must be identical in order to place opaque meshes in the same _batch set_.

[ShadowBinKey](struct.ShadowBinKey.html "struct bevy::pbr::ShadowBinKey")

Data used to bin each object in the shadow map phase.

[ShadowSamplers](struct.ShadowSamplers.html "struct bevy::pbr::ShadowSamplers")

[ShadowView](struct.ShadowView.html "struct bevy::pbr::ShadowView")

[ShadowsDepthOnlyDrawFunction](struct.ShadowsDepthOnlyDrawFunction.html "struct bevy::pbr::ShadowsDepthOnlyDrawFunction")

[ShadowsDrawFunction](struct.ShadowsDrawFunction.html "struct bevy::pbr::ShadowsDrawFunction")

[SkinUniforms](struct.SkinUniforms.html "struct bevy::pbr::SkinUniforms")

The GPU buffers containing joint matrices for all skinned meshes.

[SkipGpuPreprocess](struct.SkipGpuPreprocess.html "struct bevy::pbr::SkipGpuPreprocess")

Stops the `GpuPreprocessNode` attempting to generate the buffer for this view useful to avoid duplicating effort if the bind group is shared between views

[SpecializedMaterialPipelineCache](struct.SpecializedMaterialPipelineCache.html "struct bevy::pbr::SpecializedMaterialPipelineCache")

Stores the [`SpecializedMaterialViewPipelineCache`](struct.SpecializedMaterialViewPipelineCache.html "struct bevy::pbr::SpecializedMaterialViewPipelineCache") for each view.

[SpecializedMaterialViewPipelineCache](struct.SpecializedMaterialViewPipelineCache.html "struct bevy::pbr::SpecializedMaterialViewPipelineCache")

Stores the cached render pipeline ID for each entity in a single view, as well as the last time it was changed.

[SpecializedPrepassMaterialPipelineCache](struct.SpecializedPrepassMaterialPipelineCache.html "struct bevy::pbr::SpecializedPrepassMaterialPipelineCache")

Stores the [`SpecializedPrepassMaterialViewPipelineCache`](struct.SpecializedPrepassMaterialViewPipelineCache.html "struct bevy::pbr::SpecializedPrepassMaterialViewPipelineCache") for each view.

[SpecializedPrepassMaterialViewPipelineCache](struct.SpecializedPrepassMaterialViewPipelineCache.html "struct bevy::pbr::SpecializedPrepassMaterialViewPipelineCache")

Stores the cached render pipeline ID for each entity in a single view, as well as the last time it was changed.

[SpecializedShadowMaterialPipelineCache](struct.SpecializedShadowMaterialPipelineCache.html "struct bevy::pbr::SpecializedShadowMaterialPipelineCache")

[SpecializedShadowMaterialViewPipelineCache](struct.SpecializedShadowMaterialViewPipelineCache.html "struct bevy::pbr::SpecializedShadowMaterialViewPipelineCache")

[SsaoBindGroups](struct.SsaoBindGroups.html "struct bevy::pbr::SsaoBindGroups")

A render world component that stores the bind groups necessary to perform Screen Space Ambient Occlusion.

[SsaoPipelineId](struct.SsaoPipelineId.html "struct bevy::pbr::SsaoPipelineId")

A render world component that holds the cached pipeline id for Ssao.

[StandardMaterial](struct.StandardMaterial.html "struct bevy::pbr::StandardMaterial")

A material with “standard” properties used in PBR lighting. Standard property values with pictures here: [https://google.github.io/filament/notes/material\_properties.html](https://google.github.io/filament/notes/material_properties.html).

[StandardMaterialFlags](struct.StandardMaterialFlags.html "struct bevy::pbr::StandardMaterialFlags")

Bitflags info about the material a shader is currently rendering. This is accessible in the shader in the [`StandardMaterialUniform`](struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform")

[StandardMaterialKey](struct.StandardMaterialKey.html "struct bevy::pbr::StandardMaterialKey")

The pipeline key for `StandardMaterial`, packed into 64 bits.

[StandardMaterialUniform](struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform")

The GPU representation of the uniform data of a [`StandardMaterial`](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial").

[Transmissive3d](struct.Transmissive3d.html "struct bevy::pbr::Transmissive3d")

[ViewClusterBindings](struct.ViewClusterBindings.html "struct bevy::pbr::ViewClusterBindings")

[ViewContactShadowsUniformOffset](struct.ViewContactShadowsUniformOffset.html "struct bevy::pbr::ViewContactShadowsUniformOffset")

A component that stores the offset within the [`ContactShadowsBuffer`](struct.ContactShadowsBuffer.html "struct bevy::pbr::ContactShadowsBuffer") for each view.

[ViewFogUniformOffset](struct.ViewFogUniformOffset.html "struct bevy::pbr::ViewFogUniformOffset")

Inserted on each `Entity` with an `ExtractedView` to keep track of its offset in the `gpu_fogs` `DynamicUniformBuffer` within `FogMeta`

[ViewKeyCache](struct.ViewKeyCache.html "struct bevy::pbr::ViewKeyCache")

This resource caches [`MeshPipelineKey`](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")s for each view with pre-enabled features needed to properly setup the [`MeshViewBindGroup`](struct.MeshViewBindGroup.html "struct bevy::pbr::MeshViewBindGroup") layout in specialized [`MeshPipeline`](struct.MeshPipeline.html "struct bevy::pbr::MeshPipeline")s.

[ViewKeyPrepassCache](struct.ViewKeyPrepassCache.html "struct bevy::pbr::ViewKeyPrepassCache")

[ViewLightEntities](struct.ViewLightEntities.html "struct bevy::pbr::ViewLightEntities")

A component that holds the shadow cascade views for all shadow cascades associated with a camera.

[ViewLightProbesUniformOffset](struct.ViewLightProbesUniformOffset.html "struct bevy::pbr::ViewLightProbesUniformOffset")

A component attached to each camera in the render world that stores the index of the [`LightProbesUniform`](struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform") in the [`LightProbesBuffer`](struct.LightProbesBuffer.html "struct bevy::pbr::LightProbesBuffer").

[ViewLightsUniformOffset](struct.ViewLightsUniformOffset.html "struct bevy::pbr::ViewLightsUniformOffset")

[ViewPhaseBinUnpackingBindGroup](struct.ViewPhaseBinUnpackingBindGroup.html "struct bevy::pbr::ViewPhaseBinUnpackingBindGroup")

The bind group for the `unpack_bins` shader for a single combination of view, phase, and mesh indexed-ness.

[ViewPhaseBinUnpackingBindGroups](struct.ViewPhaseBinUnpackingBindGroups.html "struct bevy::pbr::ViewPhaseBinUnpackingBindGroups")

The bind groups for the `unpack_bins` shader for a single (view, phase) combination.

[ViewScreenSpaceReflectionsUniformOffset](struct.ViewScreenSpaceReflectionsUniformOffset.html "struct bevy::pbr::ViewScreenSpaceReflectionsUniformOffset")

A component that stores the offset within the [`ScreenSpaceReflectionsBuffer`](struct.ScreenSpaceReflectionsBuffer.html "struct bevy::pbr::ScreenSpaceReflectionsBuffer") for each view.

[ViewShadowBindings](struct.ViewShadowBindings.html "struct bevy::pbr::ViewShadowBindings")

[ViewTransmissionTexture](struct.ViewTransmissionTexture.html "struct bevy::pbr::ViewTransmissionTexture")

[VolumetricFogPlugin](struct.VolumetricFogPlugin.html "struct bevy::pbr::VolumetricFogPlugin")

A plugin that implements volumetric fog.

## Enums

[AtmosphereMode](enum.AtmosphereMode.html "enum bevy::pbr::AtmosphereMode")

Selects how the atmosphere is rendered. Choose based on scene scale and volumetric shadow quality, and based on performance needs.

[FogFalloff](enum.FogFalloff.html "enum bevy::pbr::FogFalloff")

Allows switching between different fog falloff modes, and configuring their parameters.

[LightEntity](enum.LightEntity.html "enum bevy::pbr::LightEntity")

[MaterialBindGroupAllocator](enum.MaterialBindGroupAllocator.html "enum bevy::pbr::MaterialBindGroupAllocator")

A resource that places materials into bind groups and tracks their resources.

[MeshBindGroups](enum.MeshBindGroups.html "enum bevy::pbr::MeshBindGroups")

All bind groups for meshes currently loaded.

[MeshMorphBindGroupKey](enum.MeshMorphBindGroupKey.html "enum bevy::pbr::MeshMorphBindGroupKey")

Data related to morph targets that we need in order to look up the bind group for a mesh.

[MeshMorphTargetBindGroups](enum.MeshMorphTargetBindGroups.html "enum bevy::pbr::MeshMorphTargetBindGroups")

Stores bind groups for each mesh with morph targets.

[MorphIndices](enum.MorphIndices.html "enum bevy::pbr::MorphIndices")

Maps each mesh affected by morph targets to the applicable offset within the [`MorphUniforms`](struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms") buffer.

[ParallaxMappingMethod](enum.ParallaxMappingMethod.html "enum bevy::pbr::ParallaxMappingMethod")

The [parallax mapping](https://en.wikipedia.org/wiki/Parallax_mapping) method to use to compute depth based on the material’s [`depth_map`](../prelude/struct.StandardMaterial.html#structfield.depth_map "field bevy::prelude::StandardMaterial::depth_map").

[PhasePreprocessBindGroups](enum.PhasePreprocessBindGroups.html "enum bevy::pbr::PhasePreprocessBindGroups")

The compute shader bind group for the mesh preprocessing step for a single render phase on a single view.

[RenderMeshInstanceGpuQueue](enum.RenderMeshInstanceGpuQueue.html "enum bevy::pbr::RenderMeshInstanceGpuQueue")

The per-thread queues used during [`extract_meshes_for_gpu_building`](fn.extract_meshes_for_gpu_building.html "fn bevy::pbr::extract_meshes_for_gpu_building").

[RenderMeshInstances](enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances")

Information that the render world keeps about each entity that contains a mesh.

[ScreenSpaceAmbientOcclusionQualityLevel](enum.ScreenSpaceAmbientOcclusionQualityLevel.html "enum bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel")

[ScreenSpaceTransmissionQuality](enum.ScreenSpaceTransmissionQuality.html "enum bevy::pbr::ScreenSpaceTransmissionQuality")

The quality of the screen space transmission blur effect, applied to whatever’s behind transmissive objects when their `roughness` is greater than `0.0`.

## Constants

[CLUSTERED\_FORWARD\_STORAGE\_BUFFER\_COUNT](constant.CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT.html "constant bevy::pbr::CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT")

[EARLY\_SHADOW\_PASS](constant.EARLY_SHADOW_PASS.html "constant bevy::pbr::EARLY_SHADOW_PASS")

[GPU\_CLUSTERING\_INITIAL\_INDEX\_LIST\_CAPACITY](constant.GPU_CLUSTERING_INITIAL_INDEX_LIST_CAPACITY.html "constant bevy::pbr::GPU_CLUSTERING_INITIAL_INDEX_LIST_CAPACITY")

The initial capacity of the clustered object index list.

[GPU\_CLUSTERING\_INITIAL\_Z\_SLICE\_LIST\_CAPACITY](constant.GPU_CLUSTERING_INITIAL_Z_SLICE_LIST_CAPACITY.html "constant bevy::pbr::GPU_CLUSTERING_INITIAL_Z_SLICE_LIST_CAPACITY")

The initial capacity of the Z slice list.

[LATE\_SHADOW\_PASS](constant.LATE_SHADOW_PASS.html "constant bevy::pbr::LATE_SHADOW_PASS")

[LIGHTMAPS\_PER\_SLAB](constant.LIGHTMAPS_PER_SLAB.html "constant bevy::pbr::LIGHTMAPS_PER_SLAB")

The number of lightmaps that we store in a single slab, if bindless textures are in use.

[MATERIAL\_BIND\_GROUP\_INDEX](constant.MATERIAL_BIND_GROUP_INDEX.html "constant bevy::pbr::MATERIAL_BIND_GROUP_INDEX")

[MAX\_CASCADES\_PER\_LIGHT](constant.MAX_CASCADES_PER_LIGHT.html "constant bevy::pbr::MAX_CASCADES_PER_LIGHT")`webgpu` or non-`webgl` or non-WebAssembly

[MAX\_DIRECTIONAL\_LIGHTS](constant.MAX_DIRECTIONAL_LIGHTS.html "constant bevy::pbr::MAX_DIRECTIONAL_LIGHTS")`webgpu` or non-`webgl` or non-WebAssembly

[MAX\_JOINTS](constant.MAX_JOINTS.html "constant bevy::pbr::MAX_JOINTS")

Maximum number of joints supported for skinned meshes.

[MAX\_RECT\_LIGHTS](constant.MAX_RECT_LIGHTS.html "constant bevy::pbr::MAX_RECT_LIGHTS")

[MAX\_UNIFORM\_BUFFER\_CLUSTERABLE\_OBJECTS](constant.MAX_UNIFORM_BUFFER_CLUSTERABLE_OBJECTS.html "constant bevy::pbr::MAX_UNIFORM_BUFFER_CLUSTERABLE_OBJECTS")

[MAX\_VIEW\_LIGHT\_PROBES](constant.MAX_VIEW_LIGHT_PROBES.html "constant bevy::pbr::MAX_VIEW_LIGHT_PROBES")

The maximum number of each type of light probe that each view will consider.

[MESH\_PIPELINE\_VIEW\_LAYOUT\_SAFE\_MAX\_TEXTURES](constant.MESH_PIPELINE_VIEW_LAYOUT_SAFE_MAX_TEXTURES.html "constant bevy::pbr::MESH_PIPELINE_VIEW_LAYOUT_SAFE_MAX_TEXTURES")Debug-assertions enabled

How many textures are allowed in the view bind group layout (`@group(0)`) before broader compatibility with WebGL and WebGPU is at risk, due to the minimum guaranteed values for `MAX_TEXTURE_IMAGE_UNITS` (in WebGL) and `maxSampledTexturesPerShaderStage` (in WebGPU), currently both at 16.

[TONEMAPPING\_LUT\_SAMPLER\_BINDING\_INDEX](constant.TONEMAPPING_LUT_SAMPLER_BINDING_INDEX.html "constant bevy::pbr::TONEMAPPING_LUT_SAMPLER_BINDING_INDEX")

[TONEMAPPING\_LUT\_TEXTURE\_BINDING\_INDEX](constant.TONEMAPPING_LUT_TEXTURE_BINDING_INDEX.html "constant bevy::pbr::TONEMAPPING_LUT_TEXTURE_BINDING_INDEX")

## Traits

[LightProbeComponent](trait.LightProbeComponent.html "trait bevy::pbr::LightProbeComponent")

A trait implemented by all components that represent light probes.

[Material](trait.Material.html "trait bevy::pbr::Material")

Materials are used alongside [`MaterialPlugin`](../prelude/struct.MaterialPlugin.html "struct bevy::prelude::MaterialPlugin"), [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d"), and [`MeshMaterial3d`](../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d") to spawn entities that are rendered with a specific [`Material`](../prelude/trait.Material.html "trait bevy::prelude::Material") type. They serve as an easy to use high level way to render [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") entities with custom shader logic.

[MaterialExtension](trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension")

A subset of the `Material` trait for defining extensions to a base `Material`, such as the builtin `StandardMaterial`.

## Functions

[alpha\_mode\_pipeline\_key](fn.alpha_mode_pipeline_key.html "fn bevy::pbr::alpha_mode_pipeline_key")

[area\_light\_luts\_placeholder](fn.area_light_luts_placeholder.html "fn bevy::pbr::area_light_luts_placeholder")

[base\_specialize](fn.base_specialize.html "fn bevy::pbr::base_specialize")

[build\_dummy\_white\_gpu\_image](fn.build_dummy_white_gpu_image.html "fn bevy::pbr::build_dummy_white_gpu_image")

A 1x1x1 ‘all 1.0’ texture to use as a dummy texture in place of optional [`crate::pbr_material::StandardMaterial`](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial") textures

[check\_entities\_needing\_specialization](fn.check_entities_needing_specialization.html "fn bevy::pbr::check_entities_needing_specialization")

Finds 3D entities that have changed in such a way as to potentially require specialization and adds them to the [`EntitiesNeedingSpecialization`](struct.EntitiesNeedingSpecialization.html "struct bevy::pbr::EntitiesNeedingSpecialization") list.

[check\_prepass\_views\_need\_specialization](fn.check_prepass_views_need_specialization.html "fn bevy::pbr::check_prepass_views_need_specialization")

[check\_views\_lights\_need\_specialization](fn.check_views_lights_need_specialization.html "fn bevy::pbr::check_views_lights_need_specialization")

[check\_views\_need\_specialization](fn.check_views_need_specialization.html "fn bevy::pbr::check_views_need_specialization")

[clear\_indirect\_parameters\_metadata](fn.clear_indirect_parameters_metadata.html "fn bevy::pbr::clear_indirect_parameters_metadata")

[collect\_gpu\_culled\_meshes](fn.collect_gpu_culled_meshes.html "fn bevy::pbr::collect_gpu_culled_meshes")

Transfers entities from [`RenderGpuCulledEntities`](struct.RenderGpuCulledEntities.html "struct bevy::pbr::RenderGpuCulledEntities") to the [`RenderVisibleEntities`](../render/view/struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities") and [`RenderShadowMapVisibleEntities`](../render/view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities") components on each view.

[collect\_meshes\_for\_gpu\_building](fn.collect_meshes_for_gpu_building.html "fn bevy::pbr::collect_meshes_for_gpu_building")

Creates the [`RenderMeshInstanceGpu`](struct.RenderMeshInstanceGpu.html "struct bevy::pbr::RenderMeshInstanceGpu")s and [`MeshInputUniform`](struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")s when GPU preprocessing is in use.

[early\_gpu\_preprocess](fn.early_gpu_preprocess.html "fn bevy::pbr::early_gpu_preprocess")

[early\_prepass\_build\_indirect\_parameters](fn.early_prepass_build_indirect_parameters.html "fn bevy::pbr::early_prepass_build_indirect_parameters")

[extract\_ambient\_light](fn.extract_ambient_light.html "fn bevy::pbr::extract_ambient_light")

[extract\_ambient\_light\_resource](fn.extract_ambient_light_resource.html "fn bevy::pbr::extract_ambient_light_resource")

[extract\_atmosphere](fn.extract_atmosphere.html "fn bevy::pbr::extract_atmosphere")

For each camera with [`AtmosphereSettings`](struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings"), picks the nearest [`Atmosphere`](../light/struct.Atmosphere.html "struct bevy::light::Atmosphere") by world-space distance to its origin, copies it as [`ExtractedAtmosphere`](struct.ExtractedAtmosphere.html "struct bevy::pbr::ExtractedAtmosphere"), and builds [`GpuAtmosphereSettings`](struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings").

[extract\_camera\_previous\_view\_data](fn.extract_camera_previous_view_data.html "fn bevy::pbr::extract_camera_previous_view_data")

[extract\_clusters\_for\_cpu\_clustering](fn.extract_clusters_for_cpu_clustering.html "fn bevy::pbr::extract_clusters_for_cpu_clustering")

Extracts the clusters that the CPU produced into the render world.

[extract\_entities\_needs\_specialization](fn.extract_entities_needs_specialization.html "fn bevy::pbr::extract_entities_needs_specialization")

[extract\_entities\_that\_need\_specializations\_removed](fn.extract_entities_that_need_specializations_removed.html "fn bevy::pbr::extract_entities_that_need_specializations_removed")

A system that adds entities that were judged to need their specializations removed to the appropriate table in [`DirtySpecializations`](../render/camera/struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations").

[extract\_lights](fn.extract_lights.html "fn bevy::pbr::extract_lights")

[extract\_meshes\_for\_cpu\_building](fn.extract_meshes_for_cpu_building.html "fn bevy::pbr::extract_meshes_for_cpu_building")

Extracts meshes from the main world into the render world, populating the [`RenderMeshInstances`](enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances").

[extract\_meshes\_for\_gpu\_building](fn.extract_meshes_for_gpu_building.html "fn bevy::pbr::extract_meshes_for_gpu_building")

Extracts meshes from the main world to thread-local buffers in the render world.

[extract\_morphs](fn.extract_morphs.html "fn bevy::pbr::extract_morphs")

[extract\_shadow\_filtering\_method](fn.extract_shadow_filtering_method.html "fn bevy::pbr::extract_shadow_filtering_method")

[extract\_shadow\_lod\_origin](fn.extract_shadow_lod_origin.html "fn bevy::pbr::extract_shadow_lod_origin")

An extraction system that determines the origin for LOD computation for point and spot light shadow maps and updates the [`RenderShadowLodOrigin`](../render/view/struct.RenderShadowLodOrigin.html "struct bevy::render::view::RenderShadowLodOrigin") with the result.

[extract\_skins](fn.extract_skins.html "fn bevy::pbr::extract_skins")

[get\_bind\_group\_layout\_entries](fn.get_bind_group_layout_entries.html "fn bevy::pbr::get_bind_group_layout_entries")

[get\_bindings](fn.get_bindings.html "fn bevy::pbr::get_bindings")

[get\_image\_texture](fn.get_image_texture.html "fn bevy::pbr::get_image_texture")

[get\_mesh\_instance\_world\_from\_local](fn.get_mesh_instance_world_from_local.html "fn bevy::pbr::get_mesh_instance_world_from_local")

Returns the world-from-local transform for the given mesh instance.

[gpu\_clustering\_is\_enabled](fn.gpu_clustering_is_enabled.html "fn bevy::pbr::gpu_clustering_is_enabled")

A run condition that tests whether GPU clustering is enabled.

[gpu\_clustering\_is\_enabled\_during\_extraction](fn.gpu_clustering_is_enabled_during_extraction.html "fn bevy::pbr::gpu_clustering_is_enabled_during_extraction")

A run condition that tests whether GPU clustering is enabled.

[init\_fallback\_bindless\_resources](fn.init_fallback_bindless_resources.html "fn bevy::pbr::init_fallback_bindless_resources")

[init\_global\_clusterable\_object\_meta](fn.init_global_clusterable_object_meta.html "fn bevy::pbr::init_global_clusterable_object_meta")

[init\_material\_pipeline](fn.init_material_pipeline.html "fn bevy::pbr::init_material_pipeline")

[init\_mesh\_pipeline\_view\_layouts](fn.init_mesh_pipeline_view_layouts.html "fn bevy::pbr::init_mesh_pipeline_view_layouts")

[init\_prepass\_pipeline](fn.init_prepass_pipeline.html "fn bevy::pbr::init_prepass_pipeline")

[init\_prepass\_view\_bind\_group](fn.init_prepass_view_bind_group.html "fn bevy::pbr::init_prepass_view_bind_group")

[init\_render\_lightmaps](fn.init_render_lightmaps.html "fn bevy::pbr::init_render_lightmaps")

[init\_screen\_space\_reflections\_pipeline](fn.init_screen_space_reflections_pipeline.html "fn bevy::pbr::init_screen_space_reflections_pipeline")

[init\_shadow\_samplers](fn.init_shadow_samplers.html "fn bevy::pbr::init_shadow_samplers")

[late\_gpu\_preprocess](fn.late_gpu_preprocess.html "fn bevy::pbr::late_gpu_preprocess")

[late\_prepass\_build\_indirect\_parameters](fn.late_prepass_build_indirect_parameters.html "fn bevy::pbr::late_prepass_build_indirect_parameters")

[late\_sweep\_material\_instances](fn.late_sweep_material_instances.html "fn bevy::pbr::late_sweep_material_instances")

Removes mesh materials from [`RenderMaterialInstances`](struct.RenderMaterialInstances.html "struct bevy::pbr::RenderMaterialInstances") when their [`ViewVisibility`](../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility") components are removed.

[main\_build\_indirect\_parameters](fn.main_build_indirect_parameters.html "fn bevy::pbr::main_build_indirect_parameters")

Builds indirect parameters for the main opaque and transparent passes.

[main\_transmissive\_pass\_3d](fn.main_transmissive_pass_3d.html "fn bevy::pbr::main_transmissive_pass_3d")

[material\_uses\_bindless\_resources](fn.material_uses_bindless_resources.html "fn bevy::pbr::material_uses_bindless_resources")

Returns true if the material will _actually_ use bindless resources or false if it won’t.

[no\_automatic\_morph\_batching](fn.no_automatic_morph_batching.html "fn bevy::pbr::no_automatic_morph_batching")

[per\_view\_shadow\_pass](fn.per_view_shadow_pass.html "fn bevy::pbr::per_view_shadow_pass")

Renders the shadow maps that are associated with a specific view.

[prepare\_clusters\_for\_cpu\_clustering](fn.prepare_clusters_for_cpu_clustering.html "fn bevy::pbr::prepare_clusters_for_cpu_clustering")

Creates and populates the GPU buffers that store clusters when CPU clustering is being used.

[prepare\_fog](fn.prepare_fog.html "fn bevy::pbr::prepare_fog")

Prepares fog metadata and writes the fog-related uniform buffers to the GPU

[prepare\_lights](fn.prepare_lights.html "fn bevy::pbr::prepare_lights")

[prepare\_material\_bind\_groups](fn.prepare_material_bind_groups.html "fn bevy::pbr::prepare_material_bind_groups")

Creates and/or recreates any bind groups that contain materials that were modified this frame.

[prepare\_mesh\_bind\_groups](fn.prepare_mesh_bind_groups.html "fn bevy::pbr::prepare_mesh_bind_groups")

Creates the per-mesh bind groups for each type of mesh and each phase.

[prepare\_mesh\_view\_bind\_groups](fn.prepare_mesh_view_bind_groups.html "fn bevy::pbr::prepare_mesh_view_bind_groups")

[prepare\_morph\_descriptors](fn.prepare_morph_descriptors.html "fn bevy::pbr::prepare_morph_descriptors")

A system that writes [`GpuMorphDescriptor`](struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor") values to the [`MorphUniforms`](struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms") for each mesh instance with morph targets.

[prepare\_prepass\_view\_bind\_group](fn.prepare_prepass_view_bind_group.html "fn bevy::pbr::prepare_prepass_view_bind_group")

[prepare\_preprocess\_bind\_groups](fn.prepare_preprocess_bind_groups.html "fn bevy::pbr::prepare_preprocess_bind_groups")

A system that attaches buffers to bind groups for the variants of the compute shaders relating to mesh preprocessing.

[prepare\_preprocess\_pipelines](fn.prepare_preprocess_pipelines.html "fn bevy::pbr::prepare_preprocess_pipelines")

A system that specializes the pipelines relating to mesh preprocessing if necessary.

[prepare\_previous\_view\_uniforms](fn.prepare_previous_view_uniforms.html "fn bevy::pbr::prepare_previous_view_uniforms")

[prepare\_skins](fn.prepare_skins.html "fn bevy::pbr::prepare_skins")

Uploads the buffers containing the joints to the GPU.

[prepare\_ssr\_pipelines](fn.prepare_ssr_pipelines.html "fn bevy::pbr::prepare_ssr_pipelines")

Sets up screen space reflection pipelines for each applicable view.

[prepare\_ssr\_settings](fn.prepare_ssr_settings.html "fn bevy::pbr::prepare_ssr_settings")

Gathers up screen space reflection settings for each applicable view and writes them into a GPU buffer.

[queue\_material\_meshes](fn.queue_material_meshes.html "fn bevy::pbr::queue_material_meshes")

For each view, iterates over all the meshes visible from that view and adds them to [`BinnedRenderPhase`](../render/render_phase/struct.BinnedRenderPhase.html "struct bevy::render::render_phase::BinnedRenderPhase")s or [`SortedRenderPhase`](../render/render_phase/struct.SortedRenderPhase.html "struct bevy::render::render_phase::SortedRenderPhase")s as appropriate.

[queue\_prepass\_material\_meshes](fn.queue_prepass_material_meshes.html "fn bevy::pbr::queue_prepass_material_meshes")

[queue\_shadows](fn.queue_shadows.html "fn bevy::pbr::queue_shadows")

For each shadow cascade, iterates over all the meshes “visible” from it and adds them to [`BinnedRenderPhase`](../render/render_phase/struct.BinnedRenderPhase.html "struct bevy::render::render_phase::BinnedRenderPhase")s or [`SortedRenderPhase`](../render/render_phase/struct.SortedRenderPhase.html "struct bevy::render::render_phase::SortedRenderPhase")s as appropriate.

[screen\_space\_reflections](fn.screen_space_reflections.html "fn bevy::pbr::screen_space_reflections")

[set\_mesh\_motion\_vector\_flags](fn.set_mesh_motion_vector_flags.html "fn bevy::pbr::set_mesh_motion_vector_flags")

A system that sets the [`RenderMeshInstanceFlags`](struct.RenderMeshInstanceFlags.html "struct bevy::pbr::RenderMeshInstanceFlags") for each mesh based on whether the previous frame had skins and/or morph targets.

[setup\_morph\_and\_skinning\_defs](fn.setup_morph_and_skinning_defs.html "fn bevy::pbr::setup_morph_and_skinning_defs")

[shared\_shadow\_pass](fn.shared_shadow_pass.html "fn bevy::pbr::shared_shadow_pass")

Renders the shadow maps that aren’t associated with a specific view.

[skins\_use\_uniform\_buffers](fn.skins_use_uniform_buffers.html "fn bevy::pbr::skins_use_uniform_buffers")

Returns true if skinning must use uniforms (and dynamic offsets) because storage buffers aren’t supported on the current platform.

[stbn\_placeholder](fn.stbn_placeholder.html "fn bevy::pbr::stbn_placeholder")

[tonemapping\_pipeline\_key](fn.tonemapping_pipeline_key.html "fn bevy::pbr::tonemapping_pipeline_key")

[unpack\_bins](fn.unpack_bins.html "fn bevy::pbr::unpack_bins")

A rendering system that invokes a compute shader for each batch set in order to generate preprocessing jobs for the subsequent mesh preprocessing shader.

[update\_mesh\_previous\_global\_transforms](fn.update_mesh_previous_global_transforms.html "fn bevy::pbr::update_mesh_previous_global_transforms")

[update\_previous\_view\_data](fn.update_previous_view_data.html "fn bevy::pbr::update_previous_view_data")

[write\_material\_bind\_group\_buffers](fn.write_material_bind_group_buffers.html "fn bevy::pbr::write_material_bind_group_buffers")

Uploads the contents of all buffers that the [`MaterialBindGroupAllocator`](enum.MaterialBindGroupAllocator.html "enum bevy::pbr::MaterialBindGroupAllocator") manages to the GPU.

[write\_mesh\_culling\_data\_buffer](fn.write_mesh_culling_data_buffer.html "fn bevy::pbr::write_mesh_culling_data_buffer")

Writes the information needed to do GPU mesh culling to the GPU.

[write\_morph\_buffers](fn.write_morph_buffers.html "fn bevy::pbr::write_morph_buffers")

A system that writes the buffers inside [`MorphUniforms`](struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms") to the GPU.

## Type Aliases

[DrawDepthOnlyPrepass](type.DrawDepthOnlyPrepass.html "type bevy::pbr::DrawDepthOnlyPrepass")

[DrawMaterial](type.DrawMaterial.html "type bevy::pbr::DrawMaterial")

[DrawPrepass](type.DrawPrepass.html "type bevy::pbr::DrawPrepass")