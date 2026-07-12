[bevy](../../index.html)::[color](../index.html)::[prelude](index.html)

# Trait Mix 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#33)

```rust
pub trait Mix: Sized {
    // Required method
    fn mix(&self, other: &Self, factor: f32) -> Self;

    // Provided method
    fn mix_assign(&mut self, other: Self, factor: f32) { ... }
}
```

Linear interpolation of two colors within a given color space.

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#36)

#### fn [mix](#tymethod.mix)(&self, other: &Self, factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Linearly interpolate between this and another color, by factor. Factor should be between 0.0 and 1.0.

## Provided Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#40)

#### fn [mix\_assign](#method.mix_assign)(&mut self, other: Self, factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Linearly interpolate between this and another color, by factor, storing the result in this color. Factor should be between 0.0 and 1.0.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#895)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#114)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#85)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#88)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#96)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#118)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#207)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#96)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#113)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#262)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#139)

### impl [Mix](../../prelude/trait.Mix.html "trait bevy::prelude::Mix") for [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")