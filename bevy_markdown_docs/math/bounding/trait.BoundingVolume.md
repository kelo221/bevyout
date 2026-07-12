[bevy](../../index.html)::[math](../index.html)::[bounding](index.html)

# Trait BoundingVolume 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#13)

```rust
pub trait BoundingVolume: Sized {
    type Translation: Clone + Copy + PartialEq;
    type Rotation: Clone + Copy + PartialEq;
    type HalfSize;

    // Required methods
    fn center(&self) -> Self::Translation;
    fn half_size(&self) -> Self::HalfSize;
    fn visible_area(&self) -> f32;
    fn contains(&self, other: &Self) -> bool;
    fn merge(&self, other: &Self) -> Self;
    fn grow(&self, amount: impl Into<Self::HalfSize>) -> Self;
    fn shrink(&self, amount: impl Into<Self::HalfSize>) -> Self;
    fn scale_around_center(&self, scale: impl Into<Self::HalfSize>) -> Self;
    fn translate_by(&mut self, translation: impl Into<Self::Translation>);
    fn rotate_by(&mut self, rotation: impl Into<Self::Rotation>);

    // Provided methods
    fn transformed_by(
        self,
        translation: impl Into<Self::Translation>,
        rotation: impl Into<Self::Rotation>,
    ) -> Self { ... }
    fn transform_by(
        &mut self,
        translation: impl Into<Self::Translation>,
        rotation: impl Into<Self::Rotation>,
    ) { ... }
    fn translated_by(self, translation: impl Into<Self::Translation>) -> Self { ... }
    fn rotated_by(self, rotation: impl Into<Self::Rotation>) -> Self { ... }
}
```

A trait that generalizes different bounding volumes. Bounding volumes are simplified shapes that are used to get simpler ways to check for overlapping elements or finding intersections.

This trait supports both 2D and 3D bounding shapes.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#15)

#### type [Translation](#associatedtype.Translation): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")

The position type used for the volume. This should be `Vec2` for 2D and `Vec3` for 3D.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#18)

#### type [Rotation](#associatedtype.Rotation): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")

The rotation type used for the volume. This should be `Rot2` for 2D and `Quat` for 3D.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#23)

#### type [HalfSize](#associatedtype.HalfSize)

The type used for the size of the bounding volume. Usually a half size. For example an `f32` radius for a circle, or a `Vec3` with half sizes for x, y and z for a 3D axis-aligned bounding box

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#26)

#### fn [center](#tymethod.center)(&self) -> Self::[Translation](trait.BoundingVolume.html#associatedtype.Translation "type bevy::math::bounding::BoundingVolume::Translation")

Returns the center of the bounding volume.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#29)

#### fn [half\_size](#tymethod.half_size)(&self) -> Self::[HalfSize](trait.BoundingVolume.html#associatedtype.HalfSize "type bevy::math::bounding::BoundingVolume::HalfSize")

Returns the half size of the bounding volume.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#37)

#### fn [visible\_area](#tymethod.visible_area)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the visible surface area of the bounding volume. This method can be useful to make decisions about merging bounding volumes, using a Surface Area Heuristic.

For 2D shapes this would simply be the area of the shape. For 3D shapes this would usually be half the area of the shape.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#40)

#### fn [contains](#tymethod.contains)(&self, other: &Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this bounding volume contains another one.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#43)

#### fn [merge](#tymethod.merge)(&self, other: &Self) -> Self

Computes the smallest bounding volume that contains both `self` and `other`.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#46)

#### fn [grow](#tymethod.grow)(&self, amount: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[HalfSize](trait.BoundingVolume.html#associatedtype.HalfSize "type bevy::math::bounding::BoundingVolume::HalfSize")\>) -> Self

Increases the size of the bounding volume in each direction by the given amount.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#49)

