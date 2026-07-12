[bevy](../index.html)::[math](index.html)

# Struct DVec4 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#31)

```rust
#[repr(C)]pub struct DVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
```

A 4-dimensional vector.

## Fields

`x: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)``y: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)``z: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)``w: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#38)

### impl [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#40)

#### pub const [ZERO](#associatedconstant.ZERO): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#43)

#### pub const [ONE](#associatedconstant.ONE): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#46)

#### pub const [NEG\_ONE](#associatedconstant.NEG_ONE): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All negative ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#49)

#### pub const [MIN](#associatedconstant.MIN): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All `f64::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#52)

#### pub const [MAX](#associatedconstant.MAX): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All `f64::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#55)

#### pub const [NAN](#associatedconstant.NAN): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All `f64::NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#58)

#### pub const [INFINITY](#associatedconstant.INFINITY): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All `f64::INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#61)

#### pub const [NEG\_INFINITY](#associatedconstant.NEG_INFINITY): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

All `f64::NEG_INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#64)

#### pub const [X](#associatedconstant.X): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#67)

#### pub const [Y](#associatedconstant.Y): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#70)

#### pub const [Z](#associatedconstant.Z): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#73)

#### pub const [W](#associatedconstant.W): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the positive W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#76)

#### pub const [NEG\_X](#associatedconstant.NEG_X): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the negative X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#79)

#### pub const [NEG\_Y](#associatedconstant.NEG_Y): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the negative Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#82)

#### pub const [NEG\_Z](#associatedconstant.NEG_Z): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the negative Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#85)

#### pub const [NEG\_W](#associatedconstant.NEG_W): [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

A unit vector pointing along the negative W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#88)

#### pub const [AXES](#associatedconstant.AXES): \[[DVec4](struct.DVec4.html "struct bevy::math::DVec4"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#91)

#### pub const [USES\_CORE\_SIMD](#associatedconstant.USES_CORE_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec4 uses Rust Portable SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#93)

#### pub const [USES\_NEON](#associatedconstant.USES_NEON): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec4 uses Arm NEON

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#95)

#### pub const [USES\_SCALAR\_MATH](#associatedconstant.USES_SCALAR_MATH): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

DVec4 uses scalar math

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#97)

#### pub const [USES\_SSE2](#associatedconstant.USES_SSE2): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec4 uses Intel SSE2

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#99)

#### pub const [USES\_WASM\_SIMD](#associatedconstant.USES_WASM_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec4 uses WebAssembly 128-bit SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#101)

#### pub const [USES\_WASM32\_SIMD](#associatedconstant.USES_WASM32_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

👎Deprecated since 0.31.0:

Renamed to USES\_WASM\_SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#106)

#### pub const fn [new](#method.new)(x: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), y: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), z: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), w: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a new vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#113)

#### pub const fn [splat](#method.splat)(v: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a vector with all elements set to `v`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#128-130)

#### pub fn [map](#method.map)<F>(self, f: F) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#142)

#### pub fn [select](#method.select)(mask: [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4"), if\_true: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"), if\_false: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#154)

#### pub const fn [from\_array](#method.from_array)(a: \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#161)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z, w]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#172)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a vector from the first 4 values in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#183)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\])

Writes the elements of `self` to the first 4 elements in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#192)

#### pub fn [truncate](#method.truncate)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.

Truncation to [`DVec3`](struct.DVec3.html "struct bevy::math::DVec3") may also be performed by using [`self.xyz()`](../prelude/trait.Vec4Swizzles.html#tymethod.xyz "method bevy::prelude::Vec4Swizzles::xyz").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#204)

#### pub fn [project](#method.project)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Projects a homogeneous coordinate to 3D space by performing perspective divide.

##### Panics

Will panic if `self.w` is `0` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#211)

#### pub fn [with\_x](#method.with_x)(self, x: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a 4D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#219)

#### pub fn [with\_y](#method.with_y)(self, y: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a 4D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#227)

#### pub fn [with\_z](#method.with_z)(self, z: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a 4D vector from `self` with the given value of `z`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#235)

#### pub fn [with\_w](#method.with_w)(self, w: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Creates a 4D vector from `self` with the given value of `w`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#243)

#### pub fn [dot](#method.dot)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#250)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#262)

#### pub fn [min](#method.min)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#279)

#### pub fn [max](#method.max)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#300)

#### pub fn [clamp](#method.clamp)(self, min: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"), max: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Component-wise clamping of values, similar to [`f64::clamp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.clamp "method f64::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#313)

#### pub fn [min\_element](#method.min_element)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#326)

#### pub fn [max\_element](#method.max_element)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#335)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#356)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#378)

#### pub fn [element\_sum](#method.element_sum)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#387)

#### pub fn [element\_product](#method.element_product)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#398)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#414)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#430)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#446)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#462)

#### pub fn [cmple](#method.cmple)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#478)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#490)

#### pub fn [abs](#method.abs)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the absolute value of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#506)

#### pub fn [signum](#method.signum)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector with elements representing the sign of `self`.

