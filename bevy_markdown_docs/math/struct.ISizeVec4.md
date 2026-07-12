[bevy](../index.html)::[math](index.html)

# Struct ISizeVec4 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#34)

```rust
#[repr(C)]pub struct ISizeVec4 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub w: isize,
}
```

A 4-dimensional vector.

## Fields

`x: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)``y: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)``z: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)``w: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#41)

### impl [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#43)

#### pub const [ZERO](#associatedconstant.ZERO): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#46)

#### pub const [ONE](#associatedconstant.ONE): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#49)

#### pub const [NEG\_ONE](#associatedconstant.NEG_ONE): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

All negative ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#52)

#### pub const [MIN](#associatedconstant.MIN): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

All `isize::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#55)

#### pub const [MAX](#associatedconstant.MAX): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

All `isize::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#58)

#### pub const [X](#associatedconstant.X): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#61)

#### pub const [Y](#associatedconstant.Y): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#64)

#### pub const [Z](#associatedconstant.Z): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#67)

#### pub const [W](#associatedconstant.W): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the positive W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#70)

#### pub const [NEG\_X](#associatedconstant.NEG_X): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the negative X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#73)

#### pub const [NEG\_Y](#associatedconstant.NEG_Y): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the negative Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#76)

#### pub const [NEG\_Z](#associatedconstant.NEG_Z): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the negative Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#79)

#### pub const [NEG\_W](#associatedconstant.NEG_W): [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

A unit vector pointing along the negative W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#82)

#### pub const [AXES](#associatedconstant.AXES): \[[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#87)

#### pub const fn [new](#method.new)(x: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), y: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), z: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), w: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a new vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#94)

#### pub const fn [splat](#method.splat)(v: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a vector with all elements set to `v`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#109-111)

#### pub fn [map](#method.map)<F>(self, f: F) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#123)

#### pub fn [select](#method.select)(mask: [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4"), if\_true: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), if\_false: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#135)

#### pub const fn [from\_array](#method.from_array)(a: \[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#142)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z, w]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#153)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\]) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a vector from the first 4 values in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#164)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\])

Writes the elements of `self` to the first 4 elements in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#173)

#### pub fn [truncate](#method.truncate)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.

Truncation to [`ISizeVec3`](struct.ISizeVec3.html "struct bevy::math::ISizeVec3") may also be performed by using [`self.xyz()`](../prelude/trait.Vec4Swizzles.html#tymethod.xyz "method bevy::prelude::Vec4Swizzles::xyz").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#181)

#### pub fn [with\_x](#method.with_x)(self, x: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a 4D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#189)

#### pub fn [with\_y](#method.with_y)(self, y: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a 4D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#197)

#### pub fn [with\_z](#method.with_z)(self, z: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a 4D vector from `self` with the given value of `z`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#205)

#### pub fn [with\_w](#method.with_w)(self, w: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Creates a 4D vector from `self` with the given value of `w`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#213)

#### pub fn [dot](#method.dot)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#220)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#229)

#### pub fn [min](#method.min)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#243)

#### pub fn [max](#method.max)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#261)

#### pub fn [clamp](#method.clamp)(self, min: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), max: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Component-wise clamping of values, similar to [`isize::clamp`](https://doc.rust-lang.org/nightly/std/primitive.isize.html#method.clamp "method isize::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#271)

#### pub fn [min\_element](#method.min_element)(self) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#281)

#### pub fn [max\_element](#method.max_element)(self) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#290)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#311)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#333)

#### pub fn [element\_sum](#method.element_sum)(self) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#342)

#### pub fn [element\_product](#method.element_product)(self) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#353)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#369)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#385)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#401)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#417)

#### pub fn [cmple](#method.cmple)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#433)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#445)

#### pub fn [abs](#method.abs)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the absolute value of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#461)

#### pub fn [signum](#method.signum)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector with elements representing the sign of `self`.

*   `0` if the number is zero
*   `1` if the number is positive
*   `-1` if the number is negative

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#479)

#### pub fn [is\_negative\_bitmask](#method.is_negative_bitmask)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.

A negative element results in a `1` bit and a positive element in a `0` bit. Element `x` goes into the first lowest bit, element `y` into the second, etc.

An element is negative if it has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#490)

#### pub fn [length\_squared](#method.length_squared)(self) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

Computes the squared length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#497)

#### pub fn [distance\_squared](#method.distance_squared)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

Compute the squared euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#507)

#### pub fn [div\_euclid](#method.div_euclid)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns the element-wise quotient of \[Euclidean division\] of `self` by `rhs`.

##### Panics

This function will panic if any `rhs` element is 0 or the division results in overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#524)

#### pub fn [rem\_euclid](#method.rem_euclid)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns the element-wise remainder of [Euclidean division](https://doc.rust-lang.org/nightly/std/primitive.isize.html#method.rem_euclid "method isize::rem_euclid") of `self` by `rhs`.

##### Panics

This function will panic if any `rhs` element is 0 or the division results in overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#543)

#### pub fn [manhattan\_distance](#method.manhattan_distance)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

##### Overflow

This method may overflow if the result is greater than [`usize::MAX`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#associatedconstant.MAX "associated constant usize::MAX").

See also [`checked_manhattan_distance`](struct.ISizeVec4.html#method.checked_manhattan_distance "method bevy::math::ISizeVec4::checked_manhattan_distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#557)

#### pub fn [checked\_manhattan\_distance](#method.checked_manhattan_distance)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

This will returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the result is greater than [`usize::MAX`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#associatedconstant.MAX "associated constant usize::MAX").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#569)

#### pub fn [chebyshev\_distance](#method.chebyshev_distance)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Computes the [chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance) between two points.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#585)

#### pub fn [as\_vec4](#method.as_vec4)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Casts all elements of `self` to `f32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#592)

