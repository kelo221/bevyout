[bevy](../../index.html)::[math](../index.html)::[f64](index.html)

# Struct DVec3 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#28)

```rust
#[repr(C)]pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

A 3-dimensional vector.

## Fields

`x: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)``y: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)``z: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#34)

### impl [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#36)

#### pub const [ZERO](#associatedconstant.ZERO): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#39)

#### pub const [ONE](#associatedconstant.ONE): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#42)

#### pub const [NEG\_ONE](#associatedconstant.NEG_ONE): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All negative ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#45)

#### pub const [MIN](#associatedconstant.MIN): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All `f64::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#48)

#### pub const [MAX](#associatedconstant.MAX): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All `f64::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#51)

#### pub const [NAN](#associatedconstant.NAN): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All `f64::NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#54)

#### pub const [INFINITY](#associatedconstant.INFINITY): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All `f64::INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#57)

#### pub const [NEG\_INFINITY](#associatedconstant.NEG_INFINITY): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

All `f64::NEG_INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#60)

#### pub const [X](#associatedconstant.X): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#63)

#### pub const [Y](#associatedconstant.Y): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#66)

#### pub const [Z](#associatedconstant.Z): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#69)

#### pub const [NEG\_X](#associatedconstant.NEG_X): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

A unit vector pointing along the negative X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#72)

#### pub const [NEG\_Y](#associatedconstant.NEG_Y): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

A unit vector pointing along the negative Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#75)

#### pub const [NEG\_Z](#associatedconstant.NEG_Z): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

A unit vector pointing along the negative Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#78)

#### pub const [AXES](#associatedconstant.AXES): \[[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#81)

#### pub const [USES\_CORE\_SIMD](#associatedconstant.USES_CORE_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec3 uses Rust Portable SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#83)

#### pub const [USES\_NEON](#associatedconstant.USES_NEON): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec3 uses Arm NEON

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#85)

#### pub const [USES\_SCALAR\_MATH](#associatedconstant.USES_SCALAR_MATH): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

DVec3 uses scalar math

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#87)

#### pub const [USES\_SSE2](#associatedconstant.USES_SSE2): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec3 uses Intel SSE2

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#89)

#### pub const [USES\_WASM\_SIMD](#associatedconstant.USES_WASM_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

DVec3 uses WebAssembly 128-bit SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#91)

#### pub const [USES\_WASM32\_SIMD](#associatedconstant.USES_WASM32_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

👎Deprecated since 0.31.0:

Renamed to USES\_WASM\_SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#96)

#### pub const fn [new](#method.new)(x: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), y: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), z: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a new vector.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_cubes.rs ([line 543](../../../src/many_cubes/many_cubes.rs.html#543))

```rust
540fn spherical_polar_to_cartesian(p: DVec2) -> DVec3 {
541    let (sin_theta, cos_theta) = p.x.sin_cos();
542    let (sin_phi, cos_phi) = p.y.sin_cos();
543    DVec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi)
544}
```

Hide additional examples

examples/stress\_tests/many\_lights.rs ([line 131](../../../src/many_lights/many_lights.rs.html#131))

```rust
128fn spherical_polar_to_cartesian(p: DVec2) -> DVec3 {
129    let (sin_theta, cos_theta) = p.x.sin_cos();
130    let (sin_phi, cos_phi) = p.y.sin_cos();
131    DVec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi)
132}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#103)

#### pub const fn [splat](#method.splat)(v: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a vector with all elements set to `v`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#110-112)

#### pub fn [map](#method.map)<F>(self, f: F) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#124)

#### pub fn [select](#method.select)(mask: [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3"), if\_true: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), if\_false: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#135)

#### pub const fn [from\_array](#method.from_array)(a: \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#142)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#153)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a vector from the first 3 values in `slice`.

##### Panics

Panics if `slice` is less than 3 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#164)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\])

Writes the elements of `self` to the first 3 elements in `slice`.

##### Panics

Panics if `slice` is less than 3 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#183)

#### pub fn [extend](#method.extend)(self, w: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

Creates a 4D vector from `self` and the given `w` value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#192)

#### pub fn [truncate](#method.truncate)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.

