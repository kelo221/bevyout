[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function hypot 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#302)

```rust
pub fn hypot(x: f32, y: f32) -> f32
```

Compute the distance between the origin and a point `(x, y)` on the Euclidean plane.

Equivalently, compute the length of the hypotenuse of a right-angle triangle with other sides having length `x.abs()` and `y.abs()`.

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/3d/motion\_blur.rs ([line 294](../../../src/motion_blur/motion_blur.rs.html#294))

```rust
286fn race_track_pos(offset: f32, t: f32) -> Vec2 {
287    let x_tweak = 2.0;
288    let y_tweak = 3.0;
289    let scale = 8.0;
290    let x0 = ops::sin(x_tweak * t);
291    let y0 = ops::cos(y_tweak * t);
292    let dx = x_tweak * ops::cos(x_tweak * t);
293    let dy = y_tweak * -ops::sin(y_tweak * t);
294    let dl = ops::hypot(dx, dy);
295    let x = x0 + offset * dy / dl;
296    let y = y0 - offset * dx / dl;
297    Vec2::new(x, y) * scale
298}
```