*   `1.0` if the number is positive, `+0.0` or `INFINITY`
*   `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
*   `NAN` if the number is `NAN`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#518)

#### pub fn [copysign](#method.copysign)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector with signs of `rhs` and the magnitudes of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#536)

#### pub fn [is\_negative\_bitmask](#method.is_negative_bitmask)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.

A negative element results in a `1` bit and a positive element in a `0` bit. Element `x` goes into the first lowest bit, element `y` into the second, etc.

An element is negative if it has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#547)

#### pub fn [is\_finite](#method.is_finite)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#556)

#### pub fn [is\_finite\_mask](#method.is_finite_mask)(self) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Performs `is_finite` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#568)

#### pub fn [is\_nan](#method.is_nan)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#577)

#### pub fn [is\_nan\_mask](#method.is_nan_mask)(self) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Performs `is_nan` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#590)

#### pub fn [length](#method.length)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#600)

#### pub fn [length\_squared](#method.length_squared)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the squared length of `self`.

This is faster than `length()` as it avoids a square root operation.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#609)

#### pub fn [length\_recip](#method.length_recip)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes `1.0 / length()`.

For valid results, `self` must _not_ be of length zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#616)

#### pub fn [distance](#method.distance)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the Euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#623)

#### pub fn [distance\_squared](#method.distance_squared)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Compute the squared euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#630)

#### pub fn [div\_euclid](#method.div_euclid)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the element-wise quotient of \[Euclidean division\] of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#644)

#### pub fn [rem\_euclid](#method.rem_euclid)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the element-wise remainder of [Euclidean division](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.rem_euclid "method f64::rem_euclid") of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#664)

#### pub fn [normalize](#method.normalize)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns `self` normalized to length 1.0.

For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.

See also [`Self::try_normalize()`](struct.DVec4.html#method.try_normalize "method bevy::math::DVec4::try_normalize") and [`Self::normalize_or_zero()`](struct.DVec4.html#method.normalize_or_zero "method bevy::math::DVec4::normalize_or_zero").

##### Panics

Will panic if the resulting normalized vector is not finite when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#679)

#### pub fn [try\_normalize](#method.try_normalize)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>

Returns `self` normalized to length 1.0 if possible, else returns `None`.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be `None`.

See also [`Self::normalize_or_zero()`](struct.DVec4.html#method.normalize_or_zero "method bevy::math::DVec4::normalize_or_zero").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#697)

#### pub fn [normalize\_or](#method.normalize_or)(self, fallback: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns `self` normalized to length 1.0 if possible, else returns a fallback value.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be the fallback value.

See also [`Self::try_normalize()`](struct.DVec4.html#method.try_normalize "method bevy::math::DVec4::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#714)

#### pub fn [normalize\_or\_zero](#method.normalize_or_zero)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns `self` normalized to length 1.0 if possible, else returns zero.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be zero.

See also [`Self::try_normalize()`](struct.DVec4.html#method.try_normalize "method bevy::math::DVec4::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#723)

#### pub fn [normalize\_and\_length](#method.normalize_and_length)(self) -> ([DVec4](struct.DVec4.html "struct bevy::math::DVec4"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Returns `self` normalized to length 1.0 and the length of `self`.

If `self` is zero length then `(Self::X, 0.0)` is returned.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#738)

#### pub fn [is\_normalized](#method.is_normalized)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether `self` is length `1.0` or not.

Uses a precision threshold of approximately `1e-4`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#751)

#### pub fn [project\_onto](#method.project_onto)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` is zero length when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#770)

#### pub fn [reject\_from](#method.reject_from)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#783)

#### pub fn [project\_onto\_normalized](#method.project_onto_normalized)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#801)

#### pub fn [reject\_from\_normalized](#method.reject_from_normalized)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#809)

#### pub fn [round](#method.round)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the nearest integer to a number for each element of `self`. Round half-way cases away from 0.0.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#822)

#### pub fn [floor](#method.floor)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the largest integer less than or equal to a number for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#835)

#### pub fn [ceil](#method.ceil)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the smallest integer greater than or equal to a number for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#848)

#### pub fn [trunc](#method.trunc)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the integer part each element of `self`. This means numbers are always truncated towards zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#862)

#### pub fn [step](#method.step)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing `0.0` if `rhs < self` and 1.0 otherwise.

Similar to glsl’s step(edge, x), which translates into edge.step(x)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#869)

#### pub fn [saturate](#method.saturate)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing all elements of `self` clamped to the range of `[0, 1]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#881)

#### pub fn [fract](#method.fract)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the fractional part of the vector as `self - self.trunc()`.

Note that this differs from the GLSL implementation of `fract` which returns `self - self.floor()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#893)

#### pub fn [fract\_gl](#method.fract_gl)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the fractional part of the vector as `self - self.floor()`.

Note that this differs from the Rust implementation of `fract` which returns `self - self.trunc()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#901)

#### pub fn [exp](#method.exp)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing `e^self` (the exponential function) for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#913)

#### pub fn [exp2](#method.exp2)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing `2^self` for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#926)

#### pub fn [ln](#method.ln)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the natural logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#939)

#### pub fn [log2](#method.log2)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the base 2 logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#951)

#### pub fn [powf](#method.powf)(self, n: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing each element of `self` raised to the power of `n`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#964)

#### pub fn [sqrt](#method.sqrt)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the square root for each element of `self`. This returns NaN when the element is negative.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#976)

#### pub fn [cos](#method.cos)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#988)

#### pub fn [sin](#method.sin)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the sine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1000)

#### pub fn [sin\_cos](#method.sin_cos)(self) -> ([DVec4](struct.DVec4.html "struct bevy::math::DVec4"), [DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Returns a tuple of two vectors containing the sine and cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1015)

#### pub fn [recip](#method.recip)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1032)

#### pub fn [lerp](#method.lerp)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"), s: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs a linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly extrapolated.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1042)

#### pub fn [move\_towards](#method.move_towards)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"), d: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Moves towards `rhs` based on the value `d`.

When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1057)

#### pub fn [midpoint](#method.midpoint)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Calculates the midpoint between `self` and `rhs`.

The midpoint is the average of, or halfway point between, two vectors. `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)` while being slightly cheaper to compute.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1072)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"), max\_abs\_diff: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two vectors contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1083)

#### pub fn [clamp\_length](#method.clamp_length)(self, min: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), max: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector with a length no less than `min` and no more than `max`.

##### Panics

Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1103)

#### pub fn [clamp\_length\_max](#method.clamp_length_max)(self, max: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector with a length no more than `max`.

##### Panics

Will panic if `max` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1120)

#### pub fn [clamp\_length\_min](#method.clamp_length_min)(self, min: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a vector with a length no less than `min`.

##### Panics

Will panic if `min` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1139)

#### pub fn [mul\_add](#method.mul_add)(self, a: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"), b: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` _may_ be more performant than an unfused multiply-add if the target architecture has a dedicated fma CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1158)

#### pub fn [reflect](#method.reflect)(self, normal: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the reflection vector for a given incident vector `self` and surface normal `normal`.

`normal` must be normalized.

##### Panics

Will panic if `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1174)

#### pub fn [refract](#method.refract)(self, normal: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"), eta: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the refraction direction for a given incident vector `self`, surface normal `normal` and ratio of indices of refraction, `eta`. When total internal reflection occurs, a zero vector will be returned.

`self` and `normal` must be normalized.

##### Panics

Will panic if `self` or `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1189)

#### pub fn [as\_vec4](#method.as_vec4)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Casts all elements of `self` to `f32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1196)

#### pub fn [as\_i8vec4](#method.as_i8vec4)(self) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1203)

#### pub fn [as\_u8vec4](#method.as_u8vec4)(self) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1210)

#### pub fn [as\_i16vec4](#method.as_i16vec4)(self) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1217)

#### pub fn [as\_u16vec4](#method.as_u16vec4)(self) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1224)

#### pub fn [as\_ivec4](#method.as_ivec4)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1231)

#### pub fn [as\_uvec4](#method.as_uvec4)(self) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Casts all elements of `self` to `u32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1238)

#### pub fn [as\_i64vec4](#method.as_i64vec4)(self) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1245)

