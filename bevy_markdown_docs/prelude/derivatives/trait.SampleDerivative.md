[bevy](../../index.html)::[prelude](../index.html)::[derivatives](index.html)

# Trait SampleDerivative 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#71)

```rust
pub trait SampleDerivative<T>: Curve<T>where
    T: HasTangent,{
    // Required method
    fn sample_with_derivative_unchecked(&self, t: f32) -> WithDerivative<T>;

    // Provided methods
    fn sample_with_derivative(&self, t: f32) -> Option<WithDerivative<T>> { ... }
    fn sample_with_derivative_clamped(&self, t: f32) -> WithDerivative<T> { ... }
}
```

A trait for curves that can sample derivatives in addition to values.

Types that implement this trait automatically implement [`CurveWithDerivative`](trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative"); the curve produced by [`with_derivative`](trait.CurveWithDerivative.html#tymethod.with_derivative "method bevy::prelude::derivatives::CurveWithDerivative::with_derivative") uses the sampling defined in the trait implementation.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#81)

#### fn [sample\_with\_derivative\_unchecked](#tymethod.sample_with_derivative_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

Sample this curve at the parameter value `t`, extracting the associated value in addition to its derivative. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain.

See [`Curve::sample_unchecked`](../trait.Curve.html#tymethod.sample_unchecked "method bevy::prelude::Curve::sample_unchecked") for more information.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#85)

#### fn [sample\_with\_derivative](#method.sample_with_derivative)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>>

Sample this curve’s value and derivative at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#94)

#### fn [sample\_with\_derivative\_clamped](#method.sample_with_derivative_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

Sample this curve’s value and derivative at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#64)

### impl<P> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<P> for [CubicCurve](../struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#25)

### impl<P> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<P> for [CubicSegment](../struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#139)

### impl<P> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<P> for [RationalCurve](../struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#100)

### impl<P> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<P> for [RationalSegment](../struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#43-47)

### impl<T, C, D> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [ChainCurve](../struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>, D: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#81-85)

### impl<T, C, D> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [ContinuationCurve](../struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

where T: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>, D: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#100-104)

### impl<T, C, D> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for D

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = C>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#147-150)

### impl<T, C> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [ForeverCurve](../struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#171-174)

### impl<T, C> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [PingPongCurve](../struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#123-126)

### impl<T, C> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [RepeatCurve](../struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#288-291)

### impl<T, C> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [ReverseCurve](../struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#16-18)

### impl<T> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [ConstantCurve](../struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#211-218)

### impl<U, V, S, T, C, D> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<[(S, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for [ZipCurve](../struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

where U: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, S: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = U>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<S>, D: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#330-335)

### impl<V, T, C, D> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [CurveReparamCurve](../struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

where V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>, D: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#255-259)

### impl<V, T, C> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), T)> for [GraphCurve](../struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

where V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#397-401)

### impl<V, T, C> [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for [LinearReparamCurve](../struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

where V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleDerivative](trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,