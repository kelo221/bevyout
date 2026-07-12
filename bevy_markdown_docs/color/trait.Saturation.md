[bevy](../index.html)::[color](index.html)

# Trait Saturation 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#116)

```rust
pub trait Saturation: Sized {
    // Required methods
    fn with_saturation(&self, saturation: f32) -> Self;
    fn saturation(&self) -> f32;
    fn set_saturation(&mut self, saturation: f32);
}
```

Trait for manipulating the saturation of a color.

When working with color spaces that do not have native saturation components the operations are performed in [`Hsla`](../prelude/struct.Hsla.html "struct bevy::prelude::Hsla").

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#118)

#### fn [with\_saturation](#tymethod.with_saturation)(&self, saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Return a new version of this color with the saturation channel set to the given value.

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#121)

#### fn [saturation](#tymethod.saturation)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Return the saturation of this color \[0.0, 1.0\].

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#124)

#### fn [set\_saturation](#tymethod.set_saturation)(&mut self, saturation: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Sets the saturation of this color.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#857)

### impl [Saturation](../prelude/trait.Saturation.html "trait bevy::prelude::Saturation") for [Color](../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#166)

### impl [Saturation](../prelude/trait.Saturation.html "trait bevy::prelude::Saturation") for [Hsla](../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#137)

### impl [Saturation](../prelude/trait.Saturation.html "trait bevy::prelude::Saturation") for [Hsva](../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")