#### pub fn [as\_dvec4](#method.as_dvec4)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#599)

#### pub fn [as\_i8vec4](#method.as_i8vec4)(self) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#606)

#### pub fn [as\_u8vec4](#method.as_u8vec4)(self) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#613)

#### pub fn [as\_i16vec4](#method.as_i16vec4)(self) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#620)

#### pub fn [as\_u16vec4](#method.as_u16vec4)(self) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#627)

#### pub fn [as\_ivec4](#method.as_ivec4)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#634)

#### pub fn [as\_uvec4](#method.as_uvec4)(self) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Casts all elements of `self` to `u32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#641)

#### pub fn [as\_i64vec4](#method.as_i64vec4)(self) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#648)

#### pub fn [as\_u64vec4](#method.as_u64vec4)(self) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#655)

#### pub fn [as\_usizevec4](#method.as_usizevec4)(self) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Casts all elements of `self` to `usize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#669)

#### pub const fn [checked\_add](#method.checked_add)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#695)

#### pub const fn [checked\_sub](#method.checked_sub)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#721)

#### pub const fn [checked\_mul](#method.checked_mul)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `Some([self.x * rhs.x, self.y * rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#747)

#### pub const fn [checked\_div](#method.checked_div)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `Some([self.x / rhs.x, self.y / rhs.y, ..])` but returns `None` on any division by zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#773)

#### pub const fn [wrapping\_add](#method.wrapping_add)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#787)

#### pub const fn [wrapping\_sub](#method.wrapping_sub)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#801)

#### pub const fn [wrapping\_mul](#method.wrapping_mul)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#815)

#### pub const fn [wrapping\_div](#method.wrapping_div)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#829)

#### pub const fn [saturating\_add](#method.saturating_add)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the saturating addition of `self` and `rhs`.

In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#843)

#### pub const fn [saturating\_sub](#method.saturating_sub)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the saturating subtraction of `self` and `rhs`.

In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#857)

#### pub const fn [saturating\_mul](#method.saturating_mul)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the saturating multiplication of `self` and `rhs`.

In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#871)

#### pub const fn [saturating\_div](#method.saturating_div)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the saturating division of `self` and `rhs`.

In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#885)

#### pub const fn [checked\_add\_unsigned](#method.checked_add_unsigned)(self, rhs: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>

