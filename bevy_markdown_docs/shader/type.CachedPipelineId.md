[bevy](../index.html)::[shader](index.html)

# Type Alias CachedPipelineId 

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/shader_cache.rs.html#39)

```rust
pub type CachedPipelineId = usize;
```

An id of a pipeline, typically in the [`PipelineCache`](https://docs.rs/bevy/latest/bevy/render/render_resource/struct.PipelineCache.html) Typically corresponds to a unique combination of [`Shader`](../prelude/struct.Shader.html "struct bevy::prelude::Shader") and [`ShaderDefVal`](enum.ShaderDefVal.html "enum bevy::shader::ShaderDefVal")s.