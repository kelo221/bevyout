[bevy](../../index.html)::[math](../index.html)::[common\_traits](index.html)

# Trait ScalarField 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#118-127)

```rust
pub trait ScalarField:
    Mul<Output = Self>
    + Div<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Default
    + Debug
    + Clone
    + Copy {
    const ZERO: Self;
    const ONE: Self;

    // Provided method
    fn recip(self) -> Self { ... }
}
```

A type that supports the operations of a scalar field. An implementation should support:

*   Addition and subtraction
*   Multiplication and division
*   Negation
*   Zero (additive identity)
*   One (multiplicative identity)

Within the limitations of floating point arithmetic, all the following are required to hold:

*   (Associativity of addition) For all `u, v, w: Self`, `(u + v) + w == u + (v + w)`.
*   (Commutativity of addition) For all `u, v: Self`, `u + v == v + u`.
*   (Additive identity) For all `v: Self`, `v + Self::ZERO == v`.
*   (Additive inverse) For all `v: Self`, `v - v == v + (-v) == Self::ZERO`.
*   (Associativity of multiplication) For all `u, v, w: Self`, `(u * v) * w == u * (v * w)`.
*   (Commutativity of multiplication) For all `u, v: Self`, `u * v == v * u`.
*   (Multiplicative identity) For all `v: Self`, `v * Self::ONE == v`.
*   (Multiplicative inverse) For all `v: Self`, `v / v == v * v.inverse() == Self::ONE`.
*   (Distributivity over addition) For all `a, b: Self`, `u, v: Self`, `(u + v) * a == u * a + v * a`.

## Required Associated Constants

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#130)

#### const [ZERO](#associatedconstant.ZERO): Self

The additive identity.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#132)

#### const [ONE](#associatedconstant.ONE): Self

The multiplicative identity.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#135)

#### fn [recip](#method.recip)(self) -> Self

The multiplicative inverse of this element. This is equivalent to `1.0 / self`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#140)

### impl [ScalarField](../trait.ScalarField.html "trait bevy::math::ScalarField") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#141)

#### const [ZERO](#associatedconstant.ZERO): [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) = 0.0

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#142)

#### const [ONE](#associatedconstant.ONE): [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) = 1.0

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#145)

### impl [ScalarField](../trait.ScalarField.html "trait bevy::math::ScalarField") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#146)

#### const [ZERO](#associatedconstant.ZERO): [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html) = 0.0

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#147)

#### const [ONE](#associatedconstant.ONE): [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html) = 1.0

## Implementors