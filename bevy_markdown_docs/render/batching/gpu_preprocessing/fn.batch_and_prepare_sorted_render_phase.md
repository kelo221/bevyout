[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function batch\_and\_prepare\_sorted\_render\_phase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#1568-1580)

```rust
pub fn batch_and_prepare_sorted_render_phase<I, GFBD>(
    phase_batched_instance_buffers: ResMut<'_, PhaseBatchedInstanceBuffers<I, <GFBD as GetBatchData>::BufferData>>,
    phase_indirect_parameters_buffers: ResMut<'_, PhaseIndirectParametersBuffers<I>>,
    sorted_render_phases: ResMut<'_, ViewSortedRenderPhases<I>>,
    views: Query<'_, '_, (&ExtractedView, Has<NoIndirectDrawing>, Has<OcclusionCulling>)>,
    system_param_item: StaticSystemParam<'_, '_, <GFBD as GetBatchData>::Param>,
)where
    I: CachedRenderPipelinePhaseItem + SortedPhaseItem,
    GFBD: GetFullBatchData,
```

Batch the items in a sorted render phase, when GPU instance buffer building is in use. This means comparing metadata needed to draw each phase item and trying to combine the draws into a batch.