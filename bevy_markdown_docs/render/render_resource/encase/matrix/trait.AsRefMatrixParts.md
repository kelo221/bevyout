[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[matrix](index.html)

# Trait AsRefMatrixParts 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/matrix.rs.html#18)

```rust
pub trait AsRefMatrixParts<T, const C: usize, const R: usize>where
    T: MatrixScalar,{
    // Required method
    fn as_ref_parts(&self) -> &[[T; R]; C];
}
```

Enables reading from the matrix (via `&[[T; R]; C]`)

## Required Methods

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/types/matrix.rs.html#19)

#### fn [as\_ref\_parts](#tymethod.as_ref_parts)(&self) -> &\[[\[T; R\]](https://doc.rust-lang.org/nightly/std/primitive.array.html); [C](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#55)

### impl [AsRefMatrixParts](trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 2, 2> for [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

where [Mat2](../../../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#56)

### impl [AsRefMatrixParts](trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 3, 3> for [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

where [Mat3](../../../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

### impl [AsRefMatrixParts](trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4> for [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../../../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),