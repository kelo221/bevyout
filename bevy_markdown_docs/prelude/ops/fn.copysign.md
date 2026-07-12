[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function copysign 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#572)

```rust
pub fn copysign(x: f32, y: f32) -> f32
```

Returns a number composed of the magnitude of `x` and the sign of `y`.

Equal to `x` if the sign of `x` and `y` are the same, otherwise equal to `-x`. If `x` is a `NaN`, then a `NaN` with the sign bit of `y` is returned. Note, however, that conserving the sign bit on `NaN` across arithmetical operations is not generally guaranteed.