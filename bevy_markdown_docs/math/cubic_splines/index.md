[bevy](../../index.html)::[math](../index.html)

# Module cubic\_splines 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#34)

Provides types for building cubic splines for rendering curves and use with animation easing.

## Structs

[CubicBSpline](struct.CubicBSpline.html "struct bevy::math::cubic_splines::CubicBSpline")`alloc`

A spline interpolated continuously across the nearest four control points. The curve does not necessarily pass through any of the control points.

[CubicBezier](struct.CubicBezier.html "struct bevy::math::cubic_splines::CubicBezier")`alloc`

A spline composed of a single cubic Bezier curve.

[CubicBezierError](struct.CubicBezierError.html "struct bevy::math::cubic_splines::CubicBezierError")

An error returned during cubic curve generation for cubic Bezier curves indicating that a segment of control points was not present.

[CubicCardinalSpline](struct.CubicCardinalSpline.html "struct bevy::math::cubic_splines::CubicCardinalSpline")`alloc`

A spline interpolated continuously across the nearest four control points, with the position of the curve specified at every control point and the tangents computed automatically. The associated [`CubicCurve`](../../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve") has one segment between each pair of adjacent control points.

[CubicCurve](struct.CubicCurve.html "struct bevy::math::cubic_splines::CubicCurve")`alloc`

A collection of [`CubicSegment`](../../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")s chained into a single parametric curve. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, N]`, where `N` is its number of segments.

[CubicHermite](struct.CubicHermite.html "struct bevy::math::cubic_splines::CubicHermite")`alloc`

A spline interpolated continuously between the nearest two control points, with the position and velocity of the curve specified at both control points. This curve passes through all control points, with the specified velocity which includes direction and parametric speed.

[CubicNurbs](struct.CubicNurbs.html "struct bevy::math::cubic_splines::CubicNurbs")`alloc`

Non-uniform Rational B-Splines (NURBS) are a powerful generalization of the [`CubicBSpline`](../../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline") which can represent a much more diverse class of curves (like perfect circles and ellipses).

[CubicSegment](struct.CubicSegment.html "struct bevy::math::cubic_splines::CubicSegment")

A segment of a cubic curve, used to hold precomputed coefficients for fast interpolation. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, 1]`.

[InsufficientDataError](struct.InsufficientDataError.html "struct bevy::math::cubic_splines::InsufficientDataError")

An error indicating that a spline construction didn’t have enough control points to generate a curve.

[LinearSpline](struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")`alloc`

A spline interpolated linearly between the nearest 2 points.

[RationalCurve](struct.RationalCurve.html "struct bevy::math::cubic_splines::RationalCurve")`alloc`

A collection of [`RationalSegment`](../../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")s chained into a single parametric curve. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, N]`, where `N` is the number of segments.

[RationalSegment](struct.RationalSegment.html "struct bevy::math::cubic_splines::RationalSegment")

A segment of a rational cubic curve, used to hold precomputed coefficients for fast interpolation. It is a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, 1]`.

## Enums

[CubicNurbsError](enum.CubicNurbsError.html "enum bevy::math::cubic_splines::CubicNurbsError")

Error during construction of [`CubicNurbs`](../../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")

## Traits

[CubicGenerator](trait.CubicGenerator.html "trait bevy::math::cubic_splines::CubicGenerator")`alloc`

Implement this on cubic splines that can generate a cubic curve from their spline parameters.

[CyclicCubicGenerator](trait.CyclicCubicGenerator.html "trait bevy::math::cubic_splines::CyclicCubicGenerator")`alloc`

Implement this on cubic splines that can generate a cyclic cubic curve from their spline parameters.

[RationalGenerator](trait.RationalGenerator.html "trait bevy::math::cubic_splines::RationalGenerator")`alloc`

Implement this on cubic splines that can generate a rational cubic curve from their spline parameters.