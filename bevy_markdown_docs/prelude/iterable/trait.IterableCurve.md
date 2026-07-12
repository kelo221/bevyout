[bevy](../../index.html)::[prelude](../index.html)::[iterable](index.html)

# Trait IterableCurve 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/iterable.rs.html#14)

```rust
pub trait IterableCurve<T> {
    // Required methods
    fn domain(&self) -> Interval;
    fn sample_iter_unchecked(&self, t: f32) -> impl Iterator<Item = T>;

    // Provided methods
    fn sample_iter_clamped(&self, t: f32) -> impl Iterator<Item = T> { ... }
    fn sample_iter(&self, t: f32) -> Option<impl Iterator<Item = T>> { ... }
}
```

A curve which provides samples in the form of [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")s.

This is an abstraction that provides an interface for curves which look like `Curve<Vec<T>>` but side-stepping issues with allocation on sampling. This happens when the size of an output array cannot be known statically.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/iterable.rs.html#16)

#### fn [domain](#tymethod.domain)(&self) -> [Interval](../struct.Interval.html "struct bevy::prelude::Interval")

The interval over which this curve is parametrized.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/iterable.rs.html#25)

#### fn [sample\_iter\_unchecked](#tymethod.sample_iter_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample a point on this curve at the parameter value `t`, producing an iterator over values. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain.

Values sampled from outside of a curve’s domain are generally considered invalid; data which is nonsensical or otherwise useless may be returned in such a circumstance, and extrapolation beyond a curve’s domain should not be relied upon.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/iterable.rs.html#29)

#### fn [sample\_iter\_clamped](#method.sample_iter_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample this curve at a specified time `t`, producing an iterator over sampled values. The parameter `t` is clamped to the domain of the curve.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/iterable.rs.html#36)

#### fn [sample\_iter](#method.sample_iter)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>>

Sample this curve at a specified time `t`, producing an iterator over sampled values. If the parameter `t` does not lie in the curve’s domain, `None` is returned.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/iterable.rs.html#46-48)

### impl<T> [IterableCurve](trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve")<T> for [ConstantCurve](../struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<[Vec](../struct.Vec.html "struct bevy::prelude::Vec")<T>>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#290-292)

### impl<T> [IterableCurve](trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve")<T> for [WideCubicKeyframeCurve](../../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>

where T: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#180-182)

### impl<T> [IterableCurve](trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve")<T> for [WideLinearKeyframeCurve](../../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>

where T: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#234-236)

### impl<T> [IterableCurve](trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve")<T> for [WideSteppedKeyframeCurve](../../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),