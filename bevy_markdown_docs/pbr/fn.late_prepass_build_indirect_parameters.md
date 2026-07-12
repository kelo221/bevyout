[bevy](../index.html)::[pbr](index.html)

# Function late\_prepass\_build\_indirect\_parameters 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#969-975)

```rust
pub fn late_prepass_build_indirect_parameters(
    preprocess_pipelines: Res<'_, PreprocessPipelines>,
    build_indirect_params_bind_groups: Option<Res<'_, BuildIndirectParametersBindGroups>>,
    pipeline_cache: Res<'_, PipelineCache>,
    indirect_parameters_buffers: Option<Res<'_, IndirectParametersBuffers>>,
    ctx: RenderContext<'_, '_>,
)
```