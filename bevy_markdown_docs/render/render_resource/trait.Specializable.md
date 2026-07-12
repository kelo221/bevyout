[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait Specializable 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#26)

```rust
pub trait Specializable {
    type Descriptor: PartialEq + Clone + Send + Sync;
    type CachedId: Clone + Send + Sync;

    // Required methods
    fn queue(
        pipeline_cache: &PipelineCache,
        descriptor: Self::Descriptor,
    ) -> Self::CachedId;
    fn get_descriptor(
        pipeline_cache: &PipelineCache,
        id: Self::CachedId,
    ) -> &Self::Descriptor;
}
```

Defines a type that is able to be “specialized” and cached by creating and transforming its descriptor type. This is implemented for [`RenderPipeline`](struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline") and [`ComputePipeline`](struct.ComputePipeline.html "struct bevy::render::render_resource::ComputePipeline"), and likely will not have much utility for other types.

See docs on [`Specializer`](trait.Specializer.html "trait bevy::render::render_resource::Specializer") for more info.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#27)

#### type [Descriptor](#associatedtype.Descriptor): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#28)

#### type [CachedId](#associatedtype.CachedId): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#29)

#### fn [queue](#tymethod.queue)( pipeline\_cache: &[PipelineCache](struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache"), descriptor: Self::[Descriptor](trait.Specializable.html#associatedtype.Descriptor "type bevy::render::render_resource::Specializable::Descriptor"), ) -> Self::[CachedId](trait.Specializable.html#associatedtype.CachedId "type bevy::render::render_resource::Specializable::CachedId")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#30)

#### fn [get\_descriptor](#tymethod.get_descriptor)( pipeline\_cache: &[PipelineCache](struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache"), id: Self::[CachedId](trait.Specializable.html#associatedtype.CachedId "type bevy::render::render_resource::Specializable::CachedId"), ) -> &Self::[Descriptor](trait.Specializable.html#associatedtype.Descriptor "type bevy::render::render_resource::Specializable::Descriptor")

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#49)

### impl [Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable") for [ComputePipeline](struct.ComputePipeline.html "struct bevy::render::render_resource::ComputePipeline")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#50)

#### type [Descriptor](#associatedtype.Descriptor) = [ComputePipelineDescriptor](../../material/descriptor/struct.ComputePipelineDescriptor.html "struct bevy::material::descriptor::ComputePipelineDescriptor")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#52)

#### type [CachedId](#associatedtype.CachedId) = [CachedComputePipelineId](../../material/descriptor/struct.CachedComputePipelineId.html "struct bevy::material::descriptor::CachedComputePipelineId")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#33)

### impl [Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable") for [RenderPipeline](struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#34)

#### type [Descriptor](#associatedtype.Descriptor) = [RenderPipelineDescriptor](../../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#35)

#### type [CachedId](#associatedtype.CachedId) = [CachedRenderPipelineId](../../material/descriptor/struct.CachedRenderPipelineId.html "struct bevy::material::descriptor::CachedRenderPipelineId")