[bevy](../index.html)::[math](index.html)

# Trait Affine3Ext 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/affine3.rs.html#4)

```rust
pub trait Affine3Ext {
    // Required methods
    fn from_transpose(transposed: [Vec4; 3]) -> Self;
    fn to_transpose(self) -> [Vec4; 3];
    fn inverse_transpose_3x3(self) -> ([Vec4; 2], f32);
}
```

Extension trait for [`Affine3`](struct.Affine3.html "struct bevy::math::Affine3")

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/affine3.rs.html#8)

#### fn [from\_transpose](#tymethod.from_transpose)(transposed: \[[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> Self

Generates an [`Affine3`](struct.Affine3.html "struct bevy::math::Affine3") from a transposed 3x4 matrix.

This is the inverse of [`Self::to_transpose`](trait.Affine3Ext.html#tymethod.to_transpose "method bevy::math::Affine3Ext::to_transpose").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/affine3.rs.html#10)

#### fn [to\_transpose](#tymethod.to_transpose)(self) -> \[[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Calculates the transpose of the affine 4x3 matrix to a 3x4 and formats it for packing into GPU buffers

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/affine3.rs.html#12)

#### fn [inverse\_transpose\_3x3](#tymethod.inverse_transpose_3x3)(self) -> (\[[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Calculates the inverse transpose of the 3x3 matrix and formats it for packing into GPU buffers

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/affine3.rs.html#15)

### impl [Affine3Ext](trait.Affine3Ext.html "trait bevy::math::Affine3Ext") for [Affine3](struct.Affine3.html "struct bevy::math::Affine3")