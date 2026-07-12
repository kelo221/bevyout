[bevy](../../index.html)::[math](../index.html)::[curve](index.html)

# Trait CurveResampleExt 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#777)

```rust
pub trait CurveResampleExt<T>: Curve<T> {
    // Provided methods
    fn resample<I>(
        &self,
        segments: usize,
        interpolation: I,
    ) -> Result<SampleCurve<T, I>, ResamplingError>
       where I: Fn(&T, &T, f32) -> T { ... }
    fn resample_auto(
        &self,
        segments: usize,
    ) -> Result<SampleAutoCurve<T>, ResamplingError>
       where T: StableInterpolate { ... }
    fn resample_uneven<I>(
        &self,
        sample_times: impl IntoIterator<Item = f32>,
        interpolation: I,
    ) -> Result<UnevenSampleCurve<T, I>, ResamplingError>
       where I: Fn(&T, &T, f32) -> T { ... }
    fn resample_uneven_auto(
        &self,
        sample_times: impl IntoIterator<Item = f32>,
    ) -> Result<UnevenSampleAutoCurve<T>, ResamplingError>
       where T: StableInterpolate { ... }
}
```

Available on **crate features `alloc` and `curve`** only.

Extension trait implemented by [curves](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), allowing access to generic resampling methods as well as those based on [stable interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

This trait is automatically implemented for all curves.

For more information, see the [module-level documentation](index.html "mod bevy::math::curve").

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#801-807)

#### fn [resample](#method.resample)<I>( &self, segments: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), interpolation: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[SampleCurve](../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over equally spaced sample values, using the provided `interpolation` to interpolate between adjacent samples. The curve is interpolated on `segments` segments between samples. For example, if `segments` is 1, only the start and end points of the curve are used as samples; if `segments` is 2, a sample at the midpoint is taken as well, and so on.

The interpolation takes two values by reference together with a scalar parameter and produces an owned value. The expectation is that `interpolation(&x, &y, 0.0)` and `interpolation(&x, &y, 1.0)` are equivalent to `x` and `y` respectively.

##### Errors

If `segments` is zero or if this curve has unbounded domain, then a [`ResamplingError`](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError") is returned.

##### Example

```rust
let quarter_rotation = FunctionCurve::new(interval(0.0, 90.0).unwrap(), |t| Rot2::degrees(t));
// A curve which only stores three data points and uses `nlerp` to interpolate them:
let resampled_rotation = quarter_rotation.resample(3, |x, y, t| x.nlerp(*y, t));
```

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#830-832)

#### fn [resample\_auto](#method.resample_auto)( &self, segments: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[SampleAutoCurve](../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where T: [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over equally spaced sample values, using [automatic interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") to interpolate between adjacent samples. The curve is interpolated on `segments` segments between samples. For example, if `segments` is 1, only the start and end points of the curve are used as samples; if `segments` is 2, a sample at the midpoint is taken as well, and so on.

##### Errors

If `segments` is zero or if this curve has unbounded domain, a [`ResamplingError`](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError") is returned.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#863-869)

#### fn [resample\_uneven](#method.resample_uneven)<I>( &self, sample\_times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, interpolation: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UnevenSampleCurve](../../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over samples taken at a given set of times. The given `interpolation` is used to interpolate adjacent samples, and the `sample_times` are expected to contain at least two valid times within the curve’s domain interval.

Redundant sample times, non-finite sample times, and sample times outside of the domain are filtered out. With an insufficient quantity of data, a [`ResamplingError`](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError") is returned.

The domain of the produced curve stretches between the first and last sample times of the iterator.

The interpolation takes two values by reference together with a scalar parameter and produces an owned value. The expectation is that `interpolation(&x, &y, 0.0)` and `interpolation(&x, &y, 1.0)` are equivalent to `x` and `y` respectively.

##### Errors

If `sample_times` doesn’t contain at least two distinct times after filtering, a [`ResamplingError`](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError") is returned.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#905-910)

#### fn [resample\_uneven\_auto](#method.resample_uneven_auto)( &self, sample\_times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UnevenSampleAutoCurve](../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where T: [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by [automatic interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") over samples taken at the given set of times. The given `sample_times` are expected to contain at least two valid times within the curve’s domain interval.

Redundant sample times, non-finite sample times, and sample times outside of the domain are simply filtered out. With an insufficient quantity of data, a [`ResamplingError`](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError") is returned.

The domain of the produced [`UnevenSampleAutoCurve`](../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve") stretches between the first and last sample times of the iterator.

##### Errors

If `sample_times` doesn’t contain at least two distinct times after filtering, a [`ResamplingError`](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError") is returned.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#930)

### impl<C, T> [CurveResampleExt](../../prelude/trait.CurveResampleExt.html "trait bevy::prelude::CurveResampleExt")<T> for C

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),