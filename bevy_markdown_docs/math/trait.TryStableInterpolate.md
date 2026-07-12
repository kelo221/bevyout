[bevy](../index.html)::[math](index.html)

# Trait TryStableInterpolate 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#577)

```rust
pub trait TryStableInterpolate: Clone {
    type Error;

    // Required method
    fn try_interpolate_stable(
        &self,
        other: &Self,
        t: f32,
    ) -> Result<Self, Self::Error>;
}
```

A trait that indicates that a value _may_ be interpolable via [`StableInterpolate`](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"). An interpolation may fail if the values have different units - for example, attempting to interpolate between [`Val::Px`](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html#variant.Px) and [`Val::Percent`](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html#variant.Percent) will fail, even though they are the same Rust type.

Fallible interpolation can be used for animated transitions, which can be set up to fail gracefully if the values cannot be interpolated. For example, a transition could smoothly go from `Val::Px(10)` to `Val::Px(20)`, but if the user attempts to go from `Val::Px(10)` to `Val::Percent(10)`, the animation player can detect the failure and simply snap to the new value without interpolating.

An animation clip system can incorporate fallible interpolation to support a broad set of sequenced parameter values. This can include numeric types, which always interpolate, enum types, which may or may not interpolate depending on the units, and non-interpolable types, which always jump immediately to the new value without interpolation. This means, for example, that you can have an animation track whose value type is a boolean or a string.

Interpolation for simple number and coordinate types will always succeed, as will any type that implements [`StableInterpolate`](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"). Types which have different variants such as [`Val`](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html) and [`Color`](https://docs.rs/bevy/latest/bevy/color/enum.Color.html) will only fail if the units are different. Note that [`Color`](https://docs.rs/bevy/latest/bevy/color/enum.Color.html) has its own, non-fallible mixing methods, but those entail automatically converting between different color spaces, and is both expensive and complex. [`TryStableInterpolate`](trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") is more conservative, and doesn’t automatically convert between color spaces. This produces a color interpolation that has more predictable performance.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#579)

#### type [Error](#associatedtype.Error)

Error produced when the value cannot be interpolated.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#583)

#### fn [try\_interpolate\_stable](#tymethod.try_interpolate_stable)( &self, other: &Self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, Self::[Error](trait.TryStableInterpolate.html#associatedtype.Error "type bevy::math::TryStableInterpolate::Error")\>

Attempt to interpolate the value. This may fail if the two interpolation values have different units, or if the type is not interpolable.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#933)

### impl [TryStableInterpolate](trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") for [Color](../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#934)

#### type [Error](#associatedtype.Error) = [MismatchedUnitsError](struct.MismatchedUnitsError.html "struct bevy::math::MismatchedUnitsError")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#486)

### impl [TryStableInterpolate](trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") for [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#487)

#### type [Error](#associatedtype.Error) = [MismatchedUnitsError](struct.MismatchedUnitsError.html "struct bevy::math::MismatchedUnitsError")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#586)

### impl<T> [TryStableInterpolate](trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") for T

where T: [StableInterpolate](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#587)

#### type [Error](#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")