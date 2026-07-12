[bevy](../../index.html)::[math](../index.html)::[usize](index.html)

# Struct USizeVec3 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#31)

```rust
#[repr(C)]pub struct USizeVec3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
```

A 3-dimensional vector.

## Fields

`x: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)``y: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)``z: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#37)

### impl [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#39)

#### pub const [ZERO](#associatedconstant.ZERO): [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#42)

#### pub const [ONE](#associatedconstant.ONE): [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#45)

#### pub const [MIN](#associatedconstant.MIN): [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

All `usize::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#48)

#### pub const [MAX](#associatedconstant.MAX): [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

All `usize::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#51)

#### pub const [X](#associatedconstant.X): [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#54)

#### pub const [Y](#associatedconstant.Y): [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#57)

#### pub const [Z](#associatedconstant.Z): [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#60)

#### pub const [AXES](#associatedconstant.AXES): \[[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#65)

#### pub const fn [new](#method.new)(x: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), y: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), z: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a new vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#72)

#### pub const fn [splat](#method.splat)(v: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a vector with all elements set to `v`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#79-81)

#### pub fn [map](#method.map)<F>(self, f: F) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#93)

#### pub fn [select](#method.select)(mask: [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3"), if\_true: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), if\_false: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#104)

#### pub const fn [from\_array](#method.from_array)(a: \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#111)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#122)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\]) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a vector from the first 3 values in `slice`.

##### Panics

Panics if `slice` is less than 3 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#133)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\])

Writes the elements of `self` to the first 3 elements in `slice`.

##### Panics

Panics if `slice` is less than 3 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#152)

#### pub fn [extend](#method.extend)(self, w: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

Creates a 4D vector from `self` and the given `w` value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#161)

#### pub fn [truncate](#method.truncate)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.

Truncation may also be performed by using [`self.xy()`](../../prelude/trait.Vec3Swizzles.html#tymethod.xy "method bevy::prelude::Vec3Swizzles::xy").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#169)

#### pub fn [with\_x](#method.with_x)(self, x: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a 3D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#177)

#### pub fn [with\_y](#method.with_y)(self, y: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a 3D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#185)

#### pub fn [with\_z](#method.with_z)(self, z: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Creates a 3D vector from `self` with the given value of `z`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#193)

#### pub fn [dot](#method.dot)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#200)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#207)

#### pub fn [cross](#method.cross)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Computes the cross product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#220)

#### pub fn [min](#method.min)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#233)

#### pub fn [max](#method.max)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#250)

#### pub fn [clamp](#method.clamp)(self, min: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), max: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Component-wise clamping of values, similar to [`usize::clamp`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#method.clamp "method usize::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#260)

#### pub fn [min\_element](#method.min_element)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#270)

#### pub fn [max\_element](#method.max_element)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#279)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#296)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#314)

#### pub fn [element\_sum](#method.element_sum)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#323)

#### pub fn [element\_product](#method.element_product)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#334)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#345)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#356)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#367)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#378)

#### pub fn [cmple](#method.cmple)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#389)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#397)

#### pub fn [length\_squared](#method.length_squared)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Computes the squared length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#411)

#### pub fn [manhattan\_distance](#method.manhattan_distance)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

##### Overflow

This method may overflow if the result is greater than [`usize::MAX`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#associatedconstant.MAX "associated constant usize::MAX").

See also [`checked_manhattan_distance`](../struct.USizeVec3.html#method.checked_manhattan_distance "method bevy::math::USizeVec3::checked_manhattan_distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#422)

#### pub fn [checked\_manhattan\_distance](#method.checked_manhattan_distance)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Computes the [manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between two points.

This will returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the result is greater than [`usize::MAX`](https://doc.rust-lang.org/nightly/std/primitive.usize.html#associatedconstant.MAX "associated constant usize::MAX").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#433)

#### pub fn [chebyshev\_distance](#method.chebyshev_distance)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Computes the [chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance) between two points.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#448)

#### pub fn [as\_vec3](#method.as_vec3)(self) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Casts all elements of `self` to `f32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#455)

#### pub fn [as\_vec3a](#method.as_vec3a)(self) -> [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Casts all elements of `self` to `f32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#462)

#### pub fn [as\_dvec3](#method.as_dvec3)(self) -> [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#469)

#### pub fn [as\_i8vec3](#method.as_i8vec3)(self) -> [I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#476)

#### pub fn [as\_u8vec3](#method.as_u8vec3)(self) -> [U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#483)

#### pub fn [as\_i16vec3](#method.as_i16vec3)(self) -> [I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#490)

