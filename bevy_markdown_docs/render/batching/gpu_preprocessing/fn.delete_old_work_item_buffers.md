[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function delete\_old\_work\_item\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#1541-1547)

```rust
pub fn delete_old_work_item_buffers<GFBD>(
    gpu_batched_instance_buffers: ResMut<'_, BatchedInstanceBuffers<<GFBD as GetBatchData>::BufferData, <GFBD as GetFullBatchData>::BufferInputData>>,
    extracted_views: Query<'_, '_, &ExtractedView>,
)where
    GFBD: GetFullBatchData,
```

A system that removes GPU preprocessing work item buffers that correspond to deleted [`ExtractedView`](../../view/struct.ExtractedView.html "struct bevy::render::view::ExtractedView")s.

This is a separate system from [`clear_batched_gpu_instance_buffers`](fn.clear_batched_gpu_instance_buffers.html "fn bevy::render::batching::gpu_preprocessing::clear_batched_gpu_instance_buffers") because [`ExtractedView`](../../view/struct.ExtractedView.html "struct bevy::render::view::ExtractedView")s aren’t created until after the extraction phase is completed.