Returns a vector containing the wrapping addition of `self` and unsigned vector `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#911)

#### pub const fn [checked\_sub\_unsigned](#method.checked_sub_unsigned)(self, rhs: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>

Returns a vector containing the wrapping subtraction of `self` and unsigned vector `rhs`.

In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#937)

#### pub const fn [wrapping\_add\_unsigned](#method.wrapping_add_unsigned)(self, rhs: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the wrapping addition of `self` and unsigned vector `rhs`.

In other words this computes `[self.x.wrapping_add_unsigned(rhs.x), self.y.wrapping_add_unsigned(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#951)

#### pub const fn [wrapping\_sub\_unsigned](#method.wrapping_sub_unsigned)(self, rhs: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the wrapping subtraction of `self` and unsigned vector `rhs`.

In other words this computes `[self.x.wrapping_sub_unsigned(rhs.x), self.y.wrapping_sub_unsigned(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#965)

#### pub const fn [saturating\_add\_unsigned](#method.saturating_add_unsigned)(self, rhs: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

In other words this computes `[self.x.saturating_add_unsigned(rhs.x), self.y.saturating_add_unsigned(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#979)

#### pub const fn [saturating\_sub\_unsigned](#method.saturating_sub_unsigned)(self, rhs: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a vector containing the saturating subtraction of `self` and unsigned vector `rhs`.

In other words this computes `[self.x.saturating_sub_unsigned(rhs.x), self.y.saturating_sub_unsigned(rhs.y), ..]`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1286)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1287)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1289)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1299)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1300)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1302)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1307)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1308)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1310)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1353)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1354)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1356)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1361)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1362)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1364)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1315)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1316)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1318)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1340)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1341)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1343)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1369)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1370)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1372)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1323)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1325)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1333)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1335)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1387)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1389)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1377)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1379)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1728)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1730)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1721)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1723)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1817)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1818)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1820)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1830)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1831)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1833)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1838)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1839)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1841)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1983)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1984)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1986)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1991)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1992)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1994)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1846)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1847)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1849)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1970)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1971)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1973)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1999)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2000)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2002)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1854)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1856)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1861)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1863)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2014)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2016)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2007)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2009)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1868)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1869)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1871)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1881)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1882)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1884)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1889)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1890)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1892)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2034)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2035)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2037)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2042)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2043)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2045)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1897)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1898)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1900)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2021)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2022)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2024)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2050)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2051)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2053)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1905)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1907)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1912)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1914)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2065)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2067)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2058)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2060)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1919)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1920)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1922)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1932)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1933)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1935)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1940)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1941)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1943)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2085)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2086)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2088)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2093)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2094)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2096)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1948)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1949)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1951)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2072)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2073)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2075)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2101)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2102)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2104)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1956)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1958)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1963)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1965)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2116)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2118)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2109)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2111)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3120)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3121)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#989)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#991)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1153)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Deserialize expects a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1153)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3114)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3115)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#996)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#997)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#999)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1009)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1010)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1012)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1017)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1018)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1020)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1063)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1064)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1066)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1071)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1072)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1074)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1025)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1026)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1028)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1050)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1051)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1053)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1079)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1080)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1082)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1033)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1035)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1043)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1045)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1097)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1099)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1087)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1089)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3180)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2"), [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2"))> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3182)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2"), [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2"))) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3173)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2"), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3175)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2"), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3159)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3"), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3161)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3"), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3166)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3"))> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3168)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3"))) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3145)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3147)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html), [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3307)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3309)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3320)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Available on **non-crate feature `scalar-math`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3322)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3187)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3189)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3199)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3201)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3138)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for \[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3140)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> \[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3211)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3213)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3131)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3133)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3087)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3088)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3090)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3101)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3103)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> &mut <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1141)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1142)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1144)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1154)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1155)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1157)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1162)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1163)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1165)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1208)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1209)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1211)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1216)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1217)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1219)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1170)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1171)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1173)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1195)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1196)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1198)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1224)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1225)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1227)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1178)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1180)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1188)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1190)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1242)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1244)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1232)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1234)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1775)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1776)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1778)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1788)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1791)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1796)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1797)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1799)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1809)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1810)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1812)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#26)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1755)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1757-1759)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1765)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1767-1769)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1576)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1577)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1579)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1589)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1590)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1592)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1597)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1598)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1600)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1643)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1644)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1646)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1651)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1652)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1654)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1605)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1606)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1608)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1630)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1631)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1633)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1659)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1660)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1662)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1613)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1615)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1623)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1625)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1677)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1679)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1667)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1669)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1153)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Serialize as a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1153)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3026)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3027)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3029)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3034)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3035)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3037)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2136)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2137)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2139)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2144)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2145)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2147)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2238)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2239)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2241)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2246)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2247)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2249)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2340)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2341)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2343)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2348)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2349)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2351)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2442)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2443)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2445)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2450)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2451)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2453)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2544)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2545)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2547)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2552)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2553)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2555)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2646)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2647)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2649)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2654)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2655)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2657)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2748)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2749)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2751)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2756)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2757)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2759)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2850)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2851)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2853)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2858)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2859)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2861)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3013)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3014)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3016)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3042)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3043)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3045)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2123)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2124)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2126)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2152)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2153)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2155)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2225)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2226)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2228)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2254)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2255)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2257)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2327)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2328)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2330)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2356)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2357)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2359)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2429)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2430)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2432)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2458)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2459)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2461)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2531)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2532)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2534)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2560)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2561)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2563)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2633)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2634)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2636)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2662)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2663)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2665)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2735)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2736)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2738)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2764)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2765)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2767)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2837)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2838)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2840)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2866)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2867)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2869)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2167)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2169)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2269)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2271)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2371)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2373)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2473)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2475)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2575)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2577)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2677)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2679)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2779)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2781)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2881)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2883)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2160)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2162)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2262)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2264)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2364)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2366)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2466)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2468)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2568)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2570)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2670)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2672)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2772)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2774)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2874)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2876)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3063)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3064)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3066)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3071)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3072)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3074)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2187)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2188)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2190)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2195)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2196)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2198)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2289)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2290)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2292)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2297)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2298)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2300)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2391)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2392)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2394)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2399)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2400)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2402)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2493)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2494)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2496)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2501)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2502)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2504)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2595)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2596)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2598)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2603)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2604)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2606)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2697)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2698)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2700)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2705)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2706)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2708)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2799)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2800)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2802)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2807)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2808)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2810)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2901)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2902)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2904)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2909)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2910)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2912)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3050)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3051)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3053)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3079)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3080)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3082)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2174)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2175)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2177)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2203)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2204)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2206)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2276)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2277)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2279)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2305)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2306)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2308)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2378)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2379)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2381)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2407)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2408)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2410)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2480)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2481)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2483)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2509)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2510)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2512)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2582)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2583)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2585)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2611)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2612)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2614)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2684)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2685)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2687)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2713)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2714)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2716)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2786)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2787)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2789)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2815)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2816)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2818)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2888)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2889)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2891)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2917)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2918)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2920)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2218)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2220)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2320)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2322)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2422)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2424)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2524)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2526)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2626)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2628)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2728)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2730)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2830)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2832)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2932)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2934)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2211)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2213)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2313)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2315)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2415)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2417)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2517)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2519)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2619)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2621)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2721)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2723)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2823)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2825)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2925)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#2927)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#25)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1431)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1432)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1434)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1444)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1445)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1447)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1452)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1453)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1455)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1498)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1499)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1501)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1506)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1507)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1509)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1460)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1461)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1463)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1485)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1486)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1488)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1514)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1515)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1517)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1468)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1470)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1478)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1480)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1532)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1534)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1522)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1524)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1735)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1737-1739)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1745)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#1747-1749)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3293)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3294)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3297)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3290)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3291)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3294)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4"), <[I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3117)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3118)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3121)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4"), <[U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3286)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3287)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3290)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4"), <[I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3115)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3116)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3119)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4"), <[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3282)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3283)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3286)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4"), <[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3113)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3114)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3117)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3278)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3279)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3282)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4"), <[I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3111)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3112)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3115)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4"), <[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3122)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3123)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3126)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4"), <[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3279)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3280)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3283)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3223)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3224)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3227)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3251)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3252)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3255)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3265)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3266)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3269)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3237)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3238)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3241)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#5)

### impl [Vec4Swizzles](../prelude/trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#6)

#### type [Vec2](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec2) = [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#8)

#### type [Vec3](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec3) = [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#11)

#### fn [xx](../prelude/trait.Vec4Swizzles.html#tymethod.xx)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#19)

#### fn [xy](../prelude/trait.Vec4Swizzles.html#tymethod.xy)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#27)

#### fn [with\_xy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xy)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#32)

#### fn [xz](../prelude/trait.Vec4Swizzles.html#tymethod.xz)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#40)

#### fn [with\_xz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xz)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#45)

#### fn [xw](../prelude/trait.Vec4Swizzles.html#tymethod.xw)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#53)

#### fn [with\_xw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xw)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#58)

