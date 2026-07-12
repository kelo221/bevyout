[bevy](../../index.html)::[math](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#73)

The math prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Annulus](struct.Annulus.html "struct bevy::math::prelude::Annulus")

A primitive shape formed by the region between two circles, also known as a ring.

[Arc2d](struct.Arc2d.html "struct bevy::math::prelude::Arc2d")

A primitive representing an arc between two points on a circle.

[BVec2](struct.BVec2.html "struct bevy::math::prelude::BVec2")

A 2-dimensional `bool` vector mask.

[BVec3](struct.BVec3.html "struct bevy::math::prelude::BVec3")

A 3-dimensional `bool` vector mask.

[BVec4](struct.BVec4.html "struct bevy::math::prelude::BVec4")

A 4-dimensional `bool` vector mask.

[BVec3A](struct.BVec3A.html "struct bevy::math::prelude::BVec3A")

A 3-dimensional SIMD vector mask.

[BVec4A](struct.BVec4A.html "struct bevy::math::prelude::BVec4A")

A 4-dimensional SIMD vector mask.

[BackInCurve](struct.BackInCurve.html "struct bevy::math::prelude::BackInCurve")

`f(t) = 2.70158 * t³ - 1.70158 * t²`

[BackInOutCurve](struct.BackInOutCurve.html "struct bevy::math::prelude::BackInOutCurve")

Behaves as `BackIn` for t < 0.5 and as `BackOut` for t >= 0.5

[BackOutCurve](struct.BackOutCurve.html "struct bevy::math::prelude::BackOutCurve")

`f(t) = 1.0 + 2.70158 * (t - 1.0)³ + 1.70158 * (t - 1.0)²`

[BounceInCurve](struct.BounceInCurve.html "struct bevy::math::prelude::BounceInCurve")

bouncy at the start!

[BounceInOutCurve](struct.BounceInOutCurve.html "struct bevy::math::prelude::BounceInOutCurve")

Behaves as `BounceIn` for t < 0.5 and as `BounceOut` for t >= 0.5

[BounceOutCurve](struct.BounceOutCurve.html "struct bevy::math::prelude::BounceOutCurve")

bouncy at the end!

[Capsule2d](struct.Capsule2d.html "struct bevy::math::prelude::Capsule2d")

A 2D capsule primitive, also known as a stadium or pill shape.

[Capsule3d](struct.Capsule3d.html "struct bevy::math::prelude::Capsule3d")

A 3D capsule primitive centered on the origin A three-dimensional capsule is defined as a surface at a distance (radius) from a line

[ChainCurve](struct.ChainCurve.html "struct bevy::math::prelude::ChainCurve")

The curve that results from chaining one curve with another. The second curve is effectively reparametrized so that its start is at the end of the first.

[Circle](struct.Circle.html "struct bevy::math::prelude::Circle")

A circle primitive, representing the set of points some distance from the origin

[CircularInCurve](struct.CircularInCurve.html "struct bevy::math::prelude::CircularInCurve")

`f(t) = 1.0 - sqrt(1.0 - t²)`

[CircularInOutCurve](struct.CircularInOutCurve.html "struct bevy::math::prelude::CircularInOutCurve")

Behaves as `CircularIn` for t < 0.5 and as `CircularOut` for t >= 0.5

[CircularOutCurve](struct.CircularOutCurve.html "struct bevy::math::prelude::CircularOutCurve")

`f(t) = sqrt((2.0 - t) * t)`

[CircularSector](struct.CircularSector.html "struct bevy::math::prelude::CircularSector")

A primitive representing a circular sector: a pie slice of a circle.

[CircularSegment](struct.CircularSegment.html "struct bevy::math::prelude::CircularSegment")

A primitive representing a circular segment: the area enclosed by the arc of a circle and its chord (the line between its endpoints).

[Cone](struct.Cone.html "struct bevy::math::prelude::Cone")

A cone primitive centered on the midpoint between the tip of the cone and the center of its base.

[ConicalFrustum](struct.ConicalFrustum.html "struct bevy::math::prelude::ConicalFrustum")

A conical frustum primitive. A conical frustum can be created by slicing off a section of a cone.

