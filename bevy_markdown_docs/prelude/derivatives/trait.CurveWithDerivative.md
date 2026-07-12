[bevy](../../index.html)::[prelude](../index.html)::[derivatives](index.html)

# Trait CurveWithDerivative 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#40)

```rust
pub trait CurveWithDerivative<T>: Sized + SampleDerivative<T>where
    T: HasTangent,{
    // Required method
    fn with_derivative(self) -> SampleDerivativeWrapper<Self>;
}
```

Trait for curves that have a well-defined notion of derivative, allowing for derivatives to be extracted along with values.

This is implemented by implementing [`SampleDerivative`](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative").

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#47)

#### fn [with\_derivative](#tymethod.with_derivative)(self) -> [SampleDerivativeWrapper](struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<Self>

This curve, but with its first derivative included in sampling.

Notably, the output type is a `Curve<WithDerivative<T>>`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#212-215)

### impl<T, C> [CurveWithDerivative](trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative")<T> for C

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,