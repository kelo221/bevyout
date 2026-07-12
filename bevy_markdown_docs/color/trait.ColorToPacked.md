[bevy](../index.html)::[color](index.html)

# Trait ColorToPacked 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#148)

```rust
pub trait ColorToPacked {
    // Required methods
    fn to_u8_array(self) -> [u8; 4];
    fn to_u8_array_no_alpha(self) -> [u8; 3];
    fn from_u8_array(color: [u8; 4]) -> Self;
    fn from_u8_array_no_alpha(color: [u8; 3]) -> Self;
}
```

Trait with methods for converting colors to packed non-color types

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#150)

#### fn [to\_u8\_array](#tymethod.to_u8_array)(self) -> \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Convert to \[u8; 4\] where that makes sense (Srgba is most relevant)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#152)

#### fn [to\_u8\_array\_no\_alpha](#tymethod.to_u8_array_no_alpha)(self) -> \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Convert to \[u8; 3\] where that makes sense (Srgba is most relevant)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#154)

#### fn [from\_u8\_array](#tymethod.from_u8_array)(color: \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> Self

Convert from \[u8; 4\] where that makes sense (Srgba is most relevant)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#156)

#### fn [from\_u8\_array\_no\_alpha](#tymethod.from_u8_array_no_alpha)(color: \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> Self

Convert to \[u8; 3\] where that makes sense (Srgba is most relevant)

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#306)

### impl [ColorToPacked](../prelude/trait.ColorToPacked.html "trait bevy::prelude::ColorToPacked") for [LinearRgba](../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#361)

### impl [ColorToPacked](../prelude/trait.ColorToPacked.html "trait bevy::prelude::ColorToPacked") for [Srgba](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")