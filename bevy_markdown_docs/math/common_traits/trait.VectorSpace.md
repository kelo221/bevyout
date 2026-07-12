[bevy](../../index.html)::[math](../index.html)::[common\_traits](index.html)

# Trait VectorSpace 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#31-40)

```rust
pub trait VectorSpace:
    Mul<Self::Scalar, Output = Self>
    + Div<Self::Scalar, Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + Default
    + Debug
    + Clone
    + Copy {
    type Scalar: ScalarField;

    const ZERO: Self;

    // Provided method
    fn lerp(self, rhs: Self, t: Self::Scalar) -> Self { ... }
}
```

A type that supports the mathematical operations of a real vector space, irrespective of dimension. In particular, this means that the implementing type supports:

*   Scalar multiplication and division on the right by elements of `Self::Scalar`
*   Negation
*   Addition and subtraction
*   Zero

Within the limitations of floating point arithmetic, all the following are required to hold:

*   (Associativity of addition) For all `u, v, w: Self`, `(u + v) + w == u + (v + w)`.
*   (Commutativity of addition) For all `u, v: Self`, `u + v == v + u`.
*   (Additive identity) For all `v: Self`, `v + Self::ZERO == v`.
*   (Additive inverse) For all `v: Self`, `v - v == v + (-v) == Self::ZERO`.
*   (Compatibility of multiplication) For all `a, b: Self::Scalar`, `v: Self`, `v * (a * b) == (v * a) * b`.
*   (Multiplicative identity) For all `v: Self`, `v * 1.0 == v`.
*   (Distributivity for vector addition) For all `a: Self::Scalar`, `u, v: Self`, `(u + v) * a == u * a + v * a`.
*   (Distributivity for scalar addition) For all `a, b: Self::Scalar`, `v: Self`, `v * (a + b) == v * a + v * b`.

Note that, because implementing types use floating point arithmetic, they are not required to actually implement `PartialEq` or `Eq`.

## Required Associated Constants

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#46)

#### const [ZERO](#associatedconstant.ZERO): Self

The zero vector, which is the identity of addition for the vector space type.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#43)

#### type [Scalar](#associatedtype.Scalar): [ScalarField](../trait.ScalarField.html "trait bevy::math::ScalarField")

The scalar type of this vector space.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#55)

#### fn [lerp](#method.lerp)(self, rhs: Self, t: Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")) -> Self

Perform vector space linear interpolation between this element and another, based on the parameter `t`. When `t` is `0`, `self` is recovered. When `t` is `1`, `rhs` is recovered.

Note that the value of `t` is not clamped by this function, so extrapolating outside of the interval `[0,1]` is allowed.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#90)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#92)

#### const [ZERO](#associatedconstant.ZERO): [DVec2](../struct.DVec2.html "struct bevy::math::DVec2") = DVec2::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#91)

#### type [Scalar](#associatedtype.Scalar) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#85)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#87)

#### const [ZERO](#associatedconstant.ZERO): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3") = DVec3::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#86)

#### type [Scalar](#associatedtype.Scalar) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#80)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#82)

#### const [ZERO](#associatedconstant.ZERO): [DVec4](../struct.DVec4.html "struct bevy::math::DVec4") = DVec4::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#81)

#### type [Scalar](#associatedtype.Scalar) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#38)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#38)

#### const [ZERO](#associatedconstant.ZERO): [Laba](../../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#38)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#40)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#40)

#### const [ZERO](#associatedconstant.ZERO): [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#40)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#38)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#38)

#### const [ZERO](#associatedconstant.ZERO): [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#38)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#41)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#41)

#### const [ZERO](#associatedconstant.ZERO): [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#41)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#75)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#77)

#### const [ZERO](#associatedconstant.ZERO): [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") = Vec2::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#76)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#65)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#67)

#### const [ZERO](#associatedconstant.ZERO): [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") = Vec3::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#66)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#60)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#62)

#### const [ZERO](#associatedconstant.ZERO): [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4") = Vec4::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#61)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#70)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#72)

#### const [ZERO](#associatedconstant.ZERO): [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") = Vec3A::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#71)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#38)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#38)

#### const [ZERO](#associatedconstant.ZERO): [Xyza](../../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#38)

#### type [Scalar](#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#227-230)

### impl<F, V, W> [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Sum](../struct.Sum.html "struct bevy::math::Sum")<V, W>

where F: [ScalarField](../trait.ScalarField.html "trait bevy::math::ScalarField"), V: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = F>, W: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = F>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#233)

#### const [ZERO](#associatedconstant.ZERO): [Sum](../struct.Sum.html "struct bevy::math::Sum")<V, W>

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#232)

#### type [Scalar](#associatedtype.Scalar) = F

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#96)

### impl<T> [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for T

where T: [ScalarField](../trait.ScalarField.html "trait bevy::math::ScalarField"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#98)

#### const [ZERO](#associatedconstant.ZERO): T = Self::ZERO

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#97)

#### type [Scalar](#associatedtype.Scalar) = T