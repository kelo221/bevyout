[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[internal](index.html)

# Trait ReadFrom 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#255)

```rust
pub trait ReadFrom {
    // Required method
    fn read_from<B>(&mut self, reader: &mut Reader<B>)
       where B: BufferRef;
}
```

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#256-258)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#146)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#146)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#145)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#145)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#85)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#85)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#84)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#84)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#115)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#115)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#114)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#114)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#52)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#52)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#54)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#54)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#53)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#53)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/array.rs.html#84-86)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/array.rs.html#89)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

### impl<T> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

### impl<T> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Cell](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html "struct core::cell::Cell")<T>

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

### impl<T> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>: [Truncate](../rts_array/trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") + [Length](../rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a mut [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

### impl<T> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: [Truncate](../rts_array/trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") + [Length](../rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a mut [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

### impl<T> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [Truncate](../rts_array/trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") + [Length](../rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

#### fn [read\_from](#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#48)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ArrayLength](../struct.ArrayLength.html "struct bevy::render::render_resource::encase::ArrayLength")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#518)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [AtmosphereTransform](../../../../pbr/resources/struct.AtmosphereTransform.html "struct bevy::pbr::resources::AtmosphereTransform")

where [AtmosphereTransform](../../../../pbr/resources/struct.AtmosphereTransform.html "struct bevy::pbr::resources::AtmosphereTransform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#106)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ChromaticAberrationUniform](../../../../post_process/effect_stack/struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform")

where [ChromaticAberrationUniform](../../../../post_process/effect_stack/struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#478)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform")

where [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<10>>, [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#113)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ColorMaterialUniform](../../../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform")

where [ColorMaterialUniform](../../../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#61)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ContactShadowsUniform](../../../../pbr/struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform")

where [ContactShadowsUniform](../../../../pbr/struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#141)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [DepthOfFieldUniform](../../../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform")

where [DepthOfFieldUniform](../../../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#237)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [DownsamplingConstants](../../../../core_pipeline/mip_generation/struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants")

where [DownsamplingConstants](../../../../core_pipeline/mip_generation/struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#539)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [FilteringConstants](../../../../pbr/generate/struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants")

where [FilteringConstants](../../../../pbr/generate/struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#97)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ForwardDecalMaterialExtUniform](../../../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform")

where [ForwardDecalMaterialExtUniform](../../../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#51)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [FrameTimeGraphConfigUniform](../../../../dev_tools/frame_time_graph/struct.FrameTimeGraphConfigUniform.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphConfigUniform")

where [FrameTimeGraphConfigUniform](../../../../dev_tools/frame_time_graph/struct.FrameTimeGraphConfigUniform.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphConfigUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GlobalsUniform](../../../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

where [GlobalsUniform](../../../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#476)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuAtmosphere](../../../../pbr/resources/struct.GpuAtmosphere.html "struct bevy::pbr::resources::GpuAtmosphere")

where [GpuAtmosphere](../../../../pbr/resources/struct.GpuAtmosphere.html "struct bevy::pbr::resources::GpuAtmosphere"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuAtmosphereSettings](../../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

where [GpuAtmosphereSettings](../../../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#929)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuBinUnpackingMetadata](../../../batching/gpu_preprocessing/struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata")

where [GpuBinUnpackingMetadata](../../../batching/gpu_preprocessing/struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [61](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#110)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuClusteredLight](../../../../pbr/struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight")

where [GpuClusteredLight](../../../../pbr/struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#147)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade")

where [GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#154)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight")

where [GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<13>>, \[[GpuDirectionalCascade](../../../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/fog.rs.html#17)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuFog](../../../../pbr/struct.GpuFog.html "struct bevy::pbr::GpuFog")

where [GpuFog](../../../../pbr/struct.GpuFog.html "struct bevy::pbr::GpuFog"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#195)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuLights](../../../../pbr/struct.GpuLights.html "struct bevy::pbr::GpuLights")

where [GpuLights](../../../../pbr/struct.GpuLights.html "struct bevy::pbr::GpuLights"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<9>>, \[[GpuDirectionalLight](../../../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight"); [10](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), \[[GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight"); [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#174)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuMorphDescriptor](../../../../pbr/struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor")

where [GpuMorphDescriptor](../../../../pbr/struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#184)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight")

where [GpuRectLight](../../../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<7>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#666)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [GpuRenderBinnedMeshInstance](../../../render_phase/struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance")

where [GpuRenderBinnedMeshInstance](../../../render_phase/struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#9)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

where [IVec2](../../../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 2>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#13)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

where [IVec3](../../../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 3>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../../../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#877)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IndirectBatchSet](../../../batching/gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet")

where [IndirectBatchSet](../../../batching/gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#814)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IndirectParametersCpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata")

where [IndirectParametersCpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#845)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IndirectParametersGpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata")

where [IndirectParametersGpuMetadata](../../../batching/gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#778)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IndirectParametersIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed")

where [IndirectParametersIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#797)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IndirectParametersNonIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed")

where [IndirectParametersNonIndexed](../../../batching/gpu_preprocessing/struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<4>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#562)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [LatePreprocessWorkItemIndirectParameters](../../../batching/gpu_preprocessing/struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters")

where [LatePreprocessWorkItemIndirectParameters](../../../batching/gpu_preprocessing/struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#104)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [LensDistortionUniform](../../../../post_process/effect_stack/struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform")

where [LensDistortionUniform](../../../../post_process/effect_stack/struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#121)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [LightProbesUniform](../../../../pbr/struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform")

where [LightProbesUniform](../../../../pbr/struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<9>>, \[RenderLightProbe; [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#371)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [LinearRgba](../../../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

Available on **crate feature `encase`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#59)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

where [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsMutMatrixParts](../matrix/trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2, 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#60)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

where [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsMutMatrixParts](../matrix/trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3, 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsMutMatrixParts](../matrix/trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#217)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Mesh2dUniform](../../../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform")

where [Mesh2dUniform](../../../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#632)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [MeshCullingData](../../../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData")

where [MeshCullingData](../../../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#562)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [MeshInputUniform](../../../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")

where [MeshInputUniform](../../../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#514)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [MeshUniform](../../../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform")

where [MeshUniform](../../../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [MorphAttributes](../../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

where [MorphAttributes](../../../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<6>>, [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#141)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [OitFragmentNode](../../../../core_pipeline/oit/struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode")

where [OitFragmentNode](../../../../core_pipeline/oit/struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [OrderIndependentTransparencySettings](../../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

where [OrderIndependentTransparencySettings](../../../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [PbrDeferredLightingDepthId](../../../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

where [PbrDeferredLightingDepthId](../../../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#757)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [PreprocessWorkItem](../../../batching/gpu_preprocessing/struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem")

where [PreprocessWorkItem](../../../batching/gpu_preprocessing/struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#101)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [PreviousViewData](../../../../core_pipeline/prepass/struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData")

where [PreviousViewData](../../../../core_pipeline/prepass/struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<5>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#207)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [RenderClusteredDecal](../../../../pbr/decal/clustered/struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal")

where [RenderClusteredDecal](../../../../pbr/decal/clustered/struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#143)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ScreenSpaceReflectionsUniform](../../../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform")

where [ScreenSpaceReflectionsUniform](../../../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<11>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#95)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [SkyboxUniforms](../../../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms")

where [SkyboxUniforms](../../../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<2>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#205)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [SmaaInfoUniform](../../../../anti_alias/smaa/struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform")

where [SmaaInfoUniform](../../../../anti_alias/smaa/struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<1>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#79)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [SpriteMaterialUniform](../../../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform")

where [SpriteMaterialUniform](../../../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<12>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#1011)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [StandardMaterialUniform](../../../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform")

where [StandardMaterialUniform](../../../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<23>>, [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [UVec2](../../../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 2>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#12)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

where [UVec3](../../../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 3>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](../../../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [AsMutVectorParts](../vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#610)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [ViewUniform](../../../view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform")

where [ViewUniform](../../../view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<16>>, [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec3](../../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), \[[Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [6](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [ColorGradingUniform](../../../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#113)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [VignetteUniform](../../../../post_process/effect_stack/struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform")

where [VignetteUniform](../../../../post_process/effect_stack/struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<8>>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec2](../../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec4](../../../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#399)

### impl [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [WireframeVertexPullParams](../../../../pbr/wireframe/struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams")

where [WireframeVertexPullParams](../../../../pbr/wireframe/struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams"): [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = StructMetadata<3>>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + for<'\_\_> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#116)

### impl<T> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Box](../../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#251)

### impl<T> [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [CreateFrom](trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") + [ReadFrom](trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"), [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [Truncate](../rts_array/trait.Truncate.html "trait bevy::render::render_resource::encase::rts_array::Truncate") + [Length](../rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length") + [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<T> + [ShaderType](../../trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = ArrayMetadata>, &'a mut [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,