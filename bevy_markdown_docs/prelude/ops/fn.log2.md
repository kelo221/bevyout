[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function log2 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#276)

```rust
pub fn log2(x: f32) -> f32
```

Returns the base 2 logarithm of the number.

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/gizmos/axes.rs ([line 176](../../../src/axes/axes.rs.html#176))

```rust
175fn elerp(v1: Vec3, v2: Vec3, t: f32) -> Vec3 {
176    let x_factor_log = (1. - t) * ops::log2(v1.x) + t * ops::log2(v2.x);
177    let y_factor_log = (1. - t) * ops::log2(v1.y) + t * ops::log2(v2.y);
178    let z_factor_log = (1. - t) * ops::log2(v1.z) + t * ops::log2(v2.z);
179
180    Vec3::new(
181        ops::exp2(x_factor_log),
182        ops::exp2(y_factor_log),
183        ops::exp2(z_factor_log),
184    )
185}
```