[ConstantCurve](struct.ConstantCurve.html "struct bevy::math::prelude::ConstantCurve")

A curve with a constant value over its domain.

[ContinuationCurve](struct.ContinuationCurve.html "struct bevy::math::prelude::ContinuationCurve")

The curve that results from chaining two curves.

[ConvexPolygon](struct.ConvexPolygon.html "struct bevy::math::prelude::ConvexPolygon")`alloc`

A convex polygon with `N` vertices.

[CubicBSpline](struct.CubicBSpline.html "struct bevy::math::prelude::CubicBSpline")`alloc`

A spline interpolated continuously across the nearest four control points. The curve does not necessarily pass through any of the control points.

[CubicBezier](struct.CubicBezier.html "struct bevy::math::prelude::CubicBezier")`alloc`

A spline composed of a single cubic Bezier curve.

[CubicCardinalSpline](struct.CubicCardinalSpline.html "struct bevy::math::prelude::CubicCardinalSpline")`alloc`

A spline interpolated continuously across the nearest four control points, with the position of the curve specified at every control point and the tangents computed automatically. The associated [`CubicCurve`](../../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve") has one segment between each pair of adjacent control points.

[CubicCurve](struct.CubicCurve.html "struct bevy::math::prelude::CubicCurve")`alloc`

A collection of [`CubicSegment`](../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")s chained into a single parametric curve. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, N]`, where `N` is its number of segments.

[CubicHermite](struct.CubicHermite.html "struct bevy::math::prelude::CubicHermite")`alloc`

A spline interpolated continuously between the nearest two control points, with the position and velocity of the curve specified at both control points. This curve passes through all control points, with the specified velocity which includes direction and parametric speed.

[CubicInCurve](struct.CubicInCurve.html "struct bevy::math::prelude::CubicInCurve")

`f(t) = t³`

[CubicInOutCurve](struct.CubicInOutCurve.html "struct bevy::math::prelude::CubicInOutCurve")

Behaves as `CubicIn` for t < 0.5 and as `CubicOut` for t >= 0.5

[CubicNurbs](struct.CubicNurbs.html "struct bevy::math::prelude::CubicNurbs")`alloc`

Non-uniform Rational B-Splines (NURBS) are a powerful generalization of the [`CubicBSpline`](../../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline") which can represent a much more diverse class of curves (like perfect circles and ellipses).

[CubicOutCurve](struct.CubicOutCurve.html "struct bevy::math::prelude::CubicOutCurve")

`f(t) = (t - 1.0)³ + 1.0`

[CubicSegment](struct.CubicSegment.html "struct bevy::math::prelude::CubicSegment")

A segment of a cubic curve, used to hold precomputed coefficients for fast interpolation. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, 1]`.

[Cuboid](struct.Cuboid.html "struct bevy::math::prelude::Cuboid")

A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not required to be the same.

[CurveReparamCurve](struct.CurveReparamCurve.html "struct bevy::math::prelude::CurveReparamCurve")

A curve that has been reparametrized by another curve, using that curve to transform the sample times before sampling. Curves of this type are produced by [`CurveExt::reparametrize_by_curve`](../../prelude/trait.CurveExt.html#method.reparametrize_by_curve "method bevy::prelude::CurveExt::reparametrize_by_curve").

[Cylinder](struct.Cylinder.html "struct bevy::math::prelude::Cylinder")

A cylinder primitive centered on the origin

[Dir2](struct.Dir2.html "struct bevy::math::prelude::Dir2")

A normalized vector pointing in a direction in 2D space

[Dir3](struct.Dir3.html "struct bevy::math::prelude::Dir3")

A normalized vector pointing in a direction in 3D space

[Dir3A](struct.Dir3A.html "struct bevy::math::prelude::Dir3A")

A normalized SIMD vector pointing in a direction in 3D space.

[EasingCurve](struct.EasingCurve.html "struct bevy::math::prelude::EasingCurve")

A [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") that is defined by

[ElasticCurve](struct.ElasticCurve.html "struct bevy::math::prelude::ElasticCurve")

`f(omega,t) = 1 - (1 - t)²(2sin(omega * t) / omega + cos(omega * t))`, parametrized by `omega`

[ElasticInCurve](struct.ElasticInCurve.html "struct bevy::math::prelude::ElasticInCurve")

`f(t) = -2.0^(10.0 * t - 10.0) * sin((t * 10.0 - 10.75) * 2.0 * π / 3.0)`

[ElasticInOutCurve](struct.ElasticInOutCurve.html "struct bevy::math::prelude::ElasticInOutCurve")

Behaves as `ElasticIn` for t < 0.5 and as `ElasticOut` for t >= 0.5

[ElasticOutCurve](struct.ElasticOutCurve.html "struct bevy::math::prelude::ElasticOutCurve")

`f(t) = 2.0^(-10.0 * t) * sin((t * 10.0 - 0.75) * 2.0 * π / 3.0) + 1.0`

[Ellipse](struct.Ellipse.html "struct bevy::math::prelude::Ellipse")

An ellipse primitive, which is like a circle, but the width and height can be different

[EvenCore](struct.EvenCore.html "struct bevy::math::prelude::EvenCore")`alloc`

The data core of a curve derived from evenly-spaced samples. The intention is to use this in addition to explicit or inferred interpolation information in user-space in order to implement curves using [`domain`](../../prelude/struct.EvenCore.html#method.domain "method bevy::prelude::EvenCore::domain") and [`sample_with`](../../prelude/struct.EvenCore.html#method.sample_with "method bevy::prelude::EvenCore::sample_with").

[ExponentialInCurve](struct.ExponentialInCurve.html "struct bevy::math::prelude::ExponentialInCurve")

`f(t) ≈ 2.0^(10.0 * (t - 1.0))`

[ExponentialInOutCurve](struct.ExponentialInOutCurve.html "struct bevy::math::prelude::ExponentialInOutCurve")

Behaves as `ExponentialIn` for t < 0.5 and as `ExponentialOut` for t >= 0.5

[ExponentialOutCurve](struct.ExponentialOutCurve.html "struct bevy::math::prelude::ExponentialOutCurve")

`f(t) ≈ 1.0 - 2.0^(-10.0 * t)`

[Extrusion](struct.Extrusion.html "struct bevy::math::prelude::Extrusion")

A 3D shape representing an extruded 2D `base_shape`.

[ForeverCurve](struct.ForeverCurve.html "struct bevy::math::prelude::ForeverCurve")

The curve that results from repeating a curve forever.

[FunctionCurve](struct.FunctionCurve.html "struct bevy::math::prelude::FunctionCurve")

A curve defined by a function together with a fixed domain.

[GraphCurve](struct.GraphCurve.html "struct bevy::math::prelude::GraphCurve")

A curve that is the graph of another curve over its parameter space. Curves of this type are produced by [`CurveExt::graph`](../../prelude/trait.CurveExt.html#method.graph "method bevy::prelude::CurveExt::graph").

[HalfSpace](struct.HalfSpace.html "struct bevy::math::prelude::HalfSpace")

A region of 3D space, specifically an open set whose border is a bisecting 2D plane.

[IRect](struct.IRect.html "struct bevy::math::prelude::IRect")

A rectangle defined by two opposite corners.

[IVec2](struct.IVec2.html "struct bevy::math::prelude::IVec2")

A 2-dimensional vector.

[IVec3](struct.IVec3.html "struct bevy::math::prelude::IVec3")

A 3-dimensional vector.

[IVec4](struct.IVec4.html "struct bevy::math::prelude::IVec4")

A 4-dimensional vector.

[InfinitePlane3d](struct.InfinitePlane3d.html "struct bevy::math::prelude::InfinitePlane3d")

An unbounded plane in 3D space. It forms a separating surface through the origin, stretching infinitely far

[Interval](struct.Interval.html "struct bevy::math::prelude::Interval")

A nonempty closed interval, possibly unbounded in either direction.

[Isometry2d](struct.Isometry2d.html "struct bevy::math::prelude::Isometry2d")

An isometry in two dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

[Isometry3d](struct.Isometry3d.html "struct bevy::math::prelude::Isometry3d")

An isometry in three dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

[Line2d](struct.Line2d.html "struct bevy::math::prelude::Line2d")

An infinite line going through the origin along a direction in 2D space.

[Line3d](struct.Line3d.html "struct bevy::math::prelude::Line3d")

An infinite line going through the origin along a direction in 3D space.

[LinearCurve](struct.LinearCurve.html "struct bevy::math::prelude::LinearCurve")

`f(t) = t`

[LinearReparamCurve](struct.LinearReparamCurve.html "struct bevy::math::prelude::LinearReparamCurve")

A curve that has had its domain changed by a linear reparameterization (stretching and scaling). Curves of this type are produced by [`CurveExt::reparametrize_linear`](../../prelude/trait.CurveExt.html#method.reparametrize_linear "method bevy::prelude::CurveExt::reparametrize_linear").

[MapCurve](struct.MapCurve.html "struct bevy::math::prelude::MapCurve")

A curve whose samples are defined by mapping samples from another curve through a given function. Curves of this type are produced by [`CurveExt::map`](../../prelude/trait.CurveExt.html#method.map "method bevy::prelude::CurveExt::map").

[Mat2](struct.Mat2.html "struct bevy::math::prelude::Mat2")

A 2x2 column major matrix.

[Mat3](struct.Mat3.html "struct bevy::math::prelude::Mat3")

A 3x3 column major matrix.

[Mat4](struct.Mat4.html "struct bevy::math::prelude::Mat4")

A 4x4 column major matrix.

[Mat3A](struct.Mat3A.html "struct bevy::math::prelude::Mat3A")

A 3x3 column major matrix.

[PingPongCurve](struct.PingPongCurve.html "struct bevy::math::prelude::PingPongCurve")

The curve that results from chaining a curve with its reversed version. The transition point is guaranteed to make no jump.

[Plane2d](struct.Plane2d.html "struct bevy::math::prelude::Plane2d")

An unbounded plane in 2D space. It forms a separating surface through the origin, stretching infinitely far

[Plane3d](struct.Plane3d.html "struct bevy::math::prelude::Plane3d")

A bounded plane in 3D space. It forms a surface starting from the origin with a defined height and width.

[Polygon](struct.Polygon.html "struct bevy::math::prelude::Polygon")`alloc`

A polygon with N vertices.

[Polyline2d](struct.Polyline2d.html "struct bevy::math::prelude::Polyline2d")`alloc`

A series of connected line segments in 2D space.

[Polyline3d](struct.Polyline3d.html "struct bevy::math::prelude::Polyline3d")`alloc`

A series of connected line segments in 3D space.

[QuadraticInCurve](struct.QuadraticInCurve.html "struct bevy::math::prelude::QuadraticInCurve")

`f(t) = t²`

[QuadraticInOutCurve](struct.QuadraticInOutCurve.html "struct bevy::math::prelude::QuadraticInOutCurve")

Behaves as `QuadraticIn` for t < 0.5 and as `QuadraticOut` for t >= 0.5

[QuadraticOutCurve](struct.QuadraticOutCurve.html "struct bevy::math::prelude::QuadraticOutCurve")

`f(t) = -(t * (t - 2.0))`

[QuarticInCurve](struct.QuarticInCurve.html "struct bevy::math::prelude::QuarticInCurve")

`f(t) = t⁴`

[QuarticInOutCurve](struct.QuarticInOutCurve.html "struct bevy::math::prelude::QuarticInOutCurve")

Behaves as `QuarticIn` for t < 0.5 and as `QuarticOut` for t >= 0.5

[QuarticOutCurve](struct.QuarticOutCurve.html "struct bevy::math::prelude::QuarticOutCurve")

`f(t) = 1.0 - (1.0 - t)⁴`

[Quat](struct.Quat.html "struct bevy::math::prelude::Quat")

A quaternion representing an orientation.

[QuinticInCurve](struct.QuinticInCurve.html "struct bevy::math::prelude::QuinticInCurve")

`f(t) = t⁵`

[QuinticInOutCurve](struct.QuinticInOutCurve.html "struct bevy::math::prelude::QuinticInOutCurve")

Behaves as `QuinticIn` for t < 0.5 and as `QuinticOut` for t >= 0.5

[QuinticOutCurve](struct.QuinticOutCurve.html "struct bevy::math::prelude::QuinticOutCurve")

`f(t) = (t - 1.0)⁵ + 1.0`

[RationalCurve](struct.RationalCurve.html "struct bevy::math::prelude::RationalCurve")`alloc`

A collection of [`RationalSegment`](../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")s chained into a single parametric curve. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, N]`, where `N` is the number of segments.

[RationalSegment](struct.RationalSegment.html "struct bevy::math::prelude::RationalSegment")

A segment of a rational cubic curve, used to hold precomputed coefficients for fast interpolation. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, 1]`.

[Ray2d](struct.Ray2d.html "struct bevy::math::prelude::Ray2d")

An infinite half-line starting at `origin` and going in `direction` in 2D space.

[Ray3d](struct.Ray3d.html "struct bevy::math::prelude::Ray3d")

An infinite half-line starting at `origin` and going in `direction` in 3D space.

[Rect](struct.Rect.html "struct bevy::math::prelude::Rect")

A rectangle defined by two opposite corners.

[Rectangle](struct.Rectangle.html "struct bevy::math::prelude::Rectangle")

A rectangle primitive, which is like a square, except that the width and height can be different

[RegularPolygon](struct.RegularPolygon.html "struct bevy::math::prelude::RegularPolygon")

A polygon centered on the origin where all vertices lie on a circle, equally far apart.

[ReparamCurve](struct.ReparamCurve.html "struct bevy::math::prelude::ReparamCurve")

A curve whose sample space is mapped onto that of some base curve’s before sampling. Curves of this type are produced by [`CurveExt::reparametrize`](../../prelude/trait.CurveExt.html#method.reparametrize "method bevy::prelude::CurveExt::reparametrize").

[RepeatCurve](struct.RepeatCurve.html "struct bevy::math::prelude::RepeatCurve")

The curve that results from repeating a curve `N` times.

[ReverseCurve](struct.ReverseCurve.html "struct bevy::math::prelude::ReverseCurve")

The curve that results from reversing another.

[Rhombus](struct.Rhombus.html "struct bevy::math::prelude::Rhombus")

A rhombus primitive, also known as a diamond shape. A four sided polygon, centered on the origin, where opposite sides are parallel but without requiring right angles.

[Ring](struct.Ring.html "struct bevy::math::prelude::Ring")

A 2D shape representing the ring version of a base shape.

[Rot2](struct.Rot2.html "struct bevy::math::prelude::Rot2")

A 2D rotation.

[SampleAutoCurve](struct.SampleAutoCurve.html "struct bevy::math::prelude::SampleAutoCurve")

A curve that is defined by neighbor interpolation over a set of evenly-spaced samples, interpolated automatically using [a particularly well-behaved interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[SampleCurve](struct.SampleCurve.html "struct bevy::math::prelude::SampleCurve")

A curve that is defined by explicit neighbor interpolation over a set of evenly-spaced samples.

[Segment2d](struct.Segment2d.html "struct bevy::math::prelude::Segment2d")

A line segment defined by two endpoints in 2D space.

[Segment3d](struct.Segment3d.html "struct bevy::math::prelude::Segment3d")

A line segment defined by two endpoints in 3D space.

[SineInCurve](struct.SineInCurve.html "struct bevy::math::prelude::SineInCurve")

`f(t) = 1.0 - cos(t * π / 2.0)`

[SineInOutCurve](struct.SineInOutCurve.html "struct bevy::math::prelude::SineInOutCurve")

Behaves as `SineIn` for t < 0.5 and as `SineOut` for t >= 0.5

[SineOutCurve](struct.SineOutCurve.html "struct bevy::math::prelude::SineOutCurve")

`f(t) = sin(t * π / 2.0)`

[SmoothStepCurve](struct.SmoothStepCurve.html "struct bevy::math::prelude::SmoothStepCurve")

`f(t) = 3t² - 2t³`

[SmoothStepInCurve](struct.SmoothStepInCurve.html "struct bevy::math::prelude::SmoothStepInCurve")

Behaves as the first half of [`SmoothStepCurve`](../../prelude/struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmoothStepOutCurve](struct.SmoothStepOutCurve.html "struct bevy::math::prelude::SmoothStepOutCurve")

Behaves as the second half of [`SmoothStepCurve`](../../prelude/struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmootherStepCurve](struct.SmootherStepCurve.html "struct bevy::math::prelude::SmootherStepCurve")

`f(t) = 6t⁵ - 15t⁴ + 10t³`

[SmootherStepInCurve](struct.SmootherStepInCurve.html "struct bevy::math::prelude::SmootherStepInCurve")

Behaves as the first half of [`SmootherStepCurve`](../../prelude/struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[SmootherStepOutCurve](struct.SmootherStepOutCurve.html "struct bevy::math::prelude::SmootherStepOutCurve")

Behaves as the second half of [`SmootherStepCurve`](../../prelude/struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[Sphere](struct.Sphere.html "struct bevy::math::prelude::Sphere")

A sphere primitive, representing the set of all points some distance from the origin

[StepsCurve](struct.StepsCurve.html "struct bevy::math::prelude::StepsCurve")

`n` steps connecting the start and the end. Jumping behavior is customizable via [`JumpAt`](../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt"). See [`JumpAt`](../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt") for all the options and visual examples.

[Tetrahedron](struct.Tetrahedron.html "struct bevy::math::prelude::Tetrahedron")

A tetrahedron primitive.

[Torus](struct.Torus.html "struct bevy::math::prelude::Torus")

A torus primitive, often representing a ring or donut shape The set of points some distance from a circle centered at the origin

[Triangle2d](struct.Triangle2d.html "struct bevy::math::prelude::Triangle2d")

A triangle in 2D space

[Triangle3d](struct.Triangle3d.html "struct bevy::math::prelude::Triangle3d")

A 3D triangle primitive.

[URect](struct.URect.html "struct bevy::math::prelude::URect")

A rectangle defined by two opposite corners.

[UVec2](struct.UVec2.html "struct bevy::math::prelude::UVec2")

A 2-dimensional vector.

[UVec3](struct.UVec3.html "struct bevy::math::prelude::UVec3")

A 3-dimensional vector.

[UVec4](struct.UVec4.html "struct bevy::math::prelude::UVec4")

A 4-dimensional vector.

[UnevenCore](struct.UnevenCore.html "struct bevy::math::prelude::UnevenCore")`alloc`

The data core of a curve defined by unevenly-spaced samples or keyframes. The intention is to use this in concert with implicitly or explicitly-defined interpolation in user-space in order to implement the curve interface using [`domain`](../../prelude/struct.UnevenCore.html#method.domain "method bevy::prelude::UnevenCore::domain") and [`sample_with`](../../prelude/struct.UnevenCore.html#method.sample_with "method bevy::prelude::UnevenCore::sample_with").

[UnevenSampleAutoCurve](struct.UnevenSampleAutoCurve.html "struct bevy::math::prelude::UnevenSampleAutoCurve")

A curve that is defined by interpolation over unevenly spaced samples, interpolated automatically using [a particularly well-behaved interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[UnevenSampleCurve](struct.UnevenSampleCurve.html "struct bevy::math::prelude::UnevenSampleCurve")

A curve that is defined by interpolation over unevenly spaced samples with explicit interpolation.

[Vec2](struct.Vec2.html "struct bevy::math::prelude::Vec2")

A 2-dimensional vector.

[Vec3](struct.Vec3.html "struct bevy::math::prelude::Vec3")

A 3-dimensional vector.

[Vec4](struct.Vec4.html "struct bevy::math::prelude::Vec4")

A 4-dimensional vector.

[Vec3A](struct.Vec3A.html "struct bevy::math::prelude::Vec3A")

A 3-dimensional vector.

[ViewFrustum](struct.ViewFrustum.html "struct bevy::math::prelude::ViewFrustum")

A region of 3D space defined by the intersection of 6 [`HalfSpace`](../../prelude/struct.HalfSpace.html "struct bevy::prelude::HalfSpace")s.

[ZipCurve](struct.ZipCurve.html "struct bevy::math::prelude::ZipCurve")

A curve that combines the output data from two constituent curves into a tuple output. Curves of this type are produced by [`CurveExt::zip`](../../prelude/trait.CurveExt.html#method.zip "method bevy::prelude::CurveExt::zip").

## Enums

[ChainError](enum.ChainError.html "enum bevy::math::prelude::ChainError")

An error indicating that an end-to-end composition couldn’t be performed because of malformed inputs.

[ConvexPolygonError](enum.ConvexPolygonError.html "enum bevy::math::prelude::ConvexPolygonError")`alloc`

An error that happens when creating a [`ConvexPolygon`](../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon").

[CubicNurbsError](enum.CubicNurbsError.html "enum bevy::math::prelude::CubicNurbsError")

Error during construction of [`CubicNurbs`](../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")

[EaseFunction](enum.EaseFunction.html "enum bevy::math::prelude::EaseFunction")

Curve functions over the [unit interval](../../prelude/struct.Interval.html#associatedconstant.UNIT "associated constant bevy::prelude::Interval::UNIT"), commonly used for easing transitions.

[EulerRot](enum.EulerRot.html "enum bevy::math::prelude::EulerRot")

Euler rotation sequences.

[JumpAt](enum.JumpAt.html "enum bevy::math::prelude::JumpAt")

Configuration options for the [`EaseFunction::Steps`](../../prelude/enum.EaseFunction.html#variant.Steps "variant bevy::prelude::EaseFunction::Steps") curves. This closely replicates the [CSS step function specification](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/steps#description).

[LinearReparamError](enum.LinearReparamError.html "enum bevy::math::prelude::LinearReparamError")

An error indicating that a linear reparameterization couldn’t be performed because of malformed inputs.

[PingPongError](enum.PingPongError.html "enum bevy::math::prelude::PingPongError")

An error indicating that a ping ponging of a curve couldn’t be performed because of malformed inputs.

[RepeatError](enum.RepeatError.html "enum bevy::math::prelude::RepeatError")

An error indicating that a repetition of a curve couldn’t be performed because of malformed inputs.

[ResamplingError](enum.ResamplingError.html "enum bevy::math::prelude::ResamplingError")

An error indicating that a resampling operation could not be performed because of malformed inputs.

[ReverseError](enum.ReverseError.html "enum bevy::math::prelude::ReverseError")

An error indicating that a reversion of a curve couldn’t be performed because of malformed inputs.

[TorusKind](enum.TorusKind.html "enum bevy::math::prelude::TorusKind")

The type of torus determined by the minor and major radii

[WindingOrder](enum.WindingOrder.html "enum bevy::math::prelude::WindingOrder")

The winding order for a set of points

## Traits

[CubicGenerator](trait.CubicGenerator.html "trait bevy::math::prelude::CubicGenerator")`alloc`

Implement this on cubic splines that can generate a cubic curve from their spline parameters.

[Curve](trait.Curve.html "trait bevy::math::prelude::Curve")

A trait for a type that can represent values of type `T` parametrized over a fixed interval.

[CurveExt](trait.CurveExt.html "trait bevy::math::prelude::CurveExt")

Extension trait implemented by [curves](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), allowing access to a number of adaptors and convenience methods.

[CurveResampleExt](trait.CurveResampleExt.html "trait bevy::math::prelude::CurveResampleExt")`alloc`

Extension trait implemented by [curves](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), allowing access to generic resampling methods as well as those based on [stable interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[CyclicCubicGenerator](trait.CyclicCubicGenerator.html "trait bevy::math::prelude::CyclicCubicGenerator")`alloc`

Implement this on cubic splines that can generate a cyclic cubic curve from their spline parameters.

[Ease](trait.Ease.html "trait bevy::math::prelude::Ease")

A type whose values can be eased between.

[FloatExt](trait.FloatExt.html "trait bevy::math::prelude::FloatExt")

A trait for extending [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") with extra methods.

[FromRng](trait.FromRng.html "trait bevy::math::prelude::FromRng")

Ergonomics trait for a type with a [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution, allowing values to be generated uniformly from an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") by a method in its own namespace.

[Inset](trait.Inset.html "trait bevy::math::prelude::Inset")

A primitive that can be resized uniformly.

[Measured2d](trait.Measured2d.html "trait bevy::math::prelude::Measured2d")

A trait for getting measurements of 2D shapes

[Measured3d](trait.Measured3d.html "trait bevy::math::prelude::Measured3d")

A trait for getting measurements of 3D shapes

[Primitive2d](trait.Primitive2d.html "trait bevy::math::prelude::Primitive2d")

A marker trait for 2D primitives

[Primitive3d](trait.Primitive3d.html "trait bevy::math::prelude::Primitive3d")

A marker trait for 3D primitives

[RationalGenerator](trait.RationalGenerator.html "trait bevy::math::prelude::RationalGenerator")`alloc`

Implement this on cubic splines that can generate a rational cubic curve from their spline parameters.

[ShapeSample](trait.ShapeSample.html "trait bevy::math::prelude::ShapeSample")

Exposes methods to uniformly sample a variety of primitive shapes.

[StableInterpolate](trait.StableInterpolate.html "trait bevy::math::prelude::StableInterpolate")

A type with a natural interpolation that provides strong subdivision guarantees.

[ToRing](trait.ToRing.html "trait bevy::math::prelude::ToRing")

Provides a convenience method for converting a primitive to a [`Ring`](../../prelude/struct.Ring.html "struct bevy::prelude::Ring"), with a given thickness.

[Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::math::prelude::Vec2Swizzles")

[Vec3Swizzles](trait.Vec3Swizzles.html "trait bevy::math::prelude::Vec3Swizzles")

[Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::math::prelude::Vec4Swizzles")

## Functions

[bvec2](fn.bvec2.html "fn bevy::math::prelude::bvec2")

Creates a 2-dimensional `bool` vector mask.

[bvec3](fn.bvec3.html "fn bevy::math::prelude::bvec3")

Creates a 3-dimensional `bool` vector mask.

[bvec4](fn.bvec4.html "fn bevy::math::prelude::bvec4")

Creates a 4-dimensional `bool` vector mask.

[bvec3a](fn.bvec3a.html "fn bevy::math::prelude::bvec3a")

Creates a 3-dimensional `bool` vector mask.

[bvec4a](fn.bvec4a.html "fn bevy::math::prelude::bvec4a")

Creates a 4-dimensional `bool` vector mask.

[interval](fn.interval.html "fn bevy::math::prelude::interval")

Create an [`Interval`](../../prelude/struct.Interval.html "struct bevy::prelude::Interval") with a given `start` and `end`. Alias of [`Interval::new`](../../prelude/struct.Interval.html#method.new "associated function bevy::prelude::Interval::new").

[ivec2](fn.ivec2.html "fn bevy::math::prelude::ivec2")

Creates a 2-dimensional vector.

[ivec3](fn.ivec3.html "fn bevy::math::prelude::ivec3")

Creates a 3-dimensional vector.

[ivec4](fn.ivec4.html "fn bevy::math::prelude::ivec4")

Creates a 4-dimensional vector.

[mat2](fn.mat2.html "fn bevy::math::prelude::mat2")

Creates a 2x2 matrix from two column vectors.

[mat3](fn.mat3.html "fn bevy::math::prelude::mat3")

Creates a 3x3 matrix from three column vectors.

[mat4](fn.mat4.html "fn bevy::math::prelude::mat4")

Creates a 4x4 matrix from four column vectors.

[mat3a](fn.mat3a.html "fn bevy::math::prelude::mat3a")

Creates a 3x3 matrix from three column vectors.

[quat](fn.quat.html "fn bevy::math::prelude::quat")

Creates a quaternion from `x`, `y`, `z` and `w` values.

[uvec2](fn.uvec2.html "fn bevy::math::prelude::uvec2")

Creates a 2-dimensional vector.

[uvec3](fn.uvec3.html "fn bevy::math::prelude::uvec3")

Creates a 3-dimensional vector.

[uvec4](fn.uvec4.html "fn bevy::math::prelude::uvec4")

Creates a 4-dimensional vector.

[vec2](fn.vec2.html "fn bevy::math::prelude::vec2")

Creates a 2-dimensional vector.

[vec3](fn.vec3.html "fn bevy::math::prelude::vec3")

Creates a 3-dimensional vector.

[vec4](fn.vec4.html "fn bevy::math::prelude::vec4")

Creates a 4-dimensional vector.

[vec3a](fn.vec3a.html "fn bevy::math::prelude::vec3a")

Creates a 3-dimensional vector.