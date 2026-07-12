[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[internal](index.html)

# Trait CreateFrom 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#261)

```rust
pub trait CreateFrom: Sized {
    // Required method
    fn create_from<B>(reader: &mut Reader<B>) -> Self
       where B: BufferRef;
}
```

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#262-264)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> Self

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#146)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#146)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#145)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#145)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#85)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#85)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#84)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#84)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#115)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#115)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#114)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#114)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#52)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#52)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#54)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#54)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#53)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#53)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/array.rs.html#104-106)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/array.rs.html#109)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Cell](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html "struct core::cell::Cell")<T>

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Cell](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html "struct core::cell::Cell")<T>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, T>

where T: [ToOwned](../../../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned")<Owned = T> + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, T>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

#### fn [create\_from](#tymethod.create_from)<B>(reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#55)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ArrayLength](../struct.ArrayLength.html "struct bevy::render::render_resource::encase::ArrayLength")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#518)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [AtmosphereTransform](../../../../pbr/resources/struct.AtmosphereTransform.html "struct bevy::pbr::resources::AtmosphereTransform")

where [AtmosphereTransform](../../../../pbr/resources/struct.AtmosphereTransform.html "struct bevy::pbr::resources::AtmosphereTransform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#106)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ChromaticAberrationUniform](../../../../post_process/effect_stack/struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform")

where [ChromaticAberrationUniform](../../../../post_process/effect_stack/struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#478)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform")

where [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<10>>, [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#113)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ColorMaterialUniform](../../../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform")

where [ColorMaterialUniform](../../../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#61)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ContactShadowsUniform](../../../../pbr/struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform")

where [ContactShadowsUniform](../../../../pbr/struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#141)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [DepthOfFieldUniform](../../../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform")

where [DepthOfFieldUniform](../../../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#237)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [DownsamplingConstants](../../../../core_pipeline/mip_generation/struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants")

where [DownsamplingConstants](../../../../core_pipeline/mip_generation/struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#539)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [FilteringConstants](../../../../pbr/generate/struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants")

where [FilteringConstants](../../../../pbr/generate/struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#97)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ForwardDecalMaterialExtUniform](../../../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform")

where [ForwardDecalMaterialExtUniform](../../../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#51)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [FrameTimeGraphConfigUniform](../../../../dev_tools/frame_time_graph/struct.FrameTimeGraphConfigUniform.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphConfigUniform")

where [FrameTimeGraphConfigUniform](../../../../dev_tools/frame_time_graph/struct.FrameTimeGraphConfigUniform.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphConfigUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GlobalsUniform](../../../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

where [GlobalsUniform](../../../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#476)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuAtmosphere](../../../../pbr/resources/struct.GpuAtmosphere.html "struct bevy::pbr::resources::GpuAtmosphere")

where [GpuAtmosphere](../../../../pbr/resources/struct.GpuAtmosphere.html "struct bevy::pbr::resources::GpuAtmosphere"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuAtmosphereSettings](../../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

where [GpuAtmosphereSettings](../../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#929)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuBinUnpackingMetadata](../../../batching/gpu_preprocessing/struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata")

where [GpuBinUnpackingMetadata](../../../batching/gpu_preprocessing/struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [61](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#110)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuClusteredLight](../../../../pbr/struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight")

where [GpuClusteredLight](../../../../pbr/struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#147)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade")

where [GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#154)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight")

where [GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<13>>, \[[GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/fog.rs.html#17)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuFog](../../../../pbr/struct.GpuFog.html "struct bevy::pbr::GpuFog")

where [GpuFog](../../../../pbr/struct.GpuFog.html "struct bevy::pbr::GpuFog"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#195)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuLights](../../../../pbr/struct.GpuLights.html "struct bevy::pbr::GpuLights")

where [GpuLights](../../../../pbr/struct.GpuLights.html "struct bevy::pbr::GpuLights"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<9>>, \[[GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight"); [10](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), \[[GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight"); [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#174)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuMorphDescriptor](../../../../pbr/struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor")

where [GpuMorphDescriptor](../../../../pbr/struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#184)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight")

where [GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<7>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#666)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [GpuRenderBinnedMeshInstance](../../../render_phase/struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance")

where [GpuRenderBinnedMeshInstance](../../../render_phase/struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#9)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

where [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 2>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#13)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

where [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 3>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#877)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IndirectBatchSet](../../../batching/gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet")

where [IndirectBatchSet](../../../batching/gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#814)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IndirectParametersCpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata")

where [IndirectParametersCpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#845)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IndirectParametersGpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata")

where [IndirectParametersGpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#778)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IndirectParametersIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed")

where [IndirectParametersIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#797)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IndirectParametersNonIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed")

where [IndirectParametersNonIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#562)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [LatePreprocessWorkItemIndirectParameters](../../../batching/gpu_preprocessing/struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters")

where [LatePreprocessWorkItemIndirectParameters](../../../batching/gpu_preprocessing/struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#104)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [LensDistortionUniform](../../../../post_process/effect_stack/struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform")

where [LensDistortionUniform](../../../../post_process/effect_stack/struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#121)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [LightProbesUniform](../../../../pbr/struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform")

where [LightProbesUniform](../../../../pbr/struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<9>>, \[RenderLightProbe; [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#391)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [LinearRgba](../../../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

Available on **crate feature `encase`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#59)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

where [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [FromMatrixParts](../matrix/trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2, 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#60)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

where [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [FromMatrixParts](../matrix/trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3, 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [FromMatrixParts](../matrix/trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#217)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Mesh2dUniform](../../../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform")

where [Mesh2dUniform](../../../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#632)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [MeshCullingData](../../../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData")

where [MeshCullingData](../../../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#562)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [MeshInputUniform](../../../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")

where [MeshInputUniform](../../../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#514)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [MeshUniform](../../../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform")

where [MeshUniform](../../../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [MorphAttributes](../../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

where [MorphAttributes](../../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#141)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [OitFragmentNode](../../../../core_pipeline/oit/struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode")

where [OitFragmentNode](../../../../core_pipeline/oit/struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [OrderIndependentTransparencySettings](../../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

where [OrderIndependentTransparencySettings](../../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [PbrDeferredLightingDepthId](../../../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

where [PbrDeferredLightingDepthId](../../../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#757)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [PreprocessWorkItem](../../../batching/gpu_preprocessing/struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem")

where [PreprocessWorkItem](../../../batching/gpu_preprocessing/struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#101)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [PreviousViewData](../../../../core_pipeline/prepass/struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData")

where [PreviousViewData](../../../../core_pipeline/prepass/struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#207)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [RenderClusteredDecal](../../../../pbr/decal/clustered/struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal")

where [RenderClusteredDecal](../../../../pbr/decal/clustered/struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#143)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ScreenSpaceReflectionsUniform](../../../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform")

where [ScreenSpaceReflectionsUniform](../../../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#95)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [SkyboxUniforms](../../../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms")

where [SkyboxUniforms](../../../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#205)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [SmaaInfoUniform](../../../../anti_alias/smaa/struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform")

where [SmaaInfoUniform](../../../../anti_alias/smaa/struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#79)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [SpriteMaterialUniform](../../../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform")

where [SpriteMaterialUniform](../../../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#1011)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [StandardMaterialUniform](../../../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform")

where [StandardMaterialUniform](../../../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<23>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#12)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

where [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 3>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [FromVectorParts](../vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#610)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [ViewUniform](../../../view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform")

where [ViewUniform](../../../view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<16>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [6](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#113)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [VignetteUniform](../../../../post_process/effect_stack/struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform")

where [VignetteUniform](../../../../post_process/effect_stack/struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#399)

### impl [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [WireframeVertexPullParams](../../../../pbr/wireframe/struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams")

where [WireframeVertexPullParams](../../../../pbr/wireframe/struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + for<'\_\_> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#119)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Arc](../../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#116)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Box](../../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#251)

### impl<T> [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"), [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,