#### pub fn [as\_u16vec3](#method.as_u16vec3)(self) -> [U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#497)

#### pub fn [as\_ivec3](#method.as_ivec3)(self) -> [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#504)

#### pub fn [as\_uvec3](#method.as_uvec3)(self) -> [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

Casts all elements of `self` to `u32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#511)

#### pub fn [as\_i64vec3](#method.as_i64vec3)(self) -> [I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#518)

#### pub fn [as\_u64vec3](#method.as_u64vec3)(self) -> [U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#525)

#### pub fn [as\_isizevec3](#method.as_isizevec3)(self) -> [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#534)

#### pub const fn [checked\_add](#method.checked_add)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#556)

#### pub const fn [checked\_sub](#method.checked_sub)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `Some([self.x - rhs.x, self.y - rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#578)

#### pub const fn [checked\_mul](#method.checked_mul)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `Some([self.x * rhs.x, self.y * rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#600)

#### pub const fn [checked\_div](#method.checked_div)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `Some([self.x / rhs.x, self.y / rhs.y, ..])` but returns `None` on any division by zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#622)

#### pub const fn [wrapping\_add](#method.wrapping_add)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the wrapping addition of `self` and `rhs`.

In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#635)

#### pub const fn [wrapping\_sub](#method.wrapping_sub)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the wrapping subtraction of `self` and `rhs`.

In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#648)

#### pub const fn [wrapping\_mul](#method.wrapping_mul)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the wrapping multiplication of `self` and `rhs`.

In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#661)

#### pub const fn [wrapping\_div](#method.wrapping_div)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the wrapping division of `self` and `rhs`.

In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#674)

#### pub const fn [saturating\_add](#method.saturating_add)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the saturating addition of `self` and `rhs`.

In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#687)

#### pub const fn [saturating\_sub](#method.saturating_sub)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the saturating subtraction of `self` and `rhs`.

In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#700)

#### pub const fn [saturating\_mul](#method.saturating_mul)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the saturating multiplication of `self` and `rhs`.

In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#713)

#### pub const fn [saturating\_div](#method.saturating_div)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the saturating division of `self` and `rhs`.

In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#726)

#### pub const fn [checked\_add\_signed](#method.checked_add_signed)(self, rhs: [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>

Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.

In other words this computes `Some([self.x + rhs.x, self.y + rhs.y, ..])` but returns `None` on any overflow.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#748)

#### pub const fn [wrapping\_add\_signed](#method.wrapping_add_signed)(self, rhs: [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the wrapping addition of `self` and signed vector `rhs`.

In other words this computes `[self.x.wrapping_add_signed(rhs.x), self.y.wrapping_add_signed(rhs.y), ..]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#761)

#### pub const fn [saturating\_add\_signed](#method.saturating_add_signed)(self, rhs: [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a vector containing the saturating addition of `self` and signed vector `rhs`.

In other words this computes `[self.x.saturating_add_signed(rhs.x), self.y.saturating_add_signed(rhs.y), ..]`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1057)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1058)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1060)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1069)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1070)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1072)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1077)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1078)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1080)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1173)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1174)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1176)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1181)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1182)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1184)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1121)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1122)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1124)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1129)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1130)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1132)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1085)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1086)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1088)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1161)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1162)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1164)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1189)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1190)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1192)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1109)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1110)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1112)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1137)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1138)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1140)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1093)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1095)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1102)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1104)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1154)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1156)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1145)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1147)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1484)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1486)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1477)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1479)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1551)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1552)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1554)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1563)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1564)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1566)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1571)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1572)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1574)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1713)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1714)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1716)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1721)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1722)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1724)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1579)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1580)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1582)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1701)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1702)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1704)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output "type core::ops::bit::BitAnd::Output")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1729)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1730)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1732)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `&` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1587)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1589)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1594)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1596)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1744)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1746)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1737)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1739)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `&=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1601)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1602)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1604)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1613)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1614)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1616)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1621)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1622)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1624)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1763)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1764)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1766)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1771)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1774)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1629)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1630)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1632)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1751)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1752)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1754)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output "type core::ops::bit::BitOr::Output")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1779)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1780)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1782)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `|` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1637)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1639)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1644)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1646)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1794)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1796)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1787)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1789)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `|=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1651)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1652)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1654)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1663)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1664)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1666)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1671)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1672)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1674)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1813)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1814)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1816)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1821)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1822)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1824)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1679)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1680)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1682)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1801)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1802)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1804)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output "type core::ops::bit::BitXor::Output")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1829)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1830)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1832)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `^` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1687)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1689)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1694)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1696)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1844)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1846)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1837)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1839)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `^=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2826)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2827)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#770)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#772)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1228)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Deserialize expects a sequence of 3 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1228)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2820)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2821)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#777)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#778)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#780)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#789)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#790)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#792)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#797)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#798)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#800)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#893)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#894)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#896)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#901)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#902)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#904)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#841)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#842)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#844)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#849)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#850)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#852)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#805)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#806)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#808)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#881)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#882)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#884)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#909)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#910)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#912)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#829)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#830)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#832)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#857)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#858)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#860)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#813)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#815)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#822)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#824)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#874)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#876)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#865)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#867)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2864)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2"), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2866)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2"), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2850)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2852)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2976)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2978)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec3](../../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2983)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec3A](../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2985)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec3A](../../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2871)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2873)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2878)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2880)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2843)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2845)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2836)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2838)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2795)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2796)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2798)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2808)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2810)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> &mut <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#917)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#918)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#920)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#929)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#930)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#932)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#937)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#938)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#940)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1033)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1034)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1036)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1041)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1042)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1044)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#981)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#982)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#984)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#989)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#990)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#992)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#945)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#946)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#948)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1021)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1022)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1024)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1049)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1050)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1052)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#969)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#970)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#972)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#997)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#998)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1000)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#953)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#955)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#962)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#964)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1014)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1016)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1005)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1007)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1531)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1532)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1534)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1543)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1544)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1546)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the unary `!` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#24)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1511)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1513-1515)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1521)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1523-1525)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1337)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1338)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1340)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1349)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1350)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1352)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1357)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1358)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1360)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1453)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1454)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1456)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1461)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1462)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1464)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1401)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1402)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1404)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1409)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1410)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1412)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1365)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1366)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1368)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1441)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1442)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1444)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1469)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1470)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1472)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1389)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1390)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1392)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1417)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1418)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1420)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1373)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1375)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1382)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1384)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1434)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1436)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1425)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1427)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1228)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Serialize as a sequence of 3 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1228)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2663)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2664)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2666)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2671)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2672)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2674)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2735)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2736)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2738)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2743)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2744)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2746)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1863)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1864)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1866)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1871)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1872)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1874)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1963)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1964)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1966)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1971)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1972)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1974)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2063)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2064)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2066)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2071)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2072)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2074)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2163)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2164)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2166)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2171)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2172)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2174)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2263)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2264)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2266)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2271)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2272)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2274)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2363)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2364)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2366)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2371)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2372)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2374)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2463)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2464)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2466)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2471)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2472)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2474)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2563)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2564)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2566)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2571)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2572)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2574)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2651)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2652)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2654)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2679)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2680)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2682)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2723)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2724)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2726)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2751)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2752)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2754)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1851)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1852)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1854)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1879)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1880)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1882)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1951)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1952)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1954)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1979)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1980)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1982)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2051)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2052)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2054)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2079)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2080)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2082)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2151)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2152)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2154)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2179)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2180)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2182)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2251)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2252)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2254)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2279)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2280)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2282)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2351)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2352)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2354)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2379)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2380)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2382)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2451)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2452)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2454)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2479)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2480)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2482)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2551)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2552)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2554)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output "type core::ops::bit::Shl::Output")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2579)

