[bevy](../index.html)::[math](index.html)

# Struct IVec4 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#34)

```rust
#[repr(C)]pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}
```

A 4-dimensional vector.

## Fields

`x: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)``y: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)``z: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)``w: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#41)

### impl [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#43)

#### pub const [ZERO](#associatedconstant.ZERO): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#46)

#### pub const [ONE](#associatedconstant.ONE): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#49)

#### pub const [NEG\_ONE](#associatedconstant.NEG_ONE): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

All negative ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#52)

#### pub const [MIN](#associatedconstant.MIN): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

All `i32::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#55)

#### pub const [MAX](#associatedconstant.MAX): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

All `i32::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#58)

#### pub const [X](#associatedconstant.X): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#61)

#### pub const [Y](#associatedconstant.Y): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#64)

#### pub const [Z](#associatedconstant.Z): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#67)

#### pub const [W](#associatedconstant.W): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the positive W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#70)

#### pub const [NEG\_X](#associatedconstant.NEG_X): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the negative X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#73)

#### pub const [NEG\_Y](#associatedconstant.NEG_Y): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the negative Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#76)

#### pub const [NEG\_Z](#associatedconstant.NEG_Z): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the negative Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#79)

#### pub const [NEG\_W](#associatedconstant.NEG_W): [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

A unit vector pointing along the negative W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#82)

#### pub const [AXES](#associatedconstant.AXES): \[[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#87)

#### pub const fn [new](#method.new)(x: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), y: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), z: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), w: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a new vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#94)

#### pub const fn [splat](#method.splat)(v: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a vector with all elements set to `v`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#109-111)

#### pub fn [map](#method.map)<F>(self, f: F) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#123)

#### pub fn [select](#method.select)(mask: [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4"), if\_true: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), if\_false: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#135)

#### pub const fn [from\_array](#method.from_array)(a: \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#142)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z, w]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#153)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\]) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a vector from the first 4 values in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#164)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\])

Writes the elements of `self` to the first 4 elements in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#173)

#### pub fn [truncate](#method.truncate)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.

Truncation to [`IVec3`](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3") may also be performed by using [`self.xyz()`](../prelude/trait.Vec4Swizzles.html#tymethod.xyz "method bevy::prelude::Vec4Swizzles::xyz").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#181)

#### pub fn [with\_x](#method.with_x)(self, x: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a 4D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#189)

#### pub fn [with\_y](#method.with_y)(self, y: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a 4D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#197)

#### pub fn [with\_z](#method.with_z)(self, z: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a 4D vector from `self` with the given value of `z`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#205)

#### pub fn [with\_w](#method.with_w)(self, w: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Creates a 4D vector from `self` with the given value of `w`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#213)

#### pub fn [dot](#method.dot)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#220)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#229)

#### pub fn [min](#method.min)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#243)

#### pub fn [max](#method.max)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#261)

#### pub fn [clamp](#method.clamp)(self, min: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), max: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Component-wise clamping of values, similar to [`i32::clamp`](https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.clamp "method i32::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#271)

#### pub fn [min\_element](#method.min_element)(self) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#281)

#### pub fn [max\_element](#method.max_element)(self) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#290)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#311)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#333)

#### pub fn [element\_sum](#method.element_sum)(self) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#342)

#### pub fn [element\_product](#method.element_product)(self) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#353)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#369)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#385)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#401)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#417)

#### pub fn [cmple](#method.cmple)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#433)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#445)

#### pub fn [abs](#method.abs)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the absolute value of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#461)

#### pub fn [signum](#method.signum)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector with elements representing the sign of `self`.

*   `0` if the number is zero
*   `1` if the number is positive
*   `-1` if the number is negative

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#479)

#### pub fn [is\_negative\_bitmask](#method.is_negative_bitmask)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.

A negative element results in a `1` bit and a positive element in a `0` bit. Element `x` goes into the first lowest bit, element `y` into the second, etc.

An element is negative if it has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#490)

#### pub fn [length\_squared](#method.length_squared)(self) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Computes the squared length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#497)

#### pub fn [distance\_squared](#method.distance_squared)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

Compute the squared euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#507)

#### pub fn [div\_euclid](#method.div_euclid)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns the element-wise quotient of \[Euclidean division\] of `self` by `rhs`.

##### Panics

This function will panic if any `rhs` element is 0 or the division results in overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#524)

#### pub fn [rem\_euclid](#method.rem_euclid)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns the element-wise remainder of [Euclidean division](https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.rem_euclid "method i32::rem_euclid") of `self` by `rhs`.

##### Panics

This function will panic if any `rhs` element is 0 or the division results in overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#543)

#### pub fn [manhattan\_distance](#method.manhattan_distance)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

##### Overflow

This method may overflow if the result is greater than [`u32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#associatedconstant.MAX "associated constant u32::MAX").

