[bevy](../index.html)::[prelude](index.html)

# Trait CyclicCubicGenerator 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#926)

```rust
pub trait CyclicCubicGenerator<P>where
    P: VectorSpace,{
    type Error;

    // Required method
    fn to_curve_cyclic(&self) -> Result<CubicCurve<P>, Self::Error>;
}
```

Available on **crate feature `alloc`** only.

Implement this on cubic splines that can generate a cyclic cubic curve from their spline parameters.

This makes sense only when the control data can be interpreted cyclically.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#928)

#### type [Error](#associatedtype.Error)

An error type indicating why construction might fail.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#932)

#### fn [to\_curve\_cyclic](#tymethod.to_curve_cyclic)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[CubicCurve](struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>, Self::[Error](trait.CyclicCubicGenerator.html#associatedtype.Error "type bevy::prelude::CyclicCubicGenerator::Error")\>

Build a cyclic [`CubicCurve`](struct.CubicCurve.html "struct bevy::prelude::CubicCurve") by computing the interpolation coefficients for each curve segment, treating the control data as cyclic so that the result is a closed curve.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#496)

### impl<P> [CyclicCubicGenerator](trait.CyclicCubicGenerator.html "trait bevy::prelude::CyclicCubicGenerator")<P> for [CubicBSpline](struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#497)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#353)

### impl<P> [CyclicCubicGenerator](trait.CyclicCubicGenerator.html "trait bevy::prelude::CyclicCubicGenerator")<P> for [CubicCardinalSpline](struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#354)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#204)

### impl<P> [CyclicCubicGenerator](trait.CyclicCubicGenerator.html "trait bevy::prelude::CyclicCubicGenerator")<P> for [CubicHermite](struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#205)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#879)

### impl<P> [CyclicCubicGenerator](trait.CyclicCubicGenerator.html "trait bevy::prelude::CyclicCubicGenerator")<P> for [LinearSpline](../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#880)

#### type [Error](#associatedtype.Error) = [InsufficientDataError](../math/cubic_splines/struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")