[bevy](../index.html)::[prelude](index.html)

# Trait CubicGenerator 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#914)

```rust
pub trait CubicGenerator<P>where
    P: VectorSpace,{
    type Error;

    // Required method
    fn to_curve(&self) -> Result<CubicCurve<P>, Self::Error>;
}
```

Available on **crate feature `alloc`** only.

Implement this on cubic splines that can generate a cubic curve from their spline parameters.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#916)

#### type [Error](#associatedtype.Error)

An error type indicating why construction might fail.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#919)

#### fn [to\_curve](#tymethod.to_curve)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[CubicCurve](struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>, Self::[Error](trait.CubicGenerator.html#associatedtype.Error "type bevy::prelude::CubicGenerator::Error")\>

Build a [`CubicCurve`](struct.CubicCurve.html "struct bevy::prelude::CubicCurve") by computing the interpolation coefficients for each curve segment.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#473)

### impl<P> [CubicGenerator](trait.CubicGenerator.html "trait bevy::prelude::CubicGenerator")<P> for [CubicBSpline](struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#474)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#71)

### impl<P> [CubicGenerator](trait.CubicGenerator.html "trait bevy::prelude::CubicGenerator")<P> for [CubicBezier](struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#72)

#### type [Error](#associatedtype.Error) = [CubicBezierError](../math/cubic_splines/struct.CubicBezierError.html "struct bevy::math::cubic_splines::CubicBezierError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#315)

### impl<P> [CubicGenerator](trait.CubicGenerator.html "trait bevy::prelude::CubicGenerator")<P> for [CubicCardinalSpline](struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#316)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#179)

### impl<P> [CubicGenerator](trait.CubicGenerator.html "trait bevy::prelude::CubicGenerator")<P> for [CubicHermite](struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#180)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#854)

### impl<P> [CubicGenerator](trait.CubicGenerator.html "trait bevy::prelude::CubicGenerator")<P> for [LinearSpline](../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#855)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")