#### pub fn [as\_u64vec4](#method.as_u64vec4)(self) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1252)

#### pub fn [as\_isizevec4](#method.as_isizevec4)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1264)

#### pub fn [as\_usizevec4](#method.as_usizevec4)(self) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Casts all elements of `self` to `usize`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1571)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1572)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1574)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1584)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1585)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1587)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1592)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1593)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1595)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1638)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1639)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1641)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1646)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1647)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1649)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1600)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1603)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1625)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1626)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1628)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1654)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1655)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1657)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1608)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1610)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1618)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1620)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1672)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1674)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1662)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1664)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#90)

### impl [Animatable](../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#90)

#### fn [interpolate](../prelude/trait.Animatable.html#tymethod.interpolate)(a: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4"), b: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Interpolates between `a` and `b` with an interpolation factor of `time`. [Read more](../prelude/trait.Animatable.html#tymethod.interpolate)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#90)

#### fn [blend](../prelude/trait.Animatable.html#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](../prelude/struct.BlendInput.html "struct bevy::prelude::BlendInput")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>>) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Blends one or more values together. [Read more](../prelude/trait.Animatable.html#tymethod.blend)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2013)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2015)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2006)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2008)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#22)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#22)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#22)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2122)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2123)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1274)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1276)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Deserialize expects a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2108)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2109)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1281)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1282)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1284)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1294)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1295)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1297)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1302)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1303)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1305)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1348)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1349)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1351)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1356)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1357)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1359)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1310)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1311)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1313)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1335)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1336)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1338)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1364)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1365)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1367)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1318)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1320)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1328)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1330)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1382)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1384)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1372)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1374)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2182)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([DVec2](struct.DVec2.html "struct bevy::math::DVec2"), [DVec2](struct.DVec2.html "struct bevy::math::DVec2"))> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2184)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([DVec2](struct.DVec2.html "struct bevy::math::DVec2"), [DVec2](struct.DVec2.html "struct bevy::math::DVec2"))) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2175)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([DVec2](struct.DVec2.html "struct bevy::math::DVec2"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2177)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([DVec2](struct.DVec2.html "struct bevy::math::DVec2"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2161)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([DVec3](struct.DVec3.html "struct bevy::math::DVec3"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2163)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([DVec3](struct.DVec3.html "struct bevy::math::DVec3"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2168)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [DVec3](struct.DVec3.html "struct bevy::math::DVec3"))> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2170)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [DVec3](struct.DVec3.html "struct bevy::math::DVec3"))) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2147)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2149)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2225)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2227)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2238)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Available on **non-crate feature `scalar-math`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2240)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1196)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[DQuat](struct.DQuat.html "struct bevy::math::DQuat")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1198)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(q: [DQuat](struct.DQuat.html "struct bevy::math::DQuat")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2140)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2142)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2201)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2203)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2213)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2215)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2189)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2191)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2133)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2135)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[DVec4](struct.DVec4.html "struct bevy::math::DVec4") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2081)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2082)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2084)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[DVec4](struct.DVec4.html "struct bevy::math::DVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2095)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2097)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[DVec4](struct.DVec4.html "struct bevy::math::DVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [DVec4](struct.DVec4.html "struct bevy::math::DVec4"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1426)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1427)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1429)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1577)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DMat4](struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1578)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1580)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1585)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DMat4](struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1586)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1588)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1439)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1440)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1442)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1447)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1448)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1450)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1493)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1494)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1496)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1501)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1502)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1504)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1569)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DMat4](struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1570)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1572)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> <[DMat4](struct.DMat4.html "struct bevy::math::DMat4") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1593)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DMat4](struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1594)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat4.rs.html#1596)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1455)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1456)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1458)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1480)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1481)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1483)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1509)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1510)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1512)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1463)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1465)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1473)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1475)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1527)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1529)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1517)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1519)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2060)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2061)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2063)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2073)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2074)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2076)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#326)

### impl [NormedVectorSpace](trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#328)

#### fn [norm](trait.NormedVectorSpace.html#tymethod.norm)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The size of this element. The return value should always be nonnegative.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#333)

#### fn [norm\_squared](trait.NormedVectorSpace.html#method.norm_squared)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The squared norm of this element. Computing this is often faster than computing [`NormedVectorSpace::norm`](trait.NormedVectorSpace.html#tymethod.norm "method bevy::math::NormedVectorSpace::norm").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#259)

#### fn [distance](trait.NormedVectorSpace.html#method.distance)(self, rhs: Self) -> Self::[Scalar](trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The distance between this element and another, as determined by the norm.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#266)

#### fn [distance\_squared](trait.NormedVectorSpace.html#method.distance_squared)(self, rhs: Self) -> Self::[Scalar](trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The squared distance between this element and another, as determined by the norm. Note that this is often faster to compute in practice than [`NormedVectorSpace::distance`](trait.NormedVectorSpace.html#method.distance "method bevy::math::NormedVectorSpace::distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#22)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#22)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#333)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#333)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#333)

#### fn [reflect\_clone](../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#23)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2040)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2042-2044)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2050)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2052-2054)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1861)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1862)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1864)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1874)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1875)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1877)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1882)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1883)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1885)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1928)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1929)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1931)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1936)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1937)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1939)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1890)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1891)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1893)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1915)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1916)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1918)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1944)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1945)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1947)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1898)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1900)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1908)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1910)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1962)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1964)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1952)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1954)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#665)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#665)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec4<[UniformFloat](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/float/struct.UniformFloat.html "struct rand::distr::uniform::float::UniformFloat")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Serialize as a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#22)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1716)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1717)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1719)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1729)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1730)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1732)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1737)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1738)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1740)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1783)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1784)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1786)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1791)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1792)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1794)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1745)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1746)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1748)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1770)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1771)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1773)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1799)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1802)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1753)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1755)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1763)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1765)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[DVec4](struct.DVec4.html "struct bevy::math::DVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1817)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1819)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1807)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#1809)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2020)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2022-2024)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2030)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [DVec4](struct.DVec4.html "struct bevy::math::DVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2032-2034)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [DVec4](struct.DVec4.html "struct bevy::math::DVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#5)

### impl [Vec4Swizzles](../prelude/trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#6)

#### type [Vec2](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec2) = [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#8)

#### type [Vec3](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec3) = [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#11)

#### fn [xx](../prelude/trait.Vec4Swizzles.html#tymethod.xx)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#19)

#### fn [xy](../prelude/trait.Vec4Swizzles.html#tymethod.xy)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#27)

#### fn [with\_xy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xy)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#32)

#### fn [xz](../prelude/trait.Vec4Swizzles.html#tymethod.xz)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#40)