#### fn [yx](../prelude/trait.Vec4Swizzles.html#tymethod.yx)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#66)

#### fn [with\_yx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yx)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#71)

#### fn [yy](../prelude/trait.Vec4Swizzles.html#tymethod.yy)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#79)

#### fn [yz](../prelude/trait.Vec4Swizzles.html#tymethod.yz)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#87)

#### fn [with\_yz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yz)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#92)

#### fn [yw](../prelude/trait.Vec4Swizzles.html#tymethod.yw)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#100)

#### fn [with\_yw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yw)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#105)

#### fn [zx](../prelude/trait.Vec4Swizzles.html#tymethod.zx)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#113)

#### fn [with\_zx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zx)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#118)

#### fn [zy](../prelude/trait.Vec4Swizzles.html#tymethod.zy)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#126)

#### fn [with\_zy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zy)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#131)

#### fn [zz](../prelude/trait.Vec4Swizzles.html#tymethod.zz)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#139)

#### fn [zw](../prelude/trait.Vec4Swizzles.html#tymethod.zw)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#147)

#### fn [with\_zw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zw)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#152)

#### fn [wx](../prelude/trait.Vec4Swizzles.html#tymethod.wx)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#160)

#### fn [with\_wx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wx)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#165)

#### fn [wy](../prelude/trait.Vec4Swizzles.html#tymethod.wy)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#173)

#### fn [with\_wy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wy)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#178)

#### fn [wz](../prelude/trait.Vec4Swizzles.html#tymethod.wz)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#186)

#### fn [with\_wz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wz)(self, rhs: [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#191)

#### fn [ww](../prelude/trait.Vec4Swizzles.html#tymethod.ww)(self) -> [ISizeVec2](struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#199)

#### fn [xxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#204)

#### fn [xxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#209)

#### fn [xxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#214)

#### fn [xxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#219)

#### fn [xyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#224)

#### fn [xyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#229)

#### fn [xyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#234)

#### fn [with\_xyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyz)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#239)

#### fn [xyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#244)

#### fn [with\_xyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyw)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#249)

#### fn [xzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#254)

#### fn [xzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#259)

#### fn [with\_xzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzy)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#264)

#### fn [xzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#269)

#### fn [xzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#274)

#### fn [with\_xzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzw)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#279)

#### fn [xwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#284)

#### fn [xwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#289)

#### fn [with\_xwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwy)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#294)

#### fn [xwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#299)

#### fn [with\_xwz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwz)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#304)

#### fn [xww](../prelude/trait.Vec4Swizzles.html#tymethod.xww)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#309)

#### fn [yxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#314)

#### fn [yxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#319)

#### fn [yxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#324)

#### fn [with\_yxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxz)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#329)

#### fn [yxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#334)

#### fn [with\_yxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxw)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#339)

#### fn [yyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#344)

#### fn [yyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#349)

#### fn [yyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#354)

#### fn [yyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#359)

#### fn [yzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#364)

#### fn [with\_yzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzx)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#369)

#### fn [yzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#374)

#### fn [yzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#379)

#### fn [yzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#384)

#### fn [with\_yzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzw)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#389)

#### fn [ywx](../prelude/trait.Vec4Swizzles.html#tymethod.ywx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#394)

#### fn [with\_ywx](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywx)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#399)

#### fn [ywy](../prelude/trait.Vec4Swizzles.html#tymethod.ywy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#404)

#### fn [ywz](../prelude/trait.Vec4Swizzles.html#tymethod.ywz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#409)

#### fn [with\_ywz](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywz)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#414)

#### fn [yww](../prelude/trait.Vec4Swizzles.html#tymethod.yww)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#419)

#### fn [zxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#424)

#### fn [zxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#429)

#### fn [with\_zxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxy)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#434)

#### fn [zxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#439)

#### fn [zxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#444)

#### fn [with\_zxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxw)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#449)

#### fn [zyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#454)

#### fn [with\_zyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyx)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#459)

#### fn [zyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#464)

#### fn [zyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#469)

#### fn [zyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#474)

#### fn [with\_zyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyw)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#479)

#### fn [zzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#484)

#### fn [zzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#489)

#### fn [zzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#494)

#### fn [zzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#499)

#### fn [zwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#504)

#### fn [with\_zwx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwx)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#509)

#### fn [zwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#514)

#### fn [with\_zwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwy)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#519)

#### fn [zwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#524)

#### fn [zww](../prelude/trait.Vec4Swizzles.html#tymethod.zww)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#529)

#### fn [wxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#534)

#### fn [wxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#539)

#### fn [with\_wxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxy)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#544)

#### fn [wxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#549)

#### fn [with\_wxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxz)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#554)

#### fn [wxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#559)

#### fn [wyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#564)

#### fn [with\_wyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyx)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#569)

#### fn [wyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#574)

#### fn [wyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#579)

#### fn [with\_wyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyz)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#584)

#### fn [wyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#589)

#### fn [wzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#594)

#### fn [with\_wzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzx)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#599)

#### fn [wzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#604)

#### fn [with\_wzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzy)(self, rhs: [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#609)

#### fn [wzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#614)

#### fn [wzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzw)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#619)

#### fn [wwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwx)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#624)

#### fn [wwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwy)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#629)

#### fn [wwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwz)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#634)

#### fn [www](../prelude/trait.Vec4Swizzles.html#tymethod.www)(self) -> [ISizeVec3](struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#639)

#### fn [xxxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#644)

#### fn [xxxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#649)

#### fn [xxxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#654)

#### fn [xxxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#659)

#### fn [xxyx](../prelude/trait.Vec4Swizzles.html#tymethod.xxyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#664)

#### fn [xxyy](../prelude/trait.Vec4Swizzles.html#tymethod.xxyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#669)

