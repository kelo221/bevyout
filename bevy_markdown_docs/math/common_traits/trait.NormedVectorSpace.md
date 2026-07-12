[bevy](../../index.html)::[math](../index.html)::[common\_traits](index.html)

# Trait NormedVectorSpace 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#246)

```rust
pub trait NormedVectorSpace: VectorSpace {
    // Required method
    fn norm(self) -> Self::Scalar;

    // Provided methods
    fn norm_squared(self) -> Self::Scalar { ... }
    fn distance(self, rhs: Self) -> Self::Scalar { ... }
    fn distance_squared(self, rhs: Self) -> Self::Scalar { ... }
}
```

A type that supports the operations of a normed vector space; i.e. a norm operation in addition to those of [`VectorSpace`](../trait.VectorSpace.html "trait bevy::math::VectorSpace"). Specifically, the implementor must guarantee that the following relationships hold, within the limitations of floating point arithmetic:

*   (Nonnegativity) For all `v: Self`, `v.norm() >= 0.0`.
*   (Positive definiteness) For all `v: Self`, `v.norm() == 0.0` implies `v == Self::ZERO`.
*   (Absolute homogeneity) For all `c: Self::Scalar`, `v: Self`, `(v * c).norm() == v.norm() * c.abs()`.
*   (Triangle inequality) For all `v, w: Self`, `(v + w).norm() <= v.norm() + w.norm()`.

Note that, because implementing types use floating point arithmetic, they are not required to actually implement `PartialEq` or `Eq`.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#248)

#### fn [norm](#tymethod.norm)(self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The size of this element. The return value should always be nonnegative.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#253)

#### fn [norm\_squared](#method.norm_squared)(self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The squared norm of this element. Computing this is often faster than computing [`NormedVectorSpace::norm`](../trait.NormedVectorSpace.html#tymethod.norm "method bevy::math::NormedVectorSpace::norm").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#259)

#### fn [distance](#method.distance)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The distance between this element and another, as determined by the norm.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#266)

#### fn [distance\_squared](#method.distance_squared)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The squared distance between this element and another, as determined by the norm. Note that this is often faster to compute in practice than [`NormedVectorSpace::distance`](../trait.NormedVectorSpace.html#method.distance "method bevy::math::NormedVectorSpace::distance").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#319)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#321)

#### fn [norm](#tymethod.norm)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#362)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#365)

#### fn [norm](#tymethod.norm)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#350)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#338)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#326)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#307)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#283)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#271)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#295)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")