#### fn [with\_xz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xz)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#45)

#### fn [xw](../prelude/trait.Vec4Swizzles.html#tymethod.xw)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#53)

#### fn [with\_xw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xw)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#58)

#### fn [yx](../prelude/trait.Vec4Swizzles.html#tymethod.yx)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#66)

#### fn [with\_yx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yx)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#71)

#### fn [yy](../prelude/trait.Vec4Swizzles.html#tymethod.yy)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#79)

#### fn [yz](../prelude/trait.Vec4Swizzles.html#tymethod.yz)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#87)

#### fn [with\_yz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yz)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#92)

#### fn [yw](../prelude/trait.Vec4Swizzles.html#tymethod.yw)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#100)

#### fn [with\_yw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yw)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#105)

#### fn [zx](../prelude/trait.Vec4Swizzles.html#tymethod.zx)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#113)

#### fn [with\_zx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zx)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#118)

#### fn [zy](../prelude/trait.Vec4Swizzles.html#tymethod.zy)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#126)

#### fn [with\_zy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zy)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#131)

#### fn [zz](../prelude/trait.Vec4Swizzles.html#tymethod.zz)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#139)

#### fn [zw](../prelude/trait.Vec4Swizzles.html#tymethod.zw)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#147)

#### fn [with\_zw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zw)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#152)

#### fn [wx](../prelude/trait.Vec4Swizzles.html#tymethod.wx)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#160)

#### fn [with\_wx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wx)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#165)

#### fn [wy](../prelude/trait.Vec4Swizzles.html#tymethod.wy)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#173)

#### fn [with\_wy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wy)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#178)

#### fn [wz](../prelude/trait.Vec4Swizzles.html#tymethod.wz)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#186)

#### fn [with\_wz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wz)(self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#191)

#### fn [ww](../prelude/trait.Vec4Swizzles.html#tymethod.ww)(self) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#199)

#### fn [xxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#204)

#### fn [xxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#209)

#### fn [xxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#214)

#### fn [xxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#219)

#### fn [xyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#224)

#### fn [xyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#229)

#### fn [xyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#234)

#### fn [with\_xyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyz)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#239)

#### fn [xyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#244)

#### fn [with\_xyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyw)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#249)

#### fn [xzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#254)

#### fn [xzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#259)

#### fn [with\_xzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzy)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#264)

#### fn [xzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#269)

#### fn [xzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#274)

#### fn [with\_xzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzw)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#279)

#### fn [xwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#284)

#### fn [xwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#289)

#### fn [with\_xwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwy)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#294)

#### fn [xwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#299)

#### fn [with\_xwz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwz)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#304)

#### fn [xww](../prelude/trait.Vec4Swizzles.html#tymethod.xww)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#309)

#### fn [yxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#314)

#### fn [yxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#319)

#### fn [yxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#324)

#### fn [with\_yxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxz)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#329)

#### fn [yxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#334)

#### fn [with\_yxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxw)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#339)

#### fn [yyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#344)

#### fn [yyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#349)

#### fn [yyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#354)

#### fn [yyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#359)

#### fn [yzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#364)

#### fn [with\_yzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzx)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#369)

#### fn [yzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#374)

#### fn [yzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#379)

#### fn [yzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#384)

#### fn [with\_yzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzw)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#389)

#### fn [ywx](../prelude/trait.Vec4Swizzles.html#tymethod.ywx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#394)

#### fn [with\_ywx](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywx)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#399)

#### fn [ywy](../prelude/trait.Vec4Swizzles.html#tymethod.ywy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#404)

#### fn [ywz](../prelude/trait.Vec4Swizzles.html#tymethod.ywz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#409)

#### fn [with\_ywz](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywz)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#414)

#### fn [yww](../prelude/trait.Vec4Swizzles.html#tymethod.yww)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#419)

#### fn [zxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#424)

#### fn [zxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#429)

#### fn [with\_zxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxy)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#434)

#### fn [zxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#439)

#### fn [zxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#444)

#### fn [with\_zxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxw)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#449)

#### fn [zyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#454)

#### fn [with\_zyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyx)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#459)

#### fn [zyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#464)

#### fn [zyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#469)

#### fn [zyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#474)

#### fn [with\_zyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyw)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#479)

#### fn [zzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#484)

#### fn [zzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#489)

#### fn [zzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#494)

#### fn [zzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#499)

#### fn [zwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#504)

#### fn [with\_zwx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwx)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#509)

#### fn [zwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#514)

#### fn [with\_zwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwy)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#519)

#### fn [zwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#524)

#### fn [zww](../prelude/trait.Vec4Swizzles.html#tymethod.zww)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#529)

#### fn [wxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#534)

#### fn [wxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#539)

#### fn [with\_wxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxy)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#544)

#### fn [wxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#549)

#### fn [with\_wxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxz)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#554)

#### fn [wxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#559)

#### fn [wyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#564)

#### fn [with\_wyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyx)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#569)

#### fn [wyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#574)

#### fn [wyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#579)

#### fn [with\_wyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyz)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#584)

#### fn [wyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#589)

#### fn [wzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#594)

#### fn [with\_wzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzx)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#599)

#### fn [wzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#604)

#### fn [with\_wzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzy)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#609)

#### fn [wzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#614)

#### fn [wzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzw)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#619)

#### fn [wwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwx)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#624)

#### fn [wwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwy)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#629)

#### fn [wwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwz)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#634)

#### fn [www](../prelude/trait.Vec4Swizzles.html#tymethod.www)(self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#639)

#### fn [xxxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#644)

#### fn [xxxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#649)

#### fn [xxxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#654)

#### fn [xxxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#659)

#### fn [xxyx](../prelude/trait.Vec4Swizzles.html#tymethod.xxyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#664)

#### fn [xxyy](../prelude/trait.Vec4Swizzles.html#tymethod.xxyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#669)

#### fn [xxyz](../prelude/trait.Vec4Swizzles.html#tymethod.xxyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#674)

#### fn [xxyw](../prelude/trait.Vec4Swizzles.html#tymethod.xxyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#679)

#### fn [xxzx](../prelude/trait.Vec4Swizzles.html#tymethod.xxzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#684)

#### fn [xxzy](../prelude/trait.Vec4Swizzles.html#tymethod.xxzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#689)

#### fn [xxzz](../prelude/trait.Vec4Swizzles.html#tymethod.xxzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#694)

#### fn [xxzw](../prelude/trait.Vec4Swizzles.html#tymethod.xxzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#699)

#### fn [xxwx](../prelude/trait.Vec4Swizzles.html#tymethod.xxwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#704)

#### fn [xxwy](../prelude/trait.Vec4Swizzles.html#tymethod.xxwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#709)