### impl [Shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html "trait core::ops::bit::Shl")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2580)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `<<` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2582)

#### fn [shl](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `<<` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shl.html#tymethod.shl)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1894)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1896)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1994)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1996)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2094)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2096)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2194)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2196)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2294)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2296)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2394)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2396)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2494)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2496)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2594)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2596)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1887)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1889)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1987)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1989)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2087)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2089)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2187)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2189)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2287)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2289)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2387)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2389)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2487)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2489)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2587)

### impl [ShlAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html "trait core::ops::bit::ShlAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2589)

#### fn [shl\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `<<=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShlAssign.html#tymethod.shl_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2699)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2700)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2702)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2707)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2708)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2710)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2771)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2772)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2774)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2779)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2780)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2782)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1913)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1914)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1916)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1921)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1922)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1924)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2013)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2014)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2016)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2021)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2022)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2024)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2113)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2114)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2116)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2121)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2122)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2124)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2213)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2214)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2216)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2221)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2222)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2224)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2313)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2314)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2316)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2321)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2322)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2324)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2413)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2414)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2416)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2421)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2422)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2424)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2513)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2514)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2516)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2521)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2522)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2524)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2613)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2614)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2616)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2621)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2622)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2624)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2687)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2688)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2690)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2715)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2716)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2718)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2759)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2760)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2762)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2787)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2788)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2790)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1901)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1902)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1904)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1929)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1930)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1932)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2001)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2002)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2004)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2029)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2030)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2032)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2101)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2102)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2104)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2129)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2130)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2132)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2201)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2202)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2204)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2229)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2230)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2232)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2301)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2302)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2304)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2329)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2330)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2332)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2401)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2402)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2404)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2429)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2430)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2432)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2501)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2502)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2504)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2529)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2530)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2532)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2601)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2602)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2604)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output "type core::ops::bit::Shr::Output")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2629)