Truncation may also be performed by using [`self.xy()`](../../prelude/trait.Vec3Swizzles.html#tymethod.xy "method bevy::prelude::Vec3Swizzles::xy").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#204)

#### pub fn [from\_homogeneous](#method.from_homogeneous)(v: [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Projects a homogeneous coordinate to 3D space by performing perspective divide.

##### Panics

Will panic if `v.w` is `0` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#212)

#### pub fn [to\_homogeneous](#method.to_homogeneous)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

Creates a homogeneous coordinate from `self`, equivalent to `self.extend(1.0)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#219)

#### pub fn [with\_x](#method.with_x)(self, x: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a 3D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#227)

#### pub fn [with\_y](#method.with_y)(self, y: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a 3D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#235)

#### pub fn [with\_z](#method.with_z)(self, z: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Creates a 3D vector from `self` with the given value of `z`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#243)

#### pub fn [dot](#method.dot)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#250)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#257)

#### pub fn [cross](#method.cross)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Computes the cross product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#273)

#### pub fn [min](#method.min)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#289)

#### pub fn [max](#method.max)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#309)

#### pub fn [clamp](#method.clamp)(self, min: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), max: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Component-wise clamping of values, similar to [`f64::clamp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.clamp "method f64::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#322)

#### pub fn [min\_element](#method.min_element)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#335)

#### pub fn [max\_element](#method.max_element)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#344)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#361)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#379)

#### pub fn [element\_sum](#method.element_sum)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#388)

#### pub fn [element\_product](#method.element_product)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#399)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#410)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#421)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#432)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#443)

#### pub fn [cmple](#method.cmple)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#454)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#461)

#### pub fn [abs](#method.abs)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the absolute value of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#476)

#### pub fn [signum](#method.signum)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector with elements representing the sign of `self`.

*   `1.0` if the number is positive, `+0.0` or `INFINITY`
*   `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
*   `NAN` if the number is `NAN`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#487)

#### pub fn [copysign](#method.copysign)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector with signs of `rhs` and the magnitudes of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#504)

#### pub fn [is\_negative\_bitmask](#method.is_negative_bitmask)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.

A negative element results in a `1` bit and a positive element in a `0` bit. Element `x` goes into the first lowest bit, element `y` into the second, etc.

An element is negative if it has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#514)

#### pub fn [is\_finite](#method.is_finite)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#523)

#### pub fn [is\_finite\_mask](#method.is_finite_mask)(self) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Performs `is_finite` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#530)

#### pub fn [is\_nan](#method.is_nan)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#539)

#### pub fn [is\_nan\_mask](#method.is_nan_mask)(self) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Performs `is_nan` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#547)

#### pub fn [length](#method.length)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#557)

#### pub fn [length\_squared](#method.length_squared)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the squared length of `self`.

This is faster than `length()` as it avoids a square root operation.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#566)

#### pub fn [length\_recip](#method.length_recip)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes `1.0 / length()`.

For valid results, `self` must _not_ be of length zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#573)

#### pub fn [distance](#method.distance)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Computes the Euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#580)

#### pub fn [distance\_squared](#method.distance_squared)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Compute the squared euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#587)

#### pub fn [div\_euclid](#method.div_euclid)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the element-wise quotient of \[Euclidean division\] of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#600)

#### pub fn [rem\_euclid](#method.rem_euclid)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the element-wise remainder of [Euclidean division](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.rem_euclid "method f64::rem_euclid") of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#619)

#### pub fn [normalize](#method.normalize)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns `self` normalized to length 1.0.

For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.

See also [`Self::try_normalize()`](../struct.DVec3.html#method.try_normalize "method bevy::math::DVec3::try_normalize") and [`Self::normalize_or_zero()`](../struct.DVec3.html#method.normalize_or_zero "method bevy::math::DVec3::normalize_or_zero").

##### Panics

Will panic if the resulting normalized vector is not finite when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#634)

#### pub fn [try\_normalize](#method.try_normalize)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>

Returns `self` normalized to length 1.0 if possible, else returns `None`.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be `None`.

See also [`Self::normalize_or_zero()`](../struct.DVec3.html#method.normalize_or_zero "method bevy::math::DVec3::normalize_or_zero").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#652)

#### pub fn [normalize\_or](#method.normalize_or)(self, fallback: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns `self` normalized to length 1.0 if possible, else returns a fallback value.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be the fallback value.

See also [`Self::try_normalize()`](../struct.DVec3.html#method.try_normalize "method bevy::math::DVec3::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#669)

#### pub fn [normalize\_or\_zero](#method.normalize_or_zero)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns `self` normalized to length 1.0 if possible, else returns zero.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be zero.

See also [`Self::try_normalize()`](../struct.DVec3.html#method.try_normalize "method bevy::math::DVec3::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#678)

#### pub fn [normalize\_and\_length](#method.normalize_and_length)(self) -> ([DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Returns `self` normalized to length 1.0 and the length of `self`.

If `self` is zero length then `(Self::X, 0.0)` is returned.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#693)

#### pub fn [is\_normalized](#method.is_normalized)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether `self` is length `1.0` or not.

Uses a precision threshold of approximately `1e-4`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#706)

#### pub fn [project\_onto](#method.project_onto)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` is zero length when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#725)

#### pub fn [reject\_from](#method.reject_from)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#738)

#### pub fn [project\_onto\_normalized](#method.project_onto_normalized)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#756)

#### pub fn [reject\_from\_normalized](#method.reject_from_normalized)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#764)

#### pub fn [round](#method.round)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the nearest integer to a number for each element of `self`. Round half-way cases away from 0.0.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#776)

#### pub fn [floor](#method.floor)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the largest integer less than or equal to a number for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#788)

#### pub fn [ceil](#method.ceil)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the smallest integer greater than or equal to a number for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#800)

#### pub fn [trunc](#method.trunc)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the integer part each element of `self`. This means numbers are always truncated towards zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#813)

#### pub fn [step](#method.step)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing `0.0` if `rhs < self` and 1.0 otherwise.

Similar to glsl’s step(edge, x), which translates into edge.step(x)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#820)

#### pub fn [saturate](#method.saturate)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing all elements of `self` clamped to the range of `[0, 1]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#832)

#### pub fn [fract](#method.fract)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the fractional part of the vector as `self - self.trunc()`.

Note that this differs from the GLSL implementation of `fract` which returns `self - self.floor()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#844)

#### pub fn [fract\_gl](#method.fract_gl)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the fractional part of the vector as `self - self.floor()`.

Note that this differs from the Rust implementation of `fract` which returns `self - self.trunc()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#852)

#### pub fn [exp](#method.exp)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing `e^self` (the exponential function) for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#859)

#### pub fn [exp2](#method.exp2)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing `2^self` for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#867)

#### pub fn [ln](#method.ln)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the natural logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#875)

#### pub fn [log2](#method.log2)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the base 2 logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#882)

#### pub fn [powf](#method.powf)(self, n: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing each element of `self` raised to the power of `n`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#894)

#### pub fn [sqrt](#method.sqrt)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the square root for each element of `self`. This returns NaN when the element is negative.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#901)

#### pub fn [cos](#method.cos)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#908)

#### pub fn [sin](#method.sin)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the sine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#915)

#### pub fn [sin\_cos](#method.sin_cos)(self) -> ([DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Returns a tuple of two vectors containing the sine and cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#929)

#### pub fn [recip](#method.recip)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#945)

#### pub fn [lerp](#method.lerp)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), s: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs a linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly extrapolated.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#955)

#### pub fn [move\_towards](#method.move_towards)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), d: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Moves towards `rhs` based on the value `d`.

When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#970)

#### pub fn [midpoint](#method.midpoint)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Calculates the midpoint between `self` and `rhs`.

The midpoint is the average of, or halfway point between, two vectors. `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)` while being slightly cheaper to compute.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#985)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), max\_abs\_diff: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two vectors contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#996)

#### pub fn [clamp\_length](#method.clamp_length)(self, min: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), max: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector with a length no less than `min` and no more than `max`.

##### Panics

Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1016)

