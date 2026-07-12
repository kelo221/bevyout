[bevy](../index.html)::[pbr](index.html)

# Function prepare\_preprocess\_pipelines 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1523-1538)

```rust
pub fn prepare_preprocess_pipelines(
    pipeline_cache: Res<'_, PipelineCache>,
    render_device: Res<'_, RenderDevice>,
    specialized_preprocess_pipelines: ResMut<'_, SpecializedComputePipelines<PreprocessPipeline>>,
    specialized_reset_indirect_batch_sets_pipelines: ResMut<'_, SpecializedComputePipelines<ResetIndirectBatchSetsPipeline>>,
    specialized_build_indirect_parameters_pipelines: ResMut<'_, SpecializedComputePipelines<BuildIndirectParametersPipeline>>,
    specialized_bin_unpacking_pipelines: ResMut<'_, SpecializedComputePipelines<BinUnpackingPipeline>>,
    preprocess_pipelines: ResMut<'_, PreprocessPipelines>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
)
```

A system that specializes the pipelines relating to mesh preprocessing if necessary.

These pipelines include those corresponding to the mesh preprocessing shader itself, in addition to those corresponding to the indirect batch set resetting shader, the indirect parameters building shader, and the bin unpacking shader.