### impl [Shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html "trait core::ops::bit::Shr")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2630)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `>>` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2632)

#### fn [shr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)(self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `>>` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Shr.html#tymethod.shr)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1944)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1946)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2044)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2046)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2144)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2146)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2244)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2246)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2344)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2346)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2444)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2446)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2544)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2546)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2644)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<&[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2646)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: &[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1937)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1939)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2037)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2039)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2137)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2139)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2237)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2239)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2337)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2339)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2437)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2439)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2537)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2539)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2637)

### impl [ShrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html "trait core::ops::bit::ShrAssign")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2639)

#### fn [shr\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)(&mut self, rhs: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html))

Performs the `>>=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/bit/trait.ShrAssign.html#tymethod.shr_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#23)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1197)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1198)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1200)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1209)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1210)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1212)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1217)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1218)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1220)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1313)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1314)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1316)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1321)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1322)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1324)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1261)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1262)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1264)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1269)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1270)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1272)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1225)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1226)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1228)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1301)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1302)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1304)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1329)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1330)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1332)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1249)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1250)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1252)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1277)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1278)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1280)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1233)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1235)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1242)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1244)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1294)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1296)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1285)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1287)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1491)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1493-1495)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1501)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#1503-1505)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2885)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2886)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2889)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2898)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2899)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2902)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2924)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2925)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2928)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2963)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2964)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2967)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2911)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2912)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2915)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2950)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2951)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2954)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec3.rs.html#3133)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec3.rs.html#3134)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i8/i8vec3.rs.html#3137)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3"), <[I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec3.rs.html#2975)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec3.rs.html#2976)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u8/u8vec3.rs.html#2979)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3"), <[U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec3.rs.html#3121)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec3.rs.html#3122)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i16/i16vec3.rs.html#3125)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3"), <[I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec3.rs.html#2969)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec3.rs.html#2970)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u16/u16vec3.rs.html#2973)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3"), <[U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec3.rs.html#3109)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec3.rs.html#3110)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i32/ivec3.rs.html#3113)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3"), <[IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec3.rs.html#2963)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec3.rs.html#2964)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec3.rs.html#2967)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"), <[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec3.rs.html#3097)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec3.rs.html#3098)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/i64/i64vec3.rs.html#3101)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3"), <[I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec3.rs.html#2957)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec3.rs.html#2958)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u64/u64vec3.rs.html#2961)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3"), <[U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec3.rs.html#3089)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\> for [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec3.rs.html#3090)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/isize/isizevec3.rs.html#3093)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( v: [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3"), <[ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2937)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2938)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [TryFromIntError](https://doc.rust-lang.org/nightly/core/num/error/struct.TryFromIntError.html "struct core::num::error::TryFromIntError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#2941)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(v: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3"), <[USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#6)

#### type [Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#8)

#### type [Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4) = [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#11)

#### fn [xx](../../prelude/trait.Vec3Swizzles.html#tymethod.xx)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#19)

#### fn [xy](../../prelude/trait.Vec3Swizzles.html#tymethod.xy)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#27)

#### fn [with\_xy](../../prelude/trait.Vec3Swizzles.html#tymethod.with_xy)(self, rhs: [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#32)

#### fn [xz](../../prelude/trait.Vec3Swizzles.html#tymethod.xz)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#40)

#### fn [with\_xz](../../prelude/trait.Vec3Swizzles.html#tymethod.with_xz)(self, rhs: [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#45)

#### fn [yx](../../prelude/trait.Vec3Swizzles.html#tymethod.yx)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#53)

#### fn [with\_yx](../../prelude/trait.Vec3Swizzles.html#tymethod.with_yx)(self, rhs: [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#58)

#### fn [yy](../../prelude/trait.Vec3Swizzles.html#tymethod.yy)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#66)

#### fn [yz](../../prelude/trait.Vec3Swizzles.html#tymethod.yz)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#74)

#### fn [with\_yz](../../prelude/trait.Vec3Swizzles.html#tymethod.with_yz)(self, rhs: [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#79)

#### fn [zx](../../prelude/trait.Vec3Swizzles.html#tymethod.zx)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#87)

#### fn [with\_zx](../../prelude/trait.Vec3Swizzles.html#tymethod.with_zx)(self, rhs: [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#92)

#### fn [zy](../../prelude/trait.Vec3Swizzles.html#tymethod.zy)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#100)

#### fn [with\_zy](../../prelude/trait.Vec3Swizzles.html#tymethod.with_zy)(self, rhs: [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#105)

#### fn [zz](../../prelude/trait.Vec3Swizzles.html#tymethod.zz)(self) -> [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#113)

#### fn [xxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#118)

#### fn [xxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#123)

#### fn [xxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#128)

#### fn [xyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#133)

#### fn [xyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#138)

#### fn [xzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#143)

#### fn [xzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#148)

#### fn [xzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#153)

#### fn [yxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#158)

#### fn [yxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#163)

#### fn [yxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#168)

#### fn [yyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#173)

#### fn [yyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#178)

#### fn [yyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#183)

#### fn [yzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#188)

#### fn [yzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#193)

#### fn [yzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#198)

#### fn [zxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#203)

#### fn [zxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#208)

#### fn [zxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#213)

#### fn [zyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#218)

#### fn [zyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#223)

#### fn [zyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#228)

#### fn [zzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzx)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#233)

#### fn [zzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzy)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#238)

#### fn [zzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzz)(self) -> [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#243)

#### fn [xxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#248)

#### fn [xxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#253)

#### fn [xxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#258)

#### fn [xxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#263)

#### fn [xxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#268)

#### fn [xxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#273)

#### fn [xxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#278)

#### fn [xxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#283)

#### fn [xxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xxzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#288)

#### fn [xyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#293)

#### fn [xyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#298)

#### fn [xyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#303)

#### fn [xyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#308)

#### fn [xyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#313)

#### fn [xyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#318)

#### fn [xyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#323)

#### fn [xyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#328)

#### fn [xyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xyzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#333)

#### fn [xzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#338)

#### fn [xzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#343)

#### fn [xzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#348)

#### fn [xzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#353)

#### fn [xzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#358)

#### fn [xzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#363)

#### fn [xzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#368)

#### fn [xzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#373)

#### fn [xzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.xzzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#378)

#### fn [yxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#383)

#### fn [yxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#388)

#### fn [yxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#393)

#### fn [yxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#398)

#### fn [yxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#403)

#### fn [yxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#408)

#### fn [yxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#413)

#### fn [yxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#418)

#### fn [yxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yxzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#423)

#### fn [yyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#428)

#### fn [yyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#433)

#### fn [yyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#438)

#### fn [yyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#443)

#### fn [yyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#448)

#### fn [yyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#453)

#### fn [yyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#458)

#### fn [yyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#463)

#### fn [yyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yyzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#468)

#### fn [yzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#473)

#### fn [yzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#478)

#### fn [yzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#483)

#### fn [yzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#488)

#### fn [yzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#493)

#### fn [yzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#498)

#### fn [yzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#503)

#### fn [yzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#508)

#### fn [yzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.yzzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#513)

#### fn [zxxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#518)

#### fn [zxxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#523)

#### fn [zxxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#528)

#### fn [zxyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#533)

#### fn [zxyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#538)

#### fn [zxyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#543)

#### fn [zxzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#548)

#### fn [zxzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#553)

#### fn [zxzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zxzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#558)

#### fn [zyxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#563)

#### fn [zyxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#568)

#### fn [zyxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#573)

#### fn [zyyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#578)

#### fn [zyyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#583)

#### fn [zyyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#588)

#### fn [zyzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#593)

#### fn [zyzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#598)

#### fn [zyzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zyzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#603)

#### fn [zzxx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#608)

#### fn [zzxy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#613)

#### fn [zzxz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzxz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#618)

#### fn [zzyx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#623)

#### fn [zzyy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#628)

#### fn [zzyz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzyz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#633)

#### fn [zzzx](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzx)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#638)

#### fn [zzzy](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzy)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#643)

#### fn [zzzz](../../prelude/trait.Vec3Swizzles.html#tymethod.zzzz)(self) -> [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#103)

#### fn [xyz](../../prelude/trait.Vec3Swizzles.html#method.xyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/usize/usizevec3.rs.html#24)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../../ecs/label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../../ecs/label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}