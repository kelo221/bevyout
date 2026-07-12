[bevy](../index.html)::[prelude](index.html)

# Struct UVec4 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#34)

```rust
#[repr(C)]pub struct UVec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}
```

A 4-dimensional vector.

## Fields

`x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)``y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)``z: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)``w: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#41)

### impl [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#43)

#### pub const [ZERO](#associatedconstant.ZERO): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#46)

#### pub const [ONE](#associatedconstant.ONE): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#49)

#### pub const [MIN](#associatedconstant.MIN): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

All `u32::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#52)

#### pub const [MAX](#associatedconstant.MAX): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

All `u32::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#55)

#### pub const [X](#associatedconstant.X): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#58)

#### pub const [Y](#associatedconstant.Y): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#61)

#### pub const [Z](#associatedconstant.Z): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#64)

#### pub const [W](#associatedconstant.W): [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

A unit vector pointing along the positive W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#67)

#### pub const [AXES](#associatedconstant.AXES): \[[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#72)

#### pub const fn [new](#method.new)(x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), z: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), w: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a new vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#79)

#### pub const fn [splat](#method.splat)(v: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a vector with all elements set to `v`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#94-96)

#### pub fn [map](#method.map)<F>(self, f: F) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#108)

#### pub fn [select](#method.select)(mask: [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4"), if\_true: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), if\_false: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#120)

#### pub const fn [from\_array](#method.from_array)(a: \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#127)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z, w]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#138)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\]) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a vector from the first 4 values in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#149)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\])

Writes the elements of `self` to the first 4 elements in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#158)

#### pub fn [truncate](#method.truncate)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.

Truncation to [`UVec3`](struct.UVec3.html "struct bevy::prelude::UVec3") may also be performed by using [`self.xyz()`](trait.Vec4Swizzles.html#tymethod.xyz "method bevy::prelude::Vec4Swizzles::xyz").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#166)

#### pub fn [with\_x](#method.with_x)(self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a 4D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#174)

#### pub fn [with\_y](#method.with_y)(self, y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a 4D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#182)

#### pub fn [with\_z](#method.with_z)(self, z: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a 4D vector from `self` with the given value of `z`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#190)

#### pub fn [with\_w](#method.with_w)(self, w: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Creates a 4D vector from `self` with the given value of `w`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#198)

#### pub fn [dot](#method.dot)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#205)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#214)

#### pub fn [min](#method.min)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#228)

#### pub fn [max](#method.max)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#246)

#### pub fn [clamp](#method.clamp)(self, min: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), max: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Component-wise clamping of values, similar to [`u32::clamp`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#method.clamp "method u32::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#256)

#### pub fn [min\_element](#method.min_element)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#266)

#### pub fn [max\_element](#method.max_element)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#275)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#296)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#318)

#### pub fn [element\_sum](#method.element_sum)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#327)

#### pub fn [element\_product](#method.element_product)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#338)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#354)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#370)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#386)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#402)

#### pub fn [cmple](#method.cmple)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#418)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#431)

#### pub fn [length\_squared](#method.length_squared)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the squared length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#445)

#### pub fn [manhattan\_distance](#method.manhattan_distance)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

##### Overflow

This method may overflow if the result is greater than [`u32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#associatedconstant.MAX "associated constant u32::MAX").

