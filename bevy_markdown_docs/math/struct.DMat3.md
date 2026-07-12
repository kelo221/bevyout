[bevy](../index.html)::[math](index.html)

# Struct DMat3 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#54)

```rust
#[repr(C)]pub struct DMat3 {
    pub x_axis: DVec3,
    pub y_axis: DVec3,
    pub z_axis: DVec3,
}
```

A 3x3 column major matrix.

This 3x3 matrix type features convenience methods for creating and using linear and affine transformations. If you are primarily dealing with 2D affine transformations the [`DAffine2`](struct.DAffine2.html "struct bevy::math::DAffine2") type is much faster and more space efficient than using a 3x3 matrix.

Linear transformations including 3D rotation and scale can be created using methods such as [`Self::from_diagonal()`](struct.DMat3.html#method.from_diagonal "associated function bevy::math::DMat3::from_diagonal"), [`Self::from_quat()`](struct.DMat3.html#method.from_quat "associated function bevy::math::DMat3::from_quat"), [`Self::from_axis_angle()`](struct.DMat3.html#method.from_axis_angle "associated function bevy::math::DMat3::from_axis_angle"), [`Self::from_rotation_x()`](struct.DMat3.html#method.from_rotation_x "associated function bevy::math::DMat3::from_rotation_x"), [`Self::from_rotation_y()`](struct.DMat3.html#method.from_rotation_y "associated function bevy::math::DMat3::from_rotation_y"), or [`Self::from_rotation_z()`](struct.DMat3.html#method.from_rotation_z "associated function bevy::math::DMat3::from_rotation_z").

The resulting matrices can be use to transform 3D vectors using regular vector multiplication.

Affine transformations including 2D translation, rotation and scale can be created using methods such as [`Self::from_translation()`](struct.DMat3.html#method.from_translation "associated function bevy::math::DMat3::from_translation"), [`Self::from_angle()`](struct.DMat3.html#method.from_angle "associated function bevy::math::DMat3::from_angle"), [`Self::from_scale()`](struct.DMat3.html#method.from_scale "associated function bevy::math::DMat3::from_scale") and [`Self::from_scale_angle_translation()`](struct.DMat3.html#method.from_scale_angle_translation "associated function bevy::math::DMat3::from_scale_angle_translation").

The [`Self::transform_point2()`](struct.DMat3.html#method.transform_point2 "method bevy::math::DMat3::transform_point2") and [`Self::transform_vector2()`](struct.DMat3.html#method.transform_vector2 "method bevy::math::DMat3::transform_vector2") convenience methods are provided for performing affine transforms on 2D vectors and points. These multiply 2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for vectors respectively. These methods assume that `Self` contains a valid affine transform.

## Fields

`x_axis: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")``y_axis: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")``z_axis: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#60)

### impl [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#62)

#### pub const [ZERO](#associatedconstant.ZERO): [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

A 3x3 matrix with all elements set to `0.0`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#65)

#### pub const [IDENTITY](#associatedconstant.IDENTITY): [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

A 3x3 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#68)

#### pub const [NAN](#associatedconstant.NAN): [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

All NAN:s.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#94)

#### pub const fn [from\_cols](#method.from_cols)(x\_axis: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), y\_axis: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), z\_axis: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3x3 matrix from three column vectors.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#107)

#### pub const fn [from\_cols\_array](#method.from_cols_array)(m: &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3x3 matrix from a `[f64; 9]` array stored in column major order. If your data is stored in row major you will need to `transpose` the returned matrix.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#115)

#### pub const fn [to\_cols\_array](#method.to_cols_array)(&self) -> \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Creates a `[f64; 9]` array storing data in column major order. If you require data in row major order `transpose` the matrix first.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#134)

#### pub const fn [from\_cols\_array\_2d](#method.from_cols_array_2d)(m: &\[\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3x3 matrix from a `[[f64; 3]; 3]` 3D array stored in column major order. If your data is in row major order you will need to `transpose` the returned matrix.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#146)

#### pub const fn [to\_cols\_array\_2d](#method.to_cols_array_2d)(&self) -> \[\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Creates a `[[f64; 3]; 3]` 3D array storing data in column major order. If you require data in row major order `transpose` the matrix first.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#158)

#### pub const fn [from\_diagonal](#method.from_diagonal)(diagonal: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#167)

#### pub fn [from\_mat4](#method.from_mat4)(m: [DMat4](struct.DMat4.html "struct bevy::math::DMat4")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#183)

#### pub fn [from\_mat4\_minor](#method.from_mat4_minor)(m: [DMat4](struct.DMat4.html "struct bevy::math::DMat4"), i: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), j: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3x3 matrix from the minor of the given 4x4 matrix, discarding the `i`th column and `j`th row.

##### Panics

Panics if `i` or `j` is greater than 3.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#212)

#### pub fn [from\_quat](#method.from_quat)(rotation: [DQuat](struct.DQuat.html "struct bevy::math::DQuat")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3D rotation matrix from the given quaternion.

##### Panics

Will panic if `rotation` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#243)

#### pub fn [from\_axis\_angle](#method.from_axis_angle)(axis: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in radians).

##### Panics

Will panic if `axis` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#265)

#### pub fn [from\_euler](#method.from_euler)(order: [EulerRot](../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot"), a: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), b: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), c: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in radians).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#279)

#### pub fn [to\_euler](#method.to_euler)(&self, order: [EulerRot](../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")) -> ([f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Extract Euler angles with the given Euler rotation order.

Note if the input matrix contains scales, shears, or other non-rotation transformations then the resulting Euler angles will be ill-defined.

##### Panics

Will panic if any input matrix column is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#291)

#### pub fn [from\_rotation\_x](#method.from_rotation_x)(angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#303)

#### pub fn [from\_rotation\_y](#method.from_rotation_y)(angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#315)

#### pub fn [from\_rotation\_z](#method.from_rotation_z)(angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#330)

#### pub fn [from\_translation](#method.from_translation)(translation: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates an affine transformation matrix from the given 2D `translation`.

The resulting matrix can be used to transform 2D points and vectors. See [`Self::transform_point2()`](struct.DMat3.html#method.transform_point2 "method bevy::math::DMat3::transform_point2") and [`Self::transform_vector2()`](struct.DMat3.html#method.transform_vector2 "method bevy::math::DMat3::transform_vector2").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#345)

#### pub fn [from\_angle](#method.from_angle)(angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates an affine transformation matrix from the given 2D rotation `angle` (in radians).

The resulting matrix can be used to transform 2D points and vectors. See [`Self::transform_point2()`](struct.DMat3.html#method.transform_point2 "method bevy::math::DMat3::transform_point2") and [`Self::transform_vector2()`](struct.DMat3.html#method.transform_vector2 "method bevy::math::DMat3::transform_vector2").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#361)

#### pub fn [from\_scale\_angle\_translation](#method.from_scale_angle_translation)( scale: [DVec2](struct.DVec2.html "struct bevy::math::DVec2"), angle: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html), translation: [DVec2](struct.DVec2.html "struct bevy::math::DVec2"), ) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in radians) and `translation`.

The resulting matrix can be used to transform 2D points and vectors. See [`Self::transform_point2()`](struct.DMat3.html#method.transform_point2 "method bevy::math::DMat3::transform_point2") and [`Self::transform_vector2()`](struct.DMat3.html#method.transform_vector2 "method bevy::math::DMat3::transform_vector2").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#380)

#### pub fn [from\_scale](#method.from_scale)(scale: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates an affine transformation matrix from the given non-uniform 2D `scale`.

The resulting matrix can be used to transform 2D points and vectors. See [`Self::transform_point2()`](struct.DMat3.html#method.transform_point2 "method bevy::math::DMat3::transform_point2") and [`Self::transform_vector2()`](struct.DMat3.html#method.transform_vector2 "method bevy::math::DMat3::transform_vector2").

##### Panics

Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#396)

#### pub fn [from\_mat2](#method.from_mat2)(m: [DMat2](struct.DMat2.html "struct bevy::math::DMat2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates an affine transformation matrix from the given 2x2 matrix.

The resulting matrix can be used to transform 2D points and vectors. See [`Self::transform_point2()`](struct.DMat3.html#method.transform_point2 "method bevy::math::DMat3::transform_point2") and [`Self::transform_vector2()`](struct.DMat3.html#method.transform_vector2 "method bevy::math::DMat3::transform_vector2").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#407)

#### pub const fn [from\_cols\_slice](#method.from_cols_slice)(slice: &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\]) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a 3x3 matrix from the first 9 values in `slice`.

##### Panics

Panics if `slice` is less than 9 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#420)

#### pub fn [write\_cols\_to\_slice](#method.write_cols_to_slice)(&self, slice: &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\])

Writes the columns of `self` to the first 9 elements in `slice`.

##### Panics

Panics if `slice` is less than 9 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#439)

#### pub fn [col](#method.col)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Returns the matrix column for the given `index`.

##### Panics

Panics if `index` is greater than 2.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#454)

#### pub fn [col\_mut](#method.col_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Returns a mutable reference to the matrix column for the given `index`.

##### Panics

Panics if `index` is greater than 2.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#470)

#### pub fn [row](#method.row)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Returns the matrix row for the given `index`.

##### Panics

Panics if `index` is greater than 2.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#483)

#### pub fn [is\_finite](#method.is_finite)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#490)

#### pub fn [is\_nan](#method.is_nan)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#497)

#### pub fn [transpose](#method.transpose)(&self) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Returns the transpose of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#508)

#### pub fn [diagonal](#method.diagonal)(&self) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Returns the diagonal of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#515)

#### pub fn [determinant](#method.determinant)(&self) -> [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

Returns the determinant of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#560)

#### pub fn [inverse](#method.inverse)(&self) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Returns the inverse of `self`.

If the matrix is not invertible the returned matrix will be invalid.

##### Panics

Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#567)

#### pub fn [try\_inverse](#method.try_inverse)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>

Returns the inverse of `self` or `None` if the matrix is not invertible.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#579)

#### pub fn [inverse\_or\_zero](#method.inverse_or_zero)(&self) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Returns the inverse of `self` or `DMat3::ZERO` if the matrix is not invertible.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#594)

#### pub fn [transform\_point2](#method.transform_point2)(&self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

Transforms the given 2D vector as a point.

This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.

This method assumes that `self` contains a valid affine transform.

##### Panics

Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#610)

#### pub fn [transform\_vector2](#method.transform_vector2)(&self, rhs: [DVec2](struct.DVec2.html "struct bevy::math::DVec2")) -> [DVec2](struct.DVec2.html "struct bevy::math::DVec2")

Rotates the given 2D vector.

This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.

This method assumes that `self` contains a valid affine transform.

##### Panics

Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#624)

#### pub fn [look\_to\_lh](#method.look_to_lh)(dir: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), up: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a left-handed view matrix using a facing direction and an up direction.

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

##### Panics

Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#637)

#### pub fn [look\_to\_rh](#method.look_to_rh)(dir: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), up: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a right-handed view matrix using a facing direction and an up direction.

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

##### Panics

Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#661)

#### pub fn [look\_at\_lh](#method.look_at_lh)(eye: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), center: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), up: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a left-handed view matrix using a camera position, a focal point and an up direction.

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

##### Panics

Will panic if `up` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#674)

#### pub fn [look\_at\_rh](#method.look_at_rh)(eye: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), center: [DVec3](struct.DVec3.html "struct bevy::math::DVec3"), up: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Creates a right-handed view matrix using a camera position, a focal point and an up direction.

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

##### Panics

Will panic if `up` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#681)

#### pub fn [mul\_vec3](#method.mul_vec3)(&self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Transforms a 3D vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#691)

#### pub fn [mul\_transpose\_vec3](#method.mul_transpose_vec3)(&self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Transforms a 3D vector by the transpose of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#702)

#### pub fn [mul\_mat3](#method.mul_mat3)(&self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Multiplies two 3x3 matrices.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#709)

#### pub fn [add\_mat3](#method.add_mat3)(&self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Adds two 3x3 matrices.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#716)

#### pub fn [sub\_mat3](#method.sub_mat3)(&self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Subtracts two 3x3 matrices.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#723)

#### pub fn [mul\_scalar](#method.mul_scalar)(&self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Multiplies a 3x3 matrix by a scalar.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#736)

#### pub fn [mul\_diagonal\_scale](#method.mul_diagonal_scale)(&self, scale: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Multiply `self` by a scaling vector `scale`. This is faster than creating a whole diagonal scaling matrix and then multiplying that. This operation is commutative.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#747)

#### pub fn [div\_scalar](#method.div_scalar)(&self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Divides a 3x3 matrix by a scalar.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#759)

#### pub fn [recip](#method.recip)(&self) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Returns a matrix containing the reciprocal `1.0/n` of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#778)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(&self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3"), max\_abs\_diff: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two matrices contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#787)

#### pub fn [abs](#method.abs)(&self) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Takes the absolute value of each element in `self`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#792)

#### pub fn [as\_mat3](#method.as_mat3)(&self) -> [Mat3](../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#808)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#809)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#811)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#820)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#821)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#823)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#828)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#829)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#831)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#836)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#837)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#839)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#844)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#846)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#851)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#853)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1216)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1218)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1209)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1211)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [9](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#47)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#47)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#47)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1223)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1224)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#801)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#803)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Deserialize expects a sequence of 9 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1233)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1234)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1128)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1129)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1131)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1136)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1137)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1139)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1120)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1121)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1123)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1144)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1145)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1147)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1159)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1161)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1152)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1154)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#437)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#439)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(m: [DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[DMat3](struct.DMat3.html "struct bevy::math::DMat3") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [DMat3](struct.DMat3.html "struct bevy::math::DMat3"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#924)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#925)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#927)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#490)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#491)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#493)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#498)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#499)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#501)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#457)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#458)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#460)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#465)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#466)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#468)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#936)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#937)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#939)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#944)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#945)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#947)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#982)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](struct.DVec3.html "struct bevy::math::DVec3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#983)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#985)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#990)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[DVec3](struct.DVec3.html "struct bevy::math::DVec3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#991)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#993)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1046)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1047)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1049)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1054)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1055)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1057)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#481)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#482)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#485)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#506)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#507)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#509)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#448)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#449)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#452)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> <[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#473)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#474)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#476)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#952)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#953)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#955)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#974)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](struct.DVec3.html "struct bevy::math::DVec3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#975)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#977)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> <[DMat3](struct.DMat3.html "struct bevy::math::DMat3") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](struct.DVec3.html "struct bevy::math::DVec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#998)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[DVec3](struct.DVec3.html "struct bevy::math::DVec3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#999)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1001)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [DVec3](struct.DVec3.html "struct bevy::math::DVec3")) -> [DVec3](struct.DVec3.html "struct bevy::math::DVec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1038)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1039)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1041)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1062)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1063)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1065)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#960)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#962)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#521)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#523)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#967)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#969)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1077)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1079)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#514)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/daffine2.rs.html#516)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [DAffine2](struct.DAffine2.html "struct bevy::math::DAffine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1070)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1072)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#908)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#909)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#911)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> <[DMat3](struct.DMat3.html "struct bevy::math::DMat3") as [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output "type core::ops::arith::Neg::Output")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#916)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#917)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#919)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1202)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1204)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#389)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#389)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#389)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#48)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1184)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1185-1187)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1193)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1194-1196)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Serialize as a sequence of 9 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1076-1078)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#858)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#859)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#861)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#870)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#871)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#873)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#878)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#879)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#881)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#886)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for &[DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#887)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#889)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3")) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#894)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#896)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [DMat3](struct.DMat3.html "struct bevy::math::DMat3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#901)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#903)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[DMat3](struct.DMat3.html "struct bevy::math::DMat3"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1166)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1167-1169)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1175)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [DMat3](struct.DMat3.html "struct bevy::math::DMat3")\> for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#1176-1178)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [DMat3](struct.DMat3.html "struct bevy::math::DMat3")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dmat3.rs.html#48)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [DMat3](struct.DMat3.html "struct bevy::math::DMat3")

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}