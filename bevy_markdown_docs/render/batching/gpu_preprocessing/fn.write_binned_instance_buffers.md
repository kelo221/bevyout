[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function write\_binned\_instance\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#2439-2447)

```rust
pub fn write_binned_instance_buffers<BPI, GFBD>(
    views: Query<'_, '_, &ExtractedView>,
    view_binned_render_phases: ResMut<'_, ViewBinnedRenderPhases<BPI>>,
    bin_unpacking_buffers: ResMut<'_, BinUnpackingBuffers>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
)where
    BPI: BinnedPhaseItem,
    GFBD: GetFullBatchData,
```

Writes the bin data for each render phase to the GPU.

The bin data consists of the IDs of the mesh instances, as well as the metadata needed for the `unpack_bins` shader to unpack them.