#### fn [xxyz](../prelude/trait.Vec4Swizzles.html#tymethod.xxyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#674)

#### fn [xxyw](../prelude/trait.Vec4Swizzles.html#tymethod.xxyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#679)

#### fn [xxzx](../prelude/trait.Vec4Swizzles.html#tymethod.xxzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#684)

#### fn [xxzy](../prelude/trait.Vec4Swizzles.html#tymethod.xxzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#689)

#### fn [xxzz](../prelude/trait.Vec4Swizzles.html#tymethod.xxzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#694)

#### fn [xxzw](../prelude/trait.Vec4Swizzles.html#tymethod.xxzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#699)

#### fn [xxwx](../prelude/trait.Vec4Swizzles.html#tymethod.xxwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#704)

#### fn [xxwy](../prelude/trait.Vec4Swizzles.html#tymethod.xxwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#709)

#### fn [xxwz](../prelude/trait.Vec4Swizzles.html#tymethod.xxwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#714)

#### fn [xxww](../prelude/trait.Vec4Swizzles.html#tymethod.xxww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#719)

#### fn [xyxx](../prelude/trait.Vec4Swizzles.html#tymethod.xyxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#724)

#### fn [xyxy](../prelude/trait.Vec4Swizzles.html#tymethod.xyxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#729)

#### fn [xyxz](../prelude/trait.Vec4Swizzles.html#tymethod.xyxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#734)

#### fn [xyxw](../prelude/trait.Vec4Swizzles.html#tymethod.xyxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#739)

#### fn [xyyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#744)

#### fn [xyyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#749)

#### fn [xyyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#754)

#### fn [xyyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#759)

#### fn [xyzx](../prelude/trait.Vec4Swizzles.html#tymethod.xyzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#764)

#### fn [xyzy](../prelude/trait.Vec4Swizzles.html#tymethod.xyzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#769)

#### fn [xyzz](../prelude/trait.Vec4Swizzles.html#tymethod.xyzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#774)

#### fn [xywx](../prelude/trait.Vec4Swizzles.html#tymethod.xywx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#779)

#### fn [xywy](../prelude/trait.Vec4Swizzles.html#tymethod.xywy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#784)

#### fn [xywz](../prelude/trait.Vec4Swizzles.html#tymethod.xywz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#789)

#### fn [xyww](../prelude/trait.Vec4Swizzles.html#tymethod.xyww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#794)

#### fn [xzxx](../prelude/trait.Vec4Swizzles.html#tymethod.xzxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#799)

#### fn [xzxy](../prelude/trait.Vec4Swizzles.html#tymethod.xzxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#804)

#### fn [xzxz](../prelude/trait.Vec4Swizzles.html#tymethod.xzxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#809)

#### fn [xzxw](../prelude/trait.Vec4Swizzles.html#tymethod.xzxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#814)

#### fn [xzyx](../prelude/trait.Vec4Swizzles.html#tymethod.xzyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#819)

#### fn [xzyy](../prelude/trait.Vec4Swizzles.html#tymethod.xzyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#824)

#### fn [xzyz](../prelude/trait.Vec4Swizzles.html#tymethod.xzyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#829)

#### fn [xzyw](../prelude/trait.Vec4Swizzles.html#tymethod.xzyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#834)

#### fn [xzzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#839)

#### fn [xzzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#844)

#### fn [xzzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#849)

#### fn [xzzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#854)

#### fn [xzwx](../prelude/trait.Vec4Swizzles.html#tymethod.xzwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#859)

#### fn [xzwy](../prelude/trait.Vec4Swizzles.html#tymethod.xzwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#864)

#### fn [xzwz](../prelude/trait.Vec4Swizzles.html#tymethod.xzwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#869)

#### fn [xzww](../prelude/trait.Vec4Swizzles.html#tymethod.xzww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#874)

#### fn [xwxx](../prelude/trait.Vec4Swizzles.html#tymethod.xwxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#879)

#### fn [xwxy](../prelude/trait.Vec4Swizzles.html#tymethod.xwxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#884)

#### fn [xwxz](../prelude/trait.Vec4Swizzles.html#tymethod.xwxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#889)

#### fn [xwxw](../prelude/trait.Vec4Swizzles.html#tymethod.xwxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#894)

#### fn [xwyx](../prelude/trait.Vec4Swizzles.html#tymethod.xwyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#899)

#### fn [xwyy](../prelude/trait.Vec4Swizzles.html#tymethod.xwyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#904)

#### fn [xwyz](../prelude/trait.Vec4Swizzles.html#tymethod.xwyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#909)

#### fn [xwyw](../prelude/trait.Vec4Swizzles.html#tymethod.xwyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#914)

#### fn [xwzx](../prelude/trait.Vec4Swizzles.html#tymethod.xwzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#919)

#### fn [xwzy](../prelude/trait.Vec4Swizzles.html#tymethod.xwzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#924)

#### fn [xwzz](../prelude/trait.Vec4Swizzles.html#tymethod.xwzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#929)

#### fn [xwzw](../prelude/trait.Vec4Swizzles.html#tymethod.xwzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#934)

#### fn [xwwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#939)

#### fn [xwwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#944)

#### fn [xwwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#949)

#### fn [xwww](../prelude/trait.Vec4Swizzles.html#tymethod.xwww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#954)

#### fn [yxxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#959)

#### fn [yxxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#964)

#### fn [yxxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#969)

#### fn [yxxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#974)

#### fn [yxyx](../prelude/trait.Vec4Swizzles.html#tymethod.yxyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#979)

#### fn [yxyy](../prelude/trait.Vec4Swizzles.html#tymethod.yxyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#984)

#### fn [yxyz](../prelude/trait.Vec4Swizzles.html#tymethod.yxyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#989)

