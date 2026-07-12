[bevy](../../index.html)::[math](../index.html)::[bounding](index.html)

# Trait BoundedExtrusion 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#200)

```rust
pub trait BoundedExtrusion: Primitive2d + Bounded2d {
    // Provided methods
    fn extrusion_aabb_3d(
        &self,
        half_depth: f32,
        isometry: impl Into<Isometry3d>,
    ) -> Aabb3d { ... }
    fn extrusion_bounding_sphere(
        &self,
        half_depth: f32,
        isometry: impl Into<Isometry3d>,
    ) -> BoundingSphere { ... }
}
```

A trait implemented on 2D shapes which determines the 3D bounding volumes of their extrusions.

Since default implementations can be inferred from 2D bounding volumes, this allows a `Bounded2d` implementation on some shape `MyShape` to be extrapolated to a `Bounded3d` implementation on `Extrusion<MyShape>` without supplying any additional data; e.g.: `impl BoundedExtrusion for MyShape {}`

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#202)

#### fn [extrusion\_aabb\_3d](#method.extrusion_aabb_3d)( &self, half\_depth: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, ) -> [Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

Get an axis-aligned bounding box for an extrusion with this shape as a base and the given `half_depth`, transformed by the given `translation` and `rotation`.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#237-241)

#### fn [extrusion\_bounding\_sphere](#method.extrusion_bounding_sphere)( &self, half\_depth: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, ) -> [BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

Get a bounding sphere for an extrusion of the `base_shape` with the given `half_depth` with the given translation and rotation

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#153)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Capsule2d](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#22)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Circle](../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#41)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Ellipse](../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#72)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Line2d](../../prelude/struct.Line2d.html "struct bevy::prelude::Line2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#130)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Polygon](../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#100)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Polyline2d](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#120)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Rectangle](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#140)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [RegularPolygon](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#89)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Segment2d](../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#110)

### impl [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Triangle2d](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/extrusion.rs.html#168)

### impl<T> [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion") for [Ring](../../prelude/struct.Ring.html "struct bevy::prelude::Ring")<T>

where T: [BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion"),