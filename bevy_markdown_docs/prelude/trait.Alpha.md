[bevy](../index.html)::[prelude](index.html)

# Trait Alpha 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#59)

```rust
pub trait Alpha: Sized {
    // Required methods
    fn with_alpha(&self, alpha: f32) -> Self;
    fn alpha(&self) -> f32;
    fn set_alpha(&mut self, alpha: f32);

    // Provided methods
    fn is_fully_transparent(&self) -> bool { ... }
    fn is_fully_opaque(&self) -> bool { ... }
}
```

Methods for manipulating alpha values.

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#61)

#### fn [with\_alpha](#tymethod.with_alpha)(&self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a new version of this color with the given alpha value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#64)

#### fn [alpha](#tymethod.alpha)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the alpha component of this color.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#67)

#### fn [set\_alpha](#tymethod.set_alpha)(&mut self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Sets the alpha component of this color.

## Provided Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#70)

#### fn [is\_fully\_transparent](#method.is_fully_transparent)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Is the alpha component of this color less than or equal to 0.0?

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#75)

#### fn [is\_fully\_opaque](#method.is_fully_opaque)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Is the alpha component of this color greater than or equal to 1.0?

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#80)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#81)

#### fn [with\_alpha](#tymethod.with_alpha)(&self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#85)

#### fn [alpha](#tymethod.alpha)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#89)

#### fn [set\_alpha](#tymethod.set_alpha)(&mut self, alpha: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#519)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Color](enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#132)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Hsla](struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#103)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Hsva](struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#106)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Hwba](struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#114)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Laba](struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#136)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Lcha](struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#225)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#114)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#131)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Oklcha](struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#275)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Srgba](struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#94)

### impl [Alpha](trait.Alpha.html "trait bevy::prelude::Alpha") for [Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")