#### fn [shrink](#tymethod.shrink)(&self, amount: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[HalfSize](trait.BoundingVolume.html#associatedtype.HalfSize "type bevy::math::bounding::BoundingVolume::HalfSize")\>) -> Self

Decreases the size of the bounding volume in each direction by the given amount.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#52)

#### fn [scale\_around\_center](#tymethod.scale_around_center)(&self, scale: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[HalfSize](trait.BoundingVolume.html#associatedtype.HalfSize "type bevy::math::bounding::BoundingVolume::HalfSize")\>) -> Self

Scale the size of the bounding volume around its center by the given amount

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#81)

#### fn [translate\_by](#tymethod.translate_by)(&mut self, translation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Translation](trait.BoundingVolume.html#associatedtype.Translation "type bevy::math::bounding::BoundingVolume::Translation")\>)

Translates the bounding volume by the given translation.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#96)

#### fn [rotate\_by](#tymethod.rotate_by)(&mut self, rotation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Rotation](trait.BoundingVolume.html#associatedtype.Rotation "type bevy::math::bounding::BoundingVolume::Rotation")\>)

Rotates the bounding volume around the origin by the given rotation.

The result is a combination of the original volume and the rotated volume, so it is guaranteed to be either the same size or larger than the original.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#55-59)

#### fn [transformed\_by](#method.transformed_by)( self, translation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Translation](trait.BoundingVolume.html#associatedtype.Translation "type bevy::math::bounding::BoundingVolume::Translation")\>, rotation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Rotation](trait.BoundingVolume.html#associatedtype.Rotation "type bevy::math::bounding::BoundingVolume::Rotation")\>, ) -> Self

Transforms the bounding volume by first rotating it around the origin and then applying a translation.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#65-69)

#### fn [transform\_by](#method.transform_by)( &mut self, translation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Translation](trait.BoundingVolume.html#associatedtype.Translation "type bevy::math::bounding::BoundingVolume::Translation")\>, rotation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Rotation](trait.BoundingVolume.html#associatedtype.Rotation "type bevy::math::bounding::BoundingVolume::Rotation")\>, )

Transforms the bounding volume by first rotating it around the origin and then applying a translation.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#75)

#### fn [translated\_by](#method.translated_by)(self, translation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Translation](trait.BoundingVolume.html#associatedtype.Translation "type bevy::math::bounding::BoundingVolume::Translation")\>) -> Self

Translates the bounding volume by the given translation.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#87)

#### fn [rotated\_by](#method.rotated_by)(self, rotation: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<Self::[Rotation](trait.BoundingVolume.html#associatedtype.Rotation "type bevy::math::bounding::BoundingVolume::Rotation")\>) -> Self

Rotates the bounding volume around the origin by the given rotation.

The result is a combination of the original volume and the rotated volume, so it is guaranteed to be either the same size or larger than the original.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#113)

### impl [BoundingVolume](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume") for [Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#114)

#### type [Translation](#associatedtype.Translation) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#115)

#### type [Rotation](#associatedtype.Rotation) = [Rot2](../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#116)

#### type [HalfSize](#associatedtype.HalfSize) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#140)

### impl [BoundingVolume](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume") for [Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#141)

#### type [Translation](#associatedtype.Translation) = [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#142)

#### type [Rotation](#associatedtype.Rotation) = [Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#143)

#### type [HalfSize](#associatedtype.HalfSize) = [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#551)

### impl [BoundingVolume](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume") for [BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#552)

#### type [Translation](#associatedtype.Translation) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#553)

#### type [Rotation](#associatedtype.Rotation) = [Rot2](../../prelude/struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#554)

#### type [HalfSize](#associatedtype.HalfSize) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#591)

### impl [BoundingVolume](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume") for [BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#592)

#### type [Translation](#associatedtype.Translation) = [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#593)

#### type [Rotation](#associatedtype.Rotation) = [Quat](../../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#594)

#### type [HalfSize](#associatedtype.HalfSize) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)