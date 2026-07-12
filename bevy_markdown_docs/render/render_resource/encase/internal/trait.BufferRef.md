[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[internal](index.html)

# Trait BufferRef 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#187)

```rust
pub trait BufferRef {
    // Required methods
    fn len(&self) -> usize;
    fn read<const N: usize>(&self, offset: usize) -> &[u8; N];
    fn read_slice(&self, offset: usize, val: &mut [u8]);
}
```

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#188)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#190)

#### fn [read](#tymethod.read)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#192)

#### fn [read\_slice](#tymethod.read_slice)(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#212)

### impl [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#213)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#218)

#### fn [read](#tymethod.read)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#224)

#### fn [read\_slice](#tymethod.read_slice)(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

### impl<T> [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [read](#tymethod.read)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [read\_slice](#tymethod.read_slice)(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

### impl<T> [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [read](#tymethod.read)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [read\_slice](#tymethod.read_slice)(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

### impl<T> [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where T: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [read](#tymethod.read)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

#### fn [read\_slice](#tymethod.read_slice)(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#229)

### impl<const LEN: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [LEN](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#231)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#236)

#### fn [read](#tymethod.read)<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#241)

#### fn [read\_slice](#tymethod.read_slice)(&self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), val: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

## Implementors

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#246)

### impl [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for [Vec](../../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

### impl<T> [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for [Arc](../../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#404)

### impl<T> [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") for [Box](../../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [BufferRef](trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),