#### fn [xxwz](../prelude/trait.Vec4Swizzles.html#tymethod.xxwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#714)

#### fn [xxww](../prelude/trait.Vec4Swizzles.html#tymethod.xxww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#719)

#### fn [xyxx](../prelude/trait.Vec4Swizzles.html#tymethod.xyxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#724)

#### fn [xyxy](../prelude/trait.Vec4Swizzles.html#tymethod.xyxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#729)

#### fn [xyxz](../prelude/trait.Vec4Swizzles.html#tymethod.xyxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#734)

#### fn [xyxw](../prelude/trait.Vec4Swizzles.html#tymethod.xyxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#739)

#### fn [xyyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#744)

#### fn [xyyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#749)

#### fn [xyyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#754)

#### fn [xyyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#759)

#### fn [xyzx](../prelude/trait.Vec4Swizzles.html#tymethod.xyzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#764)

#### fn [xyzy](../prelude/trait.Vec4Swizzles.html#tymethod.xyzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#769)

#### fn [xyzz](../prelude/trait.Vec4Swizzles.html#tymethod.xyzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#774)

#### fn [xywx](../prelude/trait.Vec4Swizzles.html#tymethod.xywx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#779)

#### fn [xywy](../prelude/trait.Vec4Swizzles.html#tymethod.xywy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#784)

#### fn [xywz](../prelude/trait.Vec4Swizzles.html#tymethod.xywz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#789)

#### fn [xyww](../prelude/trait.Vec4Swizzles.html#tymethod.xyww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#794)

#### fn [xzxx](../prelude/trait.Vec4Swizzles.html#tymethod.xzxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#799)

#### fn [xzxy](../prelude/trait.Vec4Swizzles.html#tymethod.xzxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#804)

#### fn [xzxz](../prelude/trait.Vec4Swizzles.html#tymethod.xzxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#809)

#### fn [xzxw](../prelude/trait.Vec4Swizzles.html#tymethod.xzxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#814)

#### fn [xzyx](../prelude/trait.Vec4Swizzles.html#tymethod.xzyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#819)

#### fn [xzyy](../prelude/trait.Vec4Swizzles.html#tymethod.xzyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#824)

#### fn [xzyz](../prelude/trait.Vec4Swizzles.html#tymethod.xzyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#829)

#### fn [xzyw](../prelude/trait.Vec4Swizzles.html#tymethod.xzyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#834)

#### fn [xzzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#839)

#### fn [xzzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#844)

#### fn [xzzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#849)

#### fn [xzzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#854)

#### fn [xzwx](../prelude/trait.Vec4Swizzles.html#tymethod.xzwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#859)

#### fn [xzwy](../prelude/trait.Vec4Swizzles.html#tymethod.xzwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#864)

#### fn [xzwz](../prelude/trait.Vec4Swizzles.html#tymethod.xzwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#869)

#### fn [xzww](../prelude/trait.Vec4Swizzles.html#tymethod.xzww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#874)

#### fn [xwxx](../prelude/trait.Vec4Swizzles.html#tymethod.xwxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#879)

#### fn [xwxy](../prelude/trait.Vec4Swizzles.html#tymethod.xwxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#884)

#### fn [xwxz](../prelude/trait.Vec4Swizzles.html#tymethod.xwxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#889)

#### fn [xwxw](../prelude/trait.Vec4Swizzles.html#tymethod.xwxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#894)

#### fn [xwyx](../prelude/trait.Vec4Swizzles.html#tymethod.xwyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#899)

#### fn [xwyy](../prelude/trait.Vec4Swizzles.html#tymethod.xwyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#904)

#### fn [xwyz](../prelude/trait.Vec4Swizzles.html#tymethod.xwyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#909)

#### fn [xwyw](../prelude/trait.Vec4Swizzles.html#tymethod.xwyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#914)

#### fn [xwzx](../prelude/trait.Vec4Swizzles.html#tymethod.xwzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#919)

#### fn [xwzy](../prelude/trait.Vec4Swizzles.html#tymethod.xwzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#924)

#### fn [xwzz](../prelude/trait.Vec4Swizzles.html#tymethod.xwzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#929)

#### fn [xwzw](../prelude/trait.Vec4Swizzles.html#tymethod.xwzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#934)

#### fn [xwwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#939)

#### fn [xwwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#944)

#### fn [xwwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#949)

#### fn [xwww](../prelude/trait.Vec4Swizzles.html#tymethod.xwww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#954)

#### fn [yxxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#959)

#### fn [yxxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#964)

#### fn [yxxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#969)

#### fn [yxxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#974)

#### fn [yxyx](../prelude/trait.Vec4Swizzles.html#tymethod.yxyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#979)

#### fn [yxyy](../prelude/trait.Vec4Swizzles.html#tymethod.yxyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#984)

#### fn [yxyz](../prelude/trait.Vec4Swizzles.html#tymethod.yxyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#989)

#### fn [yxyw](../prelude/trait.Vec4Swizzles.html#tymethod.yxyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#994)

#### fn [yxzx](../prelude/trait.Vec4Swizzles.html#tymethod.yxzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#999)

#### fn [yxzy](../prelude/trait.Vec4Swizzles.html#tymethod.yxzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1004)

#### fn [yxzz](../prelude/trait.Vec4Swizzles.html#tymethod.yxzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1009)

#### fn [yxzw](../prelude/trait.Vec4Swizzles.html#tymethod.yxzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1014)

#### fn [yxwx](../prelude/trait.Vec4Swizzles.html#tymethod.yxwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1019)

#### fn [yxwy](../prelude/trait.Vec4Swizzles.html#tymethod.yxwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1024)

#### fn [yxwz](../prelude/trait.Vec4Swizzles.html#tymethod.yxwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1029)

#### fn [yxww](../prelude/trait.Vec4Swizzles.html#tymethod.yxww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1034)

#### fn [yyxx](../prelude/trait.Vec4Swizzles.html#tymethod.yyxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1039)

#### fn [yyxy](../prelude/trait.Vec4Swizzles.html#tymethod.yyxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1044)

#### fn [yyxz](../prelude/trait.Vec4Swizzles.html#tymethod.yyxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1049)

#### fn [yyxw](../prelude/trait.Vec4Swizzles.html#tymethod.yyxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1054)

#### fn [yyyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1059)

#### fn [yyyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1064)

#### fn [yyyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1069)

#### fn [yyyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1074)

#### fn [yyzx](../prelude/trait.Vec4Swizzles.html#tymethod.yyzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1079)

#### fn [yyzy](../prelude/trait.Vec4Swizzles.html#tymethod.yyzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1084)

