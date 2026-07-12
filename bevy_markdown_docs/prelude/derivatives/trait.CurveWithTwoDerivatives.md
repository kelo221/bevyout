[bevy](../../index.html)::[prelude](../index.html)::[derivatives](index.html)

# Trait CurveWithTwoDerivatives 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#54)

```rust
pub trait CurveWithTwoDerivatives<T>: Sized + SampleTwoDerivatives<T>where
    T: HasTangent,{
    // Required method
    fn with_two_derivatives(self) -> SampleTwoDerivativesWrapper<Self>;
}
```

Trait for curves that have a well-defined notion of second derivative, allowing for two derivatives to be extracted along with values.

This is implemented by implementing [`SampleTwoDerivatives`](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives").

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#61)

#### fn [with\_two\_derivatives](#tymethod.with_two_derivatives)(self) -> [SampleTwoDerivativesWrapper](struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<Self>

This curve, but with its first two derivatives included in sampling.

Notably, the output type is a `Curve<WithTwoDerivatives<T>>`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#222-225)

### impl<T, C> [CurveWithTwoDerivatives](trait.CurveWithTwoDerivatives.html "trait bevy::prelude::derivatives::CurveWithTwoDerivatives")<T> for C

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> + [CurveWithDerivative](trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative")<T>,