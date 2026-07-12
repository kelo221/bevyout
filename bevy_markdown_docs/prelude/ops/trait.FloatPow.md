[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Trait FloatPow 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#630)

```rust
pub trait FloatPow {
    // Required methods
    fn squared(self) -> Self;
    fn cubed(self) -> Self;
}
```

This extension trait covers shortfall in determinacy from the lack of a `libm` counterpart to `f32::powi`. Use this for the common small exponents.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#632)

#### fn [squared](#tymethod.squared)(self) -> Self

Squares the f32

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#634)

#### fn [cubed](#tymethod.cubed)(self) -> Self

Cubes the f32

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#637)

### impl [FloatPow](../../math/trait.FloatPow.html "trait bevy::math::FloatPow") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#639)

#### fn [squared](#tymethod.squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#643)

#### fn [cubed](#tymethod.cubed)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

## Implementors