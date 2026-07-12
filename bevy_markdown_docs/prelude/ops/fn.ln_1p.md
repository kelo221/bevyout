[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function ln\_1p 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#398)

```rust
pub fn ln_1p(x: f32) -> f32
```

Returns `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.

Precision is specified when the `libm` feature is enabled.