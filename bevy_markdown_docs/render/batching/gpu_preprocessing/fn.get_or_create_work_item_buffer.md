[bevy](../../../index.html)::[render](../../index.html)::[batching](../index.html)::[gpu\_preprocessing](index.html)

# Function get\_or\_create\_work\_item\_buffer 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#605-612)

```rust
pub fn get_or_create_work_item_buffer<'a, I>(
    work_item_buffers: &'a mut HashMap<RetainedViewEntity, PreprocessWorkItemBuffers>,
    view: RetainedViewEntity,
    no_indirect_drawing: bool,
    enable_gpu_occlusion_culling: bool,
) -> &'a mut PreprocessWorkItemBufferswhere
    I: 'static,
```

Returns the set of work item buffers for the given view, first creating it if necessary.

Bevy uses work item buffers to tell the mesh preprocessing compute shader which meshes are to be drawn.

You may need to call this function if you’re implementing your own custom render phases. See the `specialized_mesh_pipeline` example.