#### fn [yyzz](../prelude/trait.Vec4Swizzles.html#tymethod.yyzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1089)

#### fn [yyzw](../prelude/trait.Vec4Swizzles.html#tymethod.yyzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1094)

#### fn [yywx](../prelude/trait.Vec4Swizzles.html#tymethod.yywx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1099)

#### fn [yywy](../prelude/trait.Vec4Swizzles.html#tymethod.yywy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1104)

#### fn [yywz](../prelude/trait.Vec4Swizzles.html#tymethod.yywz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1109)

#### fn [yyww](../prelude/trait.Vec4Swizzles.html#tymethod.yyww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1114)

#### fn [yzxx](../prelude/trait.Vec4Swizzles.html#tymethod.yzxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1119)

#### fn [yzxy](../prelude/trait.Vec4Swizzles.html#tymethod.yzxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1124)

#### fn [yzxz](../prelude/trait.Vec4Swizzles.html#tymethod.yzxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1129)

#### fn [yzxw](../prelude/trait.Vec4Swizzles.html#tymethod.yzxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1134)

#### fn [yzyx](../prelude/trait.Vec4Swizzles.html#tymethod.yzyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1139)

#### fn [yzyy](../prelude/trait.Vec4Swizzles.html#tymethod.yzyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1144)

#### fn [yzyz](../prelude/trait.Vec4Swizzles.html#tymethod.yzyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1149)

#### fn [yzyw](../prelude/trait.Vec4Swizzles.html#tymethod.yzyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1154)

#### fn [yzzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1159)

#### fn [yzzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1164)

#### fn [yzzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1169)

#### fn [yzzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1174)

#### fn [yzwx](../prelude/trait.Vec4Swizzles.html#tymethod.yzwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1179)

#### fn [yzwy](../prelude/trait.Vec4Swizzles.html#tymethod.yzwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1184)

#### fn [yzwz](../prelude/trait.Vec4Swizzles.html#tymethod.yzwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1189)

#### fn [yzww](../prelude/trait.Vec4Swizzles.html#tymethod.yzww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1194)

#### fn [ywxx](../prelude/trait.Vec4Swizzles.html#tymethod.ywxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1199)

#### fn [ywxy](../prelude/trait.Vec4Swizzles.html#tymethod.ywxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1204)

#### fn [ywxz](../prelude/trait.Vec4Swizzles.html#tymethod.ywxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1209)

#### fn [ywxw](../prelude/trait.Vec4Swizzles.html#tymethod.ywxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1214)

#### fn [ywyx](../prelude/trait.Vec4Swizzles.html#tymethod.ywyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1219)

#### fn [ywyy](../prelude/trait.Vec4Swizzles.html#tymethod.ywyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1224)

#### fn [ywyz](../prelude/trait.Vec4Swizzles.html#tymethod.ywyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1229)

#### fn [ywyw](../prelude/trait.Vec4Swizzles.html#tymethod.ywyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1234)

#### fn [ywzx](../prelude/trait.Vec4Swizzles.html#tymethod.ywzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1239)

#### fn [ywzy](../prelude/trait.Vec4Swizzles.html#tymethod.ywzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1244)

#### fn [ywzz](../prelude/trait.Vec4Swizzles.html#tymethod.ywzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1249)

#### fn [ywzw](../prelude/trait.Vec4Swizzles.html#tymethod.ywzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1254)

#### fn [ywwx](../prelude/trait.Vec4Swizzles.html#tymethod.ywwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1259)

#### fn [ywwy](../prelude/trait.Vec4Swizzles.html#tymethod.ywwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1264)

#### fn [ywwz](../prelude/trait.Vec4Swizzles.html#tymethod.ywwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1269)

#### fn [ywww](../prelude/trait.Vec4Swizzles.html#tymethod.ywww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1274)

#### fn [zxxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1279)

#### fn [zxxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1284)

#### fn [zxxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1289)

#### fn [zxxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1294)

#### fn [zxyx](../prelude/trait.Vec4Swizzles.html#tymethod.zxyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1299)

#### fn [zxyy](../prelude/trait.Vec4Swizzles.html#tymethod.zxyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1304)

#### fn [zxyz](../prelude/trait.Vec4Swizzles.html#tymethod.zxyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1309)

#### fn [zxyw](../prelude/trait.Vec4Swizzles.html#tymethod.zxyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1314)

#### fn [zxzx](../prelude/trait.Vec4Swizzles.html#tymethod.zxzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1319)

#### fn [zxzy](../prelude/trait.Vec4Swizzles.html#tymethod.zxzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1324)

#### fn [zxzz](../prelude/trait.Vec4Swizzles.html#tymethod.zxzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1329)

#### fn [zxzw](../prelude/trait.Vec4Swizzles.html#tymethod.zxzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1334)

#### fn [zxwx](../prelude/trait.Vec4Swizzles.html#tymethod.zxwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1339)

#### fn [zxwy](../prelude/trait.Vec4Swizzles.html#tymethod.zxwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1344)

#### fn [zxwz](../prelude/trait.Vec4Swizzles.html#tymethod.zxwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1349)

#### fn [zxww](../prelude/trait.Vec4Swizzles.html#tymethod.zxww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1354)

#### fn [zyxx](../prelude/trait.Vec4Swizzles.html#tymethod.zyxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1359)

#### fn [zyxy](../prelude/trait.Vec4Swizzles.html#tymethod.zyxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1364)

#### fn [zyxz](../prelude/trait.Vec4Swizzles.html#tymethod.zyxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1369)

#### fn [zyxw](../prelude/trait.Vec4Swizzles.html#tymethod.zyxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1374)

#### fn [zyyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1379)

#### fn [zyyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1384)

#### fn [zyyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1389)

#### fn [zyyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1394)

#### fn [zyzx](../prelude/trait.Vec4Swizzles.html#tymethod.zyzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1399)

#### fn [zyzy](../prelude/trait.Vec4Swizzles.html#tymethod.zyzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1404)

#### fn [zyzz](../prelude/trait.Vec4Swizzles.html#tymethod.zyzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1409)

#### fn [zyzw](../prelude/trait.Vec4Swizzles.html#tymethod.zyzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1414)

#### fn [zywx](../prelude/trait.Vec4Swizzles.html#tymethod.zywx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1419)

#### fn [zywy](../prelude/trait.Vec4Swizzles.html#tymethod.zywy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1424)

#### fn [zywz](../prelude/trait.Vec4Swizzles.html#tymethod.zywz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1429)

#### fn [zyww](../prelude/trait.Vec4Swizzles.html#tymethod.zyww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1434)

#### fn [zzxx](../prelude/trait.Vec4Swizzles.html#tymethod.zzxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1439)

