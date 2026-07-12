[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function asin 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#336)

```rust
pub fn asin(x: f32) -> f32
```

Computes the arcsine of a number. Return value is in radians in the range \[-pi/2, pi/2\] or NaN if the number is outside the range \[-1, 1\].

Precision is specified when the `libm` feature is enabled.