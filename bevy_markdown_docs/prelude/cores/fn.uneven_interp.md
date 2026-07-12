[bevy](../../index.html)::[prelude](../index.html)::[cores](index.html)

# Function uneven\_interp 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#675)

```rust
pub fn uneven_interp(times: &[f32], t: f32) -> InterpolationDatum<usize>
```

Given a list of `times` and a target value, get the interpolation relationship for the target value in terms of the indices of the starting list. In a sense, this encapsulates the heart of uneven/keyframe sampling.

`times` is assumed to be sorted, deduplicated, and consisting only of finite values. It is also assumed to contain at least two values.

## Panics

This function will panic if `times` contains NAN.