[bevy](../../index.html)::[prelude](../index.html)

# Module ops 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/lib.rs.html#39)

This mod re-exports the correct versions of floating-point operations with unspecified precision in the standard library depending on whether the `libm` crate feature is enabled.

All the functions here are named according to their versions in the standard library.

It also provides `no_std` compatible alternatives to certain floating-point operations which are not provided in the [`core`](https://doc.rust-lang.org/nightly/core/index.html "mod core") library.

## Traits

[FloatPow](trait.FloatPow.html "trait bevy::prelude::ops::FloatPow")

This extension trait covers shortfall in determinacy from the lack of a `libm` counterpart to `f32::powi`. Use this for the common small exponents.

## Functions

[abs](fn.abs.html "fn bevy::prelude::ops::abs")

Computes the absolute value of x.

[acos](fn.acos.html "fn bevy::prelude::ops::acos")

Computes the arccosine of a number. Return value is in radians in Hyperbolic tangent function.

[acosh](fn.acosh.html "fn bevy::prelude::ops::acosh")

Inverse hyperbolic cosine function.

[asin](fn.asin.html "fn bevy::prelude::ops::asin")

Computes the arcsine of a number. Return value is in radians in the range \[-pi/2, pi/2\] or NaN if the number is outside the range \[-1, 1\].

[asinh](fn.asinh.html "fn bevy::prelude::ops::asinh")

Inverse hyperbolic sine function.

[atan](fn.atan.html "fn bevy::prelude::ops::atan")

Computes the arctangent of a number. Return value is in radians in the range \[-pi/2, pi/2\];

[atan2](fn.atan2.html "fn bevy::prelude::ops::atan2")

Computes the four-quadrant arctangent of `y` and `x` in radians.

[atanh](fn.atanh.html "fn bevy::prelude::ops::atanh")

Inverse hyperbolic tangent function.

[cbrt](fn.cbrt.html "fn bevy::prelude::ops::cbrt")

Returns the cube root of a number.

[ceil](fn.ceil.html "fn bevy::prelude::ops::ceil")

Returns the smallest integer greater than or equal to `x`.

[copysign](fn.copysign.html "fn bevy::prelude::ops::copysign")

Returns a number composed of the magnitude of `x` and the sign of `y`.

[cos](fn.cos.html "fn bevy::prelude::ops::cos")

Computes the cosine of a number (in radians).

[cosh](fn.cosh.html "fn bevy::prelude::ops::cosh")

Hyperbolic cosine function.

[exp](fn.exp.html "fn bevy::prelude::ops::exp")

Returns `e^(self)`, (the exponential function).

[exp2](fn.exp2.html "fn bevy::prelude::ops::exp2")

Returns `2^(self)`.

[exp\_m1](fn.exp_m1.html "fn bevy::prelude::ops::exp_m1")

Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.

[floor](fn.floor.html "fn bevy::prelude::ops::floor")

Returns the largest integer less than or equal to `x`.

[fract](fn.fract.html "fn bevy::prelude::ops::fract")

Returns the fractional part of `x`.

[hypot](fn.hypot.html "fn bevy::prelude::ops::hypot")

Compute the distance between the origin and a point `(x, y)` on the Euclidean plane.

[ln](fn.ln.html "fn bevy::prelude::ops::ln")

Returns the natural logarithm of the number.

[ln\_1p](fn.ln_1p.html "fn bevy::prelude::ops::ln_1p")

Returns `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.

[log2](fn.log2.html "fn bevy::prelude::ops::log2")

Returns the base 2 logarithm of the number.

[log10](fn.log10.html "fn bevy::prelude::ops::log10")

Returns the base 10 logarithm of the number.

[powf](fn.powf.html "fn bevy::prelude::ops::powf")

Raises a number to a floating point power.

[rem\_euclid](fn.rem_euclid.html "fn bevy::prelude::ops::rem_euclid")

Calculates the least nonnegative remainder of `x (mod y)`.

[round](fn.round.html "fn bevy::prelude::ops::round")

Returns the nearest integer to `x`. If a value is half-way between two integers, round away from `0.0`.

[sin](fn.sin.html "fn bevy::prelude::ops::sin")

Computes the sine of a number (in radians).

[sin\_cos](fn.sin_cos.html "fn bevy::prelude::ops::sin_cos")

Simultaneously computes the sine and cosine of the number, `x`. Returns `(sin(x), cos(x))`.

[sinh](fn.sinh.html "fn bevy::prelude::ops::sinh")

Hyperbolic sine function.

[sqrt](fn.sqrt.html "fn bevy::prelude::ops::sqrt")

Returns the square root of a number.

[tan](fn.tan.html "fn bevy::prelude::ops::tan")

Computes the tangent of a number (in radians).

[tanh](fn.tanh.html "fn bevy::prelude::ops::tanh")

Hyperbolic tangent function.