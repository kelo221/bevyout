[bevy](../../index.html)::[prelude](../index.html)::[easing](index.html)

# Trait Ease 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#87)

```rust
pub trait Ease: Sized {
    // Required method
    fn interpolating_curve_unbounded(start: Self, end: Self) -> impl Curve<Self>;
}
```

A type whose values can be eased between.

This requires the construction of an interpolation curve that actually extends beyond the curve segment that connects two values, because an easing curve may extrapolate before the starting value and after the ending value. This is especially common in easing functions that mimic elastic or springlike behavior.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#95)

#### fn [interpolating\_curve\_unbounded](#tymethod.interpolating_curve_unbounded)(start: Self, end: Self) -> impl [Curve](../trait.Curve.html "trait bevy::prelude::Curve")<Self>

Given `start` and `end` values, produce a curve with [unlimited domain](../struct.Interval.html#associatedconstant.EVERYWHERE "associated constant bevy::prelude::Interval::EVERYWHERE") that:

*   takes a value equivalent to `start` at `t = 0`
*   takes a value equivalent to `end` at `t = 1`
*   has constant speed everywhere, including outside of `[0, 1]`

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#203-209)

### impl<T> [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [(T₁, T₂, …, Tₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where T: [Ease](../trait.Ease.html "trait bevy::prelude::Ease"),

This trait is implemented for tuples up to 11 items long.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#203-209)

#### fn [interpolating\_curve\_unbounded](#tymethod.interpolating_curve_unbounded)(start: [(T,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html), end: [(T,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)) -> impl [Curve](../trait.Curve.html "trait bevy::prelude::Curve")<[(T,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#122)

### impl [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [Dir2](../struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#128)

### impl [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [Dir3](../struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#135)

### impl [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [Dir3A](../struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#161)

### impl [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [Isometry2d](../struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#143)

### impl [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [Isometry3d](../struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#110)

### impl [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [Quat](../struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#104)

### impl [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for [Rot2](../struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#98)

### impl<V> [Ease](../trait.Ease.html "trait bevy::prelude::Ease") for V

where V: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,