#### fn [yxyw](../prelude/trait.Vec4Swizzles.html#tymethod.yxyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#994)

#### fn [yxzx](../prelude/trait.Vec4Swizzles.html#tymethod.yxzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#999)

#### fn [yxzy](../prelude/trait.Vec4Swizzles.html#tymethod.yxzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1004)

#### fn [yxzz](../prelude/trait.Vec4Swizzles.html#tymethod.yxzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1009)

#### fn [yxzw](../prelude/trait.Vec4Swizzles.html#tymethod.yxzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1014)

#### fn [yxwx](../prelude/trait.Vec4Swizzles.html#tymethod.yxwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1019)

#### fn [yxwy](../prelude/trait.Vec4Swizzles.html#tymethod.yxwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1024)

#### fn [yxwz](../prelude/trait.Vec4Swizzles.html#tymethod.yxwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1029)

#### fn [yxww](../prelude/trait.Vec4Swizzles.html#tymethod.yxww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1034)

#### fn [yyxx](../prelude/trait.Vec4Swizzles.html#tymethod.yyxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1039)

#### fn [yyxy](../prelude/trait.Vec4Swizzles.html#tymethod.yyxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1044)

#### fn [yyxz](../prelude/trait.Vec4Swizzles.html#tymethod.yyxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1049)

#### fn [yyxw](../prelude/trait.Vec4Swizzles.html#tymethod.yyxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1054)

#### fn [yyyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1059)

#### fn [yyyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1064)

#### fn [yyyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1069)

#### fn [yyyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1074)

#### fn [yyzx](../prelude/trait.Vec4Swizzles.html#tymethod.yyzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1079)

#### fn [yyzy](../prelude/trait.Vec4Swizzles.html#tymethod.yyzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1084)

#### fn [yyzz](../prelude/trait.Vec4Swizzles.html#tymethod.yyzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1089)

#### fn [yyzw](../prelude/trait.Vec4Swizzles.html#tymethod.yyzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1094)

#### fn [yywx](../prelude/trait.Vec4Swizzles.html#tymethod.yywx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1099)

#### fn [yywy](../prelude/trait.Vec4Swizzles.html#tymethod.yywy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1104)

#### fn [yywz](../prelude/trait.Vec4Swizzles.html#tymethod.yywz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1109)

#### fn [yyww](../prelude/trait.Vec4Swizzles.html#tymethod.yyww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1114)

#### fn [yzxx](../prelude/trait.Vec4Swizzles.html#tymethod.yzxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1119)

#### fn [yzxy](../prelude/trait.Vec4Swizzles.html#tymethod.yzxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1124)

#### fn [yzxz](../prelude/trait.Vec4Swizzles.html#tymethod.yzxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1129)

#### fn [yzxw](../prelude/trait.Vec4Swizzles.html#tymethod.yzxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1134)

#### fn [yzyx](../prelude/trait.Vec4Swizzles.html#tymethod.yzyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1139)

#### fn [yzyy](../prelude/trait.Vec4Swizzles.html#tymethod.yzyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1144)

#### fn [yzyz](../prelude/trait.Vec4Swizzles.html#tymethod.yzyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1149)

#### fn [yzyw](../prelude/trait.Vec4Swizzles.html#tymethod.yzyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1154)

#### fn [yzzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1159)

#### fn [yzzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1164)

#### fn [yzzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1169)

#### fn [yzzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1174)

#### fn [yzwx](../prelude/trait.Vec4Swizzles.html#tymethod.yzwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1179)

#### fn [yzwy](../prelude/trait.Vec4Swizzles.html#tymethod.yzwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1184)

#### fn [yzwz](../prelude/trait.Vec4Swizzles.html#tymethod.yzwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1189)

#### fn [yzww](../prelude/trait.Vec4Swizzles.html#tymethod.yzww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1194)

#### fn [ywxx](../prelude/trait.Vec4Swizzles.html#tymethod.ywxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1199)

#### fn [ywxy](../prelude/trait.Vec4Swizzles.html#tymethod.ywxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1204)

#### fn [ywxz](../prelude/trait.Vec4Swizzles.html#tymethod.ywxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1209)

#### fn [ywxw](../prelude/trait.Vec4Swizzles.html#tymethod.ywxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1214)

#### fn [ywyx](../prelude/trait.Vec4Swizzles.html#tymethod.ywyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1219)

#### fn [ywyy](../prelude/trait.Vec4Swizzles.html#tymethod.ywyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1224)

#### fn [ywyz](../prelude/trait.Vec4Swizzles.html#tymethod.ywyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1229)

#### fn [ywyw](../prelude/trait.Vec4Swizzles.html#tymethod.ywyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1234)

#### fn [ywzx](../prelude/trait.Vec4Swizzles.html#tymethod.ywzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1239)

#### fn [ywzy](../prelude/trait.Vec4Swizzles.html#tymethod.ywzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1244)

#### fn [ywzz](../prelude/trait.Vec4Swizzles.html#tymethod.ywzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1249)

#### fn [ywzw](../prelude/trait.Vec4Swizzles.html#tymethod.ywzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1254)

#### fn [ywwx](../prelude/trait.Vec4Swizzles.html#tymethod.ywwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1259)

#### fn [ywwy](../prelude/trait.Vec4Swizzles.html#tymethod.ywwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1264)

#### fn [ywwz](../prelude/trait.Vec4Swizzles.html#tymethod.ywwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1269)

#### fn [ywww](../prelude/trait.Vec4Swizzles.html#tymethod.ywww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1274)

#### fn [zxxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1279)

#### fn [zxxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1284)

#### fn [zxxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1289)

#### fn [zxxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1294)

#### fn [zxyx](../prelude/trait.Vec4Swizzles.html#tymethod.zxyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1299)

