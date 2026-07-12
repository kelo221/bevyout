[bevy](../index.html)::[prelude](index.html)

# Trait Animatable 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#20)

```rust
pub trait Animatable:
    Sized
    + Reflect
    + Send
    + Sync
    + 'static {
    // Required methods
    fn interpolate(a: &Self, b: &Self, time: f32) -> Self;
    fn blend(inputs: impl Iterator<Item = BlendInput<Self>>) -> Self;
}
```

An animatable value type.

## Required Methods

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#24)

#### fn [interpolate](#tymethod.interpolate)(a: &Self, b: &Self, time: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Interpolates between `a` and `b` with an interpolation factor of `time`.

The `time` parameter here may not be clamped to the range `[0.0, 1.0]`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#29)

#### fn [blend](#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](struct.BlendInput.html "struct bevy::prelude::BlendInput")<Self>>) -> Self

Blends one or more values together.

Implementors should return a default value when no inputs are provided here.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#119)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#121)

#### fn [interpolate](#tymethod.interpolate)(a: &[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), b: &[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#126)

#### fn [blend](#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](struct.BlendInput.html "struct bevy::prelude::BlendInput")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#82)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#82)

#### fn [interpolate](#tymethod.interpolate)(a: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#82)

#### fn [blend](#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](struct.BlendInput.html "struct bevy::prelude::BlendInput")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#87)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#87)

#### fn [interpolate](#tymethod.interpolate)(a: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), b: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#87)

#### fn [blend](#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](struct.BlendInput.html "struct bevy::prelude::BlendInput")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>>) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#88)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [DVec2](../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#89)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [DVec3](../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#90)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#93)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Laba](struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#92)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#94)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#172)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Quat](struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#200)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Rot2](struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#95)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Srgba](struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#133)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#83)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Vec2](struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#99)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#85)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Vec4](struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#84)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Vec3A](struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#96)

### impl [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") for [Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")