#### pub fn [clamp\_length\_max](#method.clamp_length_max)(self, max: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector with a length no more than `max`.

##### Panics

Will panic if `max` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1033)

#### pub fn [clamp\_length\_min](#method.clamp_length_min)(self, min: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a vector with a length no less than `min`.

##### Panics

Will panic if `min` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1052)

#### pub fn [mul\_add](#method.mul_add)(self, a: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), b: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` _may_ be more performant than an unfused multiply-add if the target architecture has a dedicated fma CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1070)

#### pub fn [reflect](#method.reflect)(self, normal: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the reflection vector for a given incident vector `self` and surface normal `normal`.

`normal` must be normalized.

##### Panics

Will panic if `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1086)

#### pub fn [refract](#method.refract)(self, normal: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), eta: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the refraction direction for a given incident vector `self`, surface normal `normal` and ratio of indices of refraction, `eta`. When total internal reflection occurs, a zero vector will be returned.

`self` and `normal` must be normalized.

##### Panics

Will panic if `self` or `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1103)

#### pub fn [angle\_between](#method.angle_between)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the angle (in radians) between two vectors in the range `[0, +π]`.

The inputs do not need to be unit vectors however they must be non-zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1113)

#### pub fn [rotate\_x](#method.rotate_x)(self, angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Rotates around the x axis by `angle` (in radians).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1125)

#### pub fn [rotate\_y](#method.rotate_y)(self, angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Rotates around the y axis by `angle` (in radians).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1137)

#### pub fn [rotate\_z](#method.rotate_z)(self, angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Rotates around the z axis by `angle` (in radians).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1155)

#### pub fn [rotate\_axis](#method.rotate_axis)(self, axis: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Rotates around `axis` by `angle` (in radians).

The axis must be a unit vector.

##### Panics

Will panic if `axis` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1166)

#### pub fn [rotate\_towards](#method.rotate_towards)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), max\_angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Rotates towards `rhs` up to `max_angle` (in radians).

When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to `self.angle_between(rhs)`, the result will be parallel to `rhs`. If `max_angle` is negative, rotates towards the exact opposite of `rhs`. Will not go past the target.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1185)

#### pub fn [any\_orthogonal\_vector](#method.any_orthogonal_vector)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns some vector that is orthogonal to the given one.

The input vector must be finite and non-zero.

The output vector is not necessarily unit length. For that use [`Self::any_orthonormal_vector()`](../struct.DVec3.html#method.any_orthonormal_vector "method bevy::math::DVec3::any_orthonormal_vector") instead.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1203)

#### pub fn [any\_orthonormal\_vector](#method.any_orthonormal_vector)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns any unit vector that is orthogonal to the given one.

The input vector must be unit length.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1220)

