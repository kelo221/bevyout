[bevy](../../index.html)::[math](../index.html)::[common\_traits](index.html)

# Trait HasTangent 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#594)

```rust
pub trait HasTangent {
    type Tangent: VectorSpace;
}
```

A type that has tangents.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#596)

#### type [Tangent](#associatedtype.Tangent): [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace")

The tangent type.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#636-642)

### impl<F, U, V, M, N> [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent") for [(M, N)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where F: [ScalarField](../trait.ScalarField.html "trait bevy::math::ScalarField"), U: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = F>, V: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = F>, M: [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = U>, N: [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent")<Tangent = V>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#644)

#### type [Tangent](#associatedtype.Tangent) = [Sum](../struct.Sum.html "struct bevy::math::Sum")<<M as [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"), <N as [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent")\>

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#632)

### impl<V> [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent") for V

where V: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#633)

#### type [Tangent](#associatedtype.Tangent) = V