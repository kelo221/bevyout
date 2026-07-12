[bevy](../../index.html)::[render](../index.html)

# Module render\_resource 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#58)

## Modules

[encase](encase/index.html "mod bevy::render::render_resource::encase")

## Structs

[AtomicPodUnitBlob](struct.AtomicPodUnitBlob.html "struct bevy::render::render_resource::AtomicPodUnitBlob")

[AtomicRawBufferVec](struct.AtomicRawBufferVec.html "struct bevy::render::render_resource::AtomicRawBufferVec")

A [`RawBufferVec`](struct.RawBufferVec.html "struct bevy::render::render_resource::RawBufferVec") that holds data that implements [`AtomicPod`](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod").

[AtomicSparseBufferVec](struct.AtomicSparseBufferVec.html "struct bevy::render::render_resource::AtomicSparseBufferVec")

A GPU buffer that can grow, can be updated atomically from multiple threads on the CPU, and is sparsely updated on the GPU if only a small number of elements have changed.

[BindGroup](struct.BindGroup.html "struct bevy::render::render_resource::BindGroup")

Bind groups are responsible for binding render resources (e.g. buffers, textures, samplers) to a [`TrackedRenderPass`](../render_phase/struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass"). This makes them accessible in the pipeline (shaders) as uniforms.

[BindGroupDescriptor](struct.BindGroupDescriptor.html "struct bevy::render::render_resource::BindGroupDescriptor")

Describes a group of bindings and the resources to be bound.

[BindGroupEntries](struct.BindGroupEntries.html "struct bevy::render::render_resource::BindGroupEntries")

Helper for constructing bindgroups.

[BindGroupEntry](struct.BindGroupEntry.html "struct bevy::render::render_resource::BindGroupEntry")

An element of a [`BindGroupDescriptor`](struct.BindGroupDescriptor.html "struct bevy::render::render_resource::BindGroupDescriptor"), consisting of a bindable resource and the slot to bind it to.

[BindGroupId](struct.BindGroupId.html "struct bevy::render::render_resource::BindGroupId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[BindGroupLayout](struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout")

Bind group layouts define the interface of resources (e.g. buffers, textures, samplers) for a shader. The actual resource binding is done via a [`BindGroup`](struct.BindGroup.html "struct bevy::render::render_resource::BindGroup").

[BindGroupLayoutDescriptor](struct.BindGroupLayoutDescriptor.html "struct bevy::render::render_resource::BindGroupLayoutDescriptor")

[BindGroupLayoutEntries](struct.BindGroupLayoutEntries.html "struct bevy::render::render_resource::BindGroupLayoutEntries")

[BindGroupLayoutEntry](struct.BindGroupLayoutEntry.html "struct bevy::render::render_resource::BindGroupLayoutEntry")

Describes a single binding inside a bind group.

[BindGroupLayoutEntryBuilder](struct.BindGroupLayoutEntryBuilder.html "struct bevy::render::render_resource::BindGroupLayoutEntryBuilder")

Helper for constructing bind group layouts.

[BindGroupLayoutId](struct.BindGroupLayoutId.html "struct bevy::render::render_resource::BindGroupLayoutId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[BindingNumber](struct.BindingNumber.html "struct bevy::render::render_resource::BindingNumber")

The index of the actual binding in the bind group.

[BindingResources](struct.BindingResources.html "struct bevy::render::render_resource::BindingResources")

A pair of binding index and binding resource, used as part of [`PreparedBindGroup`](struct.PreparedBindGroup.html "struct bevy::render::render_resource::PreparedBindGroup") and [`UnpreparedBindGroup`](struct.UnpreparedBindGroup.html "struct bevy::render::render_resource::UnpreparedBindGroup").

[BindlessBufferDescriptor](struct.BindlessBufferDescriptor.html "struct bevy::render::render_resource::BindlessBufferDescriptor")

Describes a bindless buffer.

[BindlessDescriptor](struct.BindlessDescriptor.html "struct bevy::render::render_resource::BindlessDescriptor")

Information about the bindless resources in this object.

[BindlessIndex](struct.BindlessIndex.html "struct bevy::render::render_resource::BindlessIndex")

The index in the bindless index table.

[BindlessIndexTableDescriptor](struct.BindlessIndexTableDescriptor.html "struct bevy::render::render_resource::BindlessIndexTableDescriptor")

Describes the layout of the bindless index table, which maps bindless indices to indices within the binding arrays.

[Blas](struct.Blas.html "struct bevy::render::render_resource::Blas")

Bottom Level Acceleration Structure (BLAS).

[BlasBuildEntry](struct.BlasBuildEntry.html "struct bevy::render::render_resource::BlasBuildEntry")

Builds the given sets of geometry into the given [Blas](struct.Blas.html "struct bevy::render::render_resource::Blas").

[BlasTriangleGeometry](struct.BlasTriangleGeometry.html "struct bevy::render::render_resource::BlasTriangleGeometry")

Definition for a triangle geometry for a Bottom Level Acceleration Structure (BLAS).

[BlendComponent](struct.BlendComponent.html "struct bevy::render::render_resource::BlendComponent")

Describes a blend component of a [`BlendState`](struct.BlendState.html "struct bevy::render::render_resource::BlendState").

[BlendState](struct.BlendState.html "struct bevy::render::render_resource::BlendState")

Describe the blend state of a render pipeline, within [`ColorTargetState`](struct.ColorTargetState.html "struct bevy::render::render_resource::ColorTargetState").

[Buffer](struct.Buffer.html "struct bevy::render::render_resource::Buffer")

[BufferAsyncError](struct.BufferAsyncError.html "struct bevy::render::render_resource::BufferAsyncError")

Error occurred when trying to async map a buffer.

[BufferBinding](struct.BufferBinding.html "struct bevy::render::render_resource::BufferBinding")

Describes the segment of a buffer to bind.

[BufferId](struct.BufferId.html "struct bevy::render::render_resource::BufferId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[BufferInitDescriptor](struct.BufferInitDescriptor.html "struct bevy::render::render_resource::BufferInitDescriptor")

Describes a [Buffer](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer") when allocating.

[BufferSlice](struct.BufferSlice.html "struct bevy::render::render_resource::BufferSlice")

[BufferUsages](struct.BufferUsages.html "struct bevy::render::render_resource::BufferUsages")

Different ways that you can use a buffer.

[BufferVec](struct.BufferVec.html "struct bevy::render::render_resource::BufferVec")

Like [`RawBufferVec`](struct.RawBufferVec.html "struct bevy::render::render_resource::RawBufferVec"), but doesn’t require that the data type `T` be [`NoUninit`](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/no_uninit/trait.NoUninit.html "trait bytemuck::no_uninit::NoUninit").

[CachedComputePipelineId](struct.CachedComputePipelineId.html "struct bevy::render::render_resource::CachedComputePipelineId")

Index of a cached compute pipeline in a `PipelineCache`.

[CachedPipeline](struct.CachedPipeline.html "struct bevy::render::render_resource::CachedPipeline")

[CachedRenderPipelineId](struct.CachedRenderPipelineId.html "struct bevy::render::render_resource::CachedRenderPipelineId")

Index of a cached render pipeline in a `PipelineCache`.

[ColorTargetState](struct.ColorTargetState.html "struct bevy::render::render_resource::ColorTargetState")

Describes the color state of a render pipeline.

[ColorWrites](struct.ColorWrites.html "struct bevy::render::render_resource::ColorWrites")

Color write mask. Disabled color channels will not be written to.

[CommandEncoder](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder")

Encodes a series of GPU operations.

[ComputePass](struct.ComputePass.html "struct bevy::render::render_resource::ComputePass")

In-progress recording of a compute pass.

[ComputePassDescriptor](struct.ComputePassDescriptor.html "struct bevy::render::render_resource::ComputePassDescriptor")

Describes the attachments of a compute pass.

[ComputePipeline](struct.ComputePipeline.html "struct bevy::render::render_resource::ComputePipeline")

A [`ComputePipeline`](struct.ComputePipeline.html "struct bevy::render::render_resource::ComputePipeline") represents a compute pipeline and its single shader stage.

[ComputePipelineDescriptor](struct.ComputePipelineDescriptor.html "struct bevy::render::render_resource::ComputePipelineDescriptor")

Describes a compute pipeline.

[ComputePipelineId](struct.ComputePipelineId.html "struct bevy::render::render_resource::ComputePipelineId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[DefaultImageSampler](struct.DefaultImageSampler.html "struct bevy::render::render_resource::DefaultImageSampler")

A rendering resource for the default image sampler which is set during renderer initialization.

[DefaultImageSamplerDescriptor](struct.DefaultImageSamplerDescriptor.html "struct bevy::render::render_resource::DefaultImageSamplerDescriptor")

Stores the [`ImageSamplerDescriptor`](../../image/struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor") used to create the [`DefaultImageSampler`](struct.DefaultImageSampler.html "struct bevy::render::render_resource::DefaultImageSampler").

[DepthBiasState](struct.DepthBiasState.html "struct bevy::render::render_resource::DepthBiasState")

Describes the biasing setting for the depth target.

[DepthStencilState](struct.DepthStencilState.html "struct bevy::render::render_resource::DepthStencilState")

Describes the depth/stencil state in a render pipeline.

[DispatchIndirectArgs](struct.DispatchIndirectArgs.html "struct bevy::render::render_resource::DispatchIndirectArgs")

Argument buffer layout for `dispatch_indirect` commands.

[DownlevelFlags](struct.DownlevelFlags.html "struct bevy::render::render_resource::DownlevelFlags")

Binary flags listing features that may or may not be present on downlevel adapters.

[DrawIndexedIndirectArgs](struct.DrawIndexedIndirectArgs.html "struct bevy::render::render_resource::DrawIndexedIndirectArgs")

Argument buffer layout for `draw_indexed_indirect` commands.

[DrawIndirectArgs](struct.DrawIndirectArgs.html "struct bevy::render::render_resource::DrawIndirectArgs")

Argument buffer layout for `draw_indirect` commands.

[DynamicBindGroupEntries](struct.DynamicBindGroupEntries.html "struct bevy::render::render_resource::DynamicBindGroupEntries")

[DynamicBindGroupLayoutEntries](struct.DynamicBindGroupLayoutEntries.html "struct bevy::render::render_resource::DynamicBindGroupLayoutEntries")

[DynamicStorageBuffer](struct.DynamicStorageBuffer.html "struct bevy::render::render_resource::DynamicStorageBuffer")

Stores data to be transferred to the GPU and made accessible to shaders as a dynamic storage buffer.

[DynamicUniformBuffer](struct.DynamicUniformBuffer.html "struct bevy::render::render_resource::DynamicUniformBuffer")

Stores data to be transferred to the GPU and made accessible to shaders as a dynamic uniform buffer.

[DynamicUniformBufferWriter](struct.DynamicUniformBufferWriter.html "struct bevy::render::render_resource::DynamicUniformBufferWriter")

A writer that can be used to directly write elements into the target buffer.

[Extent3d](struct.Extent3d.html "struct bevy::render::render_resource::Extent3d")

Extent of a texture related operation.

[FragmentState](struct.FragmentState.html "struct bevy::render::render_resource::FragmentState")

Describes the fragment process in a render pipeline.

[GpuArrayBufferIndex](struct.GpuArrayBufferIndex.html "struct bevy::render::render_resource::GpuArrayBufferIndex")

An index into a [`GpuArrayBuffer`](enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer") for a given element.

[ImageSubresourceRange](struct.ImageSubresourceRange.html "struct bevy::render::render_resource::ImageSubresourceRange")

Subresource range within an image

[MultisampleState](struct.MultisampleState.html "struct bevy::render::render_resource::MultisampleState")

Describes the multi-sampling state of a render pipeline.

[Operations](struct.Operations.html "struct bevy::render::render_resource::Operations")

Pair of load and store operations for an attachment aspect.

[Origin3d](struct.Origin3d.html "struct bevy::render::render_resource::Origin3d")

Origin of a copy to/from a texture.

[OwnedData](struct.OwnedData.html "struct bevy::render::render_resource::OwnedData")

Data that will be copied into a GPU buffer.

[PartialBufferVec](struct.PartialBufferVec.html "struct bevy::render::render_resource::PartialBufferVec")

A hybrid of [`RawBufferVec`](struct.RawBufferVec.html "struct bevy::render::render_resource::RawBufferVec") and [`UninitBufferVec`](struct.UninitBufferVec.html "struct bevy::render::render_resource::UninitBufferVec") that allows the CPU to push elements and to leave room for uninitialized elements for the GPU to populate at the end of the array.

[PipelineCache](struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")

Cache for render and compute pipelines.

[PipelineCompilationOptions](struct.PipelineCompilationOptions.html "struct bevy::render::render_resource::PipelineCompilationOptions")

Advanced options for use when a pipeline is compiled

[PipelineLayout](struct.PipelineLayout.html "struct bevy::render::render_resource::PipelineLayout")

Handle to a pipeline layout.

[PipelineLayoutDescriptor](struct.PipelineLayoutDescriptor.html "struct bevy::render::render_resource::PipelineLayoutDescriptor")

Describes a [`PipelineLayout`](struct.PipelineLayout.html "struct bevy::render::render_resource::PipelineLayout").

[PreparedBindGroup](struct.PreparedBindGroup.html "struct bevy::render::render_resource::PreparedBindGroup")

A prepared bind group returned as a result of [`AsBindGroup::as_bind_group`](trait.AsBindGroup.html#method.as_bind_group "method bevy::render::render_resource::AsBindGroup::as_bind_group").

[PrimitiveState](struct.PrimitiveState.html "struct bevy::render::render_resource::PrimitiveState")

Describes the state of primitive assembly and rasterization in a render pipeline.

[RawBufferVec](struct.RawBufferVec.html "struct bevy::render::render_resource::RawBufferVec")

A structure for storing raw bytes that have already been properly formatted for use by the GPU.

[RawComputePipelineDescriptor](struct.RawComputePipelineDescriptor.html "struct bevy::render::render_resource::RawComputePipelineDescriptor")

Describes a compute pipeline.

[RawFragmentState](struct.RawFragmentState.html "struct bevy::render::render_resource::RawFragmentState")

Describes the fragment processing in a render pipeline.

[RawRenderPipelineDescriptor](struct.RawRenderPipelineDescriptor.html "struct bevy::render::render_resource::RawRenderPipelineDescriptor")

Describes a render (graphics) pipeline.

[RawVertexBufferLayout](struct.RawVertexBufferLayout.html "struct bevy::render::render_resource::RawVertexBufferLayout")

Specifies an interpretation of the bytes of a vertex buffer as vertex attributes.

[RawVertexState](struct.RawVertexState.html "struct bevy::render::render_resource::RawVertexState")

Describes the vertex processing in a render pipeline.

[RenderPassColorAttachment](struct.RenderPassColorAttachment.html "struct bevy::render::render_resource::RenderPassColorAttachment")

Describes a color attachment to a [`RenderPass`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html "struct wgpu::api::render_pass::RenderPass").

[RenderPassDepthStencilAttachment](struct.RenderPassDepthStencilAttachment.html "struct bevy::render::render_resource::RenderPassDepthStencilAttachment")

Describes a depth/stencil attachment to a [`RenderPass`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/render_pass/struct.RenderPass.html "struct wgpu::api::render_pass::RenderPass").

[RenderPassDescriptor](struct.RenderPassDescriptor.html "struct bevy::render::render_resource::RenderPassDescriptor")

Describes the attachments of a render pass.

[RenderPipeline](struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline")

A [`RenderPipeline`](struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline") represents a graphics pipeline and its stages (shaders), bindings and vertex buffers.

[RenderPipelineDescriptor](struct.RenderPipelineDescriptor.html "struct bevy::render::render_resource::RenderPipelineDescriptor")

Describes a render (graphics) pipeline.

[RenderPipelineId](struct.RenderPipelineId.html "struct bevy::render::render_resource::RenderPipelineId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[Sampler](struct.Sampler.html "struct bevy::render::render_resource::Sampler")

A Sampler defines how a pipeline will sample from a [`TextureView`](struct.TextureView.html "struct bevy::render::render_resource::TextureView"). They define image filters (including anisotropy) and address (wrapping) modes, among other things.

[SamplerId](struct.SamplerId.html "struct bevy::render::render_resource::SamplerId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[ShaderModule](struct.ShaderModule.html "struct bevy::render::render_resource::ShaderModule")

Handle to a compiled shader module.

[ShaderModuleDescriptor](struct.ShaderModuleDescriptor.html "struct bevy::render::render_resource::ShaderModuleDescriptor")

Descriptor for use with [`Device::create_shader_module`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.create_shader_module "method wgpu::api::device::Device::create_shader_module").

[ShaderStages](struct.ShaderStages.html "struct bevy::render::render_resource::ShaderStages")

Describes the shader stages that a binding will be visible from.

[SparseBufferId](struct.SparseBufferId.html "struct bevy::render::render_resource::SparseBufferId")

A globally-unique ID that identifies this sparse buffer.

[SparseBufferPlugin](struct.SparseBufferPlugin.html "struct bevy::render::render_resource::SparseBufferPlugin")

A plugin that allows sparse updates of GPU buffers if only a small number of elements have changed.

[SparseBufferUpdateBindGroup](struct.SparseBufferUpdateBindGroup.html "struct bevy::render::render_resource::SparseBufferUpdateBindGroup")

A single bind group for the sparse buffer update shader.

[SparseBufferUpdateBindGroups](struct.SparseBufferUpdateBindGroups.html "struct bevy::render::render_resource::SparseBufferUpdateBindGroups")

A resource, part of the render world, that stores the bind groups for each sparse buffer.

[SparseBufferUpdateJob](struct.SparseBufferUpdateJob.html "struct bevy::render::render_resource::SparseBufferUpdateJob")

Describes a sparse update operation for a buffer.

[SparseBufferUpdateJobs](struct.SparseBufferUpdateJobs.html "struct bevy::render::render_resource::SparseBufferUpdateJobs")

A resource, part of the render world, that stores all pending sparse updates to buffers.

[SparseBufferUpdatePipelines](struct.SparseBufferUpdatePipelines.html "struct bevy::render::render_resource::SparseBufferUpdatePipelines")

Pipelines for the sparse buffer update shader.

[SpecializedComputePipelines](struct.SpecializedComputePipelines.html "struct bevy::render::render_resource::SpecializedComputePipelines")

A convenience cache for creating different variants of a compute pipeline based on some key.

[SpecializedMeshPipelines](struct.SpecializedMeshPipelines.html "struct bevy::render::render_resource::SpecializedMeshPipelines")

A cache of different variants of a render pipeline based on a key and the particular mesh’s vertex buffer layout.

[SpecializedRenderPipelines](struct.SpecializedRenderPipelines.html "struct bevy::render::render_resource::SpecializedRenderPipelines")

A convenience cache for creating different variants of a render pipeline based on some key.

[StencilFaceState](struct.StencilFaceState.html "struct bevy::render::render_resource::StencilFaceState")

Describes stencil state in a render pipeline.

[StencilState](struct.StencilState.html "struct bevy::render::render_resource::StencilState")

State of the stencil operation (fixed-pipeline stage).

[StorageBuffer](struct.StorageBuffer.html "struct bevy::render::render_resource::StorageBuffer")

Stores data to be transferred to the GPU and made accessible to shaders as a storage buffer.

[SurfaceTexture](struct.SurfaceTexture.html "struct bevy::render::render_resource::SurfaceTexture")

[TexelCopyBufferLayout](struct.TexelCopyBufferLayout.html "struct bevy::render::render_resource::TexelCopyBufferLayout")

Layout of a texture in a buffer’s memory.

[Texture](struct.Texture.html "struct bevy::render::render_resource::Texture")

A GPU-accessible texture.

[TextureFormatFeatureFlags](struct.TextureFormatFeatureFlags.html "struct bevy::render::render_resource::TextureFormatFeatureFlags")

Feature flags for a texture format.

[TextureFormatFeatures](struct.TextureFormatFeatures.html "struct bevy::render::render_resource::TextureFormatFeatures")

Features supported by a given texture format

[TextureId](struct.TextureId.html "struct bevy::render::render_resource::TextureId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[TextureUsages](struct.TextureUsages.html "struct bevy::render::render_resource::TextureUsages")

Different ways that you can use a texture.

[TextureView](struct.TextureView.html "struct bevy::render::render_resource::TextureView")

Describes a [`Texture`](struct.Texture.html "struct bevy::render::render_resource::Texture") with its associated metadata required by a pipeline or [`BindGroup`](struct.BindGroup.html "struct bevy::render::render_resource::BindGroup").

[TextureViewId](struct.TextureViewId.html "struct bevy::render::render_resource::TextureViewId")

Globally unique 32-bit id, guaranteed via atomics on a static global.

[Tlas](struct.Tlas.html "struct bevy::render::render_resource::Tlas")

Top Level Acceleration Structure (TLAS).

[TlasInstance](struct.TlasInstance.html "struct bevy::render::render_resource::TlasInstance")

Safe instance for a [Tlas](struct.Tlas.html "struct bevy::render::render_resource::Tlas").

[UniformBuffer](struct.UniformBuffer.html "struct bevy::render::render_resource::UniformBuffer")

Stores data to be transferred to the GPU and made accessible to shaders as a uniform buffer.

[UninitBufferVec](struct.UninitBufferVec.html "struct bevy::render::render_resource::UninitBufferVec")

Like a [`BufferVec`](struct.BufferVec.html "struct bevy::render::render_resource::BufferVec"), but only reserves space on the GPU for elements instead of initializing them CPU-side.

[UnpreparedBindGroup](struct.UnpreparedBindGroup.html "struct bevy::render::render_resource::UnpreparedBindGroup")

a map containing `OwnedBindingResource`s, keyed by the target binding index

[Variants](struct.Variants.html "struct bevy::render::render_resource::Variants")

A cache for variants of a resource type created by a specializer. At most one resource will be created for each key.

[VertexAttribute](struct.VertexAttribute.html "struct bevy::render::render_resource::VertexAttribute")

Vertex inputs (attributes) to shaders.

[VertexState](struct.VertexState.html "struct bevy::render::render_resource::VertexState")

[WgpuAdapterInfo](struct.WgpuAdapterInfo.html "struct bevy::render::render_resource::WgpuAdapterInfo")

Information about an adapter.

[WgpuFeatures](struct.WgpuFeatures.html "struct bevy::render::render_resource::WgpuFeatures")

Features that are not guaranteed to be supported.

[WgpuLimits](struct.WgpuLimits.html "struct bevy::render::render_resource::WgpuLimits")

Represents the sets of limits an adapter/device supports.

[WgpuSampler](struct.WgpuSampler.html "struct bevy::render::render_resource::WgpuSampler")

Handle to a sampler.

[WgpuTextureView](struct.WgpuTextureView.html "struct bevy::render::render_resource::WgpuTextureView")

Handle to a texture view.

## Enums

[AddressMode](enum.AddressMode.html "enum bevy::render::render_resource::AddressMode")

How edges should be handled in texture addressing.

[AsBindGroupError](enum.AsBindGroupError.html "enum bevy::render::render_resource::AsBindGroupError")

An error that occurs during [`AsBindGroup::as_bind_group`](trait.AsBindGroup.html#method.as_bind_group "method bevy::render::render_resource::AsBindGroup::as_bind_group") calls.

[AstcBlock](enum.AstcBlock.html "enum bevy::render::render_resource::AstcBlock")

ASTC block dimensions

[AstcChannel](enum.AstcChannel.html "enum bevy::render::render_resource::AstcChannel")

ASTC RGBA channel

[BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")

Resource to be bound by a [`BindGroup`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/bind_group/struct.BindGroup.html "struct wgpu::api::bind_group::BindGroup") for use with a pipeline.

[BindingType](enum.BindingType.html "enum bevy::render::render_resource::BindingType")

Type of a binding in a [bind group layout](struct.BindGroupLayoutEntry.html "struct bevy::render::render_resource::BindGroupLayoutEntry").

[BindlessResourceType](enum.BindlessResourceType.html "enum bevy::render::render_resource::BindlessResourceType")

The type of potentially-bindless resource.

[BindlessSlabResourceLimit](enum.BindlessSlabResourceLimit.html "enum bevy::render::render_resource::BindlessSlabResourceLimit")

The maximum number of resources that can be stored in a slab.

[BlasGeometries](enum.BlasGeometries.html "enum bevy::render::render_resource::BlasGeometries")

Contains the sets of geometry that go into a [Blas](struct.Blas.html "struct bevy::render::render_resource::Blas").

[BlendFactor](enum.BlendFactor.html "enum bevy::render::render_resource::BlendFactor")

Alpha blend factor.

[BlendOperation](enum.BlendOperation.html "enum bevy::render::render_resource::BlendOperation")

Alpha blend operation.

[BufferBindingType](enum.BufferBindingType.html "enum bevy::render::render_resource::BufferBindingType")

Specific type of a buffer binding.

[CachedPipelineState](enum.CachedPipelineState.html "enum bevy::render::render_resource::CachedPipelineState")

State of a cached pipeline inserted into a [`PipelineCache`](struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache").

[CompareFunction](enum.CompareFunction.html "enum bevy::render::render_resource::CompareFunction")

Comparison function used for depth and stencil operations.

[Face](enum.Face.html "enum bevy::render::render_resource::Face")

Face of a vertex.

[FilterMode](enum.FilterMode.html "enum bevy::render::render_resource::FilterMode")

Texel mixing mode when sampling between texels.

[FrontFace](enum.FrontFace.html "enum bevy::render::render_resource::FrontFace")

Vertex winding order which classifies the “front” face of a triangle.

[GpuArrayBuffer](enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer")

Stores an array of elements to be transferred to the GPU and made accessible to shaders as a read-only array.

[IndexFormat](enum.IndexFormat.html "enum bevy::render::render_resource::IndexFormat")

Format of indices used with pipeline.

[LoadOp](enum.LoadOp.html "enum bevy::render::render_resource::LoadOp")

Operation to perform to the output attachment at the start of a render pass.

[MapMode](enum.MapMode.html "enum bevy::render::render_resource::MapMode")

Type of buffer mapping.

[MipmapFilterMode](enum.MipmapFilterMode.html "enum bevy::render::render_resource::MipmapFilterMode")

Texel mixing mode when sampling between texels.

[OwnedBindingResource](enum.OwnedBindingResource.html "enum bevy::render::render_resource::OwnedBindingResource")

An owned binding resource of any type (ex: a [`Buffer`](struct.Buffer.html "struct bevy::render::render_resource::Buffer"), [`TextureView`](struct.TextureView.html "struct bevy::render::render_resource::TextureView"), etc). This is used by types like [`PreparedBindGroup`](struct.PreparedBindGroup.html "struct bevy::render::render_resource::PreparedBindGroup") to hold a single list of all render resources used by bindings.

[Pipeline](enum.Pipeline.html "enum bevy::render::render_resource::Pipeline")

A pipeline defining the data layout and shader logic for a specific GPU task.

[PipelineDescriptor](enum.PipelineDescriptor.html "enum bevy::render::render_resource::PipelineDescriptor")

A descriptor for a [`Pipeline`](https://docs.rs/bevy/latest/bevy/render/render_resource/enum.Pipeline.html).

[PolygonMode](enum.PolygonMode.html "enum bevy::render::render_resource::PolygonMode")

Type of drawing mode for polygons

[PrimitiveTopology](enum.PrimitiveTopology.html "enum bevy::render::render_resource::PrimitiveTopology")

Primitive type the input mesh is composed of.

[SamplerBindingType](enum.SamplerBindingType.html "enum bevy::render::render_resource::SamplerBindingType")

Specific type of a sampler binding.

[ShaderSource](enum.ShaderSource.html "enum bevy::render::render_resource::ShaderSource")

Source of a shader module.

[ShaderStage](enum.ShaderStage.html "enum bevy::render::render_resource::ShaderStage")

Stage of the programmable pipeline.

[SpecializedMeshPipelineError](enum.SpecializedMeshPipelineError.html "enum bevy::render::render_resource::SpecializedMeshPipelineError")

[StencilOperation](enum.StencilOperation.html "enum bevy::render::render_resource::StencilOperation")

Operation to perform on the stencil value.

[StorageTextureAccess](enum.StorageTextureAccess.html "enum bevy::render::render_resource::StorageTextureAccess")

Specific type of a sample in a texture binding.

[StoreOp](enum.StoreOp.html "enum bevy::render::render_resource::StoreOp")

Operation to perform to the output attachment at the end of a render pass.

[TextureAspect](enum.TextureAspect.html "enum bevy::render::render_resource::TextureAspect")

Selects a subset of the data a [`Texture`](../wgpu/struct.Texture.html) holds.

[TextureDataOrder](enum.TextureDataOrder.html "enum bevy::render::render_resource::TextureDataOrder")

Order in which texture data is laid out in memory.

[TextureDimension](enum.TextureDimension.html "enum bevy::render::render_resource::TextureDimension")

Dimensionality of a texture.

[TextureFormat](enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")

Format in which a texture’s texels are stored in GPU memory.

[TextureSampleType](enum.TextureSampleType.html "enum bevy::render::render_resource::TextureSampleType")

Specific type of a sample in a texture binding.

[TextureViewDimension](enum.TextureViewDimension.html "enum bevy::render::render_resource::TextureViewDimension")

Dimensions of a particular texture view.

[VertexFormat](enum.VertexFormat.html "enum bevy::render::render_resource::VertexFormat")

Vertex Format for a [`VertexAttribute`](struct.VertexAttribute.html "struct bevy::render::render_resource::VertexAttribute") (input).

[VertexStepMode](enum.VertexStepMode.html "enum bevy::render::render_resource::VertexStepMode")

Whether a vertex buffer is indexed by vertex or by instance.

[WriteBufferRangeError](enum.WriteBufferRangeError.html "enum bevy::render::render_resource::WriteBufferRangeError")

Error returned when `write_buffer_range` fails

## Constants

[AUTO\_BINDLESS\_SLAB\_RESOURCE\_LIMIT](constant.AUTO_BINDLESS_SLAB_RESOURCE_LIMIT.html "constant bevy::render::render_resource::AUTO_BINDLESS_SLAB_RESOURCE_LIMIT")Neither iOS nor macOS

The default value for the number of resources that can be stored in a slab on this platform.

[COPY\_BUFFER\_ALIGNMENT](constant.COPY_BUFFER_ALIGNMENT.html "constant bevy::render::render_resource::COPY_BUFFER_ALIGNMENT")

Buffer to buffer copy as well as buffer clear offsets and sizes must be aligned to this number.

## Statics

[BINDING\_NUMBERS](static.BINDING_NUMBERS.html "static bevy::render::render_resource::BINDING_NUMBERS")

The binding numbers for the built-in binding arrays of each bindless resource type.

## Traits

[AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")

Converts a value to a [`BindGroup`](struct.BindGroup.html "struct bevy::render::render_resource::BindGroup") with a given [`BindGroupLayout`](struct.BindGroupLayout.html "struct bevy::render::render_resource::BindGroupLayout"), which can then be used in Bevy shaders. This trait can be derived (and generally should be). Read on for details and examples.

[AsBindGroupShaderType](trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")

Converts a value to a [`ShaderType`](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for use in a bind group.

[AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod")

Data that can be converted to an array of [`std::sync::atomic::AtomicU32`](../../platform/sync/atomic/type.AtomicU32.html "type bevy::platform::sync::atomic::AtomicU32") values.

[AtomicPodBlob](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob")

Describes a type that has the same bit pattern as another type, but is made up entirely of an array of [`std::sync::atomic::AtomicU32`](../../platform/sync/atomic/type.AtomicU32.html "type bevy::platform::sync::atomic::AtomicU32") values.

[GpuArrayBufferable](trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable")

Trait for types able to go in a [`GpuArrayBuffer`](enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer").

[IntoBindGroupLayoutEntryBuilder](trait.IntoBindGroupLayoutEntryBuilder.html "trait bevy::render::render_resource::IntoBindGroupLayoutEntryBuilder")

[IntoBindGroupLayoutEntryBuilderArray](trait.IntoBindGroupLayoutEntryBuilderArray.html "trait bevy::render::render_resource::IntoBindGroupLayoutEntryBuilderArray")

[IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")

[IntoBindingArray](trait.IntoBindingArray.html "trait bevy::render::render_resource::IntoBindingArray")

[IntoIndexedBindGroupLayoutEntryBuilderArray](trait.IntoIndexedBindGroupLayoutEntryBuilderArray.html "trait bevy::render::render_resource::IntoIndexedBindGroupLayoutEntryBuilderArray")

[IntoIndexedBindingArray](trait.IntoIndexedBindingArray.html "trait bevy::render::render_resource::IntoIndexedBindingArray")

[ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize")

Trait implemented for all [WGSL fixed-footprint types](https://gpuweb.github.io/gpuweb/wgsl/#fixed-footprint-types)

[ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")

Base trait for all [WGSL host-shareable types](https://gpuweb.github.io/gpuweb/wgsl/#host-shareable-types)

[Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable")

Defines a type that is able to be “specialized” and cached by creating and transforming its descriptor type. This is implemented for [`RenderPipeline`](struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline") and [`ComputePipeline`](struct.ComputePipeline.html "struct bevy::render::render_resource::ComputePipeline"), and likely will not have much utility for other types.

[SpecializedComputePipeline](trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline")

A trait that allows constructing different variants of a compute pipeline from a key.

[SpecializedMeshPipeline](trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline")

A trait that allows constructing different variants of a render pipeline from a key and the particular mesh’s vertex buffer layout.

[SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline")

A trait that allows constructing different variants of a render pipeline from a key.

[Specializer](trait.Specializer.html "trait bevy::render::render_resource::Specializer")

Defines a type capable of “specializing” values of a type T.

[SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey")

Defines a type that is able to be used as a key for [`Specializer`](trait.Specializer.html "trait bevy::render::render_resource::Specializer")s

## Functions

[create\_bindless\_bind\_group\_layout\_entries](fn.create_bindless_bind_group_layout_entries.html "fn bevy::render::render_resource::create_bindless_bind_group_layout_entries")

Creates the bind group layout entries common to all shaders that use bindless bind groups.

## Type Aliases

[AccelerationStructureFlags](type.AccelerationStructureFlags.html "type bevy::render::render_resource::AccelerationStructureFlags")

Flags for an acceleration structure.

[AccelerationStructureGeometryFlags](type.AccelerationStructureGeometryFlags.html "type bevy::render::render_resource::AccelerationStructureGeometryFlags")

Flags for a geometry inside a bottom level acceleration structure.

[AccelerationStructureUpdateMode](type.AccelerationStructureUpdateMode.html "type bevy::render::render_resource::AccelerationStructureUpdateMode")

Update mode for acceleration structure builds.

[BlasGeometrySizeDescriptors](type.BlasGeometrySizeDescriptors.html "type bevy::render::render_resource::BlasGeometrySizeDescriptors")

Descriptor for the size defining attributes, for a bottom level acceleration structure.

[BlasTriangleGeometrySizeDescriptor](type.BlasTriangleGeometrySizeDescriptor.html "type bevy::render::render_resource::BlasTriangleGeometrySizeDescriptor")

Descriptor for the size defining attributes of a triangle geometry, for a bottom level acceleration structure.

[BufferAddress](type.BufferAddress.html "type bevy::render::render_resource::BufferAddress")

Integral type used for [`Buffer`](../wgpu/struct.Buffer.html) offsets and sizes.

[BufferDescriptor](type.BufferDescriptor.html "type bevy::render::render_resource::BufferDescriptor")

Describes a [`Buffer`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.Buffer.html "struct wgpu::api::buffer::Buffer").

[BufferSize](type.BufferSize.html "type bevy::render::render_resource::BufferSize")

Integral type used for [`BufferSlice`](../wgpu/struct.BufferSlice.html) sizes.

[Canonical](type.Canonical.html "type bevy::render::render_resource::Canonical")

[CommandEncoderDescriptor](type.CommandEncoderDescriptor.html "type bevy::render::render_resource::CommandEncoderDescriptor")

Describes a [`CommandEncoder`](struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder").

[CreateBlasDescriptor](type.CreateBlasDescriptor.html "type bevy::render::render_resource::CreateBlasDescriptor")

Descriptor to create bottom level acceleration structures.

[CreateTlasDescriptor](type.CreateTlasDescriptor.html "type bevy::render::render_resource::CreateTlasDescriptor")

Descriptor to create top level acceleration structures.

[PollType](type.PollType.html "type bevy::render::render_resource::PollType")

Passed to [`Device::poll`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.poll "method wgpu::api::device::Device::poll") to control how and if it should block.

[SamplerDescriptor](type.SamplerDescriptor.html "type bevy::render::render_resource::SamplerDescriptor")

Describes a [`Sampler`](struct.WgpuSampler.html "struct bevy::render::render_resource::WgpuSampler").

[SparseBufferHandle](type.SparseBufferHandle.html "type bevy::render::render_resource::SparseBufferHandle")

An object that allows the sparse buffer ID to be query and holds the bind group for that sparse buffer alive.

[TexelCopyBufferInfo](type.TexelCopyBufferInfo.html "type bevy::render::render_resource::TexelCopyBufferInfo")

View of a buffer which can be used to copy to/from a texture.

[TexelCopyTextureInfo](type.TexelCopyTextureInfo.html "type bevy::render::render_resource::TexelCopyTextureInfo")

View of a texture which can be used to copy to/from a buffer/texture.

[TextureDescriptor](type.TextureDescriptor.html "type bevy::render::render_resource::TextureDescriptor")

Describes a [`Texture`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/texture/struct.Texture.html "struct wgpu::api::texture::Texture").

[TextureViewDescriptor](type.TextureViewDescriptor.html "type bevy::render::render_resource::TextureViewDescriptor")

Describes a [`TextureView`](struct.WgpuTextureView.html "struct bevy::render::render_resource::WgpuTextureView").

## Derive Macros

[AsBindGroup](derive.AsBindGroup.html "derive bevy::render::render_resource::AsBindGroup")

[ShaderType](derive.ShaderType.html "derive bevy::render::render_resource::ShaderType")

[Specializer](derive.Specializer.html "derive bevy::render::render_resource::Specializer")

Derive macro generating an impl of the trait `Specializer`

[SpecializerKey](derive.SpecializerKey.html "derive bevy::render::render_resource::SpecializerKey")

Derive macro generating the most common impl of the trait `SpecializerKey`