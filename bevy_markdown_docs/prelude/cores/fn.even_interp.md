[bevy](../../index.html)::[prelude](../index.html)::[cores](index.html)

# Function even\_interp 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#239)

```rust
pub fn even_interp(
    domain: Interval,
    samples: usize,
    t: f32,
) -> InterpolationDatum<usize>
```

Given a domain and a number of samples taken over that interval, return an [`InterpolationDatum`](enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum") that governs how samples are extracted relative to the stored data.

`domain` must be a bounded interval (i.e. `domain.is_bounded() == true`).

`samples` must be at least 2.

This function will never panic, but it may return invalid indices if its assumptions are violated.