[bevy](../../index.html)::[math](../index.html)

# Module curve 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#46)

Available on **crate feature `curve`** only.

The [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") trait, providing a domain-agnostic description of curves.

### Overview

At a high level, [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") is a trait that abstracts away the implementation details of curves, which comprise any kind of data parametrized by a single continuous variable. For example, that variable could represent time, in which case a curve would represent a value that changes over time, as in animation; on the other hand, it could represent something like displacement or distance, as in graphs, gradients, and curves in space.

The trait itself has two fundamental components: a curve must have a [domain](../../prelude/trait.Curve.html#tymethod.domain "method bevy::prelude::Curve::domain"), which is a nonempty range of `f32` values, and it must be able to be [sampled](../../prelude/trait.Curve.html#method.sample "method bevy::prelude::Curve::sample") on every one of those values, producing output of some fixed type.

A primary goal of the trait is to allow interfaces to simply accept `impl Curve<T>` as input rather than requiring for input curves to be defined in data in any particular way. This is supported by a number of interface methods which allow [changing parametrizations](../../prelude/trait.CurveExt.html#method.reparametrize "method bevy::prelude::CurveExt::reparametrize"), [mapping output](../../prelude/trait.CurveExt.html#method.map "method bevy::prelude::CurveExt::map"), and [rasterization](../../prelude/trait.CurveResampleExt.html#method.resample "method bevy::prelude::CurveResampleExt::resample").

### Analogy with `Iterator`

The `Curve` API behaves, in many ways, like a continuous counterpart to [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"). The analogy looks something like this with some of the common methods:

| Iterators | Curves |
| --- | --- |
| `map` | `map` |
| `skip`/`step_by` | `reparametrize` |
| `enumerate` | `graph` |
| `chain` | `chain` |
| `zip` | `zip` |
| `rev` | `reverse` |
| `by_ref` | `by_ref` |

Of course, there are very important differences, as well. For instance, the continuous nature of curves means that many iterator methods make little sense in the context of curves, or at least require numerical techniques. For example, the analogue of `sum` would be an integral, approximated by something like Riemann summation.

Furthermore, the two also differ greatly in their orientation to borrowing and mutation: iterators are mutated by being iterated, and by contrast, all curve methods are immutable. More information on the implications of this can be found [below](index.html#Ownership-and-borrowing "mod bevy::math::curve").

### Defining curves

Curves may be defined in a number of ways. The following are common:

*   using [functions](../../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve");
*   using [sample interpolation](../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve");
*   using [splines](../cubic_splines/index.html "mod bevy::math::cubic_splines");
*   using [easings](../../prelude/easing/index.html "mod bevy::prelude::easing").

Among these, the first is the most versatile[1](#fn1): the domain and the sampling output are just specified directly in the construction. For this reason, function curves are a reliable go-to for simple one-off constructions and procedural uses, where flexibility is desirable. For example:

```rust
// A sinusoid:
let sine_curve = FunctionCurve::new(Interval::EVERYWHERE, f32::sin);

// A sawtooth wave:
let sawtooth_curve = FunctionCurve::new(Interval::EVERYWHERE, |t| t % 1.0);

// A helix:
let helix_curve = FunctionCurve::new(Interval::EVERYWHERE, |theta| vec3(theta.sin(), theta, theta.cos()));
```

Sample-interpolated curves commonly arises in both rasterization and in animation, and this library has support for producing them in both fashions. See [below](index.html#Resampling-and-rasterization "mod bevy::math::curve") for more information about rasterization. Here is what an explicit sample-interpolated curve might look like:

```rust
// A list of angles that we want to traverse:
let angles = [
    0.0,
    -FRAC_PI_2,
    0.0,
    FRAC_PI_2,
    0.0
];

// Make each angle into a rotation by that angle:
let rotations = angles.map(|angle| Rot2::radians(angle));

// Interpolate these rotations with a `Rot2`-valued curve:
let rotation_curve = SampleAutoCurve::new(interval(0.0, 4.0).unwrap(), rotations).unwrap();
```

For more information on [spline curves](../cubic_splines/index.html "mod bevy::math::cubic_splines") and [easing curves](../../prelude/easing/index.html "mod bevy::prelude::easing"), see their respective modules.

And, of course, you are also free to define curve types yourself, implementing the trait directly. For custom sample-interpolated curves, the [`cores`](../../prelude/cores/index.html "mod bevy::prelude::cores") submodule provides machinery to avoid having to reimplement interpolation logic yourself. In many other cases, implementing the trait directly is often quite straightforward:

```rust
struct ExponentialCurve {
    exponent: f32,
}

impl Curve<f32> for ExponentialCurve {
    fn domain(&self) -> Interval {
        Interval::EVERYWHERE
    }

    fn sample_unchecked(&self, t: f32) -> f32 {
        f32::exp(self.exponent * t)
    }

    // All other trait methods can be inferred from these.
}
```

### Transforming curves

The API provides a few key ways of transforming one curve into another. These are often useful when you would like to make use of an interface that requires a curve that bears some logical relationship to one that you already have access to, but with different requirements or expectations. For example, the output type of the curves may differ, or the domain may be expected to be different. The `map` and `reparametrize` methods can help address this.

As a simple example of the kind of thing that arises in practice, let’s imagine that we have a `Curve<Vec2>` that we want to use to describe the motion of some object over time, but the interface for animation expects a `Curve<Vec3>`, since the object will move in three dimensions:

```rust
// Our original curve, which may look something like this:
let ellipse_curve = FunctionCurve::new(
    interval(0.0, TAU).unwrap(),
    |t| vec2(t.cos(), t.sin() * 2.0)
);

// Use `map` to situate this in 3D as a Curve<Vec3>; in this case, it will be in the xy-plane:
let ellipse_motion_curve = ellipse_curve.map(|pos| pos.extend(0.0));
```

We might imagine further still that the interface expects the curve to have domain `[0, 1]`. The `reparametrize` methods can address this:

```rust
// Change the domain to `[0, 1]` instead of `[0, TAU]`:
let final_curve = ellipse_motion_curve.reparametrize_linear(Interval::UNIT).unwrap();
```

Of course, there are many other ways of using these methods. In general, `map` is used for transforming the output and using it to drive something else, while `reparametrize` preserves the curve’s shape but changes the speed and direction in which it is traversed. For instance:

```rust
// A line segment curve connecting two points in the plane:
let start = vec2(-1.0, 1.0);
let end = vec2(1.0, 1.0);
let segment = FunctionCurve::new(Interval::UNIT, |t| start.lerp(end, t));

// Let's make a curve that goes back and forth along this line segment forever.
//
// Start by stretching the line segment in parameter space so that it travels along its length
// from `-1` to `1` instead of `0` to `1`:
let stretched_segment = segment.reparametrize_linear(interval(-1.0, 1.0).unwrap()).unwrap();

// Now, the *output* of `f32::sin` in `[-1, 1]` corresponds to the *input* interval of
// `stretched_segment`; the sinusoid output is mapped to the input parameter and controls how
// far along the segment we are:
let back_and_forth_curve = stretched_segment.reparametrize(Interval::EVERYWHERE, f32::sin);
```

### Combining curves

Curves become more expressive when used together. For example, maybe you want to combine two curves end-to-end:

```rust
// A line segment connecting `(-1, 0)` to `(0, 0)`:
let line_curve = FunctionCurve::new(
    Interval::UNIT,
    |t| vec2(-1.0, 0.0).lerp(vec2(0.0, 0.0), t)
);

// A half-circle curve starting at `(0, 0)`:
let half_circle_curve = FunctionCurve::new(
    interval(0.0, PI).unwrap(),
    |t| vec2(t.cos() * -1.0 + 1.0, t.sin())
);

// A curve that traverses `line_curve` and then `half_circle_curve` over the interval
// from `0` to `PI + 1`:
let combined_curve = line_curve.chain(half_circle_curve).unwrap();
```

Or, instead, maybe you want to combine two curves the _other_ way, producing a single curve that combines their output in a tuple:

```rust
// Some entity's position in 2D:
let position_curve = FunctionCurve::new(Interval::UNIT, |t| vec2(t.cos(), t.sin()));

// The same entity's orientation, described as a rotation. (In this case it will be spinning.)
let orientation_curve = FunctionCurve::new(Interval::UNIT, |t| Rot2::radians(5.0 * t));

// Both in one curve with `(Vec2, Rot2)` output:
let position_and_orientation = position_curve.zip(orientation_curve).unwrap();
```

See the documentation on [`chain`](../../prelude/trait.CurveExt.html#method.chain "method bevy::prelude::CurveExt::chain") and [`zip`](../../prelude/trait.CurveExt.html#method.zip "method bevy::prelude::CurveExt::zip") for more details on how these methods work.

### Resampling and rasterization

Sometimes, for reasons of portability, performance, or otherwise, it can be useful to ensure that curves of various provenance all actually share the same concrete type. This is the purpose of the [`resample`](../../prelude/trait.CurveResampleExt.html#method.resample "method bevy::prelude::CurveResampleExt::resample") family of functions: they allow a curve to be replaced by an approximate version of itself defined by interpolation over samples from the original curve.

In effect, this allows very different curves to be rasterized and treated uniformly. For example:

```rust
// A curve that is not easily transported because it relies on evaluating a function:
let interesting_curve = FunctionCurve::new(Interval::UNIT, |t| vec2(t * 3.0, t.exp()));

// A rasterized form of the preceding curve which is just a `SampleAutoCurve`. Inside, this
// just stores an `Interval` along with a buffer of sample data, so it's easy to serialize
// and deserialize:
let resampled_curve = interesting_curve.resample_auto(100).unwrap();

// The rasterized form can be seamlessly used as a curve itself:
let some_value = resampled_curve.sample(0.5).unwrap();
```

### Ownership and borrowing

It can be easy to get tripped up by how curves specifically interact with Rust’s ownership semantics. First of all, it’s worth noting that the API never uses `&mut self` — every method either takes ownership of the original curve or uses a shared reference.

Because of the methods that take ownership, it is useful to be aware of the following:

*   If `curve` is a curve, then `&curve` is also a curve with the same output. For convenience, `&curve` can be written as `curve.by_ref()` for use in method chaining.
*   However, `&curve` cannot outlive `curve`. In general, it is not `'static`.

In other words, `&curve` can be used to perform temporary operations without consuming `curve` (for example, to effectively pass `curve` into an API which expects an `impl Curve<T>`), but it _cannot_ be used in situations where persistence is necessary (e.g. when the curve itself must be stored for later use).

Here is a demonstration:

```rust
//`my_curve` is obtained somehow. It is a `Curve<(f32, f32)>`.
let my_curve = some_magic_constructor();

// Now, we want to sample a mapped version of `my_curve`.

// let samples: Vec<f32> = my_curve.map(|(x, y)| y).samples(50).unwrap().collect();
// ^ This would work, but it would also invalidate `my_curve`, since `map` takes ownership.

// Instead, we pass a borrowed version of `my_curve` to `map`. It lives long enough that we
// can extract samples:
let samples: Vec<f32> = my_curve.by_ref().map(|(x, y)| y).samples(50).unwrap().collect();

// This way, we retain the ability to use `my_curve` later:
let new_curve = my_curve.map(|(x,y)| x + y);
```

* * *

1.  In fact, universal as well, in some sense: if `curve` is any curve, then `FunctionCurve::new (curve.domain(), |t| curve.sample_unchecked(t))` is an equivalent function curve. [↩](#fnref1)
    

## Structs

[BackInCurve](struct.BackInCurve.html "struct bevy::math::curve::BackInCurve")

`f(t) = 2.70158 * t³ - 1.70158 * t²`

[BackInOutCurve](struct.BackInOutCurve.html "struct bevy::math::curve::BackInOutCurve")

Behaves as `BackIn` for t < 0.5 and as `BackOut` for t >= 0.5

[BackOutCurve](struct.BackOutCurve.html "struct bevy::math::curve::BackOutCurve")

`f(t) = 1.0 + 2.70158 * (t - 1.0)³ + 1.70158 * (t - 1.0)²`

[BounceInCurve](struct.BounceInCurve.html "struct bevy::math::curve::BounceInCurve")

bouncy at the start!

[BounceInOutCurve](struct.BounceInOutCurve.html "struct bevy::math::curve::BounceInOutCurve")

Behaves as `BounceIn` for t < 0.5 and as `BounceOut` for t >= 0.5

[BounceOutCurve](struct.BounceOutCurve.html "struct bevy::math::curve::BounceOutCurve")

bouncy at the end!

[ChainCurve](struct.ChainCurve.html "struct bevy::math::curve::ChainCurve")

The curve that results from chaining one curve with another. The second curve is effectively reparametrized so that its start is at the end of the first.

[CircularInCurve](struct.CircularInCurve.html "struct bevy::math::curve::CircularInCurve")

`f(t) = 1.0 - sqrt(1.0 - t²)`

[CircularInOutCurve](struct.CircularInOutCurve.html "struct bevy::math::curve::CircularInOutCurve")

Behaves as `CircularIn` for t < 0.5 and as `CircularOut` for t >= 0.5

[CircularOutCurve](struct.CircularOutCurve.html "struct bevy::math::curve::CircularOutCurve")

`f(t) = sqrt((2.0 - t) * t)`

[ConstantCurve](struct.ConstantCurve.html "struct bevy::math::curve::ConstantCurve")

A curve with a constant value over its domain.

[ContinuationCurve](struct.ContinuationCurve.html "struct bevy::math::curve::ContinuationCurve")

The curve that results from chaining two curves.

[CubicInCurve](struct.CubicInCurve.html "struct bevy::math::curve::CubicInCurve")

`f(t) = t³`

[CubicInOutCurve](struct.CubicInOutCurve.html "struct bevy::math::curve::CubicInOutCurve")

Behaves as `CubicIn` for t < 0.5 and as `CubicOut` for t >= 0.5

[CubicOutCurve](struct.CubicOutCurve.html "struct bevy::math::curve::CubicOutCurve")

`f(t) = (t - 1.0)³ + 1.0`

[CurveReparamCurve](struct.CurveReparamCurve.html "struct bevy::math::curve::CurveReparamCurve")

A curve that has been reparametrized by another curve, using that curve to transform the sample times before sampling. Curves of this type are produced by [`CurveExt::reparametrize_by_curve`](../../prelude/trait.CurveExt.html#method.reparametrize_by_curve "method bevy::prelude::CurveExt::reparametrize_by_curve").

[EasingCurve](struct.EasingCurve.html "struct bevy::math::curve::EasingCurve")

A [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") that is defined by

[ElasticCurve](struct.ElasticCurve.html "struct bevy::math::curve::ElasticCurve")

`f(omega,t) = 1 - (1 - t)²(2sin(omega * t) / omega + cos(omega * t))`, parametrized by `omega`

[ElasticInCurve](struct.ElasticInCurve.html "struct bevy::math::curve::ElasticInCurve")

`f(t) = -2.0^(10.0 * t - 10.0) * sin((t * 10.0 - 10.75) * 2.0 * π / 3.0)`

[ElasticInOutCurve](struct.ElasticInOutCurve.html "struct bevy::math::curve::ElasticInOutCurve")

Behaves as `ElasticIn` for t < 0.5 and as `ElasticOut` for t >= 0.5

[ElasticOutCurve](struct.ElasticOutCurve.html "struct bevy::math::curve::ElasticOutCurve")

`f(t) = 2.0^(-10.0 * t) * sin((t * 10.0 - 0.75) * 2.0 * π / 3.0) + 1.0`

[EvenCore](struct.EvenCore.html "struct bevy::math::curve::EvenCore")`alloc`

The data core of a curve derived from evenly-spaced samples. The intention is to use this in addition to explicit or inferred interpolation information in user-space in order to implement curves using [`domain`](../../prelude/struct.EvenCore.html#method.domain "method bevy::prelude::EvenCore::domain") and [`sample_with`](../../prelude/struct.EvenCore.html#method.sample_with "method bevy::prelude::EvenCore::sample_with").

[ExponentialInCurve](struct.ExponentialInCurve.html "struct bevy::math::curve::ExponentialInCurve")

`f(t) ≈ 2.0^(10.0 * (t - 1.0))`

[ExponentialInOutCurve](struct.ExponentialInOutCurve.html "struct bevy::math::curve::ExponentialInOutCurve")

Behaves as `ExponentialIn` for t < 0.5 and as `ExponentialOut` for t >= 0.5

[ExponentialOutCurve](struct.ExponentialOutCurve.html "struct bevy::math::curve::ExponentialOutCurve")

`f(t) ≈ 1.0 - 2.0^(-10.0 * t)`

[ForeverCurve](struct.ForeverCurve.html "struct bevy::math::curve::ForeverCurve")

The curve that results from repeating a curve forever.

[FunctionCurve](struct.FunctionCurve.html "struct bevy::math::curve::FunctionCurve")

A curve defined by a function together with a fixed domain.

[GraphCurve](struct.GraphCurve.html "struct bevy::math::curve::GraphCurve")

A curve that is the graph of another curve over its parameter space. Curves of this type are produced by [`CurveExt::graph`](../../prelude/trait.CurveExt.html#method.graph "method bevy::prelude::CurveExt::graph").

[Interval](struct.Interval.html "struct bevy::math::curve::Interval")

A nonempty closed interval, possibly unbounded in either direction.

[LinearCurve](struct.LinearCurve.html "struct bevy::math::curve::LinearCurve")

`f(t) = t`

[LinearReparamCurve](struct.LinearReparamCurve.html "struct bevy::math::curve::LinearReparamCurve")

A curve that has had its domain changed by a linear reparameterization (stretching and scaling). Curves of this type are produced by [`CurveExt::reparametrize_linear`](../../prelude/trait.CurveExt.html#method.reparametrize_linear "method bevy::prelude::CurveExt::reparametrize_linear").

[MapCurve](struct.MapCurve.html "struct bevy::math::curve::MapCurve")

A curve whose samples are defined by mapping samples from another curve through a given function. Curves of this type are produced by [`CurveExt::map`](../../prelude/trait.CurveExt.html#method.map "method bevy::prelude::CurveExt::map").

[PingPongCurve](struct.PingPongCurve.html "struct bevy::math::curve::PingPongCurve")

The curve that results from chaining a curve with its reversed version. The transition point is guaranteed to make no jump.

[QuadraticInCurve](struct.QuadraticInCurve.html "struct bevy::math::curve::QuadraticInCurve")

`f(t) = t²`

[QuadraticInOutCurve](struct.QuadraticInOutCurve.html "struct bevy::math::curve::QuadraticInOutCurve")

Behaves as `QuadraticIn` for t < 0.5 and as `QuadraticOut` for t >= 0.5

[QuadraticOutCurve](struct.QuadraticOutCurve.html "struct bevy::math::curve::QuadraticOutCurve")

`f(t) = -(t * (t - 2.0))`

[QuarticInCurve](struct.QuarticInCurve.html "struct bevy::math::curve::QuarticInCurve")

`f(t) = t⁴`

[QuarticInOutCurve](struct.QuarticInOutCurve.html "struct bevy::math::curve::QuarticInOutCurve")

Behaves as `QuarticIn` for t < 0.5 and as `QuarticOut` for t >= 0.5

[QuarticOutCurve](struct.QuarticOutCurve.html "struct bevy::math::curve::QuarticOutCurve")

`f(t) = 1.0 - (1.0 - t)⁴`

[QuinticInCurve](struct.QuinticInCurve.html "struct bevy::math::curve::QuinticInCurve")

`f(t) = t⁵`

[QuinticInOutCurve](struct.QuinticInOutCurve.html "struct bevy::math::curve::QuinticInOutCurve")

Behaves as `QuinticIn` for t < 0.5 and as `QuinticOut` for t >= 0.5

[QuinticOutCurve](struct.QuinticOutCurve.html "struct bevy::math::curve::QuinticOutCurve")

`f(t) = (t - 1.0)⁵ + 1.0`

[ReparamCurve](struct.ReparamCurve.html "struct bevy::math::curve::ReparamCurve")

A curve whose sample space is mapped onto that of some base curve’s before sampling. Curves of this type are produced by [`CurveExt::reparametrize`](../../prelude/trait.CurveExt.html#method.reparametrize "method bevy::prelude::CurveExt::reparametrize").

[RepeatCurve](struct.RepeatCurve.html "struct bevy::math::curve::RepeatCurve")

The curve that results from repeating a curve `N` times.

[ReverseCurve](struct.ReverseCurve.html "struct bevy::math::curve::ReverseCurve")

The curve that results from reversing another.

[SampleAutoCurve](struct.SampleAutoCurve.html "struct bevy::math::curve::SampleAutoCurve")

A curve that is defined by neighbor interpolation over a set of evenly-spaced samples, interpolated automatically using [a particularly well-behaved interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[SampleCurve](struct.SampleCurve.html "struct bevy::math::curve::SampleCurve")

A curve that is defined by explicit neighbor interpolation over a set of evenly-spaced samples.

[SineInCurve](struct.SineInCurve.html "struct bevy::math::curve::SineInCurve")

`f(t) = 1.0 - cos(t * π / 2.0)`

[SineInOutCurve](struct.SineInOutCurve.html "struct bevy::math::curve::SineInOutCurve")

Behaves as `SineIn` for t < 0.5 and as `SineOut` for t >= 0.5

[SineOutCurve](struct.SineOutCurve.html "struct bevy::math::curve::SineOutCurve")

`f(t) = sin(t * π / 2.0)`

[SmoothStepCurve](struct.SmoothStepCurve.html "struct bevy::math::curve::SmoothStepCurve")

`f(t) = 3t² - 2t³`

[SmoothStepInCurve](struct.SmoothStepInCurve.html "struct bevy::math::curve::SmoothStepInCurve")

Behaves as the first half of [`SmoothStepCurve`](../../prelude/struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmoothStepOutCurve](struct.SmoothStepOutCurve.html "struct bevy::math::curve::SmoothStepOutCurve")

Behaves as the second half of [`SmoothStepCurve`](../../prelude/struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmootherStepCurve](struct.SmootherStepCurve.html "struct bevy::math::curve::SmootherStepCurve")

`f(t) = 6t⁵ - 15t⁴ + 10t³`

[SmootherStepInCurve](struct.SmootherStepInCurve.html "struct bevy::math::curve::SmootherStepInCurve")

Behaves as the first half of [`SmootherStepCurve`](../../prelude/struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[SmootherStepOutCurve](struct.SmootherStepOutCurve.html "struct bevy::math::curve::SmootherStepOutCurve")

Behaves as the second half of [`SmootherStepCurve`](../../prelude/struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[StepsCurve](struct.StepsCurve.html "struct bevy::math::curve::StepsCurve")

`n` steps connecting the start and the end. Jumping behavior is customizable via [`JumpAt`](../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt"). See [`JumpAt`](../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt") for all the options and visual examples.

[UnevenCore](struct.UnevenCore.html "struct bevy::math::curve::UnevenCore")`alloc`

The data core of a curve defined by unevenly-spaced samples or keyframes. The intention is to use this in concert with implicitly or explicitly-defined interpolation in user-space in order to implement the curve interface using [`domain`](../../prelude/struct.UnevenCore.html#method.domain "method bevy::prelude::UnevenCore::domain") and [`sample_with`](../../prelude/struct.UnevenCore.html#method.sample_with "method bevy::prelude::UnevenCore::sample_with").

[UnevenSampleAutoCurve](struct.UnevenSampleAutoCurve.html "struct bevy::math::curve::UnevenSampleAutoCurve")

A curve that is defined by interpolation over unevenly spaced samples, interpolated automatically using [a particularly well-behaved interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[UnevenSampleCurve](struct.UnevenSampleCurve.html "struct bevy::math::curve::UnevenSampleCurve")

A curve that is defined by interpolation over unevenly spaced samples with explicit interpolation.

[ZipCurve](struct.ZipCurve.html "struct bevy::math::curve::ZipCurve")

A curve that combines the output data from two constituent curves into a tuple output. Curves of this type are produced by [`CurveExt::zip`](../../prelude/trait.CurveExt.html#method.zip "method bevy::prelude::CurveExt::zip").

## Enums

[ChainError](enum.ChainError.html "enum bevy::math::curve::ChainError")

An error indicating that an end-to-end composition couldn’t be performed because of malformed inputs.

[EaseFunction](enum.EaseFunction.html "enum bevy::math::curve::EaseFunction")

Curve functions over the [unit interval](../../prelude/struct.Interval.html#associatedconstant.UNIT "associated constant bevy::prelude::Interval::UNIT"), commonly used for easing transitions.

[JumpAt](enum.JumpAt.html "enum bevy::math::curve::JumpAt")

Configuration options for the [`EaseFunction::Steps`](../../prelude/enum.EaseFunction.html#variant.Steps "variant bevy::prelude::EaseFunction::Steps") curves. This closely replicates the [CSS step function specification](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/steps#description).

[LinearReparamError](enum.LinearReparamError.html "enum bevy::math::curve::LinearReparamError")

An error indicating that a linear reparameterization couldn’t be performed because of malformed inputs.

[PingPongError](enum.PingPongError.html "enum bevy::math::curve::PingPongError")

An error indicating that a ping ponging of a curve couldn’t be performed because of malformed inputs.

[RepeatError](enum.RepeatError.html "enum bevy::math::curve::RepeatError")

An error indicating that a repetition of a curve couldn’t be performed because of malformed inputs.

[ResamplingError](enum.ResamplingError.html "enum bevy::math::curve::ResamplingError")

An error indicating that a resampling operation could not be performed because of malformed inputs.

[ReverseError](enum.ReverseError.html "enum bevy::math::curve::ReverseError")

An error indicating that a reversion of a curve couldn’t be performed because of malformed inputs.

## Traits

[Curve](trait.Curve.html "trait bevy::math::curve::Curve")

A trait for a type that can represent values of type `T` parametrized over a fixed interval.

[CurveExt](trait.CurveExt.html "trait bevy::math::curve::CurveExt")

Extension trait implemented by [curves](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), allowing access to a number of adaptors and convenience methods.

[CurveResampleExt](trait.CurveResampleExt.html "trait bevy::math::curve::CurveResampleExt")`alloc`

Extension trait implemented by [curves](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), allowing access to generic resampling methods as well as those based on [stable interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[Ease](trait.Ease.html "trait bevy::math::curve::Ease")

A type whose values can be eased between.

## Functions

[interval](fn.interval.html "fn bevy::math::curve::interval")

Create an [`Interval`](../../prelude/struct.Interval.html "struct bevy::prelude::Interval") with a given `start` and `end`. Alias of [`Interval::new`](../../prelude/struct.Interval.html#method.new "associated function bevy::prelude::Interval::new").