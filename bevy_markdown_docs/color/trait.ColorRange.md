[bevy](../index.html)::[color](index.html)

# Trait ColorRange 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_range.rs.html#10)

```rust
pub trait ColorRange<T>where
    T: Mix,{
    // Required method
    fn at(&self, factor: f32) -> T;
}
```

Represents a range of colors that can be linearly interpolated, defined by a start and end point which must be in the same color space. It works for any color type that implements [`Mix`](../prelude/trait.Mix.html "trait bevy::prelude::Mix").

This is useful for defining gradients or animated color transitions.

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_range.rs.html#13)

#### fn [at](#tymethod.at)(&self, factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

Get the color value at the given interpolation factor, which should be between 0.0 (start) and 1.0 (end).

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_range.rs.html#16)

### impl<T> [ColorRange](trait.ColorRange.html "trait bevy::color::ColorRange")<T> for [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>

where T: [Mix](../prelude/trait.Mix.html "trait bevy::prelude::Mix"),

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_range.rs.html#17)

#### fn [at](#tymethod.at)(&self, factor: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

## Implementors