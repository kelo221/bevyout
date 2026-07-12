[bevy](../../index.html)::[animation](../index.html)::[prelude](index.html)

# Function interpolate\_with\_cubic\_bezier 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#232-234)

```rust
pub fn interpolate_with_cubic_bezier<T>(
    p0: &T,
    d0: &T,
    d3: &T,
    p3: &T,
    t: f32,
    duration: f32,
) -> Twhere
    T: Animatable + Clone,
```

Evaluates a cubic Bézier curve at a value `t`, given two endpoints and the derivatives at those endpoints.

The derivatives are linearly scaled by `duration`.