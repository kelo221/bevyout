[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[no\_gpu\_preprocessing](index.html)

# Function batch\_and\_prepare\_binned\_render\_phase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/no_gpu_preprocessing.rs.html#108-114)

```rust
pub fn batch_and_prepare_binned_render_phase<BPI, GFBD>(
    gpu_array_buffer: ResMut<'_, BatchedInstanceBuffer<<GFBD as GetBatchData>::BufferData>>,
    phases: ResMut<'_, ViewBinnedRenderPhases<BPI>>,
    param: StaticSystemParam<'_, '_, <GFBD as GetBatchData>::Param>,
)where
    BPI: BinnedPhaseItem,
    GFBD: GetFullBatchData,
```

Creates batches for a render phase that uses bins, when GPU batch data building isn’t in use.