[bevy](../../../../index.html)::[core\_pipeline](../../../index.html)::[mip\_generation](../../index.html)::[experimental](../index.html)::[depth](index.html)

# Function create\_downsample\_depth\_pipelines 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#304-312)

```rust
pub fn create_downsample_depth_pipelines(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    pipeline_cache: Res<'_, PipelineCache>,
    specialized_compute_pipelines: ResMut<'_, SpecializedComputePipelines<DownsampleDepthPipeline>>,
    gpu_preprocessing_support: Res<'_, GpuPreprocessingSupport>,
    downsample_depth_shader: Res<'_, DownsampleShaders>,
    has_run: Local<'_, bool>,
)
```

Creates the [`DownsampleDepthPipelines`](struct.DownsampleDepthPipelines.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipelines") if downsampling is supported on the current platform.