[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait SpecializedComputePipeline 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#85)

```rust
pub trait SpecializedComputePipeline {
    type Key: Clone + Hash + PartialEq + Eq;

    // Required method
    fn specialize(&self, key: Self::Key) -> ComputePipelineDescriptor;
}
```

A trait that allows constructing different variants of a compute pipeline from a key.

Note: This is intended for modifying your pipeline descriptor on the basis of a key. If your key contains no data then you don’t need to specialize. For example, if you are using the [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") without the `#[bind_group_data]` attribute, you don’t need to specialize. Instead, create the pipeline directly from [`PipelineCache`](struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache") and store its ID.

See [`SpecializedComputePipelines`](struct.SpecializedComputePipelines.html "struct bevy::render::render_resource::SpecializedComputePipelines") for more info.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#87)

#### type [Key](#associatedtype.Key): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq")

The key that defines each “variant” of the compute pipeline.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#90)

#### fn [specialize](#tymethod.specialize)(&self, key: Self::[Key](trait.SpecializedComputePipeline.html#associatedtype.Key "type bevy::render::render_resource::SpecializedComputePipeline::Key")) -> [ComputePipelineDescriptor](../../material/descriptor/struct.ComputePipelineDescriptor.html "struct bevy::material::descriptor::ComputePipelineDescriptor")

Construct a new compute pipeline based on the provided key.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1731)

### impl [SpecializedComputePipeline](trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline") for [BinUnpackingPipeline](../../pbr/struct.BinUnpackingPipeline.html "struct bevy::pbr::BinUnpackingPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1732)

#### type [Key](#associatedtype.Key) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1679)

### impl [SpecializedComputePipeline](trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline") for [BuildIndirectParametersPipeline](../../pbr/struct.BuildIndirectParametersPipeline.html "struct bevy::pbr::BuildIndirectParametersPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1680)

#### type [Key](#associatedtype.Key) = [BuildIndirectParametersPipelineKey](../../pbr/struct.BuildIndirectParametersPipelineKey.html "struct bevy::pbr::BuildIndirectParametersPipelineKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#443)

### impl [SpecializedComputePipeline](trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline") for [DownsampleDepthPipeline](../../core_pipeline/mip_generation/experimental/depth/struct.DownsampleDepthPipeline.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipeline")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#444)

#### type [Key](#associatedtype.Key) = [DownsampleDepthPipelineKey](../../core_pipeline/mip_generation/experimental/depth/struct.DownsampleDepthPipelineKey.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1211)

### impl [SpecializedComputePipeline](trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline") for [PreprocessPipeline](../../pbr/struct.PreprocessPipeline.html "struct bevy::pbr::PreprocessPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1212)

#### type [Key](#associatedtype.Key) = [PreprocessPipelineKey](../../pbr/struct.PreprocessPipelineKey.html "struct bevy::pbr::PreprocessPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1666)

### impl [SpecializedComputePipeline](trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline") for [ResetIndirectBatchSetsPipeline](../../pbr/struct.ResetIndirectBatchSetsPipeline.html "struct bevy::pbr::ResetIndirectBatchSetsPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1667)

#### type [Key](#associatedtype.Key) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#319)

### impl [SpecializedComputePipeline](trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline") for [SparseBufferUpdatePipelines](struct.SparseBufferUpdatePipelines.html "struct bevy::render::render_resource::SparseBufferUpdatePipelines")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#320)

#### type [Key](#associatedtype.Key) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)