#### fn [zxyy](../prelude/trait.Vec4Swizzles.html#tymethod.zxyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1304)

#### fn [zxyz](../prelude/trait.Vec4Swizzles.html#tymethod.zxyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1309)

#### fn [zxyw](../prelude/trait.Vec4Swizzles.html#tymethod.zxyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1314)

#### fn [zxzx](../prelude/trait.Vec4Swizzles.html#tymethod.zxzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1319)

#### fn [zxzy](../prelude/trait.Vec4Swizzles.html#tymethod.zxzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1324)

#### fn [zxzz](../prelude/trait.Vec4Swizzles.html#tymethod.zxzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1329)

#### fn [zxzw](../prelude/trait.Vec4Swizzles.html#tymethod.zxzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1334)

#### fn [zxwx](../prelude/trait.Vec4Swizzles.html#tymethod.zxwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1339)

#### fn [zxwy](../prelude/trait.Vec4Swizzles.html#tymethod.zxwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1344)

#### fn [zxwz](../prelude/trait.Vec4Swizzles.html#tymethod.zxwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1349)

#### fn [zxww](../prelude/trait.Vec4Swizzles.html#tymethod.zxww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1354)

#### fn [zyxx](../prelude/trait.Vec4Swizzles.html#tymethod.zyxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1359)

#### fn [zyxy](../prelude/trait.Vec4Swizzles.html#tymethod.zyxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1364)

#### fn [zyxz](../prelude/trait.Vec4Swizzles.html#tymethod.zyxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1369)

#### fn [zyxw](../prelude/trait.Vec4Swizzles.html#tymethod.zyxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1374)

#### fn [zyyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1379)

#### fn [zyyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1384)

#### fn [zyyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1389)

#### fn [zyyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1394)

#### fn [zyzx](../prelude/trait.Vec4Swizzles.html#tymethod.zyzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1399)

#### fn [zyzy](../prelude/trait.Vec4Swizzles.html#tymethod.zyzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1404)

#### fn [zyzz](../prelude/trait.Vec4Swizzles.html#tymethod.zyzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1409)

#### fn [zyzw](../prelude/trait.Vec4Swizzles.html#tymethod.zyzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1414)

#### fn [zywx](../prelude/trait.Vec4Swizzles.html#tymethod.zywx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1419)

#### fn [zywy](../prelude/trait.Vec4Swizzles.html#tymethod.zywy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1424)

#### fn [zywz](../prelude/trait.Vec4Swizzles.html#tymethod.zywz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1429)

#### fn [zyww](../prelude/trait.Vec4Swizzles.html#tymethod.zyww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1434)

#### fn [zzxx](../prelude/trait.Vec4Swizzles.html#tymethod.zzxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1439)

#### fn [zzxy](../prelude/trait.Vec4Swizzles.html#tymethod.zzxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1444)

#### fn [zzxz](../prelude/trait.Vec4Swizzles.html#tymethod.zzxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1449)

#### fn [zzxw](../prelude/trait.Vec4Swizzles.html#tymethod.zzxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1454)

#### fn [zzyx](../prelude/trait.Vec4Swizzles.html#tymethod.zzyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1459)

#### fn [zzyy](../prelude/trait.Vec4Swizzles.html#tymethod.zzyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1464)

#### fn [zzyz](../prelude/trait.Vec4Swizzles.html#tymethod.zzyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1469)

#### fn [zzyw](../prelude/trait.Vec4Swizzles.html#tymethod.zzyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1474)

#### fn [zzzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1479)

#### fn [zzzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1484)

#### fn [zzzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1489)

#### fn [zzzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1494)

#### fn [zzwx](../prelude/trait.Vec4Swizzles.html#tymethod.zzwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1499)

#### fn [zzwy](../prelude/trait.Vec4Swizzles.html#tymethod.zzwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1504)

#### fn [zzwz](../prelude/trait.Vec4Swizzles.html#tymethod.zzwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1509)

#### fn [zzww](../prelude/trait.Vec4Swizzles.html#tymethod.zzww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1514)

#### fn [zwxx](../prelude/trait.Vec4Swizzles.html#tymethod.zwxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1519)

#### fn [zwxy](../prelude/trait.Vec4Swizzles.html#tymethod.zwxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1524)

#### fn [zwxz](../prelude/trait.Vec4Swizzles.html#tymethod.zwxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1529)

#### fn [zwxw](../prelude/trait.Vec4Swizzles.html#tymethod.zwxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1534)

#### fn [zwyx](../prelude/trait.Vec4Swizzles.html#tymethod.zwyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1539)

#### fn [zwyy](../prelude/trait.Vec4Swizzles.html#tymethod.zwyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1544)

#### fn [zwyz](../prelude/trait.Vec4Swizzles.html#tymethod.zwyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1549)

#### fn [zwyw](../prelude/trait.Vec4Swizzles.html#tymethod.zwyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1554)

#### fn [zwzx](../prelude/trait.Vec4Swizzles.html#tymethod.zwzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1559)

#### fn [zwzy](../prelude/trait.Vec4Swizzles.html#tymethod.zwzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1564)

#### fn [zwzz](../prelude/trait.Vec4Swizzles.html#tymethod.zwzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1569)

#### fn [zwzw](../prelude/trait.Vec4Swizzles.html#tymethod.zwzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1574)

#### fn [zwwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1579)

#### fn [zwwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1584)

#### fn [zwwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1589)

#### fn [zwww](../prelude/trait.Vec4Swizzles.html#tymethod.zwww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1594)

#### fn [wxxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1599)

#### fn [wxxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1604)

#### fn [wxxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1609)

#### fn [wxxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1614)

