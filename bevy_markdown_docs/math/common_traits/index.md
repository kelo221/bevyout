[bevy](../../index.html)::[math](../index.html)

# Module common\_traits 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#32)

This module contains abstract mathematical traits shared by types used in `bevy_math`.

## Structs

[MismatchedUnitsError](struct.MismatchedUnitsError.html "struct bevy::math::common_traits::MismatchedUnitsError")

Error produced when the values to be interpolated are not in the same units.

[Sum](struct.Sum.html "struct bevy::math::common_traits::Sum")

A type consisting of formal sums of elements from `V` and `W`. That is, each value `Sum(v, w)` is thought of as `v + w`, with no available simplification. In particular, if `V` and `W` are [vector spaces](../trait.VectorSpace.html "trait bevy::math::VectorSpace"), then `Sum<V, W>` is a vector space whose dimension is the sum of those of `V` and `W`, and the field accessors `.0` and `.1` are vector space projections.

[WithDerivative](struct.WithDerivative.html "struct bevy::math::common_traits::WithDerivative")

A value with its derivative.

[WithTwoDerivatives](struct.WithTwoDerivatives.html "struct bevy::math::common_traits::WithTwoDerivatives")

A value together with its first and second derivatives.

## Traits

[HasTangent](trait.HasTangent.html "trait bevy::math::common_traits::HasTangent")

A type that has tangents.

[NormedVectorSpace](trait.NormedVectorSpace.html "trait bevy::math::common_traits::NormedVectorSpace")

A type that supports the operations of a normed vector space; i.e. a norm operation in addition to those of [`VectorSpace`](../trait.VectorSpace.html "trait bevy::math::VectorSpace"). Specifically, the implementor must guarantee that the following relationships hold, within the limitations of floating point arithmetic:

[ScalarField](trait.ScalarField.html "trait bevy::math::common_traits::ScalarField")

A type that supports the operations of a scalar field. An implementation should support:

[StableInterpolate](trait.StableInterpolate.html "trait bevy::math::common_traits::StableInterpolate")

A type with a natural interpolation that provides strong subdivision guarantees.

[TryStableInterpolate](trait.TryStableInterpolate.html "trait bevy::math::common_traits::TryStableInterpolate")

A trait that indicates that a value _may_ be interpolable via [`StableInterpolate`](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"). An interpolation may fail if the values have different units - for example, attempting to interpolate between [`Val::Px`](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html#variant.Px) and [`Val::Percent`](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html#variant.Percent) will fail, even though they are the same Rust type.

[VectorSpace](trait.VectorSpace.html "trait bevy::math::common_traits::VectorSpace")

A type that supports the mathematical operations of a real vector space, irrespective of dimension. In particular, this means that the implementing type supports: