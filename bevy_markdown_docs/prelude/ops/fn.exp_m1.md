[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function exp\_m1 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#389)

```rust
pub fn exp_m1(x: f32) -> f32
```

Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.

Precision is specified when the `libm` feature is enabled.