[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function collect\_buffers\_for\_phase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#2285-2295)

```rust
pub fn collect_buffers_for_phase<PI, GFBD>(
    phase_batched_instance_buffers: ResMut<'_, PhaseBatchedInstanceBuffers<PI, <GFBD as GetBatchData>::BufferData>>,
    phase_indirect_parameters_buffers: ResMut<'_, PhaseIndirectParametersBuffers<PI>>,
    batched_instance_buffers: ResMut<'_, BatchedInstanceBuffers<<GFBD as GetBatchData>::BufferData, <GFBD as GetFullBatchData>::BufferInputData>>,
    indirect_parameters_buffers: ResMut<'_, IndirectParametersBuffers>,
    indirect_parameters_buffers_settings: Res<'_, IndirectParametersBuffersSettings>,
)where
    PI: PhaseItem,
    GFBD: GetFullBatchData + Send + Sync + 'static,
```

A system that gathers up the per-phase GPU buffers and inserts them into the [`BatchedInstanceBuffers`](struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers") and [`IndirectParametersBuffers`](struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers") tables.

This runs after the [`batch_and_prepare_binned_render_phase`](fn.batch_and_prepare_binned_render_phase.html "fn bevy::render::batching::gpu_preprocessing::batch_and_prepare_binned_render_phase") or [`batch_and_prepare_sorted_render_phase`](fn.batch_and_prepare_sorted_render_phase.html "fn bevy::render::batching::gpu_preprocessing::batch_and_prepare_sorted_render_phase") systems. It takes the per-phase [`PhaseBatchedInstanceBuffers`](struct.PhaseBatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseBatchedInstanceBuffers") and [`PhaseIndirectParametersBuffers`](struct.PhaseIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseIndirectParametersBuffers") resources and inserts them into the global [`BatchedInstanceBuffers`](struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers") and [`IndirectParametersBuffers`](struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers") tables.

This system exists so that the [`batch_and_prepare_binned_render_phase`](fn.batch_and_prepare_binned_render_phase.html "fn bevy::render::batching::gpu_preprocessing::batch_and_prepare_binned_render_phase") and [`batch_and_prepare_sorted_render_phase`](fn.batch_and_prepare_sorted_render_phase.html "fn bevy::render::batching::gpu_preprocessing::batch_and_prepare_sorted_render_phase") can run in parallel with one another. If those two systems manipulated [`BatchedInstanceBuffers`](struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers") and [`IndirectParametersBuffers`](struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers") directly, then they wouldn’t be able to run in parallel.