#### fn [wxyx](../prelude/trait.Vec4Swizzles.html#tymethod.wxyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1619)

#### fn [wxyy](../prelude/trait.Vec4Swizzles.html#tymethod.wxyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1624)

#### fn [wxyz](../prelude/trait.Vec4Swizzles.html#tymethod.wxyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1629)

#### fn [wxyw](../prelude/trait.Vec4Swizzles.html#tymethod.wxyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1634)

#### fn [wxzx](../prelude/trait.Vec4Swizzles.html#tymethod.wxzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1639)

#### fn [wxzy](../prelude/trait.Vec4Swizzles.html#tymethod.wxzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1644)

#### fn [wxzz](../prelude/trait.Vec4Swizzles.html#tymethod.wxzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1649)

#### fn [wxzw](../prelude/trait.Vec4Swizzles.html#tymethod.wxzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1654)

#### fn [wxwx](../prelude/trait.Vec4Swizzles.html#tymethod.wxwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1659)

#### fn [wxwy](../prelude/trait.Vec4Swizzles.html#tymethod.wxwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1664)

#### fn [wxwz](../prelude/trait.Vec4Swizzles.html#tymethod.wxwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1669)

#### fn [wxww](../prelude/trait.Vec4Swizzles.html#tymethod.wxww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1674)

#### fn [wyxx](../prelude/trait.Vec4Swizzles.html#tymethod.wyxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1679)

#### fn [wyxy](../prelude/trait.Vec4Swizzles.html#tymethod.wyxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1684)

#### fn [wyxz](../prelude/trait.Vec4Swizzles.html#tymethod.wyxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1689)

#### fn [wyxw](../prelude/trait.Vec4Swizzles.html#tymethod.wyxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1694)

#### fn [wyyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1699)

#### fn [wyyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1704)

#### fn [wyyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1709)

#### fn [wyyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1714)

#### fn [wyzx](../prelude/trait.Vec4Swizzles.html#tymethod.wyzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1719)

#### fn [wyzy](../prelude/trait.Vec4Swizzles.html#tymethod.wyzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1724)

#### fn [wyzz](../prelude/trait.Vec4Swizzles.html#tymethod.wyzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1729)

#### fn [wyzw](../prelude/trait.Vec4Swizzles.html#tymethod.wyzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1734)

#### fn [wywx](../prelude/trait.Vec4Swizzles.html#tymethod.wywx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1739)

#### fn [wywy](../prelude/trait.Vec4Swizzles.html#tymethod.wywy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1744)

#### fn [wywz](../prelude/trait.Vec4Swizzles.html#tymethod.wywz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1749)

#### fn [wyww](../prelude/trait.Vec4Swizzles.html#tymethod.wyww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1754)

#### fn [wzxx](../prelude/trait.Vec4Swizzles.html#tymethod.wzxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1759)

#### fn [wzxy](../prelude/trait.Vec4Swizzles.html#tymethod.wzxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1764)

#### fn [wzxz](../prelude/trait.Vec4Swizzles.html#tymethod.wzxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1769)

#### fn [wzxw](../prelude/trait.Vec4Swizzles.html#tymethod.wzxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1774)

#### fn [wzyx](../prelude/trait.Vec4Swizzles.html#tymethod.wzyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1779)

#### fn [wzyy](../prelude/trait.Vec4Swizzles.html#tymethod.wzyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1784)

#### fn [wzyz](../prelude/trait.Vec4Swizzles.html#tymethod.wzyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1789)

#### fn [wzyw](../prelude/trait.Vec4Swizzles.html#tymethod.wzyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1794)

#### fn [wzzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1799)

#### fn [wzzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1804)

#### fn [wzzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1809)

#### fn [wzzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1814)

#### fn [wzwx](../prelude/trait.Vec4Swizzles.html#tymethod.wzwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1819)

#### fn [wzwy](../prelude/trait.Vec4Swizzles.html#tymethod.wzwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1824)

#### fn [wzwz](../prelude/trait.Vec4Swizzles.html#tymethod.wzwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1829)

#### fn [wzww](../prelude/trait.Vec4Swizzles.html#tymethod.wzww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1834)

#### fn [wwxx](../prelude/trait.Vec4Swizzles.html#tymethod.wwxx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1839)

#### fn [wwxy](../prelude/trait.Vec4Swizzles.html#tymethod.wwxy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1844)

#### fn [wwxz](../prelude/trait.Vec4Swizzles.html#tymethod.wwxz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1849)

#### fn [wwxw](../prelude/trait.Vec4Swizzles.html#tymethod.wwxw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1854)

#### fn [wwyx](../prelude/trait.Vec4Swizzles.html#tymethod.wwyx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1859)

#### fn [wwyy](../prelude/trait.Vec4Swizzles.html#tymethod.wwyy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1864)

#### fn [wwyz](../prelude/trait.Vec4Swizzles.html#tymethod.wwyz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1869)

#### fn [wwyw](../prelude/trait.Vec4Swizzles.html#tymethod.wwyw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1874)

#### fn [wwzx](../prelude/trait.Vec4Swizzles.html#tymethod.wwzx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1879)

#### fn [wwzy](../prelude/trait.Vec4Swizzles.html#tymethod.wwzy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1884)

#### fn [wwzz](../prelude/trait.Vec4Swizzles.html#tymethod.wwzz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1889)

#### fn [wwzw](../prelude/trait.Vec4Swizzles.html#tymethod.wwzw)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1894)

#### fn [wwwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwwx)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1899)

#### fn [wwwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwwy)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1904)

#### fn [wwwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwwz)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#1909)

#### fn [wwww](../prelude/trait.Vec4Swizzles.html#tymethod.wwww)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#481)

#### fn [xyzw](../prelude/trait.Vec4Swizzles.html#method.xyzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#26)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}