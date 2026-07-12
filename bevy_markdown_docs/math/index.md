[bevy](../index.html)

# Crate math 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#1-103)

Provides math types and functionality for the Bevy game engine.

The commonly used types are vectors like [`Vec2`](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") and [`Vec3`](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), matrices like [`Mat2`](../prelude/struct.Mat2.html "struct bevy::prelude::Mat2"), [`Mat3`](../prelude/struct.Mat3.html "struct bevy::prelude::Mat3") and [`Mat4`](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4") and orientation representations like [`Quat`](../prelude/struct.Quat.html "struct bevy::prelude::Quat").

## Modules

[bool](bool/index.html "mod bevy::math::bool")

`bool` vector mask types.

[bounding](bounding/index.html "mod bevy::math::bounding")

This module contains traits and implements for working with bounding shapes

[common\_traits](common_traits/index.html "mod bevy::math::common_traits")

This module contains abstract mathematical traits shared by types used in `bevy_math`.

[cubic\_splines](cubic_splines/index.html "mod bevy::math::cubic_splines")

Provides types for building cubic splines for rendering curves and use with animation easing.

[curve](curve/index.html "mod bevy::math::curve")`curve`

The [`Curve`](../prelude/trait.Curve.html "trait bevy::prelude::Curve") trait, providing a domain-agnostic description of curves.

[f32](f32/index.html "mod bevy::math::f32")

`f32` vector, quaternion and matrix types.

[f64](f64/index.html "mod bevy::math::f64")

`f64` vector, quaternion and matrix types.

[i8](i8/index.html "mod bevy::math::i8")

`i8` vector types.

[i16](i16/index.html "mod bevy::math::i16")

`i16` vector types.

[i32](i32/index.html "mod bevy::math::i32")

`i32` vector types.

[i64](i64/index.html "mod bevy::math::i64")

`i64` vector types.

[isize](isize/index.html "mod bevy::math::isize")

`isize` vector types.

[prelude](prelude/index.html "mod bevy::math::prelude")

The math prelude.

[primitives](primitives/index.html "mod bevy::math::primitives")

This module defines primitive shapes. The origin is (0, 0) for 2D primitives and (0, 0, 0) for 3D primitives, unless stated otherwise.

[sampling](sampling/index.html "mod bevy::math::sampling")`rand`

This module contains tools related to random sampling.

[swizzles](swizzles/index.html "mod bevy::math::swizzles")

Traits adding swizzle methods to all vector types.

[u8](u8/index.html "mod bevy::math::u8")

`u8` vector types.

[u16](u16/index.html "mod bevy::math::u16")

`u16` vector types.

[u32](u32/index.html "mod bevy::math::u32")

`u32` vector types.

[u64](u64/index.html "mod bevy::math::u64")

`u64` vector types.

[usize](usize/index.html "mod bevy::math::usize")

`usize` vector types.

## Structs

[Affine2](struct.Affine2.html "struct bevy::math::Affine2")

A 2D affine transform, which can represent translation, rotation, scaling and shear.

[Affine3](struct.Affine3.html "struct bevy::math::Affine3")

A 3D affine transform, which can represent translation, rotation, scaling and shear.