#### fn [zzxy](../prelude/trait.Vec4Swizzles.html#tymethod.zzxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1444)

#### fn [zzxz](../prelude/trait.Vec4Swizzles.html#tymethod.zzxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1449)

#### fn [zzxw](../prelude/trait.Vec4Swizzles.html#tymethod.zzxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1454)

#### fn [zzyx](../prelude/trait.Vec4Swizzles.html#tymethod.zzyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1459)

#### fn [zzyy](../prelude/trait.Vec4Swizzles.html#tymethod.zzyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1464)

#### fn [zzyz](../prelude/trait.Vec4Swizzles.html#tymethod.zzyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1469)

#### fn [zzyw](../prelude/trait.Vec4Swizzles.html#tymethod.zzyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1474)

#### fn [zzzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1479)

#### fn [zzzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1484)

#### fn [zzzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1489)

#### fn [zzzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1494)

#### fn [zzwx](../prelude/trait.Vec4Swizzles.html#tymethod.zzwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1499)

#### fn [zzwy](../prelude/trait.Vec4Swizzles.html#tymethod.zzwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1504)

#### fn [zzwz](../prelude/trait.Vec4Swizzles.html#tymethod.zzwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1509)

#### fn [zzww](../prelude/trait.Vec4Swizzles.html#tymethod.zzww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1514)

#### fn [zwxx](../prelude/trait.Vec4Swizzles.html#tymethod.zwxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1519)

#### fn [zwxy](../prelude/trait.Vec4Swizzles.html#tymethod.zwxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1524)

#### fn [zwxz](../prelude/trait.Vec4Swizzles.html#tymethod.zwxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1529)

#### fn [zwxw](../prelude/trait.Vec4Swizzles.html#tymethod.zwxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1534)

#### fn [zwyx](../prelude/trait.Vec4Swizzles.html#tymethod.zwyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1539)

#### fn [zwyy](../prelude/trait.Vec4Swizzles.html#tymethod.zwyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1544)

#### fn [zwyz](../prelude/trait.Vec4Swizzles.html#tymethod.zwyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1549)

#### fn [zwyw](../prelude/trait.Vec4Swizzles.html#tymethod.zwyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1554)

#### fn [zwzx](../prelude/trait.Vec4Swizzles.html#tymethod.zwzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1559)

#### fn [zwzy](../prelude/trait.Vec4Swizzles.html#tymethod.zwzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1564)

#### fn [zwzz](../prelude/trait.Vec4Swizzles.html#tymethod.zwzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1569)

#### fn [zwzw](../prelude/trait.Vec4Swizzles.html#tymethod.zwzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1574)

#### fn [zwwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1579)

#### fn [zwwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1584)

#### fn [zwwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1589)

#### fn [zwww](../prelude/trait.Vec4Swizzles.html#tymethod.zwww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1594)

#### fn [wxxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1599)

#### fn [wxxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1604)

#### fn [wxxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1609)

#### fn [wxxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1614)

#### fn [wxyx](../prelude/trait.Vec4Swizzles.html#tymethod.wxyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1619)

#### fn [wxyy](../prelude/trait.Vec4Swizzles.html#tymethod.wxyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1624)

#### fn [wxyz](../prelude/trait.Vec4Swizzles.html#tymethod.wxyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1629)

#### fn [wxyw](../prelude/trait.Vec4Swizzles.html#tymethod.wxyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1634)

#### fn [wxzx](../prelude/trait.Vec4Swizzles.html#tymethod.wxzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1639)

#### fn [wxzy](../prelude/trait.Vec4Swizzles.html#tymethod.wxzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1644)

#### fn [wxzz](../prelude/trait.Vec4Swizzles.html#tymethod.wxzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1649)

#### fn [wxzw](../prelude/trait.Vec4Swizzles.html#tymethod.wxzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1654)

#### fn [wxwx](../prelude/trait.Vec4Swizzles.html#tymethod.wxwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1659)

#### fn [wxwy](../prelude/trait.Vec4Swizzles.html#tymethod.wxwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1664)

#### fn [wxwz](../prelude/trait.Vec4Swizzles.html#tymethod.wxwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1669)

#### fn [wxww](../prelude/trait.Vec4Swizzles.html#tymethod.wxww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1674)

#### fn [wyxx](../prelude/trait.Vec4Swizzles.html#tymethod.wyxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1679)

#### fn [wyxy](../prelude/trait.Vec4Swizzles.html#tymethod.wyxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1684)

#### fn [wyxz](../prelude/trait.Vec4Swizzles.html#tymethod.wyxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1689)

#### fn [wyxw](../prelude/trait.Vec4Swizzles.html#tymethod.wyxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1694)

#### fn [wyyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1699)

#### fn [wyyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1704)

#### fn [wyyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1709)

#### fn [wyyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1714)

#### fn [wyzx](../prelude/trait.Vec4Swizzles.html#tymethod.wyzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1719)

#### fn [wyzy](../prelude/trait.Vec4Swizzles.html#tymethod.wyzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1724)

#### fn [wyzz](../prelude/trait.Vec4Swizzles.html#tymethod.wyzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1729)

#### fn [wyzw](../prelude/trait.Vec4Swizzles.html#tymethod.wyzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1734)

#### fn [wywx](../prelude/trait.Vec4Swizzles.html#tymethod.wywx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1739)

#### fn [wywy](../prelude/trait.Vec4Swizzles.html#tymethod.wywy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1744)

#### fn [wywz](../prelude/trait.Vec4Swizzles.html#tymethod.wywz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1749)

#### fn [wyww](../prelude/trait.Vec4Swizzles.html#tymethod.wyww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1754)

#### fn [wzxx](../prelude/trait.Vec4Swizzles.html#tymethod.wzxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1759)

#### fn [wzxy](../prelude/trait.Vec4Swizzles.html#tymethod.wzxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1764)

#### fn [wzxz](../prelude/trait.Vec4Swizzles.html#tymethod.wzxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1769)

#### fn [wzxw](../prelude/trait.Vec4Swizzles.html#tymethod.wzxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1774)

#### fn [wzyx](../prelude/trait.Vec4Swizzles.html#tymethod.wzyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1779)

#### fn [wzyy](../prelude/trait.Vec4Swizzles.html#tymethod.wzyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1784)

#### fn [wzyz](../prelude/trait.Vec4Swizzles.html#tymethod.wzyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1789)

#### fn [wzyw](../prelude/trait.Vec4Swizzles.html#tymethod.wzyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1794)

#### fn [wzzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1799)

#### fn [wzzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1804)

#### fn [wzzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1809)

#### fn [wzzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1814)

