[bevy](../../index.html)::[color](../index.html)::[prelude](index.html)

# Trait Luminance 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#6)

```rust
pub trait Luminance: Sized {
    // Required methods
    fn luminance(&self) -> f32;
    fn with_luminance(&self, value: f32) -> Self;
    fn darker(&self, amount: f32) -> Self;
    fn lighter(&self, amount: f32) -> Self;
}
```

Methods for changing the luminance of a color. Note that these methods are not guaranteed to produce consistent results across color spaces, but will be within a given space.

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#8)

#### fn [luminance](#tymethod.luminance)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the luminance of this color (0.0 - 1.0).

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#13)

#### fn [with\_luminance](#tymethod.with_luminance)(&self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a new version of this color with the given luminance. The resulting color will be clamped to the valid range for the color space; for some color spaces, clamping may cause the hue or chroma to change.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#21)

#### fn [darker](#tymethod.darker)(&self, amount: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a darker version of this color. The `amount` should be between 0.0 and 1.0. The amount represents an absolute decrease in luminance, and is distributive: `color.darker(a).darker(b) == color.darker(a + b)`. Colors are clamped to black if the amount would cause them to go below black.

For a relative decrease in luminance, you can simply `mix()` with black.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#29)

#### fn [lighter](#tymethod.lighter)(&self, amount: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a lighter version of this color. The `amount` should be between 0.0 and 1.0. The amount represents an absolute increase in luminance, and is distributive: `color.lighter(a).lighter(b) == color.lighter(a + b)`. Colors are clamped to white if the amount would cause them to go above white.

For a relative increase in luminance, you can simply `mix()` with white.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#743)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#186)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#131)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#170)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#173)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#131)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#165)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#234)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#111)

### impl [Luminance](../../prelude/trait.Luminance.html "trait bevy::prelude::Luminance") for [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")