See also [`checked_manhattan_distance`](struct.UVec4.html#method.checked_manhattan_distance "method bevy::prelude::UVec4::checked_manhattan_distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#459)

#### pub fn [checked\_manhattan\_distance](#method.checked_manhattan_distance)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

This will returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the result is greater than [`u32::MAX`](https://doc.rust-lang.org/nightly/std/primitive.u32.html#associatedconstant.MAX "associated constant u32::MAX").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#471)

#### pub fn [chebyshev\_distance](#method.chebyshev_distance)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Computes the [chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance) between two points.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#487)

#### pub fn [as\_vec4](#method.as_vec4)(self) -> [Vec4](struct.Vec4.html "struct bevy::prelude::Vec4")

Casts all elements of `self` to `f32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#494)

#### pub fn [as\_dvec4](#method.as_dvec4)(self) -> [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#501)

#### pub fn [as\_i8vec4](#method.as_i8vec4)(self) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#508)

#### pub fn [as\_u8vec4](#method.as_u8vec4)(self) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#515)

#### pub fn [as\_i16vec4](#method.as_i16vec4)(self) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#522)

#### pub fn [as\_u16vec4](#method.as_u16vec4)(self) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#529)

#### pub fn [as\_ivec4](#method.as_ivec4)(self) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#536)

#### pub fn [as\_i64vec4](#method.as_i64vec4)(self) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#543)

#### pub fn [as\_u64vec4](#method.as_u64vec4)(self) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#550)

#### pub fn [as\_isizevec4](#method.as_isizevec4)(self) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#562)

#### pub fn [as\_usizevec4](#method.as_usizevec4)(self) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Casts all elements of `self` to `usize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#576)

#### pub const fn [checked\_add](#method.checked_add)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#602)

#### pub const fn [checked\_sub](#method.checked_sub)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#628)

#### pub const fn [checked\_mul](#method.checked_mul)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `Some([self.x * rhs.x, self.y * rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#654)

#### pub const fn [checked\_div](#method.checked_div)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `Some([self.x / rhs.x, self.y / rhs.y, ..])` but returns `None` on any division by zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#680)

#### pub const fn [wrapping\_add](#method.wrapping_add)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#694)

#### pub const fn [wrapping\_sub](#method.wrapping_sub)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#708)

#### pub const fn [wrapping\_mul](#method.wrapping_mul)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#722)

#### pub const fn [wrapping\_div](#method.wrapping_div)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#736)

#### pub const fn [saturating\_add](#method.saturating_add)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the saturating addition of `self` and `rhs`.

In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#750)

#### pub const fn [saturating\_sub](#method.saturating_sub)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the saturating subtraction of `self` and `rhs`.

In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#764)

#### pub const fn [saturating\_mul](#method.saturating_mul)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the saturating multiplication of `self` and `rhs`.

In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#778)

#### pub const fn [saturating\_div](#method.saturating_div)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the saturating division of `self` and `rhs`.

In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#792)

#### pub const fn [checked\_add\_signed](#method.checked_add_signed)(self, rhs: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>

Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#818)

#### pub const fn [wrapping\_add\_signed](#method.wrapping_add_signed)(self, rhs: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.

In other words this computes `[self.x.wrapping_add_signed(rhs.x), self.y.wrapping_add_signed(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#832)

#### pub const fn [saturating\_add\_signed](#method.saturating_add_signed)(self, rhs: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a vector containing the saturating addition of `self` and signed vector `rhs`.

In other words this computes `[self.x.saturating_add_signed(rhs.x), self.y.saturating_add_signed(rhs.y), ..]`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1139)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1140)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1142)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1152)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1153)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1155)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1160)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1161)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1163)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1260)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1261)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1263)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1268)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1269)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1271)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1206)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1207)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1209)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1214)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1215)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1217)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1168)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1169)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1171)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1247)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1248)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1250)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1276)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1277)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1279)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1193)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1194)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1196)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1222)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1223)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1225)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1176)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1178)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1186)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1188)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1240)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1242)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1230)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1232)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1581)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1583)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [AsMutVectorParts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

#### fn [as\_mut\_parts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html#tymethod.as_mut_parts)(&mut self) -> &mut \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1574)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1576)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [AsRefVectorParts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

#### fn [as\_ref\_parts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html#tymethod.as_ref_parts)(&self) -> &\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1649)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1650)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1652)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1662)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1663)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1665)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1670)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1671)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1673)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1815)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1816)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1818)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1823)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1824)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1826)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1678)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1679)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1681)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1802)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1803)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1805)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1831)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1832)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1834)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1686)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1688)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1693)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1695)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1846)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1848)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1839)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1841)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1700)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1701)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1703)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1713)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1714)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1716)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1721)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1722)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1724)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1866)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1867)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1869)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1874)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1875)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1877)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1729)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1730)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1732)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1853)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1854)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1856)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1882)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1883)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1885)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1737)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1739)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1744)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1746)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1897)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1899)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1890)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1892)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1751)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1752)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1754)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1764)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1765)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1767)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1772)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1773)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1775)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1917)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1918)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1920)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1925)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1926)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1928)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1780)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1783)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1904)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1905)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1907)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1933)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1934)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1936)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1788)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1790)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1795)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1797)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1948)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1950)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1941)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1943)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"): [FromVectorParts](../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

#### fn [create\_from](../render/render_resource/encase/internal/trait.CreateFrom.html#tymethod.create_from)<B>(reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2952)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2953)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#842)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#844)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Deserialize expects a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2946)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2947)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#849)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#850)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#852)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#862)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#863)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#865)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#870)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#871)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#873)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#970)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#971)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#973)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#978)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#979)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#981)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#916)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#917)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#919)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#924)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#925)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#927)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#878)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#879)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#881)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#957)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#958)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#960)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#986)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#987)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#989)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#903)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#904)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#906)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#932)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#933)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#935)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#886)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#888)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#896)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#898)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#950)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#952)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#940)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#942)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3012)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([UVec2](struct.UVec2.html "struct bevy::prelude::UVec2"), [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2"))> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3014)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([UVec2](struct.UVec2.html "struct bevy::prelude::UVec2"), [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2"))) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3005)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([UVec2](struct.UVec2.html "struct bevy::prelude::UVec2"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3007)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([UVec2](struct.UVec2.html "struct bevy::prelude::UVec2"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2991)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([UVec3](struct.UVec3.html "struct bevy::prelude::UVec3"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2993)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([UVec3](struct.UVec3.html "struct bevy::prelude::UVec3"), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2998)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3"))> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3000)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3"))) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2977)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2979)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3141)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3143)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3154)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4A](struct.BVec4A.html "struct bevy::prelude::BVec4A")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Available on **non-crate feature `scalar-math`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3156)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4A](struct.BVec4A.html "struct bevy::prelude::BVec4A")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3019)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3021)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3031)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3033)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2213)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2215)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2970)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2972)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3252)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3254)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3043)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#3045)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2963)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2965)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [from\_reflect](trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [FromVectorParts](../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

#### fn [from\_parts](../render/render_resource/encase/vector/trait.FromVectorParts.html#tymethod.from_parts)(parts: \[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2919)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2920)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2922)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2933)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2935)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#994)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#995)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#997)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1007)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1008)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1010)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1015)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1016)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1018)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1115)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1116)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1118)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1123)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1124)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1126)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1061)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1062)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1064)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1069)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1070)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1072)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1023)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1024)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1026)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1102)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1103)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1105)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1131)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1132)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1134)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1048)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1049)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1051)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1077)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1078)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1080)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1031)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1033)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1041)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1043)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1095)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1097)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1085)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1087)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1628)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1629)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1631)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1641)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1642)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1644)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [get\_represented\_type\_info](trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [try\_apply](trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [reflect\_kind](trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [reflect\_ref](trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [reflect\_mut](trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [reflect\_owned](trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [try\_into\_reflect](trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](struct.Box.html "struct bevy::prelude::Box")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [try\_as\_reflect](trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [try\_as\_reflect\_mut](trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [into\_partial\_reflect](trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [as\_partial\_reflect](trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [as\_partial\_reflect\_mut](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#156)

#### fn [reflect\_hash](trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#156)

#### fn [reflect\_partial\_eq](trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [reflect\_partial\_cmp](trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#156)

#### fn [debug](trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#156)

#### fn [reflect\_clone](trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#26)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1608)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1610-1612)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1618)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1620-1622)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"): [AsMutVectorParts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

#### fn [read\_from](../render/render_resource/encase/internal/trait.ReadFrom.html#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [into\_any](trait.Reflect.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [as\_any](trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [as\_any\_mut](trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [into\_reflect](trait.Reflect.html#tymethod.into_reflect)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [as\_reflect](trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [as\_reflect\_mut](trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [set](trait.Reflect.html#tymethod.set)(&mut self, value: [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1429)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1430)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1432)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1442)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1443)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1445)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1450)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1451)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1453)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1550)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1551)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1553)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1558)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1559)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1561)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1496)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1497)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1499)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1504)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1505)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1507)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1458)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1459)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1461)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1537)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1538)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1540)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1566)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1567)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1569)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1483)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1484)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1486)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1512)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1513)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1515)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1466)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1468)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1476)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1478)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1530)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1532)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1520)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1522)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#707)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#707)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec4<[UniformInt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/int/struct.UniformInt.html "struct rand::distr::uniform::int::UniformInt")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Serialize as a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1198)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#232)

#### const [SHADER\_SIZE](../render/render_resource/trait.ShaderSize.html#associatedconstant.SHADER_SIZE): [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> = \_

Represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderType::min_size`](../render/render_resource/trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size"))

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#94)

#### fn [min\_size](../render/render_resource/trait.ShaderType.html#method.min_size)() -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Represents the minimum size of `Self` (equivalent to [GPUBufferBindingLayout.minBindingSize](https://gpuweb.github.io/gpuweb/#dom-gpubufferbindinglayout-minbindingsize)) [Read more](../render/render_resource/trait.ShaderType.html#method.min_size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#103)

#### fn [size](../render/render_resource/trait.ShaderType.html#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns the size of `Self` at runtime [Read more](../render/render_resource/trait.ShaderType.html#method.size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#206)

#### fn [assert\_uniform\_compat](../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)()

Asserts that `Self` meets the requirements of the [uniform address space restrictions on stored values](https://gpuweb.github.io/gpuweb/wgsl/#address-spaces-uniform) and the [uniform address space layout constraints](https://gpuweb.github.io/gpuweb/wgsl/#address-space-layout-constraints) [Read more](../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2845)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2846)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2848)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2784)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2785)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2787)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2792)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2793)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2795)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3031)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3032)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3034)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3039)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3040)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3042)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2858)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2859)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2861)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2866)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2867)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2869)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3031)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3032)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3034)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3039)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3040)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3042)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2858)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2859)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2861)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2866)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2867)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2869)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3031)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3032)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3034)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3039)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3040)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3042)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2858)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2859)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2861)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2866)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2867)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2869)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3031)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3032)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3034)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3039)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3040)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3042)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2858)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2859)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2861)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2866)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2867)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2869)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2853)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2854)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2856)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2861)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2862)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2864)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3026)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3027)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3029)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3034)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3035)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3037)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1968)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1969)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1971)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1976)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1977)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1979)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2070)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2071)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2073)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2078)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2079)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2081)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2172)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2173)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2175)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2180)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2181)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2183)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2274)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2275)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2277)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2282)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2283)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2285)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2376)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2377)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2379)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2384)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2385)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2387)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2478)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2479)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2481)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2486)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2487)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2489)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2580)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2581)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2583)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2588)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2589)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2591)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2682)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2683)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2685)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2690)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2691)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2693)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2771)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2774)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2800)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2801)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2803)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3018)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3019)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3021)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3047)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3050)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2845)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2846)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2848)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2874)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2875)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2877)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3018)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3019)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3021)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3047)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3050)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2845)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2846)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2848)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2874)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2875)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2877)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3018)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3019)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3021)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3047)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3050)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2874)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2875)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2877)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3018)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3019)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3021)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3047)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3050)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2845)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2846)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2848)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2874)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2875)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2877)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2840)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2841)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2843)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2869)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2870)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2872)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3013)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3014)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3016)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3042)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3043)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3045)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1955)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1956)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1958)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1984)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1985)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1987)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2057)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2058)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2060)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2086)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2087)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2089)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2159)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2160)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2162)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2188)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2189)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2191)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2261)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2262)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2264)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2290)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2291)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2293)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2363)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2364)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2366)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2392)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2393)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2395)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2465)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2466)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2468)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2494)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2495)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2497)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2567)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2568)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2570)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2596)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2597)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2599)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2669)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2670)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2672)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2698)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2699)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2701)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1999)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2001)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2101)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2103)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2203)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2205)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2305)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2307)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2407)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2409)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2509)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2511)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2611)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2613)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2713)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2715)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1992)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1994)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2094)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2096)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2196)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2198)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2298)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2300)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2400)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2402)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2502)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2504)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2604)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2606)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2706)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2708)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2882)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2883)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2885)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2821)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2822)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2824)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2829)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2830)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2832)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3068)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3069)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3071)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3076)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3079)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2895)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2896)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2898)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2903)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2904)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2906)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3068)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3069)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3071)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3076)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3079)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2895)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2896)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2898)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2903)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2904)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2906)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3068)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3069)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3071)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3076)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3079)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2895)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2896)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2898)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2903)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2904)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2906)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3068)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3069)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3071)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3076)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3079)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2895)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2896)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2898)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2903)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2904)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2906)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2890)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2891)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2893)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2898)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2899)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2901)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3063)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3064)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3066)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3071)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3072)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3074)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2019)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2020)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2022)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2027)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2028)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2030)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2121)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2122)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2124)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2129)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2130)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2132)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2223)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2224)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2226)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2231)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2232)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2234)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2325)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2326)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2328)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2333)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2334)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2336)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2427)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2428)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2430)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2435)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2436)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2438)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2529)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2530)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2532)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2537)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2538)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2540)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2631)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2632)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2634)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2639)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2640)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2642)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2733)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2734)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2736)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2741)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2742)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2744)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2808)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2809)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2811)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2837)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2838)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2840)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3055)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3058)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3084)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3085)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3087)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2882)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2883)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2885)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2911)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2912)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#2914)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3055)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3058)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3084)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3085)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3087)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2882)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2883)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2885)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2911)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2912)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#2914)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3055)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3058)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3084)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3085)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3087)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2911)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2912)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2914)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3055)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3058)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3084)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3085)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec4.rs.html#3087)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2882)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2883)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2885)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2911)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2912)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec4.rs.html#2914)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2877)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2878)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2880)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2906)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2907)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#2909)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3050)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3051)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3053)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3079)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3080)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3082)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2006)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2007)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2009)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2035)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2036)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2038)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2108)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2109)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2111)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2137)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2138)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2140)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2210)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2211)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2213)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2239)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2240)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2242)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2312)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2313)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2315)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2341)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2342)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2344)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2414)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2415)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2417)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2443)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2444)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2446)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2516)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2517)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2519)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2545)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2546)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2548)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2618)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2619)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2621)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2647)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2648)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2650)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2720)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2721)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2723)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2749)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2750)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2752)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2050)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2052)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2152)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2154)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2254)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2256)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2356)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2358)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2458)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2460)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2560)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2562)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2662)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2664)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2764)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2766)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2043)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2045)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2145)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2147)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2247)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2249)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2349)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2351)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2451)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2453)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2553)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2555)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2655)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2657)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2757)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#2759)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [field](trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [field\_mut](trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [field\_at](trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [field\_at\_mut](trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [name\_at](trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [index\_of\_name](trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [field\_len](trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [iter\_fields](trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [to\_dynamic\_struct](trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#25)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1284)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1285)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1287)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1297)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1298)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1300)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1305)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1306)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1308)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1405)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1406)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1408)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1413)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1414)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1416)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1351)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1352)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1354)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1359)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1360)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1362)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1313)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1314)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1316)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1392)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1393)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1395)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1421)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1422)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1424)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1338)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1339)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1341)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1367)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1368)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1370)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1321)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1323)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1331)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1333)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1385)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1387)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1375)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1377)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1588)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1590-1592)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1598)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#1600-1602)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3043)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3044)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3047)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3057)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3058)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3061)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3085)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3086)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3089)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3113)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3114)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3117)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3071)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3072)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3075)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3099)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3100)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3103)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3127)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")\> for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3128)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#3131)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"), <[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3248)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3249)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec4.rs.html#3252)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4"), <[I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3075)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3076)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec4.rs.html#3079)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4"), <[U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3244)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3245)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec4.rs.html#3248)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4"), <[I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3073)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3074)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec4.rs.html#3077)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4"), <[U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3240)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3241)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec4.rs.html#3244)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4"), <[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3094)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3095)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec4.rs.html#3098)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4"), <[USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3237)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\> for [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3238)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec4.rs.html#3241)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4"), <[ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [type\_path](trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [short\_type\_path](trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [type\_ident](trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [crate\_name](trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [module\_path](trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#6)

#### type [Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2) = [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#8)

#### type [Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3) = [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#11)

#### fn [xx](trait.Vec4Swizzles.html#tymethod.xx)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#19)

#### fn [xy](trait.Vec4Swizzles.html#tymethod.xy)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#27)

#### fn [with\_xy](trait.Vec4Swizzles.html#tymethod.with_xy)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#32)

#### fn [xz](trait.Vec4Swizzles.html#tymethod.xz)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#40)

#### fn [with\_xz](trait.Vec4Swizzles.html#tymethod.with_xz)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#45)

#### fn [xw](trait.Vec4Swizzles.html#tymethod.xw)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#53)

#### fn [with\_xw](trait.Vec4Swizzles.html#tymethod.with_xw)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#58)

#### fn [yx](trait.Vec4Swizzles.html#tymethod.yx)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#66)

#### fn [with\_yx](trait.Vec4Swizzles.html#tymethod.with_yx)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#71)

#### fn [yy](trait.Vec4Swizzles.html#tymethod.yy)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#79)

#### fn [yz](trait.Vec4Swizzles.html#tymethod.yz)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#87)

#### fn [with\_yz](trait.Vec4Swizzles.html#tymethod.with_yz)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#92)

#### fn [yw](trait.Vec4Swizzles.html#tymethod.yw)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#100)

#### fn [with\_yw](trait.Vec4Swizzles.html#tymethod.with_yw)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#105)

#### fn [zx](trait.Vec4Swizzles.html#tymethod.zx)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#113)

#### fn [with\_zx](trait.Vec4Swizzles.html#tymethod.with_zx)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#118)

#### fn [zy](trait.Vec4Swizzles.html#tymethod.zy)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#126)

#### fn [with\_zy](trait.Vec4Swizzles.html#tymethod.with_zy)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#131)

#### fn [zz](trait.Vec4Swizzles.html#tymethod.zz)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#139)

#### fn [zw](trait.Vec4Swizzles.html#tymethod.zw)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#147)

#### fn [with\_zw](trait.Vec4Swizzles.html#tymethod.with_zw)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#152)

#### fn [wx](trait.Vec4Swizzles.html#tymethod.wx)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#160)

#### fn [with\_wx](trait.Vec4Swizzles.html#tymethod.with_wx)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#165)

#### fn [wy](trait.Vec4Swizzles.html#tymethod.wy)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#173)

#### fn [with\_wy](trait.Vec4Swizzles.html#tymethod.with_wy)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#178)

#### fn [wz](trait.Vec4Swizzles.html#tymethod.wz)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#186)

#### fn [with\_wz](trait.Vec4Swizzles.html#tymethod.with_wz)(self, rhs: [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#191)

#### fn [ww](trait.Vec4Swizzles.html#tymethod.ww)(self) -> [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#199)

#### fn [xxx](trait.Vec4Swizzles.html#tymethod.xxx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#204)

#### fn [xxy](trait.Vec4Swizzles.html#tymethod.xxy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#209)

#### fn [xxz](trait.Vec4Swizzles.html#tymethod.xxz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#214)

#### fn [xxw](trait.Vec4Swizzles.html#tymethod.xxw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#219)

#### fn [xyx](trait.Vec4Swizzles.html#tymethod.xyx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#224)

#### fn [xyy](trait.Vec4Swizzles.html#tymethod.xyy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#229)

#### fn [xyz](trait.Vec4Swizzles.html#tymethod.xyz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#234)

#### fn [with\_xyz](trait.Vec4Swizzles.html#tymethod.with_xyz)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#239)

#### fn [xyw](trait.Vec4Swizzles.html#tymethod.xyw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#244)

#### fn [with\_xyw](trait.Vec4Swizzles.html#tymethod.with_xyw)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#249)

#### fn [xzx](trait.Vec4Swizzles.html#tymethod.xzx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#254)

#### fn [xzy](trait.Vec4Swizzles.html#tymethod.xzy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#259)

#### fn [with\_xzy](trait.Vec4Swizzles.html#tymethod.with_xzy)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#264)

#### fn [xzz](trait.Vec4Swizzles.html#tymethod.xzz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#269)

#### fn [xzw](trait.Vec4Swizzles.html#tymethod.xzw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#274)

#### fn [with\_xzw](trait.Vec4Swizzles.html#tymethod.with_xzw)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#279)

#### fn [xwx](trait.Vec4Swizzles.html#tymethod.xwx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#284)

#### fn [xwy](trait.Vec4Swizzles.html#tymethod.xwy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#289)

#### fn [with\_xwy](trait.Vec4Swizzles.html#tymethod.with_xwy)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#294)

#### fn [xwz](trait.Vec4Swizzles.html#tymethod.xwz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#299)

#### fn [with\_xwz](trait.Vec4Swizzles.html#tymethod.with_xwz)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#304)

#### fn [xww](trait.Vec4Swizzles.html#tymethod.xww)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#309)

#### fn [yxx](trait.Vec4Swizzles.html#tymethod.yxx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#314)

#### fn [yxy](trait.Vec4Swizzles.html#tymethod.yxy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#319)

#### fn [yxz](trait.Vec4Swizzles.html#tymethod.yxz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#324)

#### fn [with\_yxz](trait.Vec4Swizzles.html#tymethod.with_yxz)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#329)

#### fn [yxw](trait.Vec4Swizzles.html#tymethod.yxw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#334)

#### fn [with\_yxw](trait.Vec4Swizzles.html#tymethod.with_yxw)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#339)

#### fn [yyx](trait.Vec4Swizzles.html#tymethod.yyx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#344)

#### fn [yyy](trait.Vec4Swizzles.html#tymethod.yyy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#349)

#### fn [yyz](trait.Vec4Swizzles.html#tymethod.yyz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#354)

#### fn [yyw](trait.Vec4Swizzles.html#tymethod.yyw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#359)

#### fn [yzx](trait.Vec4Swizzles.html#tymethod.yzx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#364)

#### fn [with\_yzx](trait.Vec4Swizzles.html#tymethod.with_yzx)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#369)

#### fn [yzy](trait.Vec4Swizzles.html#tymethod.yzy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#374)

#### fn [yzz](trait.Vec4Swizzles.html#tymethod.yzz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#379)

#### fn [yzw](trait.Vec4Swizzles.html#tymethod.yzw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#384)

#### fn [with\_yzw](trait.Vec4Swizzles.html#tymethod.with_yzw)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#389)

#### fn [ywx](trait.Vec4Swizzles.html#tymethod.ywx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#394)

#### fn [with\_ywx](trait.Vec4Swizzles.html#tymethod.with_ywx)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#399)

#### fn [ywy](trait.Vec4Swizzles.html#tymethod.ywy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#404)

#### fn [ywz](trait.Vec4Swizzles.html#tymethod.ywz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#409)

#### fn [with\_ywz](trait.Vec4Swizzles.html#tymethod.with_ywz)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#414)

#### fn [yww](trait.Vec4Swizzles.html#tymethod.yww)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#419)

#### fn [zxx](trait.Vec4Swizzles.html#tymethod.zxx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#424)

#### fn [zxy](trait.Vec4Swizzles.html#tymethod.zxy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#429)

#### fn [with\_zxy](trait.Vec4Swizzles.html#tymethod.with_zxy)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#434)

#### fn [zxz](trait.Vec4Swizzles.html#tymethod.zxz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#439)

#### fn [zxw](trait.Vec4Swizzles.html#tymethod.zxw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#444)

#### fn [with\_zxw](trait.Vec4Swizzles.html#tymethod.with_zxw)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#449)

#### fn [zyx](trait.Vec4Swizzles.html#tymethod.zyx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#454)

#### fn [with\_zyx](trait.Vec4Swizzles.html#tymethod.with_zyx)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#459)

#### fn [zyy](trait.Vec4Swizzles.html#tymethod.zyy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#464)

#### fn [zyz](trait.Vec4Swizzles.html#tymethod.zyz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#469)

#### fn [zyw](trait.Vec4Swizzles.html#tymethod.zyw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#474)

#### fn [with\_zyw](trait.Vec4Swizzles.html#tymethod.with_zyw)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#479)

#### fn [zzx](trait.Vec4Swizzles.html#tymethod.zzx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#484)

#### fn [zzy](trait.Vec4Swizzles.html#tymethod.zzy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#489)

#### fn [zzz](trait.Vec4Swizzles.html#tymethod.zzz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#494)

#### fn [zzw](trait.Vec4Swizzles.html#tymethod.zzw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#499)

#### fn [zwx](trait.Vec4Swizzles.html#tymethod.zwx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#504)

#### fn [with\_zwx](trait.Vec4Swizzles.html#tymethod.with_zwx)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#509)

#### fn [zwy](trait.Vec4Swizzles.html#tymethod.zwy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#514)

#### fn [with\_zwy](trait.Vec4Swizzles.html#tymethod.with_zwy)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#519)

#### fn [zwz](trait.Vec4Swizzles.html#tymethod.zwz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#524)

#### fn [zww](trait.Vec4Swizzles.html#tymethod.zww)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#529)

#### fn [wxx](trait.Vec4Swizzles.html#tymethod.wxx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#534)

#### fn [wxy](trait.Vec4Swizzles.html#tymethod.wxy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#539)

#### fn [with\_wxy](trait.Vec4Swizzles.html#tymethod.with_wxy)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#544)

#### fn [wxz](trait.Vec4Swizzles.html#tymethod.wxz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#549)

#### fn [with\_wxz](trait.Vec4Swizzles.html#tymethod.with_wxz)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#554)

#### fn [wxw](trait.Vec4Swizzles.html#tymethod.wxw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#559)

#### fn [wyx](trait.Vec4Swizzles.html#tymethod.wyx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#564)

#### fn [with\_wyx](trait.Vec4Swizzles.html#tymethod.with_wyx)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#569)

#### fn [wyy](trait.Vec4Swizzles.html#tymethod.wyy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#574)

#### fn [wyz](trait.Vec4Swizzles.html#tymethod.wyz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#579)

#### fn [with\_wyz](trait.Vec4Swizzles.html#tymethod.with_wyz)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#584)

#### fn [wyw](trait.Vec4Swizzles.html#tymethod.wyw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#589)

#### fn [wzx](trait.Vec4Swizzles.html#tymethod.wzx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#594)

#### fn [with\_wzx](trait.Vec4Swizzles.html#tymethod.with_wzx)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#599)

#### fn [wzy](trait.Vec4Swizzles.html#tymethod.wzy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#604)

#### fn [with\_wzy](trait.Vec4Swizzles.html#tymethod.with_wzy)(self, rhs: [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#609)

#### fn [wzz](trait.Vec4Swizzles.html#tymethod.wzz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#614)

#### fn [wzw](trait.Vec4Swizzles.html#tymethod.wzw)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#619)

#### fn [wwx](trait.Vec4Swizzles.html#tymethod.wwx)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#624)

#### fn [wwy](trait.Vec4Swizzles.html#tymethod.wwy)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#629)

#### fn [wwz](trait.Vec4Swizzles.html#tymethod.wwz)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#634)

#### fn [www](trait.Vec4Swizzles.html#tymethod.www)(self) -> [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#639)

#### fn [xxxx](trait.Vec4Swizzles.html#tymethod.xxxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#644)

#### fn [xxxy](trait.Vec4Swizzles.html#tymethod.xxxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#649)

#### fn [xxxz](trait.Vec4Swizzles.html#tymethod.xxxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#654)

#### fn [xxxw](trait.Vec4Swizzles.html#tymethod.xxxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#659)

#### fn [xxyx](trait.Vec4Swizzles.html#tymethod.xxyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#664)

#### fn [xxyy](trait.Vec4Swizzles.html#tymethod.xxyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#669)

#### fn [xxyz](trait.Vec4Swizzles.html#tymethod.xxyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#674)

#### fn [xxyw](trait.Vec4Swizzles.html#tymethod.xxyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#679)

#### fn [xxzx](trait.Vec4Swizzles.html#tymethod.xxzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#684)

#### fn [xxzy](trait.Vec4Swizzles.html#tymethod.xxzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#689)

#### fn [xxzz](trait.Vec4Swizzles.html#tymethod.xxzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#694)

#### fn [xxzw](trait.Vec4Swizzles.html#tymethod.xxzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#699)

#### fn [xxwx](trait.Vec4Swizzles.html#tymethod.xxwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#704)

#### fn [xxwy](trait.Vec4Swizzles.html#tymethod.xxwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#709)

#### fn [xxwz](trait.Vec4Swizzles.html#tymethod.xxwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#714)

#### fn [xxww](trait.Vec4Swizzles.html#tymethod.xxww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#719)

#### fn [xyxx](trait.Vec4Swizzles.html#tymethod.xyxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#724)

#### fn [xyxy](trait.Vec4Swizzles.html#tymethod.xyxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#729)

#### fn [xyxz](trait.Vec4Swizzles.html#tymethod.xyxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#734)

#### fn [xyxw](trait.Vec4Swizzles.html#tymethod.xyxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#739)

#### fn [xyyx](trait.Vec4Swizzles.html#tymethod.xyyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#744)

#### fn [xyyy](trait.Vec4Swizzles.html#tymethod.xyyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#749)

#### fn [xyyz](trait.Vec4Swizzles.html#tymethod.xyyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#754)

#### fn [xyyw](trait.Vec4Swizzles.html#tymethod.xyyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#759)

#### fn [xyzx](trait.Vec4Swizzles.html#tymethod.xyzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#764)

#### fn [xyzy](trait.Vec4Swizzles.html#tymethod.xyzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#769)

#### fn [xyzz](trait.Vec4Swizzles.html#tymethod.xyzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#774)

#### fn [xywx](trait.Vec4Swizzles.html#tymethod.xywx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#779)

#### fn [xywy](trait.Vec4Swizzles.html#tymethod.xywy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#784)

#### fn [xywz](trait.Vec4Swizzles.html#tymethod.xywz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#789)

#### fn [xyww](trait.Vec4Swizzles.html#tymethod.xyww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#794)

#### fn [xzxx](trait.Vec4Swizzles.html#tymethod.xzxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#799)

#### fn [xzxy](trait.Vec4Swizzles.html#tymethod.xzxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#804)

#### fn [xzxz](trait.Vec4Swizzles.html#tymethod.xzxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#809)

#### fn [xzxw](trait.Vec4Swizzles.html#tymethod.xzxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#814)

#### fn [xzyx](trait.Vec4Swizzles.html#tymethod.xzyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#819)

#### fn [xzyy](trait.Vec4Swizzles.html#tymethod.xzyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#824)

#### fn [xzyz](trait.Vec4Swizzles.html#tymethod.xzyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#829)

#### fn [xzyw](trait.Vec4Swizzles.html#tymethod.xzyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#834)

#### fn [xzzx](trait.Vec4Swizzles.html#tymethod.xzzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#839)

#### fn [xzzy](trait.Vec4Swizzles.html#tymethod.xzzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#844)

#### fn [xzzz](trait.Vec4Swizzles.html#tymethod.xzzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#849)

#### fn [xzzw](trait.Vec4Swizzles.html#tymethod.xzzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#854)

#### fn [xzwx](trait.Vec4Swizzles.html#tymethod.xzwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#859)

#### fn [xzwy](trait.Vec4Swizzles.html#tymethod.xzwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#864)

#### fn [xzwz](trait.Vec4Swizzles.html#tymethod.xzwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#869)

#### fn [xzww](trait.Vec4Swizzles.html#tymethod.xzww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#874)

#### fn [xwxx](trait.Vec4Swizzles.html#tymethod.xwxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#879)

#### fn [xwxy](trait.Vec4Swizzles.html#tymethod.xwxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#884)

#### fn [xwxz](trait.Vec4Swizzles.html#tymethod.xwxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#889)

#### fn [xwxw](trait.Vec4Swizzles.html#tymethod.xwxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#894)

#### fn [xwyx](trait.Vec4Swizzles.html#tymethod.xwyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#899)

#### fn [xwyy](trait.Vec4Swizzles.html#tymethod.xwyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#904)

#### fn [xwyz](trait.Vec4Swizzles.html#tymethod.xwyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#909)

#### fn [xwyw](trait.Vec4Swizzles.html#tymethod.xwyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#914)

#### fn [xwzx](trait.Vec4Swizzles.html#tymethod.xwzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#919)

#### fn [xwzy](trait.Vec4Swizzles.html#tymethod.xwzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#924)

#### fn [xwzz](trait.Vec4Swizzles.html#tymethod.xwzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#929)

#### fn [xwzw](trait.Vec4Swizzles.html#tymethod.xwzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#934)

#### fn [xwwx](trait.Vec4Swizzles.html#tymethod.xwwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#939)

#### fn [xwwy](trait.Vec4Swizzles.html#tymethod.xwwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#944)

#### fn [xwwz](trait.Vec4Swizzles.html#tymethod.xwwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#949)

#### fn [xwww](trait.Vec4Swizzles.html#tymethod.xwww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#954)

#### fn [yxxx](trait.Vec4Swizzles.html#tymethod.yxxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#959)

#### fn [yxxy](trait.Vec4Swizzles.html#tymethod.yxxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#964)

#### fn [yxxz](trait.Vec4Swizzles.html#tymethod.yxxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#969)

#### fn [yxxw](trait.Vec4Swizzles.html#tymethod.yxxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#974)

#### fn [yxyx](trait.Vec4Swizzles.html#tymethod.yxyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#979)

#### fn [yxyy](trait.Vec4Swizzles.html#tymethod.yxyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#984)

#### fn [yxyz](trait.Vec4Swizzles.html#tymethod.yxyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#989)

#### fn [yxyw](trait.Vec4Swizzles.html#tymethod.yxyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#994)

#### fn [yxzx](trait.Vec4Swizzles.html#tymethod.yxzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#999)

#### fn [yxzy](trait.Vec4Swizzles.html#tymethod.yxzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1004)

#### fn [yxzz](trait.Vec4Swizzles.html#tymethod.yxzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1009)

#### fn [yxzw](trait.Vec4Swizzles.html#tymethod.yxzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1014)

#### fn [yxwx](trait.Vec4Swizzles.html#tymethod.yxwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1019)

#### fn [yxwy](trait.Vec4Swizzles.html#tymethod.yxwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1024)

#### fn [yxwz](trait.Vec4Swizzles.html#tymethod.yxwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1029)

#### fn [yxww](trait.Vec4Swizzles.html#tymethod.yxww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1034)

#### fn [yyxx](trait.Vec4Swizzles.html#tymethod.yyxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1039)

#### fn [yyxy](trait.Vec4Swizzles.html#tymethod.yyxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1044)

#### fn [yyxz](trait.Vec4Swizzles.html#tymethod.yyxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1049)

#### fn [yyxw](trait.Vec4Swizzles.html#tymethod.yyxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1054)

#### fn [yyyx](trait.Vec4Swizzles.html#tymethod.yyyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1059)

#### fn [yyyy](trait.Vec4Swizzles.html#tymethod.yyyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1064)

#### fn [yyyz](trait.Vec4Swizzles.html#tymethod.yyyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1069)

#### fn [yyyw](trait.Vec4Swizzles.html#tymethod.yyyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1074)

#### fn [yyzx](trait.Vec4Swizzles.html#tymethod.yyzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1079)

#### fn [yyzy](trait.Vec4Swizzles.html#tymethod.yyzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1084)

#### fn [yyzz](trait.Vec4Swizzles.html#tymethod.yyzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1089)

#### fn [yyzw](trait.Vec4Swizzles.html#tymethod.yyzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1094)

#### fn [yywx](trait.Vec4Swizzles.html#tymethod.yywx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1099)

#### fn [yywy](trait.Vec4Swizzles.html#tymethod.yywy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1104)

#### fn [yywz](trait.Vec4Swizzles.html#tymethod.yywz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1109)

#### fn [yyww](trait.Vec4Swizzles.html#tymethod.yyww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1114)

#### fn [yzxx](trait.Vec4Swizzles.html#tymethod.yzxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1119)

#### fn [yzxy](trait.Vec4Swizzles.html#tymethod.yzxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1124)

#### fn [yzxz](trait.Vec4Swizzles.html#tymethod.yzxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1129)

#### fn [yzxw](trait.Vec4Swizzles.html#tymethod.yzxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1134)

#### fn [yzyx](trait.Vec4Swizzles.html#tymethod.yzyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1139)

#### fn [yzyy](trait.Vec4Swizzles.html#tymethod.yzyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1144)

#### fn [yzyz](trait.Vec4Swizzles.html#tymethod.yzyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1149)

#### fn [yzyw](trait.Vec4Swizzles.html#tymethod.yzyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1154)

#### fn [yzzx](trait.Vec4Swizzles.html#tymethod.yzzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1159)

#### fn [yzzy](trait.Vec4Swizzles.html#tymethod.yzzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1164)

#### fn [yzzz](trait.Vec4Swizzles.html#tymethod.yzzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1169)

#### fn [yzzw](trait.Vec4Swizzles.html#tymethod.yzzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1174)

#### fn [yzwx](trait.Vec4Swizzles.html#tymethod.yzwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1179)

#### fn [yzwy](trait.Vec4Swizzles.html#tymethod.yzwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1184)

#### fn [yzwz](trait.Vec4Swizzles.html#tymethod.yzwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1189)

#### fn [yzww](trait.Vec4Swizzles.html#tymethod.yzww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1194)

#### fn [ywxx](trait.Vec4Swizzles.html#tymethod.ywxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1199)

#### fn [ywxy](trait.Vec4Swizzles.html#tymethod.ywxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1204)

#### fn [ywxz](trait.Vec4Swizzles.html#tymethod.ywxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1209)

#### fn [ywxw](trait.Vec4Swizzles.html#tymethod.ywxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1214)

#### fn [ywyx](trait.Vec4Swizzles.html#tymethod.ywyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1219)

#### fn [ywyy](trait.Vec4Swizzles.html#tymethod.ywyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1224)

#### fn [ywyz](trait.Vec4Swizzles.html#tymethod.ywyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1229)

#### fn [ywyw](trait.Vec4Swizzles.html#tymethod.ywyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1234)

#### fn [ywzx](trait.Vec4Swizzles.html#tymethod.ywzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1239)

#### fn [ywzy](trait.Vec4Swizzles.html#tymethod.ywzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1244)

#### fn [ywzz](trait.Vec4Swizzles.html#tymethod.ywzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1249)

#### fn [ywzw](trait.Vec4Swizzles.html#tymethod.ywzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1254)

#### fn [ywwx](trait.Vec4Swizzles.html#tymethod.ywwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1259)

#### fn [ywwy](trait.Vec4Swizzles.html#tymethod.ywwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1264)

#### fn [ywwz](trait.Vec4Swizzles.html#tymethod.ywwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1269)

#### fn [ywww](trait.Vec4Swizzles.html#tymethod.ywww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1274)

#### fn [zxxx](trait.Vec4Swizzles.html#tymethod.zxxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1279)

#### fn [zxxy](trait.Vec4Swizzles.html#tymethod.zxxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1284)

#### fn [zxxz](trait.Vec4Swizzles.html#tymethod.zxxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1289)

#### fn [zxxw](trait.Vec4Swizzles.html#tymethod.zxxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1294)

#### fn [zxyx](trait.Vec4Swizzles.html#tymethod.zxyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1299)

#### fn [zxyy](trait.Vec4Swizzles.html#tymethod.zxyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1304)

#### fn [zxyz](trait.Vec4Swizzles.html#tymethod.zxyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1309)

#### fn [zxyw](trait.Vec4Swizzles.html#tymethod.zxyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1314)

#### fn [zxzx](trait.Vec4Swizzles.html#tymethod.zxzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1319)

#### fn [zxzy](trait.Vec4Swizzles.html#tymethod.zxzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1324)

#### fn [zxzz](trait.Vec4Swizzles.html#tymethod.zxzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1329)

#### fn [zxzw](trait.Vec4Swizzles.html#tymethod.zxzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1334)

#### fn [zxwx](trait.Vec4Swizzles.html#tymethod.zxwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1339)

#### fn [zxwy](trait.Vec4Swizzles.html#tymethod.zxwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1344)

#### fn [zxwz](trait.Vec4Swizzles.html#tymethod.zxwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1349)

#### fn [zxww](trait.Vec4Swizzles.html#tymethod.zxww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1354)

#### fn [zyxx](trait.Vec4Swizzles.html#tymethod.zyxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1359)

#### fn [zyxy](trait.Vec4Swizzles.html#tymethod.zyxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1364)

#### fn [zyxz](trait.Vec4Swizzles.html#tymethod.zyxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1369)

#### fn [zyxw](trait.Vec4Swizzles.html#tymethod.zyxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1374)

#### fn [zyyx](trait.Vec4Swizzles.html#tymethod.zyyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1379)

#### fn [zyyy](trait.Vec4Swizzles.html#tymethod.zyyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1384)

#### fn [zyyz](trait.Vec4Swizzles.html#tymethod.zyyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1389)

#### fn [zyyw](trait.Vec4Swizzles.html#tymethod.zyyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1394)

#### fn [zyzx](trait.Vec4Swizzles.html#tymethod.zyzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1399)

#### fn [zyzy](trait.Vec4Swizzles.html#tymethod.zyzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1404)

#### fn [zyzz](trait.Vec4Swizzles.html#tymethod.zyzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1409)

#### fn [zyzw](trait.Vec4Swizzles.html#tymethod.zyzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1414)

#### fn [zywx](trait.Vec4Swizzles.html#tymethod.zywx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1419)

#### fn [zywy](trait.Vec4Swizzles.html#tymethod.zywy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1424)

#### fn [zywz](trait.Vec4Swizzles.html#tymethod.zywz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1429)

#### fn [zyww](trait.Vec4Swizzles.html#tymethod.zyww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1434)

#### fn [zzxx](trait.Vec4Swizzles.html#tymethod.zzxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1439)

#### fn [zzxy](trait.Vec4Swizzles.html#tymethod.zzxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1444)

#### fn [zzxz](trait.Vec4Swizzles.html#tymethod.zzxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1449)

#### fn [zzxw](trait.Vec4Swizzles.html#tymethod.zzxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1454)

#### fn [zzyx](trait.Vec4Swizzles.html#tymethod.zzyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1459)

#### fn [zzyy](trait.Vec4Swizzles.html#tymethod.zzyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1464)

#### fn [zzyz](trait.Vec4Swizzles.html#tymethod.zzyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1469)

#### fn [zzyw](trait.Vec4Swizzles.html#tymethod.zzyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1474)

#### fn [zzzx](trait.Vec4Swizzles.html#tymethod.zzzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1479)

#### fn [zzzy](trait.Vec4Swizzles.html#tymethod.zzzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1484)

#### fn [zzzz](trait.Vec4Swizzles.html#tymethod.zzzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1489)

#### fn [zzzw](trait.Vec4Swizzles.html#tymethod.zzzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1494)

#### fn [zzwx](trait.Vec4Swizzles.html#tymethod.zzwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1499)

#### fn [zzwy](trait.Vec4Swizzles.html#tymethod.zzwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1504)

#### fn [zzwz](trait.Vec4Swizzles.html#tymethod.zzwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1509)

#### fn [zzww](trait.Vec4Swizzles.html#tymethod.zzww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1514)

#### fn [zwxx](trait.Vec4Swizzles.html#tymethod.zwxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1519)

#### fn [zwxy](trait.Vec4Swizzles.html#tymethod.zwxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1524)

#### fn [zwxz](trait.Vec4Swizzles.html#tymethod.zwxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1529)

#### fn [zwxw](trait.Vec4Swizzles.html#tymethod.zwxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1534)

#### fn [zwyx](trait.Vec4Swizzles.html#tymethod.zwyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1539)

#### fn [zwyy](trait.Vec4Swizzles.html#tymethod.zwyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1544)

#### fn [zwyz](trait.Vec4Swizzles.html#tymethod.zwyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1549)

#### fn [zwyw](trait.Vec4Swizzles.html#tymethod.zwyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1554)

#### fn [zwzx](trait.Vec4Swizzles.html#tymethod.zwzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1559)

#### fn [zwzy](trait.Vec4Swizzles.html#tymethod.zwzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1564)

#### fn [zwzz](trait.Vec4Swizzles.html#tymethod.zwzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1569)

#### fn [zwzw](trait.Vec4Swizzles.html#tymethod.zwzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1574)

#### fn [zwwx](trait.Vec4Swizzles.html#tymethod.zwwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1579)

#### fn [zwwy](trait.Vec4Swizzles.html#tymethod.zwwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1584)

#### fn [zwwz](trait.Vec4Swizzles.html#tymethod.zwwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1589)

#### fn [zwww](trait.Vec4Swizzles.html#tymethod.zwww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1594)

#### fn [wxxx](trait.Vec4Swizzles.html#tymethod.wxxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1599)

#### fn [wxxy](trait.Vec4Swizzles.html#tymethod.wxxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1604)

#### fn [wxxz](trait.Vec4Swizzles.html#tymethod.wxxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1609)

#### fn [wxxw](trait.Vec4Swizzles.html#tymethod.wxxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1614)

#### fn [wxyx](trait.Vec4Swizzles.html#tymethod.wxyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1619)

#### fn [wxyy](trait.Vec4Swizzles.html#tymethod.wxyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1624)

#### fn [wxyz](trait.Vec4Swizzles.html#tymethod.wxyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1629)

#### fn [wxyw](trait.Vec4Swizzles.html#tymethod.wxyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1634)

#### fn [wxzx](trait.Vec4Swizzles.html#tymethod.wxzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1639)

#### fn [wxzy](trait.Vec4Swizzles.html#tymethod.wxzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1644)

#### fn [wxzz](trait.Vec4Swizzles.html#tymethod.wxzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1649)

#### fn [wxzw](trait.Vec4Swizzles.html#tymethod.wxzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1654)

#### fn [wxwx](trait.Vec4Swizzles.html#tymethod.wxwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1659)

#### fn [wxwy](trait.Vec4Swizzles.html#tymethod.wxwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1664)

#### fn [wxwz](trait.Vec4Swizzles.html#tymethod.wxwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1669)

#### fn [wxww](trait.Vec4Swizzles.html#tymethod.wxww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1674)

#### fn [wyxx](trait.Vec4Swizzles.html#tymethod.wyxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1679)

#### fn [wyxy](trait.Vec4Swizzles.html#tymethod.wyxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1684)

#### fn [wyxz](trait.Vec4Swizzles.html#tymethod.wyxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1689)

#### fn [wyxw](trait.Vec4Swizzles.html#tymethod.wyxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1694)

#### fn [wyyx](trait.Vec4Swizzles.html#tymethod.wyyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1699)

#### fn [wyyy](trait.Vec4Swizzles.html#tymethod.wyyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1704)

#### fn [wyyz](trait.Vec4Swizzles.html#tymethod.wyyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1709)

#### fn [wyyw](trait.Vec4Swizzles.html#tymethod.wyyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1714)

#### fn [wyzx](trait.Vec4Swizzles.html#tymethod.wyzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1719)

#### fn [wyzy](trait.Vec4Swizzles.html#tymethod.wyzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1724)

#### fn [wyzz](trait.Vec4Swizzles.html#tymethod.wyzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1729)

#### fn [wyzw](trait.Vec4Swizzles.html#tymethod.wyzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1734)

#### fn [wywx](trait.Vec4Swizzles.html#tymethod.wywx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1739)

#### fn [wywy](trait.Vec4Swizzles.html#tymethod.wywy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1744)

#### fn [wywz](trait.Vec4Swizzles.html#tymethod.wywz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1749)

#### fn [wyww](trait.Vec4Swizzles.html#tymethod.wyww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1754)

#### fn [wzxx](trait.Vec4Swizzles.html#tymethod.wzxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1759)

#### fn [wzxy](trait.Vec4Swizzles.html#tymethod.wzxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1764)

#### fn [wzxz](trait.Vec4Swizzles.html#tymethod.wzxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1769)

#### fn [wzxw](trait.Vec4Swizzles.html#tymethod.wzxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1774)

#### fn [wzyx](trait.Vec4Swizzles.html#tymethod.wzyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1779)

#### fn [wzyy](trait.Vec4Swizzles.html#tymethod.wzyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1784)

#### fn [wzyz](trait.Vec4Swizzles.html#tymethod.wzyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1789)

#### fn [wzyw](trait.Vec4Swizzles.html#tymethod.wzyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1794)

#### fn [wzzx](trait.Vec4Swizzles.html#tymethod.wzzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1799)

#### fn [wzzy](trait.Vec4Swizzles.html#tymethod.wzzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1804)

#### fn [wzzz](trait.Vec4Swizzles.html#tymethod.wzzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1809)

#### fn [wzzw](trait.Vec4Swizzles.html#tymethod.wzzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1814)

#### fn [wzwx](trait.Vec4Swizzles.html#tymethod.wzwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1819)

#### fn [wzwy](trait.Vec4Swizzles.html#tymethod.wzwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1824)

#### fn [wzwz](trait.Vec4Swizzles.html#tymethod.wzwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1829)

#### fn [wzww](trait.Vec4Swizzles.html#tymethod.wzww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1834)

#### fn [wwxx](trait.Vec4Swizzles.html#tymethod.wwxx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1839)

#### fn [wwxy](trait.Vec4Swizzles.html#tymethod.wwxy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1844)

#### fn [wwxz](trait.Vec4Swizzles.html#tymethod.wwxz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1849)

#### fn [wwxw](trait.Vec4Swizzles.html#tymethod.wwxw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1854)

#### fn [wwyx](trait.Vec4Swizzles.html#tymethod.wwyx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1859)

#### fn [wwyy](trait.Vec4Swizzles.html#tymethod.wwyy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1864)

#### fn [wwyz](trait.Vec4Swizzles.html#tymethod.wwyz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1869)

#### fn [wwyw](trait.Vec4Swizzles.html#tymethod.wwyw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1874)

#### fn [wwzx](trait.Vec4Swizzles.html#tymethod.wwzx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1879)

#### fn [wwzy](trait.Vec4Swizzles.html#tymethod.wwzy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1884)

#### fn [wwzz](trait.Vec4Swizzles.html#tymethod.wwzz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1889)

#### fn [wwzw](trait.Vec4Swizzles.html#tymethod.wwzw)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1894)

#### fn [wwwx](trait.Vec4Swizzles.html#tymethod.wwwx)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1899)

#### fn [wwwy](trait.Vec4Swizzles.html#tymethod.wwwy)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1904)

#### fn [wwwz](trait.Vec4Swizzles.html#tymethod.wwwz)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#1909)

#### fn [wwww](trait.Vec4Swizzles.html#tymethod.wwww)(self) -> [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#481)

#### fn [xyzw](trait.Vec4Swizzles.html#method.xyzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

### impl [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

where [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4"): [AsRefVectorParts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), 4>, [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#16)

#### fn [write\_into](../render/render_resource/encase/internal/trait.WriteInto.html#tymethod.write_into)<B>(&self, writer: &mut [Writer](../render/render_resource/encase/internal/struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](../render/render_resource/encase/internal/trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec4.rs.html#26)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

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

where T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

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

### impl<T> [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#272)

### impl<S> [GetField](trait.GetField.html "trait bevy::prelude::GetField") for S

where S: [Struct](trait.Struct.html "trait bevy::prelude::Struct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#273)

#### fn [get\_field](trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#278)

#### fn [get\_field\_mut](trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](trait.GetPath.html#method.path_mut)

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

### impl<G> [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

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

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](trait.ToOwned.html#method.clone_into)

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

### impl<T> [ToString](trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2902)

#### fn [to\_string](trait.ToString.html#tymethod.to_string)(&self) -> [String](struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`. [Read more](trait.ToString.html#tymethod.to_string)

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

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}