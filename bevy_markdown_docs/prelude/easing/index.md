[bevy](../../index.html)::[prelude](../index.html)

# Module easing 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#291)

Module containing different easing functions.

An easing function is a [`Curve`](../trait.Curve.html "trait bevy::prelude::Curve") that’s used to transition between two values. It takes a time parameter, where a time of zero means the start of the transition and a time of one means the end.

Easing functions come in a variety of shapes - one might [transition smoothly](../struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve"), while another might have a [bouncing motion](../struct.BounceInCurve.html "struct bevy::prelude::BounceInCurve").

There are several ways to use easing functions. The simplest option is a struct thats represents a single easing function, like [`SmoothStepCurve`](../struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve") and [`StepsCurve`](../struct.StepsCurve.html "struct bevy::prelude::StepsCurve"). These structs can only transition from a value of zero to a value of one.

```rust
let smoothed_value = SmoothStepCurve.sample(time);
```

```rust
let stepped_value = StepsCurve(5, JumpAt::Start).sample(time);
```

Another option is [`EaseFunction`](../enum.EaseFunction.html "enum bevy::prelude::EaseFunction"). Unlike the single function structs, which require you to choose a function at compile time, `EaseFunction` lets you choose at runtime. It can also be serialized.

```rust
let mut curve = EaseFunction::Linear;

if make_it_smooth {
    curve = EaseFunction::SmoothStep;
}

let value = curve.sample(time);
```

The final option is [`EasingCurve`](../struct.EasingCurve.html "struct bevy::prelude::EasingCurve"). This lets you transition between any two values - not just zero to one. `EasingCurve` can use any value that implements the [`Ease`](../trait.Ease.html "trait bevy::prelude::Ease") trait, including vectors and directions.

```rust
// Make a curve that smoothly transitions between two positions.
let start_position = vec2(1.0, 2.0);
let end_position = vec2(5.0, 10.0);
let curve = EasingCurve::new(start_position, end_position, EaseFunction::SmoothStep);

let smoothed_position = curve.sample(time);
```

Like `EaseFunction`, the values and easing function of `EasingCurve` can be chosen at runtime and serialized.

## Structs

[BackInCurve](struct.BackInCurve.html "struct bevy::prelude::easing::BackInCurve")

`f(t) = 2.70158 * t³ - 1.70158 * t²`

[BackInOutCurve](struct.BackInOutCurve.html "struct bevy::prelude::easing::BackInOutCurve")

Behaves as `BackIn` for t < 0.5 and as `BackOut` for t >= 0.5

[BackOutCurve](struct.BackOutCurve.html "struct bevy::prelude::easing::BackOutCurve")

`f(t) = 1.0 + 2.70158 * (t - 1.0)³ + 1.70158 * (t - 1.0)²`

[BounceInCurve](struct.BounceInCurve.html "struct bevy::prelude::easing::BounceInCurve")

bouncy at the start!

[BounceInOutCurve](struct.BounceInOutCurve.html "struct bevy::prelude::easing::BounceInOutCurve")

Behaves as `BounceIn` for t < 0.5 and as `BounceOut` for t >= 0.5

[BounceOutCurve](struct.BounceOutCurve.html "struct bevy::prelude::easing::BounceOutCurve")

bouncy at the end!

[CircularInCurve](struct.CircularInCurve.html "struct bevy::prelude::easing::CircularInCurve")

`f(t) = 1.0 - sqrt(1.0 - t²)`

[CircularInOutCurve](struct.CircularInOutCurve.html "struct bevy::prelude::easing::CircularInOutCurve")

Behaves as `CircularIn` for t < 0.5 and as `CircularOut` for t >= 0.5

[CircularOutCurve](struct.CircularOutCurve.html "struct bevy::prelude::easing::CircularOutCurve")

`f(t) = sqrt((2.0 - t) * t)`

[CubicInCurve](struct.CubicInCurve.html "struct bevy::prelude::easing::CubicInCurve")

`f(t) = t³`

[CubicInOutCurve](struct.CubicInOutCurve.html "struct bevy::prelude::easing::CubicInOutCurve")

Behaves as `CubicIn` for t < 0.5 and as `CubicOut` for t >= 0.5

[CubicOutCurve](struct.CubicOutCurve.html "struct bevy::prelude::easing::CubicOutCurve")

`f(t) = (t - 1.0)³ + 1.0`

[EasingCurve](struct.EasingCurve.html "struct bevy::prelude::easing::EasingCurve")

A [`Curve`](../trait.Curve.html "trait bevy::prelude::Curve") that is defined by

[ElasticCurve](struct.ElasticCurve.html "struct bevy::prelude::easing::ElasticCurve")

`f(omega,t) = 1 - (1 - t)²(2sin(omega * t) / omega + cos(omega * t))`, parametrized by `omega`

[ElasticInCurve](struct.ElasticInCurve.html "struct bevy::prelude::easing::ElasticInCurve")

`f(t) = -2.0^(10.0 * t - 10.0) * sin((t * 10.0 - 10.75) * 2.0 * π / 3.0)`

[ElasticInOutCurve](struct.ElasticInOutCurve.html "struct bevy::prelude::easing::ElasticInOutCurve")

Behaves as `ElasticIn` for t < 0.5 and as `ElasticOut` for t >= 0.5

[ElasticOutCurve](struct.ElasticOutCurve.html "struct bevy::prelude::easing::ElasticOutCurve")

`f(t) = 2.0^(-10.0 * t) * sin((t * 10.0 - 0.75) * 2.0 * π / 3.0) + 1.0`

[ExponentialInCurve](struct.ExponentialInCurve.html "struct bevy::prelude::easing::ExponentialInCurve")

`f(t) ≈ 2.0^(10.0 * (t - 1.0))`

[ExponentialInOutCurve](struct.ExponentialInOutCurve.html "struct bevy::prelude::easing::ExponentialInOutCurve")

Behaves as `ExponentialIn` for t < 0.5 and as `ExponentialOut` for t >= 0.5

[ExponentialOutCurve](struct.ExponentialOutCurve.html "struct bevy::prelude::easing::ExponentialOutCurve")

`f(t) ≈ 1.0 - 2.0^(-10.0 * t)`

[LinearCurve](struct.LinearCurve.html "struct bevy::prelude::easing::LinearCurve")

`f(t) = t`

[QuadraticInCurve](struct.QuadraticInCurve.html "struct bevy::prelude::easing::QuadraticInCurve")

`f(t) = t²`

[QuadraticInOutCurve](struct.QuadraticInOutCurve.html "struct bevy::prelude::easing::QuadraticInOutCurve")

Behaves as `QuadraticIn` for t < 0.5 and as `QuadraticOut` for t >= 0.5

[QuadraticOutCurve](struct.QuadraticOutCurve.html "struct bevy::prelude::easing::QuadraticOutCurve")

`f(t) = -(t * (t - 2.0))`

[QuarticInCurve](struct.QuarticInCurve.html "struct bevy::prelude::easing::QuarticInCurve")

`f(t) = t⁴`

[QuarticInOutCurve](struct.QuarticInOutCurve.html "struct bevy::prelude::easing::QuarticInOutCurve")

Behaves as `QuarticIn` for t < 0.5 and as `QuarticOut` for t >= 0.5

[QuarticOutCurve](struct.QuarticOutCurve.html "struct bevy::prelude::easing::QuarticOutCurve")

`f(t) = 1.0 - (1.0 - t)⁴`

[QuinticInCurve](struct.QuinticInCurve.html "struct bevy::prelude::easing::QuinticInCurve")

`f(t) = t⁵`

[QuinticInOutCurve](struct.QuinticInOutCurve.html "struct bevy::prelude::easing::QuinticInOutCurve")

Behaves as `QuinticIn` for t < 0.5 and as `QuinticOut` for t >= 0.5

[QuinticOutCurve](struct.QuinticOutCurve.html "struct bevy::prelude::easing::QuinticOutCurve")

`f(t) = (t - 1.0)⁵ + 1.0`

[SineInCurve](struct.SineInCurve.html "struct bevy::prelude::easing::SineInCurve")

`f(t) = 1.0 - cos(t * π / 2.0)`

[SineInOutCurve](struct.SineInOutCurve.html "struct bevy::prelude::easing::SineInOutCurve")

Behaves as `SineIn` for t < 0.5 and as `SineOut` for t >= 0.5

[SineOutCurve](struct.SineOutCurve.html "struct bevy::prelude::easing::SineOutCurve")

`f(t) = sin(t * π / 2.0)`

[SmoothStepCurve](struct.SmoothStepCurve.html "struct bevy::prelude::easing::SmoothStepCurve")

`f(t) = 3t² - 2t³`

[SmoothStepInCurve](struct.SmoothStepInCurve.html "struct bevy::prelude::easing::SmoothStepInCurve")

Behaves as the first half of [`SmoothStepCurve`](../struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmoothStepOutCurve](struct.SmoothStepOutCurve.html "struct bevy::prelude::easing::SmoothStepOutCurve")

Behaves as the second half of [`SmoothStepCurve`](../struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmootherStepCurve](struct.SmootherStepCurve.html "struct bevy::prelude::easing::SmootherStepCurve")

`f(t) = 6t⁵ - 15t⁴ + 10t³`

[SmootherStepInCurve](struct.SmootherStepInCurve.html "struct bevy::prelude::easing::SmootherStepInCurve")

Behaves as the first half of [`SmootherStepCurve`](../struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[SmootherStepOutCurve](struct.SmootherStepOutCurve.html "struct bevy::prelude::easing::SmootherStepOutCurve")

Behaves as the second half of [`SmootherStepCurve`](../struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[StepsCurve](struct.StepsCurve.html "struct bevy::prelude::easing::StepsCurve")

`n` steps connecting the start and the end. Jumping behavior is customizable via [`JumpAt`](../enum.JumpAt.html "enum bevy::prelude::JumpAt"). See [`JumpAt`](../enum.JumpAt.html "enum bevy::prelude::JumpAt") for all the options and visual examples.

## Enums

[EaseFunction](enum.EaseFunction.html "enum bevy::prelude::easing::EaseFunction")

Curve functions over the [unit interval](../struct.Interval.html#associatedconstant.UNIT "associated constant bevy::prelude::Interval::UNIT"), commonly used for easing transitions.

[JumpAt](enum.JumpAt.html "enum bevy::prelude::easing::JumpAt")

Configuration options for the [`EaseFunction::Steps`](../enum.EaseFunction.html#variant.Steps "variant bevy::prelude::EaseFunction::Steps") curves. This closely replicates the [CSS step function specification](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/steps#description).

## Traits

[Ease](trait.Ease.html "trait bevy::prelude::easing::Ease")

A type whose values can be eased between.