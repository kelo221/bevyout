[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[no\_gpu\_preprocessing](index.html)

# Function batch\_and\_prepare\_sorted\_render\_phase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/no_gpu_preprocessing.rs.html#77-83)

```rust
pub fn batch_and_prepare_sorted_render_phase<I, GBD>(
    batched_instance_buffer: ResMut<'_, BatchedInstanceBuffer<<GBD as GetBatchData>::BufferData>>,
    phases: ResMut<'_, ViewSortedRenderPhases<I>>,
    param: StaticSystemParam<'_, '_, <GBD as GetBatchData>::Param>,
)where
    I: CachedRenderPipelinePhaseItem + SortedPhaseItem,
    GBD: GetBatchData,
```

Batch the items in a sorted render phase, when GPU instance buffer building isn’t in use. This means comparing metadata needed to draw each phase item and trying to combine the draws into a batch.