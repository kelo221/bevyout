[bevy](../../index.html)::[color](../index.html)::[prelude](index.html)

# Trait ColorToComponents 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#128)

```rust
pub trait ColorToComponents {
    // Required methods
    fn to_f32_array(self) -> [f32; 4];
    fn to_f32_array_no_alpha(self) -> [f32; 3];
    fn to_vec4(self) -> Vec4;
    fn to_vec3(self) -> Vec3;
    fn from_f32_array(color: [f32; 4]) -> Self;
    fn from_f32_array_no_alpha(color: [f32; 3]) -> Self;
    fn from_vec4(color: Vec4) -> Self;
    fn from_vec3(color: Vec3) -> Self;
}
```

Trait with methods for converting colors to non-color types

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#130)

#### fn [to\_f32\_array](#tymethod.to_f32_array)(self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Convert to an f32 array

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#132)

#### fn [to\_f32\_array\_no\_alpha](#tymethod.to_f32_array_no_alpha)(self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Convert to an f32 array without the alpha value

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#134)

#### fn [to\_vec4](#tymethod.to_vec4)(self) -> [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Convert to a Vec4

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#136)

#### fn [to\_vec3](#tymethod.to_vec3)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Convert to a Vec3

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#138)

#### fn [from\_f32\_array](#tymethod.from_f32_array)(color: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> Self

Convert from an f32 array

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#140)

#### fn [from\_f32\_array\_no\_alpha](#tymethod.from_f32_array_no_alpha)(color: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> Self

Convert from an f32 array without the alpha value

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#142)

#### fn [from\_vec4](#tymethod.from_vec4)(color: [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> Self

Convert from a Vec4

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_ops.rs.html#144)

#### fn [from\_vec3](#tymethod.from_vec3)(color: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> Self

Convert from a Vec3

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#211)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Hsla](../../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#195)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Hsva](../../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#140)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Hwba](../../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#160)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#199)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Lcha](../../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#252)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#169)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#203)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#307)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#157)

### impl [ColorToComponents](../../prelude/trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents") for [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")