[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")

A 3D affine transform, which can represent translation, rotation, scaling and shear.

[AspectRatio](struct.AspectRatio.html "struct bevy::math::AspectRatio")

An `AspectRatio` is the ratio of width to height.

[BVec2](struct.BVec2.html "struct bevy::math::BVec2")

A 2-dimensional `bool` vector mask.

[BVec3](struct.BVec3.html "struct bevy::math::BVec3")

A 3-dimensional `bool` vector mask.

[BVec4](struct.BVec4.html "struct bevy::math::BVec4")

A 4-dimensional `bool` vector mask.

[BVec3A](struct.BVec3A.html "struct bevy::math::BVec3A")

A 3-dimensional SIMD vector mask.

[BVec4A](struct.BVec4A.html "struct bevy::math::BVec4A")

A 4-dimensional SIMD vector mask.

[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")

A 2D affine transform, which can represent translation, rotation, scaling and shear.

[DAffine3](struct.DAffine3.html "struct bevy::math::DAffine3")

A 3D affine transform, which can represent translation, rotation, scaling and shear.

[DMat2](struct.DMat2.html "struct bevy::math::DMat2")

A 2x2 column major matrix.

[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

A 3x3 column major matrix.

[DMat4](struct.DMat4.html "struct bevy::math::DMat4")

A 4x4 column major matrix.

[DQuat](struct.DQuat.html "struct bevy::math::DQuat")

A quaternion representing an orientation.

[DVec2](struct.DVec2.html "struct bevy::math::DVec2")

A 2-dimensional vector.

[DVec3](struct.DVec3.html "struct bevy::math::DVec3")

A 3-dimensional vector.

[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A 4-dimensional vector.

[Dir2](struct.Dir2.html "struct bevy::math::Dir2")

A normalized vector pointing in a direction in 2D space

[Dir3](struct.Dir3.html "struct bevy::math::Dir3")

A normalized vector pointing in a direction in 3D space

[Dir4](struct.Dir4.html "struct bevy::math::Dir4")

A normalized vector pointing in a direction in 4D space

[Dir3A](struct.Dir3A.html "struct bevy::math::Dir3A")

A normalized SIMD vector pointing in a direction in 3D space.

[FloatOrd](struct.FloatOrd.html "struct bevy::math::FloatOrd")

A wrapper for floats that implements [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"), and [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") traits.

[I8Vec2](struct.I8Vec2.html "struct bevy::math::I8Vec2")

A 2-dimensional vector.

[I8Vec3](struct.I8Vec3.html "struct bevy::math::I8Vec3")

A 3-dimensional vector.

[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

A 4-dimensional vector.

[I16Vec2](struct.I16Vec2.html "struct bevy::math::I16Vec2")

A 2-dimensional vector.

[I16Vec3](struct.I16Vec3.html "struct bevy::math::I16Vec3")

A 3-dimensional vector.

[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

A 4-dimensional vector.

[I64Vec2](struct.I64Vec2.html "struct bevy::math::I64Vec2")

A 2-dimensional vector.

[I64Vec3](struct.I64Vec3.html "struct bevy::math::I64Vec3")

A 3-dimensional vector.

[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

A 4-dimensional vector.

[IRect](struct.IRect.html "struct bevy::math::IRect")

A rectangle defined by two opposite corners.

[ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

A 2-dimensional vector.

[ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

A 3-dimensional vector.

[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A 4-dimensional vector.

[IVec2](struct.IVec2.html "struct bevy::math::IVec2")

A 2-dimensional vector.

[IVec3](struct.IVec3.html "struct bevy::math::IVec3")

A 3-dimensional vector.

[IVec4](struct.IVec4.html "struct bevy::math::IVec4")

A 4-dimensional vector.

[Isometry2d](struct.Isometry2d.html "struct bevy::math::Isometry2d")

An isometry in two dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

[Isometry3d](struct.Isometry3d.html "struct bevy::math::Isometry3d")

An isometry in three dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

[Mat2](struct.Mat2.html "struct bevy::math::Mat2")

A 2x2 column major matrix.

[Mat3](struct.Mat3.html "struct bevy::math::Mat3")

A 3x3 column major matrix.

[Mat4](struct.Mat4.html "struct bevy::math::Mat4")

A 4x4 column major matrix.

[Mat3A](struct.Mat3A.html "struct bevy::math::Mat3A")

A 3x3 column major matrix.

[MismatchedUnitsError](struct.MismatchedUnitsError.html "struct bevy::math::MismatchedUnitsError")

Error produced when the values to be interpolated are not in the same units.

[Quat](struct.Quat.html "struct bevy::math::Quat")

A quaternion representing an orientation.

[Ray2d](struct.Ray2d.html "struct bevy::math::Ray2d")

An infinite half-line starting at `origin` and going in `direction` in 2D space.

[Ray3d](struct.Ray3d.html "struct bevy::math::Ray3d")

An infinite half-line starting at `origin` and going in `direction` in 3D space.

[Rect](struct.Rect.html "struct bevy::math::Rect")

A rectangle defined by two opposite corners.

[Rot2](struct.Rot2.html "struct bevy::math::Rot2")

A 2D rotation.

[Sum](struct.Sum.html "struct bevy::math::Sum")

A type consisting of formal sums of elements from `V` and `W`. That is, each value `Sum(v, w)` is thought of as `v + w`, with no available simplification. In particular, if `V` and `W` are [vector spaces](trait.VectorSpace.html "trait bevy::math::VectorSpace"), then `Sum<V, W>` is a vector space whose dimension is the sum of those of `V` and `W`, and the field accessors `.0` and `.1` are vector space projections.

[U8Vec2](struct.U8Vec2.html "struct bevy::math::U8Vec2")

A 2-dimensional vector.

[U8Vec3](struct.U8Vec3.html "struct bevy::math::U8Vec3")

A 3-dimensional vector.

[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

A 4-dimensional vector.

[U16Vec2](struct.U16Vec2.html "struct bevy::math::U16Vec2")

A 2-dimensional vector.

[U16Vec3](struct.U16Vec3.html "struct bevy::math::U16Vec3")

A 3-dimensional vector.

[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

A 4-dimensional vector.

[U64Vec2](struct.U64Vec2.html "struct bevy::math::U64Vec2")

A 2-dimensional vector.

[U64Vec3](struct.U64Vec3.html "struct bevy::math::U64Vec3")

A 3-dimensional vector.

[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

A 4-dimensional vector.

[URect](struct.URect.html "struct bevy::math::URect")

A rectangle defined by two opposite corners.

[USizeVec2](struct.USizeVec2.html "struct bevy::math::USizeVec2")

A 2-dimensional vector.

[USizeVec3](struct.USizeVec3.html "struct bevy::math::USizeVec3")

A 3-dimensional vector.

[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

A 4-dimensional vector.

[UVec2](struct.UVec2.html "struct bevy::math::UVec2")

A 2-dimensional vector.

[UVec3](struct.UVec3.html "struct bevy::math::UVec3")

A 3-dimensional vector.

[UVec4](struct.UVec4.html "struct bevy::math::UVec4")

A 4-dimensional vector.

[Vec2](struct.Vec2.html "struct bevy::math::Vec2")

A 2-dimensional vector.

[Vec3](struct.Vec3.html "struct bevy::math::Vec3")

A 3-dimensional vector.

[Vec4](struct.Vec4.html "struct bevy::math::Vec4")

A 4-dimensional vector.

[Vec3A](struct.Vec3A.html "struct bevy::math::Vec3A")

A 3-dimensional vector.

[WithDerivative](struct.WithDerivative.html "struct bevy::math::WithDerivative")

A value with its derivative.

[WithTwoDerivatives](struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")

A value together with its first and second derivatives.

## Enums

[CompassOctant](enum.CompassOctant.html "enum bevy::math::CompassOctant")

A compass enum with 8 directions.

[CompassQuadrant](enum.CompassQuadrant.html "enum bevy::math::CompassQuadrant")

A compass enum with 4 directions.

[EulerRot](enum.EulerRot.html "enum bevy::math::EulerRot")

Euler rotation sequences.

[InvalidDirectionError](enum.InvalidDirectionError.html "enum bevy::math::InvalidDirectionError")

An error indicating that a direction is invalid.

## Traits

[Affine3Ext](trait.Affine3Ext.html "trait bevy::math::Affine3Ext")

Extension trait for [`Affine3`](struct.Affine3.html "struct bevy::math::Affine3")

[Curve](trait.Curve.html "trait bevy::math::Curve")

A trait for a type that can represent values of type `T` parametrized over a fixed interval.

[FloatExt](trait.FloatExt.html "trait bevy::math::FloatExt")

A trait for extending [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") with extra methods.

[FloatPow](trait.FloatPow.html "trait bevy::math::FloatPow")

This extension trait covers shortfall in determinacy from the lack of a `libm` counterpart to `f32::powi`. Use this for the common small exponents.

[FromRng](trait.FromRng.html "trait bevy::math::FromRng")

Ergonomics trait for a type with a [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution, allowing values to be generated uniformly from an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") by a method in its own namespace.

[HasTangent](trait.HasTangent.html "trait bevy::math::HasTangent")

A type that has tangents.

[NormedVectorSpace](trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace")

A type that supports the operations of a normed vector space; i.e. a norm operation in addition to those of [`VectorSpace`](trait.VectorSpace.html "trait bevy::math::VectorSpace"). Specifically, the implementor must guarantee that the following relationships hold, within the limitations of floating point arithmetic:

[ScalarField](trait.ScalarField.html "trait bevy::math::ScalarField")

A type that supports the operations of a scalar field. An implementation should support:

[ShapeSample](trait.ShapeSample.html "trait bevy::math::ShapeSample")

Exposes methods to uniformly sample a variety of primitive shapes.

[StableInterpolate](trait.StableInterpolate.html "trait bevy::math::StableInterpolate")

A type with a natural interpolation that provides strong subdivision guarantees.

[TryStableInterpolate](trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate")

A trait that indicates that a value _may_ be interpolable via [`StableInterpolate`](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"). An interpolation may fail if the values have different units - for example, attempting to interpolate between [`Val::Px`](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html#variant.Px) and [`Val::Percent`](https://docs.rs/bevy/latest/bevy/ui/enum.Val.html#variant.Percent) will fail, even though they are the same Rust type.

[Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::math::Vec2Swizzles")

[Vec3Swizzles](trait.Vec3Swizzles.html "trait bevy::math::Vec3Swizzles")

[Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::math::Vec4Swizzles")

[VectorSpace](trait.VectorSpace.html "trait bevy::math::VectorSpace")

A type that supports the mathematical operations of a real vector space, irrespective of dimension. In particular, this means that the implementing type supports:

## Functions

[bvec2](fn.bvec2.html "fn bevy::math::bvec2")

Creates a 2-dimensional `bool` vector mask.

[bvec3](fn.bvec3.html "fn bevy::math::bvec3")

Creates a 3-dimensional `bool` vector mask.

[bvec4](fn.bvec4.html "fn bevy::math::bvec4")

Creates a 4-dimensional `bool` vector mask.

[bvec3a](fn.bvec3a.html "fn bevy::math::bvec3a")

Creates a 3-dimensional `bool` vector mask.

[bvec4a](fn.bvec4a.html "fn bevy::math::bvec4a")

Creates a 4-dimensional `bool` vector mask.

[dmat2](fn.dmat2.html "fn bevy::math::dmat2")

Creates a 2x2 matrix from two column vectors.

[dmat3](fn.dmat3.html "fn bevy::math::dmat3")

Creates a 3x3 matrix from three column vectors.

[dmat4](fn.dmat4.html "fn bevy::math::dmat4")

Creates a 4x4 matrix from four column vectors.

[dquat](fn.dquat.html "fn bevy::math::dquat")

Creates a quaternion from `x`, `y`, `z` and `w` values.

[dvec2](fn.dvec2.html "fn bevy::math::dvec2")

Creates a 2-dimensional vector.

[dvec3](fn.dvec3.html "fn bevy::math::dvec3")

Creates a 3-dimensional vector.

[dvec4](fn.dvec4.html "fn bevy::math::dvec4")

Creates a 4-dimensional vector.

[i8vec2](fn.i8vec2.html "fn bevy::math::i8vec2")

Creates a 2-dimensional vector.

[i8vec3](fn.i8vec3.html "fn bevy::math::i8vec3")

Creates a 3-dimensional vector.

[i8vec4](fn.i8vec4.html "fn bevy::math::i8vec4")

Creates a 4-dimensional vector.

[i16vec2](fn.i16vec2.html "fn bevy::math::i16vec2")

Creates a 2-dimensional vector.

[i16vec3](fn.i16vec3.html "fn bevy::math::i16vec3")

Creates a 3-dimensional vector.

[i16vec4](fn.i16vec4.html "fn bevy::math::i16vec4")

Creates a 4-dimensional vector.

[i64vec2](fn.i64vec2.html "fn bevy::math::i64vec2")

Creates a 2-dimensional vector.

[i64vec3](fn.i64vec3.html "fn bevy::math::i64vec3")

Creates a 3-dimensional vector.

[i64vec4](fn.i64vec4.html "fn bevy::math::i64vec4")

Creates a 4-dimensional vector.

[isizevec2](fn.isizevec2.html "fn bevy::math::isizevec2")

Creates a 2-dimensional vector.

[isizevec3](fn.isizevec3.html "fn bevy::math::isizevec3")

Creates a 3-dimensional vector.

[isizevec4](fn.isizevec4.html "fn bevy::math::isizevec4")

Creates a 4-dimensional vector.

[ivec2](fn.ivec2.html "fn bevy::math::ivec2")

Creates a 2-dimensional vector.

[ivec3](fn.ivec3.html "fn bevy::math::ivec3")

Creates a 3-dimensional vector.

[ivec4](fn.ivec4.html "fn bevy::math::ivec4")

Creates a 4-dimensional vector.

[mat2](fn.mat2.html "fn bevy::math::mat2")

Creates a 2x2 matrix from two column vectors.

[mat3](fn.mat3.html "fn bevy::math::mat3")

Creates a 3x3 matrix from three column vectors.

[mat4](fn.mat4.html "fn bevy::math::mat4")

Creates a 4x4 matrix from four column vectors.

[mat3a](fn.mat3a.html "fn bevy::math::mat3a")

Creates a 3x3 matrix from three column vectors.

[quat](fn.quat.html "fn bevy::math::quat")

Creates a quaternion from `x`, `y`, `z` and `w` values.

[reflection\_matrix](fn.reflection_matrix.html "fn bevy::math::reflection_matrix")

Creates a 3×3 matrix that reflects points across the plane at the origin with the given normal.

[u8vec2](fn.u8vec2.html "fn bevy::math::u8vec2")

Creates a 2-dimensional vector.

[u8vec3](fn.u8vec3.html "fn bevy::math::u8vec3")

Creates a 3-dimensional vector.

[u8vec4](fn.u8vec4.html "fn bevy::math::u8vec4")

Creates a 4-dimensional vector.

[u16vec2](fn.u16vec2.html "fn bevy::math::u16vec2")

Creates a 2-dimensional vector.

[u16vec3](fn.u16vec3.html "fn bevy::math::u16vec3")

Creates a 3-dimensional vector.

[u16vec4](fn.u16vec4.html "fn bevy::math::u16vec4")

Creates a 4-dimensional vector.

[u64vec2](fn.u64vec2.html "fn bevy::math::u64vec2")

Creates a 2-dimensional vector.

[u64vec3](fn.u64vec3.html "fn bevy::math::u64vec3")

Creates a 3-dimensional vector.

[u64vec4](fn.u64vec4.html "fn bevy::math::u64vec4")

Creates a 4-dimensional vector.

[usizevec2](fn.usizevec2.html "fn bevy::math::usizevec2")

Creates a 2-dimensional vector.

[usizevec3](fn.usizevec3.html "fn bevy::math::usizevec3")

Creates a 3-dimensional vector.

[usizevec4](fn.usizevec4.html "fn bevy::math::usizevec4")

Creates a 4-dimensional vector.

[uvec2](fn.uvec2.html "fn bevy::math::uvec2")

Creates a 2-dimensional vector.

[uvec3](fn.uvec3.html "fn bevy::math::uvec3")

Creates a 3-dimensional vector.

[uvec4](fn.uvec4.html "fn bevy::math::uvec4")

Creates a 4-dimensional vector.

[vec2](fn.vec2.html "fn bevy::math::vec2")

Creates a 2-dimensional vector.

[vec3](fn.vec3.html "fn bevy::math::vec3")

Creates a 3-dimensional vector.

[vec4](fn.vec4.html "fn bevy::math::vec4")

Creates a 4-dimensional vector.

[vec3a](fn.vec3a.html "fn bevy::math::vec3a")

Creates a 3-dimensional vector.