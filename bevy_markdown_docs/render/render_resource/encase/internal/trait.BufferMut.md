[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[internal](index.html)

# Trait BufferMut 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#195)

```rust
pub trait BufferMut {
    // Required methods
    fn capacity(&self) -> usize;
    fn write<const N: usize>(&mut self, offset: usize, val: &[u8; N]);
    fn write_slice(&mut self, offset: usize, val: &[u8]);

    // Provided method
    fn try_enlarge(&mut self, wanted: usize) -> Result<(), EnlargeError> { ... }
}
```

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#196)

#### fn [capacity](#tymethod.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#198)

#### fn [write](#tymethod.write)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#200)

#### fn [write\_slice](#tymethod.write_slice)(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

## Provided Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#203)

#### fn [try\_enlarge](#method.try_enlarge)(&mut self, wanted: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [EnlargeError](struct.EnlargeError.html "struct bevy::render::render_resource::encase::internal::EnlargeError")\>

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#281)

### impl [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for \[[MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#283)

#### fn [capacity](#tymethod.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#288)

#### fn [write](#tymethod.write)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#296)

#### fn [write\_slice](#tymethod.write_slice)(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#263)

### impl [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#265)

#### fn [capacity](#tymethod.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#270)

#### fn [write](#tymethod.write)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#276)

#### fn [write\_slice](#tymethod.write_slice)(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#432)

### impl<T> [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#432)

#### fn [capacity](#tymethod.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#432)

#### fn [write](#tymethod.write)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#432)

#### fn [write\_slice](#tymethod.write_slice)(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#432)

#### fn [try\_enlarge](#method.try_enlarge)(&mut self, wanted: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [EnlargeError](struct.EnlargeError.html "struct bevy::render::render_resource::encase::internal::EnlargeError")\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#320)

### impl<const LEN: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for \[[MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>; [LEN](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#322)

#### fn [capacity](#tymethod.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#327)

#### fn [write](#tymethod.write)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#332)

#### fn [write\_slice](#tymethod.write_slice)(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#303)

### impl<const LEN: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [LEN](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#305)

#### fn [capacity](#tymethod.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#310)

#### fn [write](#tymethod.write)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#315)

#### fn [write\_slice](#tymethod.write_slice)(&mut self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#360)

### impl [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#337)

### impl [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#432)

### impl<T> [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") for [Box](../../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [BufferMut](trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),