[bevy](../../index.html)::[math](../index.html)::[bounding](index.html)

# Trait IntersectsVolume 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#104)

```rust
pub trait IntersectsVolume<Volume>where
    Volume: BoundingVolume,{
    // Required method
    fn intersects(&self, volume: &Volume) -> bool;
}
```

A trait that generalizes intersection tests against a volume. Intersection tests can be used for a variety of tasks, for example:

*   Raycasting
*   Testing for overlap
*   Checking if an object is within the view frustum of a camera

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/mod.rs.html#106)

#### fn [intersects](#tymethod.intersects)(&self, volume: [&Volume](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Check if a volume intersects with this intersection test

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#252)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")\> for [Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#142)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")\> for [AabbCast2d](struct.AabbCast2d.html "struct bevy::math::bounding::AabbCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#637)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")\> for [BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#98)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")\> for [RayCast2d](struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#276)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")\> for [Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#146)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")\> for [AabbCast3d](struct.AabbCast3d.html "struct bevy::math::bounding::AabbCast3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#687)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")\> for [BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#95)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")\> for [RayCast3d](struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#261)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")\> for [Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#628)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")\> for [BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#180)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")\> for [BoundingCircleCast](struct.BoundingCircleCast.html "struct bevy::math::bounding::BoundingCircleCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#104)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")\> for [RayCast2d](struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#283)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")\> for [Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#678)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")\> for [BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#191)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")\> for [BoundingSphereCast](struct.BoundingSphereCast.html "struct bevy::math::bounding::BoundingSphereCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#101)

### impl [IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")<[BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")\> for [RayCast3d](struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")