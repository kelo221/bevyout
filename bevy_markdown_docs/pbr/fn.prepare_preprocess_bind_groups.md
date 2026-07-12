[bevy](../index.html)::[pbr](index.html)

# Function prepare\_preprocess\_bind\_groups 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1798-1813)

```rust
pub fn prepare_preprocess_bind_groups(
    commands: Commands<'_, '_>,
    views: Query<'_, '_, (Entity, &ExtractedView)>,
    view_depth_pyramids: Query<'_, '_, (&ViewDepthPyramid, &PreviousViewUniformOffset)>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    batched_instance_buffers: Res<'_, BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
    indirect_parameters_buffers: Res<'_, IndirectParametersBuffers>,
    bin_unpacking_buffers: Res<'_, BinUnpackingBuffers>,
    mesh_culling_data_buffer: Res<'_, MeshCullingDataBuffer>,
    visibility_ranges: Res<'_, RenderVisibilityRanges>,
    view_uniforms: Res<'_, ViewUniforms>,
    previous_view_uniforms: Res<'_, PreviousViewUniforms>,
    pipelines: Res<'_, PreprocessPipelines>,
    bin_unpacking_bind_groups: ResMut<'_, BinUnpackingBindGroups>,
)
```

A system that attaches buffers to bind groups for the variants of the compute shaders relating to mesh preprocessing.