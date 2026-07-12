[bevy](../../index.html)::[math](../index.html)

# Module bounding 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#31)

This module contains traits and implements for working with bounding shapes

There are four traits used:

*   [`BoundingVolume`](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume") is a generic abstraction for any bounding volume
*   [`IntersectsVolume`](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume") abstracts intersection tests against a [`BoundingVolume`](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume")
*   [`Bounded2d`](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d")/[`Bounded3d`](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d") are abstractions for shapes to generate [`BoundingVolume`](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume")s

## Structs

[Aabb2d](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

A 2D axis-aligned bounding box, or bounding rectangle

[Aabb3d](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

A 3D axis-aligned bounding box

[AabbCast2d](struct.AabbCast2d.html "struct bevy::math::bounding::AabbCast2d")

An intersection test that casts an [`Aabb2d`](struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d") along a ray.

[AabbCast3d](struct.AabbCast3d.html "struct bevy::math::bounding::AabbCast3d")

An intersection test that casts an [`Aabb3d`](struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d") along a ray.

[BoundingCircle](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

A bounding circle

[BoundingCircleCast](struct.BoundingCircleCast.html "struct bevy::math::bounding::BoundingCircleCast")

An intersection test that casts a [`BoundingCircle`](struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle") along a ray.

[BoundingSphere](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

A bounding sphere

[BoundingSphereCast](struct.BoundingSphereCast.html "struct bevy::math::bounding::BoundingSphereCast")

An intersection test that casts a [`BoundingSphere`](struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere") along a ray.

[RayCast2d](struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

A raycast intersection test for 2D bounding volumes

[RayCast3d](struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")

A raycast intersection test for 3D bounding volumes

## Traits

[Bounded2d](trait.Bounded2d.html "trait bevy::math::bounding::Bounded2d")

A trait with methods that return 2D bounding volumes for a shape.

[Bounded3d](trait.Bounded3d.html "trait bevy::math::bounding::Bounded3d")

A trait with methods that return 3D bounding volumes for a shape.

[BoundedExtrusion](trait.BoundedExtrusion.html "trait bevy::math::bounding::BoundedExtrusion")

A trait implemented on 2D shapes which determines the 3D bounding volumes of their extrusions.

[BoundingVolume](trait.BoundingVolume.html "trait bevy::math::bounding::BoundingVolume")

A trait that generalizes different bounding volumes. Bounding volumes are simplified shapes that are used to get simpler ways to check for overlapping elements or finding intersections.

[IntersectsVolume](trait.IntersectsVolume.html "trait bevy::math::bounding::IntersectsVolume")

A trait that generalizes intersection tests against a volume. Intersection tests can be used for a variety of tasks, for example: