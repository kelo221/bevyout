[bevy](../../index.html)::[ui](../index.html)::[prelude](index.html)

# Trait InColorSpace 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#663)

```rust
pub trait InColorSpace: Sized {
    // Required method
    fn in_color_space(self, color_space: InterpolationColorSpace) -> Self;

    // Provided methods
    fn in_oklaba(self) -> Self { ... }
    fn in_oklch(self) -> Self { ... }
    fn in_oklch_long(self) -> Self { ... }
    fn in_srgb(self) -> Self { ... }
    fn in_linear_rgb(self) -> Self { ... }
}
```

Set the color space used for interpolation.

## Required Methods

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#665)

#### fn [in\_color\_space](#tymethod.in_color_space)(self, color\_space: [InterpolationColorSpace](../../prelude/enum.InterpolationColorSpace.html "enum bevy::prelude::InterpolationColorSpace")) -> Self

Interpolate in the given `color_space`.

## Provided Methods

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#668)

#### fn [in\_oklaba](#method.in_oklaba)(self) -> Self

Interpolate in `OKLab` space.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#673)

#### fn [in\_oklch](#method.in_oklch)(self) -> Self

Interpolate in OKLCH space (short hue path).

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#678)

#### fn [in\_oklch\_long](#method.in_oklch_long)(self) -> Self

Interpolate in OKLCH space (long hue path).

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#683)

#### fn [in\_srgb](#method.in_srgb)(self) -> Self

Interpolate in sRGB space.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#688)

#### fn [in\_linear\_rgb](#method.in_linear_rgb)(self) -> Self

Interpolate in linear sRGB space.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#709)

### impl [InColorSpace](../../prelude/trait.InColorSpace.html "trait bevy::prelude::InColorSpace") for [ConicGradient](../../prelude/struct.ConicGradient.html "struct bevy::prelude::ConicGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#693)

### impl [InColorSpace](../../prelude/trait.InColorSpace.html "trait bevy::prelude::InColorSpace") for [LinearGradient](../../prelude/struct.LinearGradient.html "struct bevy::prelude::LinearGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#701)

### impl [InColorSpace](../../prelude/trait.InColorSpace.html "trait bevy::prelude::InColorSpace") for [RadialGradient](../../prelude/struct.RadialGradient.html "struct bevy::prelude::RadialGradient")