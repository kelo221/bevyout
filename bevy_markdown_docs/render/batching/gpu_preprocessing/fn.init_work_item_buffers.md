[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function init\_work\_item\_buffers 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#668-676)

```rust
pub fn init_work_item_buffers(
    work_item_buffers: &mut PreprocessWorkItemBuffers,
    late_indexed_indirect_parameters_buffer: &mut RawBufferVec<LatePreprocessWorkItemIndirectParameters>,
    late_non_indexed_indirect_parameters_buffer: &mut RawBufferVec<LatePreprocessWorkItemIndirectParameters>,
)
```

Initializes work item buffers for a phase in preparation for a new frame.