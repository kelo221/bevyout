[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[matrix](index.html)

# Trait AsMutMatrixParts 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/matrix.rs.html#23)

```rust
pub trait AsMutMatrixParts<T, const C: usize, const R: usize>where
    T: MatrixScalar,{
    // Required method
    fn as_mut_parts(&mut self) -> &mut [[T; R]; C];
}
```

Enables writing to the matrix (via `&mut [[T; R]; C]`)

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/matrix.rs.html#24)

#### fn [as\_mut\_parts](#tymethod.as_mut_parts)(&mut self) -> &mut \[[\[T; R\]](https://doc.rust-lang.org/nightly/std/primitive.array.html); [C](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#55)

### impl [AsMutMatrixParts](trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2, 2> for [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

where [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#56)

### impl [AsMutMatrixParts](trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3, 3> for [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

where [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

### impl [AsMutMatrixParts](trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4> for [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),