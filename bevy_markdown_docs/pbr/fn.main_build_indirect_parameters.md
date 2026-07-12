[bevy](../index.html)::[pbr](index.html)

# Function main\_build\_indirect\_parameters 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#990-997)

```rust
pub fn main_build_indirect_parameters(
    _current_view: ViewQuery<'_, '_, Entity, Without<ShadowView>>,
    preprocess_pipelines: Res<'_, PreprocessPipelines>,
    build_indirect_params_bind_groups: Option<Res<'_, BuildIndirectParametersBindGroups>>,
    pipeline_cache: Res<'_, PipelineCache>,
    indirect_parameters_buffers: Option<Res<'_, IndirectParametersBuffers>>,
    ctx: RenderContext<'_, '_>,
)
```

Builds indirect parameters for the main opaque and transparent passes.

The unused `_current_view` parameter is necessary so that we don’t try to render a main pass for shadow views.