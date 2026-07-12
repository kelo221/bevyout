[bevy](../../index.html)::[math](../index.html)::[bounding](index.html)

# Trait Bounded3d 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#37)

```rust
pub trait Bounded3d {
    // Required methods
    fn aabb_3d(&self, isometry: impl Into<Isometry3d>) -> Aabb3d;
    fn bounding_sphere(&self, isometry: impl Into<Isometry3d>) -> BoundingSphere;
}
```

A trait with methods that return 3D bounding volumes for a shape.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#39)

#### fn [aabb\_3d](#tymethod.aabb_3d)(&self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>) -> [Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

Get an axis-aligned bounding box for the shape translated and rotated by the given isometry.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#41)

#### fn [bounding\_sphere](#tymethod.bounding_sphere)(&self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>) -> [BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

Get a bounding sphere for the shape translated and rotated by the given isometry.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#149)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Capsule3d](../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#174)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Cone](../../prelude/struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#215)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [ConicalFrustum](../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#100)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Cuboid](../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#123)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Cylinder](../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#30)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [InfinitePlane3d](../../prelude/struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#55)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Line3d](../../prelude/struct.Line3d.html "struct bevy::prelude::Line3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#90)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Polyline3d](../../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#77)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Segment3d](../../prelude/struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#18)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Sphere](../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#301)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Torus](../../prelude/struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/primitive_impls.rs.html#324)

### impl [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Triangle3d](../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#183)

### impl<T> [Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") for [Extrusion](../../prelude/struct.Extrusion.html "struct bevy::prelude::Extrusion")<T>

where T: [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion"),