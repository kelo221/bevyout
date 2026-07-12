[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[internal](index.html)

# Trait WriteInto 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#249)

```rust
pub trait WriteInto {
    // Required method
    fn write_into<B>(&self, writer: &mut Writer<B>)
       where B: BufferMut;
}
```

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#250-252)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#146)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#146)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#145)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#145)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#85)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#85)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#84)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#84)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#115)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#115)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#114)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#114)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#52)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#52)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#54)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#54)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#53)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#53)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/array.rs.html#65-67)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/array.rs.html#70)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#114)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#114)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Cell](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html "struct core::cell::Cell")<T>

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, T>

where T: [ToOwned](../../../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned")<Owned = T> + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

where [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>: [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

#### fn [write\_into](#tymethod.write_into)<B>(&self, writer: &mut [Writer](struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#41)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ArrayLength](../struct.ArrayLength.html "struct bevy::render::render_resource::encase::ArrayLength")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#518)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [AtmosphereTransform](../../../../pbr/resources/struct.AtmosphereTransform.html "struct bevy::pbr::resources::AtmosphereTransform")

where [AtmosphereTransform](../../../../pbr/resources/struct.AtmosphereTransform.html "struct bevy::pbr::resources::AtmosphereTransform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#106)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ChromaticAberrationUniform](../../../../post_process/effect_stack/struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform")

where [ChromaticAberrationUniform](../../../../post_process/effect_stack/struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#478)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform")

where [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<10>>, [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#113)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ColorMaterialUniform](../../../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform")

where [ColorMaterialUniform](../../../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#61)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ContactShadowsUniform](../../../../pbr/struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform")

where [ContactShadowsUniform](../../../../pbr/struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#141)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [DepthOfFieldUniform](../../../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform")

where [DepthOfFieldUniform](../../../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#237)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [DownsamplingConstants](../../../../core_pipeline/mip_generation/struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants")

where [DownsamplingConstants](../../../../core_pipeline/mip_generation/struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#539)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [FilteringConstants](../../../../pbr/generate/struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants")

where [FilteringConstants](../../../../pbr/generate/struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#97)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ForwardDecalMaterialExtUniform](../../../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform")

where [ForwardDecalMaterialExtUniform](../../../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#51)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [FrameTimeGraphConfigUniform](../../../../dev_tools/frame_time_graph/struct.FrameTimeGraphConfigUniform.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphConfigUniform")

where [FrameTimeGraphConfigUniform](../../../../dev_tools/frame_time_graph/struct.FrameTimeGraphConfigUniform.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphConfigUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GlobalsUniform](../../../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

where [GlobalsUniform](../../../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#476)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuAtmosphere](../../../../pbr/resources/struct.GpuAtmosphere.html "struct bevy::pbr::resources::GpuAtmosphere")

where [GpuAtmosphere](../../../../pbr/resources/struct.GpuAtmosphere.html "struct bevy::pbr::resources::GpuAtmosphere"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuAtmosphereSettings](../../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

where [GpuAtmosphereSettings](../../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#929)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuBinUnpackingMetadata](../../../batching/gpu_preprocessing/struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata")

where [GpuBinUnpackingMetadata](../../../batching/gpu_preprocessing/struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [61](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#110)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuClusteredLight](../../../../pbr/struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight")

where [GpuClusteredLight](../../../../pbr/struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#147)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade")

where [GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#154)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight")

where [GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<13>>, \[[GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/fog.rs.html#17)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuFog](../../../../pbr/struct.GpuFog.html "struct bevy::pbr::GpuFog")

where [GpuFog](../../../../pbr/struct.GpuFog.html "struct bevy::pbr::GpuFog"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#195)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuLights](../../../../pbr/struct.GpuLights.html "struct bevy::pbr::GpuLights")

where [GpuLights](../../../../pbr/struct.GpuLights.html "struct bevy::pbr::GpuLights"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<9>>, \[[GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight"); [10](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), \[[GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight"); [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#174)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuMorphDescriptor](../../../../pbr/struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor")

where [GpuMorphDescriptor](../../../../pbr/struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#184)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight")

where [GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<7>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#666)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [GpuRenderBinnedMeshInstance](../../../render_phase/struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance")

where [GpuRenderBinnedMeshInstance](../../../render_phase/struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#9)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

where [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 2>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#13)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

where [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 3>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#877)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IndirectBatchSet](../../../batching/gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet")

where [IndirectBatchSet](../../../batching/gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#814)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IndirectParametersCpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata")

where [IndirectParametersCpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#845)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IndirectParametersGpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata")

where [IndirectParametersGpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#778)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IndirectParametersIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed")

where [IndirectParametersIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#797)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IndirectParametersNonIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed")

where [IndirectParametersNonIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#562)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [LatePreprocessWorkItemIndirectParameters](../../../batching/gpu_preprocessing/struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters")

where [LatePreprocessWorkItemIndirectParameters](../../../batching/gpu_preprocessing/struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#104)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [LensDistortionUniform](../../../../post_process/effect_stack/struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform")

where [LensDistortionUniform](../../../../post_process/effect_stack/struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#121)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [LightProbesUniform](../../../../pbr/struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform")

where [LightProbesUniform](../../../../pbr/struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<9>>, \[RenderLightProbe; [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#362)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [LinearRgba](../../../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

Available on **crate feature `encase`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#59)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

where [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsRefMatrixParts](../matrix/trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2, 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#60)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

where [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsRefMatrixParts](../matrix/trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3, 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsRefMatrixParts](../matrix/trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#217)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Mesh2dUniform](../../../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform")

where [Mesh2dUniform](../../../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#632)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [MeshCullingData](../../../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData")

where [MeshCullingData](../../../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#562)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [MeshInputUniform](../../../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")

where [MeshInputUniform](../../../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#514)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [MeshUniform](../../../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform")

where [MeshUniform](../../../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [MorphAttributes](../../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

where [MorphAttributes](../../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#141)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [OitFragmentNode](../../../../core_pipeline/oit/struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode")

where [OitFragmentNode](../../../../core_pipeline/oit/struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [OrderIndependentTransparencySettings](../../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

where [OrderIndependentTransparencySettings](../../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [PbrDeferredLightingDepthId](../../../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

where [PbrDeferredLightingDepthId](../../../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#757)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [PreprocessWorkItem](../../../batching/gpu_preprocessing/struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem")

where [PreprocessWorkItem](../../../batching/gpu_preprocessing/struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#101)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [PreviousViewData](../../../../core_pipeline/prepass/struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData")

where [PreviousViewData](../../../../core_pipeline/prepass/struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#207)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [RenderClusteredDecal](../../../../pbr/decal/clustered/struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal")

where [RenderClusteredDecal](../../../../pbr/decal/clustered/struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#143)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ScreenSpaceReflectionsUniform](../../../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform")

where [ScreenSpaceReflectionsUniform](../../../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#95)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [SkyboxUniforms](../../../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms")

where [SkyboxUniforms](../../../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#205)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [SmaaInfoUniform](../../../../anti_alias/smaa/struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform")

where [SmaaInfoUniform](../../../../anti_alias/smaa/struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#79)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [SpriteMaterialUniform](../../../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform")

where [SpriteMaterialUniform](../../../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#1011)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [StandardMaterialUniform](../../../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform")

where [StandardMaterialUniform](../../../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<23>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#12)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

where [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 3>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [AsRefVectorParts](../vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#610)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [ViewUniform](../../../view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform")

where [ViewUniform](../../../view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<16>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [6](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#113)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [VignetteUniform](../../../../post_process/effect_stack/struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform")

where [VignetteUniform](../../../../post_process/effect_stack/struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#399)

### impl [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [WireframeVertexPullParams](../../../../pbr/wireframe/struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams")

where [WireframeVertexPullParams](../../../../pbr/wireframe/struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + for<'\_\_> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#119)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Arc](../../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#116)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Box](../../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#251)

### impl<T> [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, T: [WriteInto](trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),