#### pub fn [any\_orthonormal\_pair](#method.any_orthonormal_pair)(self) -> ([DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Given a unit vector return two other vectors that together form an orthonormal basis. That is, all three vectors are orthogonal to each other and are normalized.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1239)

#### pub fn [slerp](#method.slerp)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), s: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs a spherical linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly extrapolated.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1278)

#### pub fn [as\_vec3](#method.as_vec3)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Casts all elements of `self` to `f32`.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_lights.rs ([line 84](../../../src/many_lights/many_lights.rs.html#84))

```rust
41fn setup(
42    mut commands: Commands,
43    mut meshes: ResMut<Assets<Mesh>>,
44    mut materials: ResMut<Assets<StandardMaterial>>,
45) {
46    warn!(include_str!("warning_string.txt"));
47
48    const LIGHT_RADIUS: f32 = 0.3;
49    const LIGHT_INTENSITY: f32 = 1000.0;
50    const RADIUS: f32 = 50.0;
51    const N_LIGHTS: usize = 100_000;
52
53    commands.spawn((
54        Mesh3d(meshes.add(Sphere::new(RADIUS).mesh().ico(9).unwrap())),
55        MeshMaterial3d(materials.add(Color::WHITE)),
56        Transform::from_scale(Vec3::NEG_ONE),
57    ));
58
59    let mesh = meshes.add(Cuboid::default());
60    let material = materials.add(StandardMaterial {
61        base_color: DEEP_PINK.into(),
62        ..default()
63    });
64
65    // NOTE: This pattern is good for testing performance of culling as it provides roughly
66    // the same number of visible meshes regardless of the viewing angle.
67    // NOTE: f64 is used to avoid precision issues that produce visual artifacts in the distribution
68    let golden_ratio = 0.5f64 * (1.0f64 + 5.0f64.sqrt());
69
70    // Spawn N_LIGHTS many lights
71    commands.spawn_batch((0..N_LIGHTS).map(move |i| {
72        let mut rng = rng();
73
74        let spherical_polar_theta_phi = fibonacci_spiral_on_sphere(golden_ratio, i, N_LIGHTS);
75        let unit_sphere_p = spherical_polar_to_cartesian(spherical_polar_theta_phi);
76
77        (
78            PointLight {
79                range: LIGHT_RADIUS,
80                intensity: LIGHT_INTENSITY,
81                color: Color::hsl(rng.random_range(0.0..360.0), 1.0, 0.5),
82                ..default()
83            },
84            Transform::from_translation((RADIUS as f64 * unit_sphere_p).as_vec3()),
85        )
86    }));
87
88    // camera
89    match std::env::args().nth(1).as_deref() {
90        Some("orthographic") => commands.spawn((
91            Camera3d::default(),
92            Projection::from(OrthographicProjection {
93                scaling_mode: ScalingMode::FixedHorizontal {
94                    viewport_width: 20.0,
95                },
96                ..OrthographicProjection::default_3d()
97            }),
98        )),
99        _ => commands.spawn(Camera3d::default()),
100    };
101
102    // add one cube, the only one with strong handles
103    // also serves as a reference point during rotation
104    commands.spawn((
105        Mesh3d(mesh),
106        MeshMaterial3d(material),
107        Transform {
108            translation: Vec3::new(0.0, RADIUS, 0.0),
109            scale: Vec3::splat(5.0),
110            ..default()
111        },
112    ));
113}
```

Hide additional examples

