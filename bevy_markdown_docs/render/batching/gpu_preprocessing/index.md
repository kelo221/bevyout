[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)

# Module gpu\_preprocessing 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#20)

Batching functionality when GPU preprocessing is in use.

## Structs

[BatchedInstanceBuffers](struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers")

The GPU buffers holding the data needed to render batches.

[BatchingPlugin](struct.BatchingPlugin.html "struct bevy::render::batching::gpu_preprocessing::BatchingPlugin")

[BinUnpackingBuffers](struct.BinUnpackingBuffers.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingBuffers")

A resource, part of the render world, that holds all GPU buffers used for the bin unpacking shader.

[BinUnpackingBuffersKey](struct.BinUnpackingBuffersKey.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingBuffersKey")

A key used to look up the bin unpacking buffers for a specific phase of a specific view.

[BinUnpackingJob](struct.BinUnpackingJob.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingJob")

CPU-side information needed to construct the bind groups and issue the dispatch for the `unpack_bins` shader, for a single batch set.

[BinUnpackingMetadataIndex](struct.BinUnpackingMetadataIndex.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingMetadataIndex")

The index of the metadata corresponding to one bin unpacking job in the [`BinUnpackingBuffers::bin_unpacking_metadata`](struct.BinUnpackingBuffers.html#structfield.bin_unpacking_metadata "field bevy::render::batching::gpu_preprocessing::BinUnpackingBuffers::bin_unpacking_metadata") buffer.

[GpuBinUnpackingMetadata](struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata")

GPU-side information needed to unpack bins belonging to a single batch set.

[GpuOcclusionCullingWorkItemBuffers](struct.GpuOcclusionCullingWorkItemBuffers.html "struct bevy::render::batching::gpu_preprocessing::GpuOcclusionCullingWorkItemBuffers")

The work item buffers we use when GPU occlusion culling is in use.

[GpuPreprocessingSupport](struct.GpuPreprocessingSupport.html "struct bevy::render::batching::gpu_preprocessing::GpuPreprocessingSupport")

Records whether GPU preprocessing and/or GPU culling are supported on the device.

[IndirectBatchSet](struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet")

A structure, shared between CPU and GPU, that holds the number of on-GPU indirect draw commands for each _batch set_.

[IndirectParametersBuffers](struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers")

The buffers containing all the information that indirect draw commands (`multi_draw_indirect`, `multi_draw_indirect_count`) use to draw the scene.

[IndirectParametersBuffersSettings](struct.IndirectParametersBuffersSettings.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffersSettings")

Configuration for [`IndirectParametersBuffers`](struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers").

[IndirectParametersCpuMetadata](struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata")

A structure, initialized on CPU and read on GPU, that contains metadata about each batch.

[IndirectParametersGpuMetadata](struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata")

A structure, written and read on GPU, that records how many instances of each mesh are actually to be drawn.

[IndirectParametersIndexed](struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed")

The `wgpu` indirect parameters structure that specifies a GPU draw command.

[IndirectParametersNonIndexed](struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed")

The `wgpu` indirect parameters structure that specifies a GPU draw command.

[InstanceInputUniformBuffer](struct.InstanceInputUniformBuffer.html "struct bevy::render::batching::gpu_preprocessing::InstanceInputUniformBuffer")

Holds the GPU buffer of instance input data, which is the data about each mesh instance that the CPU provides.

[LatePreprocessWorkItemIndirectParameters](struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters")

A GPU-side data structure that stores the number of workgroups to dispatch for the second phase of GPU occlusion culling.

[MeshClassIndirectParametersBuffers](struct.MeshClassIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::MeshClassIndirectParametersBuffers")

The buffers containing all the information that indirect draw commands use to draw the scene, for a single mesh class (indexed or non-indexed), for a single phase.

[PhaseBatchedInstanceBuffers](struct.PhaseBatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseBatchedInstanceBuffers")

The GPU buffers holding the data needed to render batches for a single phase.

[PhaseIndirectParametersBuffers](struct.PhaseIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseIndirectParametersBuffers")

The buffers containing all the information that indirect draw commands use to draw the scene, for a single phase.

[PreprocessWorkItem](struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem")

One invocation of the preprocessing shader: i.e. one mesh instance in a view.

[PreviousInstanceInputUniformBuffer](struct.PreviousInstanceInputUniformBuffer.html "struct bevy::render::batching::gpu_preprocessing::PreviousInstanceInputUniformBuffer")

Stores the input uniforms for the previous frame.

[UntypedPhaseBatchedInstanceBuffers](struct.UntypedPhaseBatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::UntypedPhaseBatchedInstanceBuffers")

The GPU buffers holding the data needed to render batches for a single phase, without a type parameter for that phase.

[UntypedPhaseIndirectParametersBuffers](struct.UntypedPhaseIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::UntypedPhaseIndirectParametersBuffers")

The buffers containing all the information that indirect draw commands use to draw the scene, for a single phase.

[ViewPhaseBinUnpackingBuffers](struct.ViewPhaseBinUnpackingBuffers.html "struct bevy::render::batching::gpu_preprocessing::ViewPhaseBinUnpackingBuffers")

GPU buffers for the bin unpacking shader that are specific to each phase of each view.

## Enums

[GpuPreprocessingMode](enum.GpuPreprocessingMode.html "enum bevy::render::batching::gpu_preprocessing::GpuPreprocessingMode")

The amount of GPU preprocessing (compute and indirect draw) that we do.

[PreprocessWorkItemBuffers](enum.PreprocessWorkItemBuffers.html "enum bevy::render::batching::gpu_preprocessing::PreprocessWorkItemBuffers")

The buffer of GPU preprocessing work items for a single view.

## Functions

[batch\_and\_prepare\_binned\_render\_phase](fn.batch_and_prepare_binned_render_phase.html "fn bevy::render::batching::gpu_preprocessing::batch_and_prepare_binned_render_phase")

Creates batches for a render phase that uses bins.

[batch\_and\_prepare\_sorted\_render\_phase](fn.batch_and_prepare_sorted_render_phase.html "fn bevy::render::batching::gpu_preprocessing::batch_and_prepare_sorted_render_phase")

Batch the items in a sorted render phase, when GPU instance buffer building is in use. This means comparing metadata needed to draw each phase item and trying to combine the draws into a batch.

[clear\_batched\_gpu\_instance\_buffers](fn.clear_batched_gpu_instance_buffers.html "fn bevy::render::batching::gpu_preprocessing::clear_batched_gpu_instance_buffers")

A system that runs early in extraction and clears out all the [`BatchedInstanceBuffers`](struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers") for the frame.

[clear\_bin\_unpacking\_buffers](fn.clear_bin_unpacking_buffers.html "fn bevy::render::batching::gpu_preprocessing::clear_bin_unpacking_buffers")

Clears out the [`BinUnpackingBuffers`](struct.BinUnpackingBuffers.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingBuffers") in preparation for a new frame.

[clear\_indirect\_parameters\_buffers](fn.clear_indirect_parameters_buffers.html "fn bevy::render::batching::gpu_preprocessing::clear_indirect_parameters_buffers")

[collect\_buffers\_for\_phase](fn.collect_buffers_for_phase.html "fn bevy::render::batching::gpu_preprocessing::collect_buffers_for_phase")

A system that gathers up the per-phase GPU buffers and inserts them into the [`BatchedInstanceBuffers`](struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers") and [`IndirectParametersBuffers`](struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers") tables.

[delete\_old\_work\_item\_buffers](fn.delete_old_work_item_buffers.html "fn bevy::render::batching::gpu_preprocessing::delete_old_work_item_buffers")

A system that removes GPU preprocessing work item buffers that correspond to deleted [`ExtractedView`](../../view/struct.ExtractedView.html "struct bevy::render::view::ExtractedView")s.

[get\_or\_create\_work\_item\_buffer](fn.get_or_create_work_item_buffer.html "fn bevy::render::batching::gpu_preprocessing::get_or_create_work_item_buffer")

Returns the set of work item buffers for the given view, first creating it if necessary.

[init\_work\_item\_buffers](fn.init_work_item_buffers.html "fn bevy::render::batching::gpu_preprocessing::init_work_item_buffers")

Initializes work item buffers for a phase in preparation for a new frame.

[write\_batched\_instance\_buffers](fn.write_batched_instance_buffers.html "fn bevy::render::batching::gpu_preprocessing::write_batched_instance_buffers")

A system that writes all instance buffers to the GPU.

[write\_binned\_instance\_buffers](fn.write_binned_instance_buffers.html "fn bevy::render::batching::gpu_preprocessing::write_binned_instance_buffers")

Writes the bin data for each render phase to the GPU.

[write\_indirect\_parameters\_buffers](fn.write_indirect_parameters_buffers.html "fn bevy::render::batching::gpu_preprocessing::write_indirect_parameters_buffers")