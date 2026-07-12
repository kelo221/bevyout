[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function round 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#580)

```rust
pub fn round(x: f32) -> f32
```

Returns the nearest integer to `x`. If a value is half-way between two integers, round away from `0.0`.

This function always returns the precise result.