examples/stress\_tests/many\_cubes.rs ([line 200](../../../src/many_cubes/many_cubes.rs.html#200))

```rust
161fn setup(
162    mut commands: Commands,
163    args: Res<Args>,
164    mesh_assets: ResMut<Assets<Mesh>>,
165    material_assets: ResMut<Assets<StandardMaterial>>,
166    images: ResMut<Assets<Image>>,
167) {
168    warn!(include_str!("warning_string.txt"));
169
170    let args = args.into_inner();
171    let images = images.into_inner();
172    let material_assets = material_assets.into_inner();
173    let mesh_assets = mesh_assets.into_inner();
174
175    let meshes = init_meshes(args, mesh_assets);
176
177    let material_textures = init_textures(args, images);
178    let materials = init_materials(args, &material_textures, material_assets);
179
180    // We're seeding the PRNG here to make this example deterministic for testing purposes.
181    // This isn't strictly required in practical use unless you need your app to be deterministic.
182    let mut material_rng = ChaCha8Rng::seed_from_u64(42);
183    match args.layout {
184        Layout::Sphere => {
185            // NOTE: This pattern is good for testing performance of culling as it provides roughly
186            // the same number of visible meshes regardless of the viewing angle.
187            let n_points: usize = args.instance_count;
188            // NOTE: f64 is used to avoid precision issues that produce visual artifacts in the distribution
189            let radius = WIDTH as f64 * 2.5;
190            let golden_ratio = 0.5f64 * (1.0f64 + 5.0f64.sqrt());
191            for i in 0..n_points {
192                let spherical_polar_theta_phi =
193                    fibonacci_spiral_on_sphere(golden_ratio, i, n_points);
194                let unit_sphere_p = spherical_polar_to_cartesian(spherical_polar_theta_phi);
195                let (mesh, transform) = meshes.choose(&mut material_rng).unwrap();
196                commands
197                    .spawn((
198                        Mesh3d(mesh.clone()),
199                        MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
200                        Transform::from_translation((radius * unit_sphere_p).as_vec3())
201                            .looking_at(Vec3::ZERO, Vec3::Y)
202                            .mul_transform(*transform),
203                    ))
204                    .insert_if(NoFrustumCulling, || args.no_frustum_culling)
205                    .insert_if(NoAutomaticBatching, || args.no_automatic_batching)
206                    .insert_if(NoCpuCulling, || args.no_cpu_culling);
207            }
208
209            // camera
210            let mut camera = commands.spawn(Camera3d::default());
211            if args.no_indirect_drawing {
212                camera.insert(NoIndirectDrawing);
213            }
214            if args.no_cpu_culling {
215                camera.insert(NoCpuCulling);
216            }
217            if args.motion_blur {
218                camera.insert((
219                    MotionBlur {
220                        // Use an unrealistically large shutter angle so that motion blur is clearly visible.
221                        shutter_angle: 3.0,
222                        ..Default::default()
223                    },
224                    // MSAA and MotionBlur are not compatible on WebGL.
225                    #[cfg(all(
226                        feature = "webgl2",
227                        target_arch = "wasm32",
228                        not(feature = "webgpu")
229                    ))]
230                    Msaa::Off,
231                ));
232            }
233
234            // Inside-out box around the meshes onto which shadows are cast (though you cannot see them...)
235            commands.spawn((
236                Mesh3d(mesh_assets.add(Cuboid::from_size(Vec3::splat(radius as f32 * 2.2)))),
237                MeshMaterial3d(material_assets.add(StandardMaterial::from(Color::WHITE))),
238                Transform::from_scale(-Vec3::ONE),
239                NotShadowCaster,
240            ));
241        }
242        Layout::Cube => {
243            // NOTE: This pattern is good for demonstrating that frustum culling is working correctly
244            // as the number of visible meshes rises and falls depending on the viewing angle.
245            let scale = 2.5;
246
247            // Scale the width and height by the same factor so that we have the
248            // right number of instances.
249            // Because of the moiré pattern check and the fact that we're
250            // spawning 4 instances per trip around the inner loop below, we're
251            // solving the following equation for the factor variable:
252            //
253            //      4 * (9/10 * factor * width * 9/10 * factor * height) = count
254            //
255            // The solution is the value below.
256            let factor = (5.0 / 9.0) * sqrt(args.instance_count as f32)
257                / (sqrt(HEIGHT as f32) * sqrt(WIDTH as f32));
258            let dimensions = (vec2(WIDTH as f32, HEIGHT as f32) * factor)
259                .ceil()
260                .as_uvec2();
261
262            for x in 0..dimensions.x {
263                for y in 0..dimensions.y {
264                    // introduce spaces to break any kind of moiré pattern
265                    if x % 10 == 0 || y % 10 == 0 {
266                        continue;
267                    }
268                    // cube
269                    commands
270                        .spawn((
271                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
272                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
273                            Transform::from_xyz((x as f32) * scale, (y as f32) * scale, 0.0),
274                        ))
275                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
276                    commands
277                        .spawn((
278                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
279                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
280                            Transform::from_xyz(
281                                (x as f32) * scale,
282                                dimensions.y as f32 * scale,
283                                (y as f32) * scale,
284                            ),
285                        ))
286                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
287                    commands
288                        .spawn((
289                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
290                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
291                            Transform::from_xyz((x as f32) * scale, 0.0, (y as f32) * scale),
292                        ))
293                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
294                    commands
295                        .spawn((
296                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
297                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
298                            Transform::from_xyz(0.0, (x as f32) * scale, (y as f32) * scale),
299                        ))
300                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
301                }
302            }
303            // camera
304            let center = 0.5
305                * scale
306                * Vec3::new(
307                    dimensions.x as f32,
308                    dimensions.y as f32,
309                    dimensions.x as f32,
310                );
311            commands.spawn((Camera3d::default(), Transform::from_translation(center)));
312            // Inside-out box around the meshes onto which shadows are cast (though you cannot see them...)
313            commands.spawn((
314                Mesh3d(mesh_assets.add(Cuboid::from_size(2.0 * 1.1 * center))),
315                MeshMaterial3d(material_assets.add(StandardMaterial::from(Color::WHITE))),
316                Transform::from_scale(-Vec3::ONE).with_translation(center),
317                NotShadowCaster,
318            ));
319        }
320        Layout::Dense => {
321            // NOTE: This pattern is good for demonstrating a dense configuration of cubes
322            // overlapping each other, all within the camera frustum.
323            let count = args.instance_count;
324            let size = cbrt(count as f32).round();
325            let gap = 1.25;
326
327            for i in 0..count {
328                let x = i as f32 % size;
329                let y = (i as f32 / size) % size;
330                let z = i as f32 / (size * size);
331                let pos = Vec3::new(x * gap, y * gap, z * gap);
332                commands
333                    .spawn((
334                        Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
335                        MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
336                        Transform::from_translation(pos),
337                    ))
338                    .insert_if(NoCpuCulling, || args.no_cpu_culling);
339            }
340
341            // camera
342            commands.spawn((
343                Camera3d::default(),
344                Transform::from_xyz(100.0, 90.0, 100.0)
345                    .looking_at(Vec3::new(0.0, -10.0, 0.0), Vec3::Y),
346            ));
347        }
348    }
349
350    commands.spawn((
351        DirectionalLight {
352            shadow_maps_enabled: args.shadows,
353            ..default()
354        },
355        Transform::IDENTITY.looking_at(Vec3::new(0.0, -1.0, -1.0), Vec3::Y),
356    ));
357}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1285)

#### pub fn [as\_vec3a](#method.as_vec3a)(self) -> [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Casts all elements of `self` to `f32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1292)

#### pub fn [as\_i8vec3](#method.as_i8vec3)(self) -> [I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1299)

#### pub fn [as\_u8vec3](#method.as_u8vec3)(self) -> [U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1306)

#### pub fn [as\_i16vec3](#method.as_i16vec3)(self) -> [I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1313)

#### pub fn [as\_u16vec3](#method.as_u16vec3)(self) -> [U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1320)

#### pub fn [as\_ivec3](#method.as_ivec3)(self) -> [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1327)

#### pub fn [as\_uvec3](#method.as_uvec3)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

Casts all elements of `self` to `u32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1334)

#### pub fn [as\_i64vec3](#method.as_i64vec3)(self) -> [I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1341)

#### pub fn [as\_u64vec3](#method.as_u64vec3)(self) -> [U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1348)

#### pub fn [as\_isizevec3](#method.as_isizevec3)(self) -> [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1355)

#### pub fn [as\_usizevec3](#method.as_usizevec3)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Casts all elements of `self` to `usize`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1647)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1648)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1650)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1659)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1660)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1662)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1667)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1668)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1670)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1711)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1712)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1714)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1719)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1720)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1722)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1675)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1676)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1678)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1699)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1700)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1702)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1727)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1728)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1730)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1683)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1685)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1692)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1694)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1744)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1746)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1735)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1737)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#89)

### impl [Animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#89)

#### fn [interpolate](../../prelude/trait.Animatable.html#tymethod.interpolate)(a: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), b: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Interpolates between `a` and `b` with an interpolation factor of `time`. [Read more](../../prelude/trait.Animatable.html#tymethod.interpolate)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#89)

#### fn [blend](../../prelude/trait.Animatable.html#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](../../prelude/struct.BlendInput.html "struct bevy::prelude::BlendInput")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>>) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Blends one or more values together. [Read more](../../prelude/trait.Animatable.html#tymethod.blend)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2074)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2076)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2067)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2069)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#20)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#20)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#20)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2176)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2177)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1360)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1362)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Deserialize expects a sequence of 3 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2166)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2167)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1367)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1368)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1370)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1379)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1380)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1382)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1387)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1388)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1390)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1431)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1432)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1434)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1439)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1440)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1442)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1395)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1396)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1398)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1419)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1420)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1422)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1447)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1448)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1450)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1403)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1405)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1412)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1414)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1464)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1466)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1455)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1457)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2214)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([DVec2](../struct.DVec2.html "struct bevy::math::DVec2"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2216)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([DVec2](../struct.DVec2.html "struct bevy::math::DVec2"), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2200)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2202)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2242)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2244)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2249)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec3A](../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2251)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec3A](../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2193)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2195)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2228)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2230)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2235)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2237)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2221)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2223)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2186)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2188)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2141)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2142)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2144)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2154)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2156)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[DVec3](../struct.DVec3.html "struct bevy::math::DVec3") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1507)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1508)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1510)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#982)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DMat3](../struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#983)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#985)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#990)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DMat3](../struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#991)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#993)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1099)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DQuat](../struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1100)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1102)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1107)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DQuat](../struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1108)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1110)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1519)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1520)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1522)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1527)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1528)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1530)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1571)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1572)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1574)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1579)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1580)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1582)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#974)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DMat3](../struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#975)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#977)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> <[DMat3](../struct.DMat3.html "struct bevy::math::DMat3") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#998)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DMat3](../struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#999)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1001)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1086)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DQuat](../struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1094)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> <[DQuat](../struct.DQuat.html "struct bevy::math::DQuat") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Multiplies a quaternion and a 3D vector, returning the rotated vector.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1087)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1115)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DQuat](../struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1116)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dquat.rs.html#1118)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1535)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1536)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1538)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1559)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1560)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1562)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1587)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1588)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1590)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1543)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1545)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1552)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1554)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1604)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1606)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1595)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1597)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2121)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2122)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2124)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2133)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2134)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2136)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#338)

### impl [NormedVectorSpace](../trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#340)

#### fn [norm](../trait.NormedVectorSpace.html#tymethod.norm)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The size of this element. The return value should always be nonnegative.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#345)

#### fn [norm\_squared](../trait.NormedVectorSpace.html#method.norm_squared)(self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The squared norm of this element. Computing this is often faster than computing [`NormedVectorSpace::norm`](../trait.NormedVectorSpace.html#tymethod.norm "method bevy::math::NormedVectorSpace::norm").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#259)

#### fn [distance](../trait.NormedVectorSpace.html#method.distance)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The distance between this element and another, as determined by the norm.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#266)

#### fn [distance\_squared](../trait.NormedVectorSpace.html#method.distance_squared)(self, rhs: Self) -> Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The squared distance between this element and another, as determined by the norm. Note that this is often faster to compute in practice than [`NormedVectorSpace::distance`](../trait.NormedVectorSpace.html#method.distance "method bevy::math::NormedVectorSpace::distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#20)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#20)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#324)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#324)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#324)

#### fn [reflect\_clone](../../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#21)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2101)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2103-2105)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2111)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2113-2115)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1927)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1928)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1930)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1939)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1940)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1942)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1947)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1948)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1950)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1991)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1992)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1994)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1999)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2000)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2002)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1955)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1956)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1958)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1979)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1980)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1982)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2007)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2008)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2010)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1963)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1965)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1972)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1974)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2024)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2026)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2015)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2017)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#665)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#665)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec3<[UniformFloat](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/float/struct.UniformFloat.html "struct rand::distr::uniform::float::UniformFloat")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Serialize as a sequence of 3 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [field](../../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [field\_mut](../../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [field\_at](../../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [field\_at\_mut](../../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [name\_at](../../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [index\_of\_name](../../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [field\_len](../../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [iter\_fields](../../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [to\_dynamic\_struct](../../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#20)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1787)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1788)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1790)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1799)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1802)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1807)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1810)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1851)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1852)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1854)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1859)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1860)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1862)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1815)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1816)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1818)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1839)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1840)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1842)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1867)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1868)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1870)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1823)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1825)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1832)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1834)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[DVec3](../struct.DVec3.html "struct bevy::math::DVec3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1884)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1886)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1875)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#1877)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2081)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2083-2085)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2091)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\> for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#2093-2095)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#6)

#### type [Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2) = [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#8)

#### type [Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4) = [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#11)

#### fn [xx](../../prelude/trait.Vec3Swizzles.html#tymethod.xx)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#19)

#### fn [xy](../../prelude/trait.Vec3Swizzles.html#tymethod.xy)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#27)

#### fn [with\_xy](../../prelude/trait.Vec3Swizzles.html#tymethod.with_xy)(self, rhs: [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#32)

#### fn [xz](../../prelude/trait.Vec3Swizzles.html#tymethod.xz)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#40)

#### fn [with\_xz](../../prelude/trait.Vec3Swizzles.html#tymethod.with_xz)(self, rhs: [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#45)

#### fn [yx](../../prelude/trait.Vec3Swizzles.html#tymethod.yx)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#53)

#### fn [with\_yx](../../prelude/trait.Vec3Swizzles.html#tymethod.with_yx)(self, rhs: [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#58)

#### fn [yy](../../prelude/trait.Vec3Swizzles.html#tymethod.yy)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#66)

#### fn [yz](../../prelude/trait.Vec3Swizzles.html#tymethod.yz)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#74)

#### fn [with\_yz](../../prelude/trait.Vec3Swizzles.html#tymethod.with_yz)(self, rhs: [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#79)

#### fn [zx](../../prelude/trait.Vec3Swizzles.html#tymethod.zx)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#87)

#### fn [with\_zx](../../prelude/trait.Vec3Swizzles.html#tymethod.with_zx)(self, rhs: [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#92)

#### fn [zy](../../prelude/trait.Vec3Swizzles.html#tymethod.zy)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#100)

#### fn [with\_zy](../../prelude/trait.Vec3Swizzles.html#tymethod.with_zy)(self, rhs: [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#105)

#### fn [zz](../../prelude/trait.Vec3Swizzles.html#tymethod.zz)(self) -> [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#113)

#### fn [xxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#118)

#### fn [xxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#123)

#### fn [xxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#128)

#### fn [xyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#133)

#### fn [xyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#138)

#### fn [xzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#143)

#### fn [xzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#148)

#### fn [xzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#153)

#### fn [yxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#158)

#### fn [yxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#163)

#### fn [yxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#168)

#### fn [yyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#173)

#### fn [yyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#178)

#### fn [yyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#183)

#### fn [yzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#188)

#### fn [yzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#193)

#### fn [yzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#198)

#### fn [zxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#203)

#### fn [zxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#208)

#### fn [zxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#213)

#### fn [zyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#218)

#### fn [zyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#223)

#### fn [zyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#228)

#### fn [zzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzx)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#233)

#### fn [zzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzy)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#238)

#### fn [zzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzz)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#243)

#### fn [xxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#248)

#### fn [xxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#253)

#### fn [xxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#258)

#### fn [xxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#263)

#### fn [xxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#268)

#### fn [xxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#273)

#### fn [xxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#278)

#### fn [xxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#283)

#### fn [xxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#288)

#### fn [xyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#293)

#### fn [xyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#298)

#### fn [xyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#303)

#### fn [xyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#308)

#### fn [xyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#313)

#### fn [xyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#318)

#### fn [xyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#323)

#### fn [xyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#328)

#### fn [xyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#333)

#### fn [xzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#338)

#### fn [xzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#343)

#### fn [xzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#348)

#### fn [xzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#353)

#### fn [xzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#358)

#### fn [xzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#363)

#### fn [xzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#368)

#### fn [xzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#373)

#### fn [xzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#378)

#### fn [yxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#383)

#### fn [yxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#388)

#### fn [yxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#393)

#### fn [yxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#398)

#### fn [yxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#403)

#### fn [yxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#408)

#### fn [yxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#413)

#### fn [yxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#418)

#### fn [yxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#423)

#### fn [yyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#428)

#### fn [yyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#433)

#### fn [yyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#438)

#### fn [yyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#443)

#### fn [yyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#448)

#### fn [yyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#453)

#### fn [yyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#458)

#### fn [yyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#463)

#### fn [yyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#468)

#### fn [yzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#473)

#### fn [yzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#478)

#### fn [yzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#483)

#### fn [yzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#488)

#### fn [yzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#493)

#### fn [yzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#498)

#### fn [yzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#503)

#### fn [yzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#508)

#### fn [yzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#513)

#### fn [zxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#518)

#### fn [zxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#523)

#### fn [zxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#528)

#### fn [zxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#533)

#### fn [zxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#538)

#### fn [zxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#543)

#### fn [zxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#548)

#### fn [zxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#553)

#### fn [zxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#558)

#### fn [zyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#563)

#### fn [zyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#568)

#### fn [zyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#573)

#### fn [zyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#578)

#### fn [zyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#583)

#### fn [zyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#588)

#### fn [zyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#593)

#### fn [zyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#598)

#### fn [zyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#603)

#### fn [zzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#608)

#### fn [zzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#613)

#### fn [zzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#618)

#### fn [zzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#623)

#### fn [zzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#628)

#### fn [zzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#633)

#### fn [zzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzx)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#638)

#### fn [zzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzy)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#643)

#### fn [zzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzz)(self) -> [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#103)

#### fn [xyz](../../prelude/trait.Vec3Swizzles.html#method.xyz)(self) -> Self

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#85)

### impl [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#87)

#### const [ZERO](../trait.VectorSpace.html#associatedconstant.ZERO): [DVec3](../struct.DVec3.html "struct bevy::math::DVec3") = DVec3::ZERO

The zero vector, which is the identity of addition for the vector space type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#86)

#### type [Scalar](../trait.VectorSpace.html#associatedtype.Scalar) = [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

The scalar type of this vector space.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#55)

#### fn [lerp](../trait.VectorSpace.html#method.lerp)(self, rhs: Self, t: Self::[Scalar](../trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")) -> Self

Perform vector space linear interpolation between this element and another, based on the parameter `t`. When `t` is `0`, `self` is recovered. When `t` is `1`, `rhs` is recovered. [Read more](../trait.VectorSpace.html#method.lerp)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec3.rs.html#21)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

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

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

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

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](../../prelude/trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](../../prelude/trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](../../prelude/trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#632)

### impl<V> [HasTangent](../trait.HasTangent.html "trait bevy::math::HasTangent") for V

where V: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#633)

#### type [Tangent](../trait.HasTangent.html#associatedtype.Tangent) = V

The tangent type.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

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

### impl<G> [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/src/num_traits/lib.rs.html#133)

### impl<T, Base> [RefNum](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.RefNum.html "trait num_traits::RefNum")<Base> for T

where T: [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<Base, Base> + for<'r> [NumOps](https://docs.rs/num-traits/0.2.19/x86_64-unknown-linux-gnu/num_traits/trait.NumOps.html "trait num_traits::NumOps")<[&'r Base](https://doc.rust-lang.org/nightly/std/primitive.reference.html), Base>,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

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

### impl<T> [Serialize](../../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

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

### impl<T> [ToString](../../prelude/trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](../../prelude/trait.ToString.html#tymethod.to_string)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](../../prelude/trait.ToString.html#tymethod.to_string)

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

### impl<T> [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}