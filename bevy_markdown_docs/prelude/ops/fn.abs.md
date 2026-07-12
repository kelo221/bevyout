[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function abs 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#553)

```rust
pub fn abs(x: f32) -> f32
```

Computes the absolute value of x.

This function always returns the precise result.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/2d/dynamic\_mip\_generation.rs ([line 449](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#449))

```rust
448fn triangle_wave(time: f32, wavelength: f32) -> f32 {
449    2.0 * ops::abs(time / wavelength - ops::floor(time / wavelength + 0.5))
450}
```