#### fn [wzwx](../prelude/trait.Vec4Swizzles.html#tymethod.wzwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1819)

#### fn [wzwy](../prelude/trait.Vec4Swizzles.html#tymethod.wzwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1824)

#### fn [wzwz](../prelude/trait.Vec4Swizzles.html#tymethod.wzwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1829)

#### fn [wzww](../prelude/trait.Vec4Swizzles.html#tymethod.wzww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1834)

#### fn [wwxx](../prelude/trait.Vec4Swizzles.html#tymethod.wwxx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1839)

#### fn [wwxy](../prelude/trait.Vec4Swizzles.html#tymethod.wwxy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1844)

#### fn [wwxz](../prelude/trait.Vec4Swizzles.html#tymethod.wwxz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1849)

#### fn [wwxw](../prelude/trait.Vec4Swizzles.html#tymethod.wwxw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1854)

#### fn [wwyx](../prelude/trait.Vec4Swizzles.html#tymethod.wwyx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1859)

#### fn [wwyy](../prelude/trait.Vec4Swizzles.html#tymethod.wwyy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1864)

#### fn [wwyz](../prelude/trait.Vec4Swizzles.html#tymethod.wwyz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1869)

#### fn [wwyw](../prelude/trait.Vec4Swizzles.html#tymethod.wwyw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1874)

#### fn [wwzx](../prelude/trait.Vec4Swizzles.html#tymethod.wwzx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1879)

#### fn [wwzy](../prelude/trait.Vec4Swizzles.html#tymethod.wwzy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1884)

#### fn [wwzz](../prelude/trait.Vec4Swizzles.html#tymethod.wwzz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1889)

#### fn [wwzw](../prelude/trait.Vec4Swizzles.html#tymethod.wwzw)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1894)

#### fn [wwwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwwx)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1899)

#### fn [wwwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwwy)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1904)

#### fn [wwwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwwz)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#1909)

#### fn [wwww](../prelude/trait.Vec4Swizzles.html#tymethod.wwww)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#481)

#### fn [xyzw](../prelude/trait.Vec4Swizzles.html#method.xyzw)(self) -> Self

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#80)

### impl [VectorSpace](trait.VectorSpace.html "trait bevy::math::VectorSpace") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#82)

#### const [ZERO](trait.VectorSpace.html#associatedconstant.ZERO): [DVec4](struct.DVec4.html "struct bevy::math::DVec4") = DVec4::ZERO

The zero vector, which is the identity of addition for the vector space type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#81)

#### type [Scalar](trait.VectorSpace.html#associatedtype.Scalar) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The scalar type of this vector space.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#55)

#### fn [lerp](trait.VectorSpace.html#method.lerp)(self, rhs: Self, t: Self::[Scalar](trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")) -> Self

Perform vector space linear interpolation between this element and another, based on the parameter `t`. When `t` is `0`, `self` is recovered. When `t` is `1`, `rhs` is recovered. [Read more](trait.VectorSpace.html#method.lerp)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#23)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/anybitpattern.rs.html#56)

### impl<T> [AnyBitPattern](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/anybitpattern/trait.AnyBitPattern.html "trait bytemuck::anybitpattern::AnyBitPattern") for T

where T: [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/checked.rs.html#143)

### impl<T> [CheckedBitPattern](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/checked/trait.CheckedBitPattern.html "trait bytemuck::checked::CheckedBitPattern") for T

where T: [AnyBitPattern](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/anybitpattern/trait.AnyBitPattern.html "trait bytemuck::anybitpattern::AnyBitPattern"),

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/checked.rs.html#144)

#### type [Bits](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/checked/trait.CheckedBitPattern.html#associatedtype.Bits) = T

`Self` _must_ have the same layout as the specified `Bits` except for the possible invalid bit patterns being checked during [`is_valid_bit_pattern`](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/checked/trait.CheckedBitPattern.html#tymethod.is_valid_bit_pattern "associated function bytemuck::checked::CheckedBitPattern::is_valid_bit_pattern").

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/checked.rs.html#147)

#### fn [is\_valid\_bit\_pattern](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/checked/trait.CheckedBitPattern.html#tymethod.is_valid_bit_pattern)(\_bits: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

If this function returns true, then it must be valid to reinterpret `bits` as `&Self`.

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](../prelude/trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](../prelude/trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](../prelude/trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../prelude/trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#632)

### impl<V> [HasTangent](trait.HasTangent.html "trait bevy::math::HasTangent") for V

where V: [VectorSpace](trait.VectorSpace.html "trait bevy::math::VectorSpace"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#633)

#### type [Tangent](trait.HasTangent.html#associatedtype.Tangent) = V

The tangent type.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/no_uninit.rs.html#72)

### impl<T> [NoUninit](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/no_uninit/trait.NoUninit.html "trait bytemuck::no_uninit::NoUninit") for T

where T: [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/src/num_traits/lib.rs.html#143-144)

### impl<T, Rhs> [NumAssignOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumAssignOps.html "trait num_traits::NumAssignOps")<Rhs> for T

where T: [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<Rhs> + [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<Rhs> + [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<Rhs> + [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<Rhs> + [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<Rhs>,

[Source](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/src/num_traits/lib.rs.html#110-115)

### impl<T, Rhs, Output> [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<Rhs, Output> for T

where T: [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<Rhs, Output = Output> + [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<Rhs, Output = Output> + [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<Rhs, Output = Output> + [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<Rhs, Output = Output> + [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<Rhs, Output = Output>,

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/src/num_traits/lib.rs.html#133)

### impl<T, Base> [RefNum](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.RefNum.html "trait num_traits::RefNum")<Base> for T

where T: [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<Base, Base> + for<'r> [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<[&'r Base](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Base>,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/distr/uniform.rs.html#401-403)

### impl<Borrowed> [SampleBorrow](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleBorrow.html "trait rand::distr::uniform::SampleBorrow")<Borrowed> for Borrowed

where Borrowed: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/distr/uniform.rs.html#406)

#### fn [borrow](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleBorrow.html#tymethod.borrow)(&self) -> [&Borrowed](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. See [`Borrow::borrow`](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow "method core::borrow::Borrow::borrow")

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#760-762)

### impl<T> [ToSmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html "trait smol_str::ToSmolStr") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/src/smol_str/lib.rs.html#764)

#### fn [to\_smolstr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/trait.ToSmolStr.html#tymethod.to_smolstr)(&self) -> [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2900)

### impl<T> [ToString](../prelude/trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](../prelude/trait.ToString.html#tymethod.to_string)(&self) -> [String](../prelude/struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](../prelude/trait.ToString.html#tymethod.to_string)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}