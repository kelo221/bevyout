[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function batch\_and\_prepare\_binned\_render\_phase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#1793-1808)

```rust
pub fn batch_and_prepare_binned_render_phase<BPI, GFBD>(
    phase_batched_instance_buffers: ResMut<'_, PhaseBatchedInstanceBuffers<BPI, <GFBD as GetBatchData>::BufferData>>,
    phase_indirect_parameters_buffers: ResMut<'_, PhaseIndirectParametersBuffers<BPI>>,
    binned_render_phases: ResMut<'_, ViewBinnedRenderPhases<BPI>>,
    views: Query<'_, '_, (&ExtractedView, Has<NoIndirectDrawing>, Has<OcclusionCulling>), With<ExtractedView>>,
    param: StaticSystemParam<'_, '_, <GFBD as GetBatchData>::Param>,
)where
    BPI: BinnedPhaseItem,
    GFBD: GetFullBatchData,
```

Creates batches for a render phase that uses bins.