See also [`checked_manhattan_distance`](../prelude/struct.IVec4.html#method.checked_manhattan_distance "method bevy::prelude::IVec4::checked_manhattan_distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#557)

#### pub fn [checked\_manhattan\_distance](#method.checked_manhattan_distance)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

This will returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the result is greater than [`u32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#associatedconstant.MAX "associated constant u32::MAX").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#569)

#### pub fn [chebyshev\_distance](#method.chebyshev_distance)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the [chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance) between two points.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#585)

#### pub fn [as\_vec4](#method.as_vec4)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Casts all elements of `self` to `f32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#592)

#### pub fn [as\_dvec4](#method.as_dvec4)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#599)

#### pub fn [as\_i8vec4](#method.as_i8vec4)(self) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#606)

#### pub fn [as\_u8vec4](#method.as_u8vec4)(self) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#613)

#### pub fn [as\_i16vec4](#method.as_i16vec4)(self) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#620)

#### pub fn [as\_u16vec4](#method.as_u16vec4)(self) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#627)

#### pub fn [as\_uvec4](#method.as_uvec4)(self) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Casts all elements of `self` to `u32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#634)

#### pub fn [as\_i64vec4](#method.as_i64vec4)(self) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#641)

#### pub fn [as\_u64vec4](#method.as_u64vec4)(self) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#648)

#### pub fn [as\_isizevec4](#method.as_isizevec4)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#660)

#### pub fn [as\_usizevec4](#method.as_usizevec4)(self) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Casts all elements of `self` to `usize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#674)

#### pub const fn [checked\_add](#method.checked_add)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#700)

#### pub const fn [checked\_sub](#method.checked_sub)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#726)

#### pub const fn [checked\_mul](#method.checked_mul)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `Some([self.x * rhs.x, self.y * rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#752)

#### pub const fn [checked\_div](#method.checked_div)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `Some([self.x / rhs.x, self.y / rhs.y, ..])` but returns `None` on any division by zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#778)

#### pub const fn [wrapping\_add](#method.wrapping_add)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#792)

#### pub const fn [wrapping\_sub](#method.wrapping_sub)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#806)

#### pub const fn [wrapping\_mul](#method.wrapping_mul)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#820)

#### pub const fn [wrapping\_div](#method.wrapping_div)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#834)

#### pub const fn [saturating\_add](#method.saturating_add)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the saturating addition of `self` and `rhs`.

In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#848)

#### pub const fn [saturating\_sub](#method.saturating_sub)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the saturating subtraction of `self` and `rhs`.

In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#862)

#### pub const fn [saturating\_mul](#method.saturating_mul)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the saturating multiplication of `self` and `rhs`.

In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#876)

#### pub const fn [saturating\_div](#method.saturating_div)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the saturating division of `self` and `rhs`.

In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#890)

#### pub const fn [checked\_add\_unsigned](#method.checked_add_unsigned)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

Returns a vector containing the wrapping addition of `self` and unsigned vector `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#916)

#### pub const fn [checked\_sub\_unsigned](#method.checked_sub_unsigned)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

Returns a vector containing the wrapping subtraction of `self` and unsigned vector `rhs`.

In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#942)

#### pub const fn [wrapping\_add\_unsigned](#method.wrapping_add_unsigned)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the wrapping addition of `self` and unsigned vector `rhs`.

In other words this computes `[self.x.wrapping_add_unsigned(rhs.x), self.y.wrapping_add_unsigned(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#956)

#### pub const fn [wrapping\_sub\_unsigned](#method.wrapping_sub_unsigned)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the wrapping subtraction of `self` and unsigned vector `rhs`.

In other words this computes `[self.x.wrapping_sub_unsigned(rhs.x), self.y.wrapping_sub_unsigned(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#970)

#### pub const fn [saturating\_add\_unsigned](#method.saturating_add_unsigned)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

In other words this computes `[self.x.saturating_add_unsigned(rhs.x), self.y.saturating_add_unsigned(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#984)

#### pub const fn [saturating\_sub\_unsigned](#method.saturating_sub_unsigned)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a vector containing the saturating subtraction of `self` and unsigned vector `rhs`.

In other words this computes `[self.x.saturating_sub_unsigned(rhs.x), self.y.saturating_sub_unsigned(rhs.y), ..]`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1291)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1292)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1294)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1304)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1305)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1307)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1312)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1313)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1315)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1358)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1359)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1361)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1366)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1367)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1369)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1320)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1321)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1323)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1345)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1346)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1348)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1374)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1375)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1377)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1328)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1330)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1338)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1340)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1392)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1394)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1382)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1384)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1733)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1735)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [AsMutVectorParts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

#### fn [as\_mut\_parts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html#tymethod.as_mut_parts)(&mut self) -> &mut \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1726)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1728)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [AsRefVectorParts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

#### fn [as\_ref\_parts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html#tymethod.as_ref_parts)(&self) -> &\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1822)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1823)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1825)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1835)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1836)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1838)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1843)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1844)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1846)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1988)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1989)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1991)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1996)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1997)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1999)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1851)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1852)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1854)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1975)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1976)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1978)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2004)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2005)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2007)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1859)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1861)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1866)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1868)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2019)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2021)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2012)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2014)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1873)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1874)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1876)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1886)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1887)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1889)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1894)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1895)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1897)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2039)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2040)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2042)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2047)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2050)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1902)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1903)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1905)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2026)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2027)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2029)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2055)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2058)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1910)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1912)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1917)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1919)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2070)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2072)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2063)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2065)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1924)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1925)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1927)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1937)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1938)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1940)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1945)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1946)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1948)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2090)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2091)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2093)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2098)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2099)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2101)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1953)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1954)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1956)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2077)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2078)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2080)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2106)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2107)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2109)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1961)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1963)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1968)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1970)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2121)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2123)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2114)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2116)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [FromVectorParts](../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

#### fn [create\_from](../render/render_resource/encase/internal/trait.CreateFrom.html#tymethod.create_from)<B>(reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3125)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3126)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#994)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#996)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1123)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Deserialize expects a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1123)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3119)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3120)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1001)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1002)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1004)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1014)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1015)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1017)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1022)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1023)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1025)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1068)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1069)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1071)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1076)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1079)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1030)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1031)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1033)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1055)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1058)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1084)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1085)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1087)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1038)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1040)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1048)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1050)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1102)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1104)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1092)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1094)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3185)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"), [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"))> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3187)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"), [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"))) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3178)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3180)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3164)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3166)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3171)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"))> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3173)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"))) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3150)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3152)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3310)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3312)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3323)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Available on **non-crate feature `scalar-math`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3325)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3192)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3194)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3216)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3218)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2201)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2203)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3143)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3145)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3240)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3242)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3204)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3206)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3228)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3230)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3136)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3138)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [FromVectorParts](../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

#### fn [from\_parts](../render/render_resource/encase/vector/trait.FromVectorParts.html#tymethod.from_parts)(parts: \[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3092)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3093)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3095)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3106)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3108)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1146)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1147)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1149)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1159)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1160)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1162)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1167)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1168)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1170)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1213)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1214)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1216)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1221)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1222)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1224)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1175)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1176)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1178)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1200)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1201)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1203)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1229)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1230)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1232)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1183)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1185)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1193)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1195)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1247)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1249)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1237)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1239)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1780)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1783)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1793)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1794)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1796)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1801)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1802)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1804)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1814)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1815)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1817)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#38)

#### fn [reflect\_hash](../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#38)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#38)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#38)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#26)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1760)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1762-1764)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1770)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1772-1774)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [AsMutVectorParts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

#### fn [read\_from](../render/render_resource/encase/internal/trait.ReadFrom.html#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1581)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1582)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1584)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1594)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1595)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1597)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1602)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1603)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1605)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1648)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1649)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1651)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1656)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1657)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1659)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1610)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1611)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1613)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1635)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1636)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1638)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1664)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1665)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1667)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1618)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1620)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1628)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1630)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1682)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1684)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1672)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1674)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#683)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#683)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec4<[UniformInt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/int/struct.UniformInt.html "struct rand::distr::uniform::int::UniformInt")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1123)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Serialize as a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1123)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#232)

#### const [SHADER\_SIZE](../render/render_resource/trait.ShaderSize.html#associatedconstant.SHADER_SIZE): [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> = \_

Represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderType::min_size`](../render/render_resource/trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size"))

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#94)

#### fn [min\_size](../render/render_resource/trait.ShaderType.html#method.min_size)() -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Represents the minimum size of `Self` (equivalent to [GPUBufferBindingLayout.minBindingSize](https://gpuweb.github.io/gpuweb/#dom-gpubufferbindinglayout-minbindingsize)) [Read more](../render/render_resource/trait.ShaderType.html#method.min_size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#103)

#### fn [size](../render/render_resource/trait.ShaderType.html#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns the size of `Self` at runtime [Read more](../render/render_resource/trait.ShaderType.html#method.size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#206)

#### fn [assert\_uniform\_compat](../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)()

Asserts that `Self` meets the requirements of the [uniform address space restrictions on stored values](https://gpuweb.github.io/gpuweb/wgsl/#address-spaces-uniform) and the [uniform address space layout constraints](https://gpuweb.github.io/gpuweb/wgsl/#address-space-layout-constraints) [Read more](../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2944)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2945)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2947)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2957)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2958)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2960)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2965)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2966)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2968)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2784)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2785)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2787)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2792)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2793)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2795)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2957)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2958)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2960)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2965)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2966)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2968)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2784)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2785)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2787)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2792)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2793)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2795)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2957)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2958)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2960)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2965)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2966)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2968)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2784)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2785)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2787)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2792)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2793)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2795)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2957)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2958)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2960)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2965)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2966)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2968)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2784)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2785)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2787)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2792)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2793)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2795)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2779)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2780)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2782)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2787)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2788)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2790)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2952)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2953)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2955)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2960)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2961)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2963)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3031)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3032)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3034)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3039)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3040)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3042)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2141)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2142)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2144)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2149)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2150)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2152)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2243)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2244)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2246)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2251)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2252)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2254)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2345)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2346)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2348)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2353)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2354)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2356)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2447)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2448)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2450)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2455)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2456)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2458)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2549)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2550)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2552)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2557)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2558)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2560)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2651)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2652)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2654)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2659)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2660)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2662)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2753)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2754)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2756)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2761)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2762)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2764)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2855)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2856)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2858)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2863)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2864)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2866)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2944)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2945)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2947)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2973)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2974)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2976)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2771)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2774)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2800)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2801)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2803)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2944)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2945)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2947)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2973)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2974)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2976)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2771)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2774)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2800)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2801)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2803)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2973)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2974)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2976)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2771)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2774)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2800)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2801)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2803)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2944)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2945)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2947)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2973)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2974)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2976)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2771)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2774)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2800)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2801)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2803)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2766)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2767)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2769)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2795)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2796)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2798)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2939)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2940)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2942)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2968)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2969)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2971)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3018)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3019)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3021)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3047)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3050)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2128)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2129)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2131)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2157)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2158)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2160)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2230)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2231)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2233)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2259)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2260)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2262)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2332)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2333)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2335)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2361)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2362)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2364)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2434)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2435)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2437)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2463)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2464)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2466)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2536)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2537)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2539)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2565)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2566)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2568)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2638)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2639)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2641)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2667)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2668)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2670)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2740)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2741)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2743)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2769)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2770)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2772)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2842)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2843)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2845)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2871)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2872)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2874)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2172)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2174)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2274)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2276)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2376)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2378)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2478)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2480)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2580)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2582)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2682)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2684)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2784)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2786)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2886)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2888)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2165)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2167)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2267)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2269)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2369)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2371)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2471)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2473)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2573)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2575)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2675)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2677)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2777)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2779)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2879)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2881)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2981)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2982)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2984)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2994)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2995)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2997)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3002)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3003)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3005)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2821)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2822)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2824)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2829)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2830)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2832)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2994)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2995)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2997)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3002)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3003)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3005)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2821)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2822)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2824)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2829)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2830)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2832)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2994)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2995)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2997)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3002)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3003)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3005)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2821)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2822)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2824)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2829)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2830)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2832)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2994)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2995)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2997)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3002)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3003)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3005)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2821)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2822)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2824)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2829)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2830)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2832)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2816)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2817)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2819)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2824)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2825)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2827)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2989)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2990)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2992)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2997)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2998)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3000)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3068)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3069)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3071)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3076)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3079)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2192)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2193)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2195)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2200)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2201)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2203)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2294)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2295)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2297)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2302)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2303)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2305)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2396)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2397)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2399)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2404)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2405)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2407)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2498)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2499)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2501)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2506)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2507)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2509)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2600)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2601)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2603)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2608)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2609)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2611)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2702)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2703)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2705)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2710)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2711)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2713)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2804)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2805)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2807)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2812)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2813)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2815)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2906)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2907)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2909)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2914)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2915)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2917)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2981)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2982)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#2984)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3010)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3011)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3013)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2808)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2809)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2811)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2837)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2838)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2840)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2981)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2982)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#2984)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3010)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3011)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3013)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2808)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2809)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2811)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2837)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2838)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2840)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3010)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3011)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3013)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2808)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2809)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2811)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2837)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2838)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2840)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2981)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2982)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#2984)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3010)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3011)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3013)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2808)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2809)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2811)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2837)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2838)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2840)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2803)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2804)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2806)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2832)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2833)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2835)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2976)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2977)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2979)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3005)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3006)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3008)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3055)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3058)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3084)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3085)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3087)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2179)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2180)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2182)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2208)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2209)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2211)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2281)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2282)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2284)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2310)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2311)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2313)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2383)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2384)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2386)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2412)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2413)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2415)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2485)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2486)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2488)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2514)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2515)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2517)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2587)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2588)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2590)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2616)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2617)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2619)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2689)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2690)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2692)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2718)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2719)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2721)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2791)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2792)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2794)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2820)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2821)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2823)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2893)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2894)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2896)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2922)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2923)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2925)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2223)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2225)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2325)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2327)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2427)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2429)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2529)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2531)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2631)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2633)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2733)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2735)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2835)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2837)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2937)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2939)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2216)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2218)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2318)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2320)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2420)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2422)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2522)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2524)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2624)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2626)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2726)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2728)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2828)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2830)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2930)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#2932)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#25)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1436)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1437)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1439)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1449)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1450)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1452)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1457)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1458)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1460)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1503)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1504)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1506)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1511)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1512)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1514)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1465)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1466)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1468)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1490)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1491)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1493)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1519)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1520)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1522)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1473)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1475)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1483)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1485)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1537)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1539)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1527)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1529)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1740)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1742-1744)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1750)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#1752-1754)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3254)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3255)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3258)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3282)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3283)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3286)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3234)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3235)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3238)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4"), <[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3061)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3062)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3065)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4"), <[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3230)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3231)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3234)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4"), <[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3059)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3060)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3063)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4"), <[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3071)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3072)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3075)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3083)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3084)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3087)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4"), <[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3066)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3067)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3070)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4"), <[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3279)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3280)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3283)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3268)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3269)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3272)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3296)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3297)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3300)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3240)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3241)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3244)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#5)

### impl [Vec4Swizzles](../prelude/trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#6)

#### type [Vec2](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec2) = [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#8)

#### type [Vec3](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec3) = [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#11)

#### fn [xx](../prelude/trait.Vec4Swizzles.html#tymethod.xx)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#19)

#### fn [xy](../prelude/trait.Vec4Swizzles.html#tymethod.xy)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#27)

#### fn [with\_xy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xy)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#32)

#### fn [xz](../prelude/trait.Vec4Swizzles.html#tymethod.xz)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#40)

#### fn [with\_xz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xz)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#45)

#### fn [xw](../prelude/trait.Vec4Swizzles.html#tymethod.xw)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#53)

#### fn [with\_xw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xw)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#58)

#### fn [yx](../prelude/trait.Vec4Swizzles.html#tymethod.yx)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#66)

#### fn [with\_yx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yx)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#71)

#### fn [yy](../prelude/trait.Vec4Swizzles.html#tymethod.yy)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#79)

#### fn [yz](../prelude/trait.Vec4Swizzles.html#tymethod.yz)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#87)

#### fn [with\_yz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yz)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#92)

#### fn [yw](../prelude/trait.Vec4Swizzles.html#tymethod.yw)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#100)

#### fn [with\_yw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yw)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#105)

#### fn [zx](../prelude/trait.Vec4Swizzles.html#tymethod.zx)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#113)

#### fn [with\_zx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zx)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#118)

#### fn [zy](../prelude/trait.Vec4Swizzles.html#tymethod.zy)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#126)

#### fn [with\_zy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zy)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#131)

#### fn [zz](../prelude/trait.Vec4Swizzles.html#tymethod.zz)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#139)

#### fn [zw](../prelude/trait.Vec4Swizzles.html#tymethod.zw)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#147)

#### fn [with\_zw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zw)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#152)

#### fn [wx](../prelude/trait.Vec4Swizzles.html#tymethod.wx)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#160)

#### fn [with\_wx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wx)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#165)

#### fn [wy](../prelude/trait.Vec4Swizzles.html#tymethod.wy)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#173)

#### fn [with\_wy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wy)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#178)

#### fn [wz](../prelude/trait.Vec4Swizzles.html#tymethod.wz)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#186)

#### fn [with\_wz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wz)(self, rhs: [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#191)

#### fn [ww](../prelude/trait.Vec4Swizzles.html#tymethod.ww)(self) -> [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#199)

#### fn [xxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#204)

#### fn [xxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#209)

#### fn [xxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#214)

#### fn [xxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#219)

#### fn [xyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#224)

#### fn [xyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#229)

#### fn [xyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#234)

#### fn [with\_xyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyz)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#239)

#### fn [xyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#244)

#### fn [with\_xyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyw)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#249)

#### fn [xzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#254)

#### fn [xzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#259)

#### fn [with\_xzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzy)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#264)

#### fn [xzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#269)

#### fn [xzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#274)

#### fn [with\_xzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzw)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#279)

#### fn [xwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#284)

#### fn [xwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#289)

#### fn [with\_xwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwy)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#294)

#### fn [xwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#299)

#### fn [with\_xwz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwz)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#304)

#### fn [xww](../prelude/trait.Vec4Swizzles.html#tymethod.xww)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#309)

#### fn [yxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#314)

#### fn [yxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#319)

#### fn [yxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#324)

#### fn [with\_yxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxz)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#329)

#### fn [yxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#334)

#### fn [with\_yxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxw)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#339)

#### fn [yyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#344)

#### fn [yyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#349)

#### fn [yyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#354)

#### fn [yyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#359)

#### fn [yzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#364)

#### fn [with\_yzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzx)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#369)

#### fn [yzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#374)

#### fn [yzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#379)

#### fn [yzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#384)

#### fn [with\_yzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzw)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#389)

#### fn [ywx](../prelude/trait.Vec4Swizzles.html#tymethod.ywx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#394)

#### fn [with\_ywx](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywx)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#399)

#### fn [ywy](../prelude/trait.Vec4Swizzles.html#tymethod.ywy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#404)

#### fn [ywz](../prelude/trait.Vec4Swizzles.html#tymethod.ywz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#409)

#### fn [with\_ywz](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywz)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#414)

#### fn [yww](../prelude/trait.Vec4Swizzles.html#tymethod.yww)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#419)

#### fn [zxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#424)

#### fn [zxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#429)

#### fn [with\_zxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxy)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#434)

#### fn [zxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#439)

#### fn [zxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#444)

#### fn [with\_zxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxw)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#449)

#### fn [zyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#454)

#### fn [with\_zyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyx)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#459)

#### fn [zyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#464)

#### fn [zyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#469)

#### fn [zyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#474)

#### fn [with\_zyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyw)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#479)

#### fn [zzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#484)

#### fn [zzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#489)

#### fn [zzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#494)

#### fn [zzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#499)

#### fn [zwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#504)

#### fn [with\_zwx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwx)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#509)

#### fn [zwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#514)

#### fn [with\_zwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwy)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#519)

#### fn [zwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#524)

#### fn [zww](../prelude/trait.Vec4Swizzles.html#tymethod.zww)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#529)

#### fn [wxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#534)

#### fn [wxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#539)

#### fn [with\_wxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxy)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#544)

#### fn [wxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#549)

#### fn [with\_wxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxz)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#554)

#### fn [wxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#559)

#### fn [wyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#564)

#### fn [with\_wyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyx)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#569)

#### fn [wyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#574)

#### fn [wyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#579)

#### fn [with\_wyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyz)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#584)

#### fn [wyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#589)

#### fn [wzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#594)

#### fn [with\_wzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzx)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#599)

#### fn [wzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#604)

#### fn [with\_wzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzy)(self, rhs: [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#609)

#### fn [wzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#614)

#### fn [wzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzw)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#619)

#### fn [wwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwx)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#624)

#### fn [wwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwy)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#629)

#### fn [wwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwz)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#634)

#### fn [www](../prelude/trait.Vec4Swizzles.html#tymethod.www)(self) -> [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#639)

#### fn [xxxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#644)

#### fn [xxxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#649)

#### fn [xxxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#654)

#### fn [xxxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#659)

#### fn [xxyx](../prelude/trait.Vec4Swizzles.html#tymethod.xxyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#664)

#### fn [xxyy](../prelude/trait.Vec4Swizzles.html#tymethod.xxyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#669)

#### fn [xxyz](../prelude/trait.Vec4Swizzles.html#tymethod.xxyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#674)

#### fn [xxyw](../prelude/trait.Vec4Swizzles.html#tymethod.xxyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#679)

#### fn [xxzx](../prelude/trait.Vec4Swizzles.html#tymethod.xxzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#684)

#### fn [xxzy](../prelude/trait.Vec4Swizzles.html#tymethod.xxzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#689)

#### fn [xxzz](../prelude/trait.Vec4Swizzles.html#tymethod.xxzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#694)

#### fn [xxzw](../prelude/trait.Vec4Swizzles.html#tymethod.xxzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#699)

#### fn [xxwx](../prelude/trait.Vec4Swizzles.html#tymethod.xxwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#704)

#### fn [xxwy](../prelude/trait.Vec4Swizzles.html#tymethod.xxwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#709)

#### fn [xxwz](../prelude/trait.Vec4Swizzles.html#tymethod.xxwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#714)

#### fn [xxww](../prelude/trait.Vec4Swizzles.html#tymethod.xxww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#719)

#### fn [xyxx](../prelude/trait.Vec4Swizzles.html#tymethod.xyxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#724)

#### fn [xyxy](../prelude/trait.Vec4Swizzles.html#tymethod.xyxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#729)

#### fn [xyxz](../prelude/trait.Vec4Swizzles.html#tymethod.xyxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#734)

#### fn [xyxw](../prelude/trait.Vec4Swizzles.html#tymethod.xyxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#739)

#### fn [xyyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#744)

#### fn [xyyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#749)

#### fn [xyyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#754)

#### fn [xyyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#759)

#### fn [xyzx](../prelude/trait.Vec4Swizzles.html#tymethod.xyzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#764)

#### fn [xyzy](../prelude/trait.Vec4Swizzles.html#tymethod.xyzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#769)

#### fn [xyzz](../prelude/trait.Vec4Swizzles.html#tymethod.xyzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#774)

#### fn [xywx](../prelude/trait.Vec4Swizzles.html#tymethod.xywx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#779)

#### fn [xywy](../prelude/trait.Vec4Swizzles.html#tymethod.xywy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#784)

#### fn [xywz](../prelude/trait.Vec4Swizzles.html#tymethod.xywz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#789)

#### fn [xyww](../prelude/trait.Vec4Swizzles.html#tymethod.xyww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#794)

#### fn [xzxx](../prelude/trait.Vec4Swizzles.html#tymethod.xzxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#799)

#### fn [xzxy](../prelude/trait.Vec4Swizzles.html#tymethod.xzxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#804)

#### fn [xzxz](../prelude/trait.Vec4Swizzles.html#tymethod.xzxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#809)

#### fn [xzxw](../prelude/trait.Vec4Swizzles.html#tymethod.xzxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#814)

#### fn [xzyx](../prelude/trait.Vec4Swizzles.html#tymethod.xzyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#819)

#### fn [xzyy](../prelude/trait.Vec4Swizzles.html#tymethod.xzyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#824)

#### fn [xzyz](../prelude/trait.Vec4Swizzles.html#tymethod.xzyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#829)

#### fn [xzyw](../prelude/trait.Vec4Swizzles.html#tymethod.xzyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#834)

#### fn [xzzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#839)

#### fn [xzzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#844)

#### fn [xzzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#849)

#### fn [xzzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#854)

#### fn [xzwx](../prelude/trait.Vec4Swizzles.html#tymethod.xzwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#859)

#### fn [xzwy](../prelude/trait.Vec4Swizzles.html#tymethod.xzwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#864)

#### fn [xzwz](../prelude/trait.Vec4Swizzles.html#tymethod.xzwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#869)

#### fn [xzww](../prelude/trait.Vec4Swizzles.html#tymethod.xzww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#874)

#### fn [xwxx](../prelude/trait.Vec4Swizzles.html#tymethod.xwxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#879)

#### fn [xwxy](../prelude/trait.Vec4Swizzles.html#tymethod.xwxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#884)

#### fn [xwxz](../prelude/trait.Vec4Swizzles.html#tymethod.xwxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#889)

#### fn [xwxw](../prelude/trait.Vec4Swizzles.html#tymethod.xwxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#894)

#### fn [xwyx](../prelude/trait.Vec4Swizzles.html#tymethod.xwyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#899)

#### fn [xwyy](../prelude/trait.Vec4Swizzles.html#tymethod.xwyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#904)

#### fn [xwyz](../prelude/trait.Vec4Swizzles.html#tymethod.xwyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#909)

#### fn [xwyw](../prelude/trait.Vec4Swizzles.html#tymethod.xwyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#914)

#### fn [xwzx](../prelude/trait.Vec4Swizzles.html#tymethod.xwzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#919)

#### fn [xwzy](../prelude/trait.Vec4Swizzles.html#tymethod.xwzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#924)

#### fn [xwzz](../prelude/trait.Vec4Swizzles.html#tymethod.xwzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#929)

#### fn [xwzw](../prelude/trait.Vec4Swizzles.html#tymethod.xwzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#934)

#### fn [xwwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#939)

#### fn [xwwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#944)

#### fn [xwwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#949)

#### fn [xwww](../prelude/trait.Vec4Swizzles.html#tymethod.xwww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#954)

#### fn [yxxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#959)

#### fn [yxxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#964)

#### fn [yxxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#969)

#### fn [yxxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#974)

#### fn [yxyx](../prelude/trait.Vec4Swizzles.html#tymethod.yxyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#979)

#### fn [yxyy](../prelude/trait.Vec4Swizzles.html#tymethod.yxyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#984)

#### fn [yxyz](../prelude/trait.Vec4Swizzles.html#tymethod.yxyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#989)

#### fn [yxyw](../prelude/trait.Vec4Swizzles.html#tymethod.yxyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#994)

#### fn [yxzx](../prelude/trait.Vec4Swizzles.html#tymethod.yxzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#999)

#### fn [yxzy](../prelude/trait.Vec4Swizzles.html#tymethod.yxzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1004)

#### fn [yxzz](../prelude/trait.Vec4Swizzles.html#tymethod.yxzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1009)

#### fn [yxzw](../prelude/trait.Vec4Swizzles.html#tymethod.yxzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1014)

#### fn [yxwx](../prelude/trait.Vec4Swizzles.html#tymethod.yxwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1019)

#### fn [yxwy](../prelude/trait.Vec4Swizzles.html#tymethod.yxwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1024)

#### fn [yxwz](../prelude/trait.Vec4Swizzles.html#tymethod.yxwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1029)

#### fn [yxww](../prelude/trait.Vec4Swizzles.html#tymethod.yxww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1034)

#### fn [yyxx](../prelude/trait.Vec4Swizzles.html#tymethod.yyxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1039)

#### fn [yyxy](../prelude/trait.Vec4Swizzles.html#tymethod.yyxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1044)

#### fn [yyxz](../prelude/trait.Vec4Swizzles.html#tymethod.yyxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1049)

#### fn [yyxw](../prelude/trait.Vec4Swizzles.html#tymethod.yyxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1054)

#### fn [yyyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1059)

#### fn [yyyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1064)

#### fn [yyyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1069)

#### fn [yyyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1074)

#### fn [yyzx](../prelude/trait.Vec4Swizzles.html#tymethod.yyzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1079)

#### fn [yyzy](../prelude/trait.Vec4Swizzles.html#tymethod.yyzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1084)

#### fn [yyzz](../prelude/trait.Vec4Swizzles.html#tymethod.yyzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1089)

#### fn [yyzw](../prelude/trait.Vec4Swizzles.html#tymethod.yyzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1094)

#### fn [yywx](../prelude/trait.Vec4Swizzles.html#tymethod.yywx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1099)

#### fn [yywy](../prelude/trait.Vec4Swizzles.html#tymethod.yywy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1104)

#### fn [yywz](../prelude/trait.Vec4Swizzles.html#tymethod.yywz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1109)

#### fn [yyww](../prelude/trait.Vec4Swizzles.html#tymethod.yyww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1114)

#### fn [yzxx](../prelude/trait.Vec4Swizzles.html#tymethod.yzxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1119)

#### fn [yzxy](../prelude/trait.Vec4Swizzles.html#tymethod.yzxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1124)

#### fn [yzxz](../prelude/trait.Vec4Swizzles.html#tymethod.yzxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1129)

#### fn [yzxw](../prelude/trait.Vec4Swizzles.html#tymethod.yzxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1134)

#### fn [yzyx](../prelude/trait.Vec4Swizzles.html#tymethod.yzyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1139)

#### fn [yzyy](../prelude/trait.Vec4Swizzles.html#tymethod.yzyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1144)

#### fn [yzyz](../prelude/trait.Vec4Swizzles.html#tymethod.yzyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1149)

#### fn [yzyw](../prelude/trait.Vec4Swizzles.html#tymethod.yzyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1154)

#### fn [yzzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1159)

#### fn [yzzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1164)

#### fn [yzzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1169)

#### fn [yzzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1174)

#### fn [yzwx](../prelude/trait.Vec4Swizzles.html#tymethod.yzwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1179)

#### fn [yzwy](../prelude/trait.Vec4Swizzles.html#tymethod.yzwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1184)

#### fn [yzwz](../prelude/trait.Vec4Swizzles.html#tymethod.yzwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1189)

#### fn [yzww](../prelude/trait.Vec4Swizzles.html#tymethod.yzww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1194)

#### fn [ywxx](../prelude/trait.Vec4Swizzles.html#tymethod.ywxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1199)

#### fn [ywxy](../prelude/trait.Vec4Swizzles.html#tymethod.ywxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1204)

#### fn [ywxz](../prelude/trait.Vec4Swizzles.html#tymethod.ywxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1209)

#### fn [ywxw](../prelude/trait.Vec4Swizzles.html#tymethod.ywxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1214)

#### fn [ywyx](../prelude/trait.Vec4Swizzles.html#tymethod.ywyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1219)

#### fn [ywyy](../prelude/trait.Vec4Swizzles.html#tymethod.ywyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1224)

#### fn [ywyz](../prelude/trait.Vec4Swizzles.html#tymethod.ywyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1229)

#### fn [ywyw](../prelude/trait.Vec4Swizzles.html#tymethod.ywyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1234)

#### fn [ywzx](../prelude/trait.Vec4Swizzles.html#tymethod.ywzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1239)

#### fn [ywzy](../prelude/trait.Vec4Swizzles.html#tymethod.ywzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1244)

#### fn [ywzz](../prelude/trait.Vec4Swizzles.html#tymethod.ywzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1249)

#### fn [ywzw](../prelude/trait.Vec4Swizzles.html#tymethod.ywzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1254)

#### fn [ywwx](../prelude/trait.Vec4Swizzles.html#tymethod.ywwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1259)

#### fn [ywwy](../prelude/trait.Vec4Swizzles.html#tymethod.ywwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1264)

#### fn [ywwz](../prelude/trait.Vec4Swizzles.html#tymethod.ywwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1269)

#### fn [ywww](../prelude/trait.Vec4Swizzles.html#tymethod.ywww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1274)

#### fn [zxxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1279)

#### fn [zxxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1284)

#### fn [zxxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1289)

#### fn [zxxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1294)

#### fn [zxyx](../prelude/trait.Vec4Swizzles.html#tymethod.zxyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1299)

#### fn [zxyy](../prelude/trait.Vec4Swizzles.html#tymethod.zxyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1304)

#### fn [zxyz](../prelude/trait.Vec4Swizzles.html#tymethod.zxyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1309)

#### fn [zxyw](../prelude/trait.Vec4Swizzles.html#tymethod.zxyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1314)

#### fn [zxzx](../prelude/trait.Vec4Swizzles.html#tymethod.zxzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1319)

#### fn [zxzy](../prelude/trait.Vec4Swizzles.html#tymethod.zxzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1324)

#### fn [zxzz](../prelude/trait.Vec4Swizzles.html#tymethod.zxzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1329)

#### fn [zxzw](../prelude/trait.Vec4Swizzles.html#tymethod.zxzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1334)

#### fn [zxwx](../prelude/trait.Vec4Swizzles.html#tymethod.zxwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1339)

#### fn [zxwy](../prelude/trait.Vec4Swizzles.html#tymethod.zxwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1344)

#### fn [zxwz](../prelude/trait.Vec4Swizzles.html#tymethod.zxwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1349)

#### fn [zxww](../prelude/trait.Vec4Swizzles.html#tymethod.zxww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1354)

#### fn [zyxx](../prelude/trait.Vec4Swizzles.html#tymethod.zyxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1359)

#### fn [zyxy](../prelude/trait.Vec4Swizzles.html#tymethod.zyxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1364)

#### fn [zyxz](../prelude/trait.Vec4Swizzles.html#tymethod.zyxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1369)

#### fn [zyxw](../prelude/trait.Vec4Swizzles.html#tymethod.zyxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1374)

#### fn [zyyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1379)

#### fn [zyyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1384)

#### fn [zyyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1389)

#### fn [zyyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1394)

#### fn [zyzx](../prelude/trait.Vec4Swizzles.html#tymethod.zyzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1399)

#### fn [zyzy](../prelude/trait.Vec4Swizzles.html#tymethod.zyzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1404)

#### fn [zyzz](../prelude/trait.Vec4Swizzles.html#tymethod.zyzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1409)

#### fn [zyzw](../prelude/trait.Vec4Swizzles.html#tymethod.zyzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1414)

#### fn [zywx](../prelude/trait.Vec4Swizzles.html#tymethod.zywx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1419)

#### fn [zywy](../prelude/trait.Vec4Swizzles.html#tymethod.zywy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1424)

#### fn [zywz](../prelude/trait.Vec4Swizzles.html#tymethod.zywz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1429)

#### fn [zyww](../prelude/trait.Vec4Swizzles.html#tymethod.zyww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1434)

#### fn [zzxx](../prelude/trait.Vec4Swizzles.html#tymethod.zzxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1439)

#### fn [zzxy](../prelude/trait.Vec4Swizzles.html#tymethod.zzxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1444)

#### fn [zzxz](../prelude/trait.Vec4Swizzles.html#tymethod.zzxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1449)

#### fn [zzxw](../prelude/trait.Vec4Swizzles.html#tymethod.zzxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1454)

#### fn [zzyx](../prelude/trait.Vec4Swizzles.html#tymethod.zzyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1459)

#### fn [zzyy](../prelude/trait.Vec4Swizzles.html#tymethod.zzyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1464)

#### fn [zzyz](../prelude/trait.Vec4Swizzles.html#tymethod.zzyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1469)

#### fn [zzyw](../prelude/trait.Vec4Swizzles.html#tymethod.zzyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1474)

#### fn [zzzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1479)

#### fn [zzzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1484)

#### fn [zzzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1489)

#### fn [zzzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1494)

#### fn [zzwx](../prelude/trait.Vec4Swizzles.html#tymethod.zzwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1499)

#### fn [zzwy](../prelude/trait.Vec4Swizzles.html#tymethod.zzwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1504)

#### fn [zzwz](../prelude/trait.Vec4Swizzles.html#tymethod.zzwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1509)

#### fn [zzww](../prelude/trait.Vec4Swizzles.html#tymethod.zzww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1514)

#### fn [zwxx](../prelude/trait.Vec4Swizzles.html#tymethod.zwxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1519)

#### fn [zwxy](../prelude/trait.Vec4Swizzles.html#tymethod.zwxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1524)

#### fn [zwxz](../prelude/trait.Vec4Swizzles.html#tymethod.zwxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1529)

#### fn [zwxw](../prelude/trait.Vec4Swizzles.html#tymethod.zwxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1534)

#### fn [zwyx](../prelude/trait.Vec4Swizzles.html#tymethod.zwyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1539)

#### fn [zwyy](../prelude/trait.Vec4Swizzles.html#tymethod.zwyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1544)

#### fn [zwyz](../prelude/trait.Vec4Swizzles.html#tymethod.zwyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1549)

#### fn [zwyw](../prelude/trait.Vec4Swizzles.html#tymethod.zwyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1554)

#### fn [zwzx](../prelude/trait.Vec4Swizzles.html#tymethod.zwzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1559)

#### fn [zwzy](../prelude/trait.Vec4Swizzles.html#tymethod.zwzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1564)

#### fn [zwzz](../prelude/trait.Vec4Swizzles.html#tymethod.zwzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1569)

#### fn [zwzw](../prelude/trait.Vec4Swizzles.html#tymethod.zwzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1574)

#### fn [zwwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1579)

#### fn [zwwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1584)

#### fn [zwwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1589)

#### fn [zwww](../prelude/trait.Vec4Swizzles.html#tymethod.zwww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1594)

#### fn [wxxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1599)

#### fn [wxxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1604)

#### fn [wxxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1609)

#### fn [wxxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1614)

#### fn [wxyx](../prelude/trait.Vec4Swizzles.html#tymethod.wxyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1619)

#### fn [wxyy](../prelude/trait.Vec4Swizzles.html#tymethod.wxyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1624)

#### fn [wxyz](../prelude/trait.Vec4Swizzles.html#tymethod.wxyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1629)

#### fn [wxyw](../prelude/trait.Vec4Swizzles.html#tymethod.wxyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1634)

#### fn [wxzx](../prelude/trait.Vec4Swizzles.html#tymethod.wxzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1639)

#### fn [wxzy](../prelude/trait.Vec4Swizzles.html#tymethod.wxzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1644)

#### fn [wxzz](../prelude/trait.Vec4Swizzles.html#tymethod.wxzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1649)

#### fn [wxzw](../prelude/trait.Vec4Swizzles.html#tymethod.wxzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1654)

#### fn [wxwx](../prelude/trait.Vec4Swizzles.html#tymethod.wxwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1659)

#### fn [wxwy](../prelude/trait.Vec4Swizzles.html#tymethod.wxwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1664)

#### fn [wxwz](../prelude/trait.Vec4Swizzles.html#tymethod.wxwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1669)

#### fn [wxww](../prelude/trait.Vec4Swizzles.html#tymethod.wxww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1674)

#### fn [wyxx](../prelude/trait.Vec4Swizzles.html#tymethod.wyxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1679)

#### fn [wyxy](../prelude/trait.Vec4Swizzles.html#tymethod.wyxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1684)

#### fn [wyxz](../prelude/trait.Vec4Swizzles.html#tymethod.wyxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1689)

#### fn [wyxw](../prelude/trait.Vec4Swizzles.html#tymethod.wyxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1694)

#### fn [wyyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1699)

#### fn [wyyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1704)

#### fn [wyyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1709)

#### fn [wyyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1714)

#### fn [wyzx](../prelude/trait.Vec4Swizzles.html#tymethod.wyzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1719)

#### fn [wyzy](../prelude/trait.Vec4Swizzles.html#tymethod.wyzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1724)

#### fn [wyzz](../prelude/trait.Vec4Swizzles.html#tymethod.wyzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1729)

#### fn [wyzw](../prelude/trait.Vec4Swizzles.html#tymethod.wyzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1734)

#### fn [wywx](../prelude/trait.Vec4Swizzles.html#tymethod.wywx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1739)

#### fn [wywy](../prelude/trait.Vec4Swizzles.html#tymethod.wywy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1744)

#### fn [wywz](../prelude/trait.Vec4Swizzles.html#tymethod.wywz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1749)

#### fn [wyww](../prelude/trait.Vec4Swizzles.html#tymethod.wyww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1754)

#### fn [wzxx](../prelude/trait.Vec4Swizzles.html#tymethod.wzxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1759)

#### fn [wzxy](../prelude/trait.Vec4Swizzles.html#tymethod.wzxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1764)

#### fn [wzxz](../prelude/trait.Vec4Swizzles.html#tymethod.wzxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1769)

#### fn [wzxw](../prelude/trait.Vec4Swizzles.html#tymethod.wzxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1774)

#### fn [wzyx](../prelude/trait.Vec4Swizzles.html#tymethod.wzyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1779)

#### fn [wzyy](../prelude/trait.Vec4Swizzles.html#tymethod.wzyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1784)

#### fn [wzyz](../prelude/trait.Vec4Swizzles.html#tymethod.wzyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1789)

#### fn [wzyw](../prelude/trait.Vec4Swizzles.html#tymethod.wzyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1794)

#### fn [wzzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1799)

#### fn [wzzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1804)

#### fn [wzzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1809)

#### fn [wzzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1814)

#### fn [wzwx](../prelude/trait.Vec4Swizzles.html#tymethod.wzwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1819)

#### fn [wzwy](../prelude/trait.Vec4Swizzles.html#tymethod.wzwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1824)

#### fn [wzwz](../prelude/trait.Vec4Swizzles.html#tymethod.wzwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1829)

#### fn [wzww](../prelude/trait.Vec4Swizzles.html#tymethod.wzww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1834)

#### fn [wwxx](../prelude/trait.Vec4Swizzles.html#tymethod.wwxx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1839)

#### fn [wwxy](../prelude/trait.Vec4Swizzles.html#tymethod.wwxy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1844)

#### fn [wwxz](../prelude/trait.Vec4Swizzles.html#tymethod.wwxz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1849)

#### fn [wwxw](../prelude/trait.Vec4Swizzles.html#tymethod.wwxw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1854)

#### fn [wwyx](../prelude/trait.Vec4Swizzles.html#tymethod.wwyx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1859)

#### fn [wwyy](../prelude/trait.Vec4Swizzles.html#tymethod.wwyy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1864)

#### fn [wwyz](../prelude/trait.Vec4Swizzles.html#tymethod.wwyz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1869)

#### fn [wwyw](../prelude/trait.Vec4Swizzles.html#tymethod.wwyw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1874)

#### fn [wwzx](../prelude/trait.Vec4Swizzles.html#tymethod.wwzx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1879)

#### fn [wwzy](../prelude/trait.Vec4Swizzles.html#tymethod.wwzy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1884)

#### fn [wwzz](../prelude/trait.Vec4Swizzles.html#tymethod.wwzz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1889)

#### fn [wwzw](../prelude/trait.Vec4Swizzles.html#tymethod.wwzw)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1894)

#### fn [wwwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwwx)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1899)

#### fn [wwwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwwy)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1904)

#### fn [wwwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwwz)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#1909)

#### fn [wwww](../prelude/trait.Vec4Swizzles.html#tymethod.wwww)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#481)

#### fn [xyzw](../prelude/trait.Vec4Swizzles.html#method.xyzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

### impl [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

where [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"): [AsRefVectorParts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html), 4>, [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#17)

#### fn [write\_into](../render/render_resource/encase/internal/trait.WriteInto.html#tymethod.write_into)<B>(&self, writer: &mut [Writer](../render/render_resource/encase/internal/struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](../render/render_resource/encase/internal/trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#26)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../ecs/label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../ecs/label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

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

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

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

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/gpu_array_buffer.rs.html#20)

### impl<T> [GpuArrayBufferable](../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") for T

where T: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") + [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

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