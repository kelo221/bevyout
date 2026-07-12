[bevy](../../index.html)::[math](../index.html)::[bounding](index.html)

# Trait Bounded2d 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#30)

```rust
pub trait Bounded2d {
    // Required methods
    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d;
    fn bounding_circle(&self, isometry: impl Into<Isometry2d>) -> BoundingCircle;
}
```

A trait with methods that return 2D bounding volumes for a shape.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#32)

#### fn [aabb\_2d](#tymethod.aabb_2d)(&self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>) -> [Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

Get an axis-aligned bounding box for the shape translated and rotated by the given isometry.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#34)

#### fn [bounding\_circle](#tymethod.bounding_circle)(&self, isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>) -> [BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

Get a bounding circle for the shape translated and rotated by the given isometry.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#186)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Annulus](../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#67)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Arc2d](../../prelude/struct.Arc2d.html "struct bevy::prelude::Arc2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#404)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Capsule2d](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#21)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Circle](../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#99)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [CircularSector](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#138)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [CircularSegment](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#370)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [ConvexPolygon](../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#148)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Ellipse](../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#245)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Line2d](../../prelude/struct.Line2d.html "struct bevy::prelude::Line2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#222)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Plane2d](../../prelude/struct.Plane2d.html "struct bevy::prelude::Plane2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#359)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Polygon](../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#282)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Polyline2d](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#337)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Rectangle](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#380)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [RegularPolygon](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#198)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Rhombus](../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#267)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Segment2d](../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#292)

### impl [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Triangle2d](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/primitive_impls.rs.html#431)

### impl<P> [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") for [Ring](../../prelude/struct.Ring.html "struct bevy::prelude::Ring")<P>

where P: [Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d") + [Primitive2d](../../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d"),