[bevy](../index.html)::[math](index.html)

# Trait FloatExt 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/float.rs.html#2)

```rust
pub trait FloatExt {
    // Required methods
    fn lerp(self, rhs: Self, s: Self) -> Self;
    fn inverse_lerp(a: Self, b: Self, v: Self) -> Self;
    fn remap(
        self,
        in_start: Self,
        in_end: Self,
        out_start: Self,
        out_end: Self,
    ) -> Self;
    fn fract_gl(self) -> Self;
    fn step(self, value: Self) -> Self;
    fn saturate(self) -> Self;
}
```

A trait for extending [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") with extra methods.

## Required Methods

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/float.rs.html#9)

#### fn [lerp](#tymethod.lerp)(self, rhs: Self, s: Self) -> Self

Performs a linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0`, the result will be `self`. When `s` is `1`, the result will be `rhs`. When `s` is outside of the range `[0, 1]`, the result is linearly extrapolated.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/float.rs.html#18)

#### fn [inverse\_lerp](#tymethod.inverse_lerp)(a: Self, b: Self, v: Self) -> Self

Returns `v` normalized to the range `[a, b]`.

When `v` is equal to `a` the result will be `0`. When `v` is equal to `b` will be `1`.

When `v` is outside of the range `[a, b]`, the result is linearly extrapolated.

`a` and `b` must not be equal, otherwise the result will be either infinite or `NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/float.rs.html#29)

#### fn [remap](#tymethod.remap)( self, in\_start: Self, in\_end: Self, out\_start: Self, out\_end: Self, ) -> Self

Remap `self` from the input range to the output range.

When `self` is equal to `in_start` this returns `out_start`. When `self` is equal to `in_end` this returns `out_end`.

When `self` is outside of the range `[in_start, in_end]`, the result is linearly extrapolated.

`in_start` and `in_end` must not be equal, otherwise the result will be either infinite or `NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/float.rs.html#37)

#### fn [fract\_gl](#tymethod.fract_gl)(self) -> Self

Returns the fractional part of the input as `self - self.floor()`.

Note that this differs from the Rust implementation of `fract` which returns `self - self.trunc()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/float.rs.html#43)

#### fn [step](#tymethod.step)(self, value: Self) -> Self

Returns `0.0` if `value < self` and 1.0 otherwise.

Similar to glsl’s step(edge, x), which translates into edge.step(x)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/float.rs.html#47)

#### fn [saturate](#tymethod.saturate)(self) -> Self

Returns `self` clamped within the range `[0.0, 1.0]`

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/float.rs.html#5)

### impl [FloatExt](../prelude/trait.FloatExt.html "trait bevy::prelude::FloatExt") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/float.rs.html#7)

#### fn [lerp](#tymethod.lerp)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/float.rs.html#12)

#### fn [inverse\_lerp](#tymethod.inverse_lerp)(a: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), v: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/float.rs.html#17)

#### fn [remap](#tymethod.remap)(self, in\_start: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), in\_end: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), out\_start: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), out\_end: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/float.rs.html#23)

#### fn [fract\_gl](#tymethod.fract_gl)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/float.rs.html#28)

#### fn [step](#tymethod.step)(self, value: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/float.rs.html#37)

#### fn [saturate](#tymethod.saturate)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/float.rs.html#5)

### impl [FloatExt](../prelude/trait.FloatExt.html "trait bevy::prelude::FloatExt") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/float.rs.html#7)

#### fn [lerp](#tymethod.lerp)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), t: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/float.rs.html#12)

#### fn [inverse\_lerp](#tymethod.inverse_lerp)(a: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), b: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), v: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/float.rs.html#17)

#### fn [remap](#tymethod.remap)(self, in\_start: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), in\_end: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), out\_start: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), out\_end: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/float.rs.html#23)

#### fn [fract\_gl](#tymethod.fract_gl)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/float.rs.html#28)

#### fn [step](#tymethod.step)(self, value: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/float.rs.html#37)

#### fn [saturate](#tymethod.saturate)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

## Implementors