[bevy](../../index.html)::[math](../index.html)::[cubic\_splines](index.html)

# Trait RationalGenerator 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1308)

```rust
pub trait RationalGenerator<P>where
    P: VectorSpace,{
    type Error;

    // Required method
    fn to_curve(&self) -> Result<RationalCurve<P>, Self::Error>;
}
```

Available on **crate feature `alloc`** only.

Implement this on cubic splines that can generate a rational cubic curve from their spline parameters.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1310)

#### type [Error](#associatedtype.Error)

An error type indicating why construction might fail.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1313)

#### fn [to\_curve](#tymethod.to_curve)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RationalCurve](../../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>, Self::[Error](../../prelude/trait.RationalGenerator.html#associatedtype.Error "type bevy::prelude::RationalGenerator::Error")\>

Build a [`RationalCurve`](../../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve") by computing the interpolation coefficients for each curve segment.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#783)

### impl<P> [RationalGenerator](../../prelude/trait.RationalGenerator.html "trait bevy::prelude::RationalGenerator")<P> for [CubicNurbs](../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>

where P: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#784)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")