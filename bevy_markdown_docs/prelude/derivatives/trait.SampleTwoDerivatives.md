[bevy](../../index.html)::[prelude](../index.html)::[derivatives](index.html)

# Trait SampleTwoDerivatives 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#118)

```rust
pub trait SampleTwoDerivatives<T>: Curve<T>where
    T: HasTangent,{
    // Required method
    fn sample_with_two_derivatives_unchecked(
        &self,
        t: f32,
    ) -> WithTwoDerivatives<T>;

    // Provided methods
    fn sample_with_two_derivatives(
        &self,
        t: f32,
    ) -> Option<WithTwoDerivatives<T>> { ... }
    fn sample_with_two_derivatives_clamped(
        &self,
        t: f32,
    ) -> WithTwoDerivatives<T> { ... }
}
```

A trait for curves that can sample two derivatives in addition to values.

Types that implement this trait automatically implement [`CurveWithTwoDerivatives`](trait.CurveWithTwoDerivatives.html "trait bevy::prelude::derivatives::CurveWithTwoDerivatives"); the curve produced by [`with_two_derivatives`](trait.CurveWithTwoDerivatives.html#tymethod.with_two_derivatives "method bevy::prelude::derivatives::CurveWithTwoDerivatives::with_two_derivatives") uses the sampling defined in the trait implementation.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#128)

#### fn [sample\_with\_two\_derivatives\_unchecked](#tymethod.sample_with_two_derivatives_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithTwoDerivatives](../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>

Sample this curve at the parameter value `t`, extracting the associated value in addition to two derivatives. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain.

See [`Curve::sample_unchecked`](../trait.Curve.html#tymethod.sample_unchecked "method bevy::prelude::Curve::sample_unchecked") for more information.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#132)

#### fn [sample\_with\_two\_derivatives](#method.sample_with_two_derivatives)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[WithTwoDerivatives](../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>>

Sample this curve’s value and two derivatives at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#141)

#### fn [sample\_with\_two\_derivatives\_clamped](#method.sample_with_two_derivatives_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithTwoDerivatives](../../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>

Sample this curve’s value and two derivatives at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#75)

### impl<P> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<P> for [CubicCurve](../struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#35)

### impl<P> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<P> for [CubicSegment](../struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#150)

### impl<P> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<P> for [RationalCurve](../struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#110)

### impl<P> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<P> for [RationalSegment](../struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

where P: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#61-65)

### impl<T, C, D> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [ChainCurve](../struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>, D: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#101-105)

### impl<T, C, D> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [ContinuationCurve](../struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

where T: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace"), C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>, D: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#158-161)

### impl<T, C> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [ForeverCurve](../struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#190-193)

### impl<T, C> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [PingPongCurve](../struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#134-137)

### impl<T, C> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [RepeatCurve](../struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#306-309)

### impl<T, C> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [ReverseCurve](../struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#28-30)

### impl<T> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [ConstantCurve](../struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#230-237)

### impl<U, V, S, T, C, D> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<[(S, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for [ZipCurve](../struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

where U: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, S: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = U>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<S>, D: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#359-364)

### impl<V, T, C, D> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [CurveReparamCurve](../struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

where V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>, D: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#270-274)

### impl<V, T, C> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), T)> for [GraphCurve](../struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

where V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/adaptor_impls.rs.html#425-429)

### impl<V, T, C> [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T> for [LinearReparamCurve](../struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

where V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>, C: [SampleTwoDerivatives](trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,