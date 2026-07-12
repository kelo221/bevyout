[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait ShaderType 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#76)

```rust
pub trait ShaderType {
    // Provided methods
    fn min_size() -> NonZero<u64> { ... }
    fn size(&self) -> NonZero<u64> { ... }
    fn assert_uniform_compat() { ... }
}
```

Base trait for all [WGSL host-shareable types](https://gpuweb.github.io/gpuweb/wgsl/#host-shareable-types)

## Provided Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#94)

#### fn [min\_size](#method.min_size)() -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Represents the minimum size of `Self` (equivalent to [GPUBufferBindingLayout.minBindingSize](https://gpuweb.github.io/gpuweb/#dom-gpubufferbindinglayout-minbindingsize))

For [WGSL fixed-footprint types](https://gpuweb.github.io/gpuweb/wgsl/#fixed-footprint-types) it represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderSize::SHADER_SIZE`](trait.ShaderSize.html#associatedconstant.SHADER_SIZE "associated constant bevy::render::render_resource::ShaderSize::SHADER_SIZE"))

For [WGSL runtime-sized arrays](https://gpuweb.github.io/gpuweb/wgsl/#runtime-sized) and [WGSL structs containing runtime-sized arrays](https://gpuweb.github.io/gpuweb/wgsl/#struct-types) (non fixed-footprint types) this will be calculated by assuming the array has one element

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#103)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns the size of `Self` at runtime

For [WGSL fixed-footprint types](https://gpuweb.github.io/gpuweb/wgsl/#fixed-footprint-types) it’s equivalent to [`Self::min_size`](trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size") and [`ShaderSize::SHADER_SIZE`](trait.ShaderSize.html#associatedconstant.SHADER_SIZE "associated constant bevy::render::render_resource::ShaderSize::SHADER_SIZE")

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#206)

#### fn [assert\_uniform\_compat](#method.assert_uniform_compat)()

Asserts that `Self` meets the requirements of the [uniform address space restrictions on stored values](https://gpuweb.github.io/gpuweb/wgsl/#address-spaces-uniform) and the [uniform address space layout constraints](https://gpuweb.github.io/gpuweb/wgsl/#address-space-layout-constraints)

##### Examples

###### Array

Will panic since runtime-sized arrays are not compatible with the uniform address space restrictions on stored values

[ⓘ](# "This example panics")

```rust
<Vec<test_impl::Vec4f>>::assert_uniform_compat();
```

Will panic since the stride is 4 bytes

[ⓘ](# "This example panics")

```rust
<[f32; 2]>::assert_uniform_compat();
```

Will not panic since the stride is 16 bytes

```rust
<[test_impl::Vec4f; 2]>::assert_uniform_compat();
```

###### Struct

Will panic since runtime-sized arrays are not compatible with the uniform address space restrictions on stored values

[ⓘ](# "This example panics")

```rust
#[derive(ShaderType)]
struct Invalid {
    #[shader(size(runtime))]
    vec: Vec<test_impl::Vec4f>
}
Invalid::assert_uniform_compat();
```

Will panic since the inner struct’s size must be a multiple of 16

[ⓘ](# "This example panics")

```rust
#[derive(ShaderType)]
struct S {
    x: f32,
}

#[derive(ShaderType)]
struct Invalid {
    a: f32,
    b: S, // offset between fields 'a' and 'b' must be at least 16 (currently: 4)
}
Invalid::assert_uniform_compat();
```

Will not panic (fixed via #\[shader(align)\] attribute)

```rust
#[derive(ShaderType)]
struct Valid {
    a: f32,
    #[shader(align(16))]
    b: S,
}
Valid::assert_uniform_compat();
```

Will not panic (fixed via size attribute)

```rust
#[derive(ShaderType)]
struct Valid {
    #[shader(size(16))]
    a: f32,
    b: S,
}
Valid::assert_uniform_compat();
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#146)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#145)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#85)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#84)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#115)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#114)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#52)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#54)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/scalar.rs.html#53)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/array.rs.html#23)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#114)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#114)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#115)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Cell](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html "struct core::cell::Cell")<T>

where T: [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#120)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'\_, T>

where T: [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned")<Owned = T> + [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#117)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [LinkedList](https://doc.rust-lang.org/nightly/alloc/collections/linked_list/struct.LinkedList.html "struct alloc::collections::linked_list::LinkedList")<T>: [Length](encase/rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#253)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#118)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>: [Length](encase/rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#252)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html): [Length](encase/rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#250)

#### fn [size](#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#34)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ArrayLength](encase/struct.ArrayLength.html "struct bevy::render::render_resource::encase::ArrayLength")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#518)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [AtmosphereTransform](../../pbr/resources/struct.AtmosphereTransform.html "struct bevy::pbr::resources::AtmosphereTransform")

where [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#106)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ChromaticAberrationUniform](../../post_process/effect_stack/struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#478)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ColorGradingUniform](../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform")

where [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#113)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ColorMaterialUniform](../../sprite_render/struct.ColorMaterialUniform.html "struct bevy::sprite_render::ColorMaterialUniform")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#61)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ContactShadowsUniform](../../pbr/struct.ContactShadowsUniform.html "struct bevy::pbr::ContactShadowsUniform")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#141)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [DepthOfFieldUniform](../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#237)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [DownsamplingConstants](../../core_pipeline/mip_generation/struct.DownsamplingConstants.html "struct bevy::core_pipeline::mip_generation::DownsamplingConstants")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#539)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [FilteringConstants](../../pbr/generate/struct.FilteringConstants.html "struct bevy::pbr::generate::FilteringConstants")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#97)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ForwardDecalMaterialExtUniform](../../pbr/decal/struct.ForwardDecalMaterialExtUniform.html "struct bevy::pbr::decal::ForwardDecalMaterialExtUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#51)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [FrameTimeGraphConfigUniform](../../dev_tools/frame_time_graph/struct.FrameTimeGraphConfigUniform.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphConfigUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GlobalsUniform](../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#476)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuAtmosphere](../../pbr/resources/struct.GpuAtmosphere.html "struct bevy::pbr::resources::GpuAtmosphere")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuAtmosphereSettings](../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

where [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#929)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuBinUnpackingMetadata](../batching/gpu_preprocessing/struct.GpuBinUnpackingMetadata.html "struct bevy::render::batching::gpu_preprocessing::GpuBinUnpackingMetadata")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [61](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#110)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuClusteredLight](../../pbr/struct.GpuClusteredLight.html "struct bevy::pbr::GpuClusteredLight")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#147)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuDirectionalCascade](../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade")

where [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#154)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuDirectionalLight](../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight")

where \[[GpuDirectionalCascade](../../pbr/struct.GpuDirectionalCascade.html "struct bevy::pbr::GpuDirectionalCascade"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/fog.rs.html#17)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuFog](../../pbr/struct.GpuFog.html "struct bevy::pbr::GpuFog")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#195)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuLights](../../pbr/struct.GpuLights.html "struct bevy::pbr::GpuLights")

where \[[GpuDirectionalLight](../../pbr/struct.GpuDirectionalLight.html "struct bevy::pbr::GpuDirectionalLight"); [10](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), \[[GpuRectLight](../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight"); [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#174)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuMorphDescriptor](../../pbr/struct.GpuMorphDescriptor.html "struct bevy::pbr::GpuMorphDescriptor")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#184)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuRectLight](../../pbr/struct.GpuRectLight.html "struct bevy::pbr::GpuRectLight")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#666)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [GpuRenderBinnedMeshInstance](../render_phase/struct.GpuRenderBinnedMeshInstance.html "struct bevy::render::render_phase::GpuRenderBinnedMeshInstance")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#9)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

where [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#13)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

where [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IVec4](../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#877)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IndirectBatchSet](../batching/gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#814)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IndirectParametersCpuMetadata](../batching/gpu_preprocessing/struct.IndirectParametersCpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersCpuMetadata")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#845)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IndirectParametersGpuMetadata](../batching/gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#778)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IndirectParametersIndexed](../batching/gpu_preprocessing/struct.IndirectParametersIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersIndexed")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#797)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IndirectParametersNonIndexed](../batching/gpu_preprocessing/struct.IndirectParametersNonIndexed.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersNonIndexed")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#562)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [LatePreprocessWorkItemIndirectParameters](../batching/gpu_preprocessing/struct.LatePreprocessWorkItemIndirectParameters.html "struct bevy::render::batching::gpu_preprocessing::LatePreprocessWorkItemIndirectParameters")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#104)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [LensDistortionUniform](../../post_process/effect_stack/struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#121)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [LightProbesUniform](../../pbr/struct.LightProbesUniform.html "struct bevy::pbr::LightProbesUniform")

where \[RenderLightProbe; [8](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#340)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

Available on **crate feature `encase`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#59)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#60)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#217)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Mesh2dUniform](../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform")

where \[[Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), \[[Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#632)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [MeshCullingData](../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#562)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [MeshInputUniform](../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")

where \[[Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#514)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [MeshUniform](../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform")

where \[[Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), \[[Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [MorphAttributes](../../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

where [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#141)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [OitFragmentNode](../../core_pipeline/oit/struct.OitFragmentNode.html "struct bevy::core_pipeline::oit::OitFragmentNode")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#757)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [PreprocessWorkItem](../batching/gpu_preprocessing/struct.PreprocessWorkItem.html "struct bevy::render::batching::gpu_preprocessing::PreprocessWorkItem")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#101)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [PreviousViewData](../../core_pipeline/prepass/struct.PreviousViewData.html "struct bevy::core_pipeline::prepass::PreviousViewData")

where [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#207)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [RenderClusteredDecal](../../pbr/decal/clustered/struct.RenderClusteredDecal.html "struct bevy::pbr::decal::clustered::RenderClusteredDecal")

where [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#143)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ScreenSpaceReflectionsUniform](../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#95)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [SkyboxUniforms](../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#205)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [SmaaInfoUniform](../../anti_alias/smaa/struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#79)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [SpriteMaterialUniform](../../sprite_render/struct.SpriteMaterialUniform.html "struct bevy::sprite_render::SpriteMaterialUniform")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#1011)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [StandardMaterialUniform](../../pbr/struct.StandardMaterialUniform.html "struct bevy::pbr::StandardMaterialUniform")

where [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#8)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#12)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#7)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#11)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#610)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [ViewUniform](../view/struct.ViewUniform.html "struct bevy::render::view::ViewUniform")

where [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), \[[Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [6](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [ColorGradingUniform](../view/struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#113)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [VignetteUniform](../../post_process/effect_stack/struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#399)

### impl [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [WireframeVertexPullParams](../../pbr/wireframe/struct.WireframeVertexPullParams.html "struct bevy::pbr::wireframe::WireframeVertexPullParams")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#119)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/wrapper.rs.html#116)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/runtime_sized_array.rs.html#251)

### impl<T> [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [ShaderType](trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"), [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [Length](encase/rts_array/trait.Length.html "trait bevy::render::render_resource::encase::rts_array::Length"),