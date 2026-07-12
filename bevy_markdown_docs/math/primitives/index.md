[bevy](../../index.html)::[math](../index.html)

# Module primitives 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#40)

This module defines primitive shapes. The origin is (0, 0) for 2D primitives and (0, 0, 0) for 3D primitives, unless stated otherwise.

## Structs

[Annulus](struct.Annulus.html "struct bevy::math::primitives::Annulus")

A primitive shape formed by the region between two circles, also known as a ring.

[Arc2d](struct.Arc2d.html "struct bevy::math::primitives::Arc2d")

A primitive representing an arc between two points on a circle.

[Capsule2d](struct.Capsule2d.html "struct bevy::math::primitives::Capsule2d")

A 2D capsule primitive, also known as a stadium or pill shape.

[Capsule3d](struct.Capsule3d.html "struct bevy::math::primitives::Capsule3d")

A 3D capsule primitive centered on the origin A three-dimensional capsule is defined as a surface at a distance (radius) from a line

[Circle](struct.Circle.html "struct bevy::math::primitives::Circle")

A circle primitive, representing the set of points some distance from the origin

[CircularSector](struct.CircularSector.html "struct bevy::math::primitives::CircularSector")

A primitive representing a circular sector: a pie slice of a circle.

[CircularSegment](struct.CircularSegment.html "struct bevy::math::primitives::CircularSegment")

A primitive representing a circular segment: the area enclosed by the arc of a circle and its chord (the line between its endpoints).

[Cone](struct.Cone.html "struct bevy::math::primitives::Cone")

A cone primitive centered on the midpoint between the tip of the cone and the center of its base.

[ConicalFrustum](struct.ConicalFrustum.html "struct bevy::math::primitives::ConicalFrustum")

A conical frustum primitive. A conical frustum can be created by slicing off a section of a cone.

[ConvexPolygon](struct.ConvexPolygon.html "struct bevy::math::primitives::ConvexPolygon")`alloc`

A convex polygon with `N` vertices.

[Cuboid](struct.Cuboid.html "struct bevy::math::primitives::Cuboid")

A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not required to be the same.

[Cylinder](struct.Cylinder.html "struct bevy::math::primitives::Cylinder")

A cylinder primitive centered on the origin

[Ellipse](struct.Ellipse.html "struct bevy::math::primitives::Ellipse")

An ellipse primitive, which is like a circle, but the width and height can be different

[Extrusion](struct.Extrusion.html "struct bevy::math::primitives::Extrusion")

A 3D shape representing an extruded 2D `base_shape`.

[HalfSpace](struct.HalfSpace.html "struct bevy::math::primitives::HalfSpace")

A region of 3D space, specifically an open set whose border is a bisecting 2D plane.

[InfinitePlane3d](struct.InfinitePlane3d.html "struct bevy::math::primitives::InfinitePlane3d")

An unbounded plane in 3D space. It forms a separating surface through the origin, stretching infinitely far

[Line2d](struct.Line2d.html "struct bevy::math::primitives::Line2d")

An infinite line going through the origin along a direction in 2D space.

[Line3d](struct.Line3d.html "struct bevy::math::primitives::Line3d")

An infinite line going through the origin along a direction in 3D space.

[Plane2d](struct.Plane2d.html "struct bevy::math::primitives::Plane2d")

An unbounded plane in 2D space. It forms a separating surface through the origin, stretching infinitely far

[Plane3d](struct.Plane3d.html "struct bevy::math::primitives::Plane3d")

A bounded plane in 3D space. It forms a surface starting from the origin with a defined height and width.

[Polygon](struct.Polygon.html "struct bevy::math::primitives::Polygon")`alloc`

A polygon with N vertices.

[Polyline2d](struct.Polyline2d.html "struct bevy::math::primitives::Polyline2d")`alloc`

A series of connected line segments in 2D space.

[Polyline3d](struct.Polyline3d.html "struct bevy::math::primitives::Polyline3d")`alloc`

A series of connected line segments in 3D space.

[Rectangle](struct.Rectangle.html "struct bevy::math::primitives::Rectangle")

A rectangle primitive, which is like a square, except that the width and height can be different

[RegularPolygon](struct.RegularPolygon.html "struct bevy::math::primitives::RegularPolygon")

A polygon centered on the origin where all vertices lie on a circle, equally far apart.

[Rhombus](struct.Rhombus.html "struct bevy::math::primitives::Rhombus")

A rhombus primitive, also known as a diamond shape. A four sided polygon, centered on the origin, where opposite sides are parallel but without requiring right angles.

[Ring](struct.Ring.html "struct bevy::math::primitives::Ring")

A 2D shape representing the ring version of a base shape.

[Segment2d](struct.Segment2d.html "struct bevy::math::primitives::Segment2d")

A line segment defined by two endpoints in 2D space.

[Segment3d](struct.Segment3d.html "struct bevy::math::primitives::Segment3d")

A line segment defined by two endpoints in 3D space.

[Sphere](struct.Sphere.html "struct bevy::math::primitives::Sphere")

A sphere primitive, representing the set of all points some distance from the origin

[Tetrahedron](struct.Tetrahedron.html "struct bevy::math::primitives::Tetrahedron")

A tetrahedron primitive.

[Torus](struct.Torus.html "struct bevy::math::primitives::Torus")

A torus primitive, often representing a ring or donut shape The set of points some distance from a circle centered at the origin

[Triangle2d](struct.Triangle2d.html "struct bevy::math::primitives::Triangle2d")

A triangle in 2D space

[Triangle3d](struct.Triangle3d.html "struct bevy::math::primitives::Triangle3d")

A 3D triangle primitive.

[ViewFrustum](struct.ViewFrustum.html "struct bevy::math::primitives::ViewFrustum")

A region of 3D space defined by the intersection of 6 [`HalfSpace`](../../prelude/struct.HalfSpace.html "struct bevy::prelude::HalfSpace")s.

## Enums

[ConvexPolygonError](enum.ConvexPolygonError.html "enum bevy::math::primitives::ConvexPolygonError")`alloc`

An error that happens when creating a [`ConvexPolygon`](../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon").

[TorusKind](enum.TorusKind.html "enum bevy::math::primitives::TorusKind")

The type of torus determined by the minor and major radii

[WindingOrder](enum.WindingOrder.html "enum bevy::math::primitives::WindingOrder")

The winding order for a set of points

## Traits

[Inset](trait.Inset.html "trait bevy::math::primitives::Inset")

A primitive that can be resized uniformly.

[Measured2d](trait.Measured2d.html "trait bevy::math::primitives::Measured2d")

A trait for getting measurements of 2D shapes

[Measured3d](trait.Measured3d.html "trait bevy::math::primitives::Measured3d")

A trait for getting measurements of 3D shapes

[Primitive2d](trait.Primitive2d.html "trait bevy::math::primitives::Primitive2d")

A marker trait for 2D primitives

[Primitive3d](trait.Primitive3d.html "trait bevy::math::primitives::Primitive3d")

A marker trait for 3D primitives

[ToRing](trait.ToRing.html "trait bevy::math::primitives::ToRing")

Provides a convenience method for converting a primitive to a [`Ring`](../../prelude/struct.Ring.html "struct bevy::prelude::Ring"), with a given thickness.