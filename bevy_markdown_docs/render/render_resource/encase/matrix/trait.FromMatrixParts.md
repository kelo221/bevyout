[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[matrix](index.html)

# Trait FromMatrixParts 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/matrix.rs.html#28)

```rust
pub trait FromMatrixParts<T, const C: usize, const R: usize>where
    T: MatrixScalar,{
    // Required method
    fn from_parts(parts: [[T; R]; C]) -> Self;
}
```

Enables the creation of a matrix (via `[[T; R]; C]`)

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/matrix.rs.html#29)

#### fn [from\_parts](#tymethod.from_parts)(parts: \[[\[T; R\]](https://doc.rust-lang.org/nightly/std/primitive.array.html); [C](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> Self

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#55)

### impl [FromMatrixParts](trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2, 2> for [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#56)

### impl [FromMatrixParts](trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3, 3> for [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

### impl [FromMatrixParts](trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4> for [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")