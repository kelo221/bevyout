[bevy](../index.html)::[math](index.html)

# Struct Vec4 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#42)

```rust
pub struct Vec4(/* private fields */);
```

A 4-dimensional vector.

SIMD vector types are used for storage on supported platforms.

This type is 16 byte aligned.

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#44)

### impl [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#46)

#### pub const [ZERO](#associatedconstant.ZERO): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All zeroes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#49)

#### pub const [ONE](#associatedconstant.ONE): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#52)

#### pub const [NEG\_ONE](#associatedconstant.NEG_ONE): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All negative ones.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#55)

#### pub const [MIN](#associatedconstant.MIN): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All `f32::MIN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#58)

#### pub const [MAX](#associatedconstant.MAX): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All `f32::MAX`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#61)

#### pub const [NAN](#associatedconstant.NAN): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All `f32::NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#64)

#### pub const [INFINITY](#associatedconstant.INFINITY): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All `f32::INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#67)

#### pub const [NEG\_INFINITY](#associatedconstant.NEG_INFINITY): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

All `f32::NEG_INFINITY`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#70)

#### pub const [X](#associatedconstant.X): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the positive X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#73)

#### pub const [Y](#associatedconstant.Y): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the positive Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#76)

#### pub const [Z](#associatedconstant.Z): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the positive Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#79)

#### pub const [W](#associatedconstant.W): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the positive W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#82)

#### pub const [NEG\_X](#associatedconstant.NEG_X): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the negative X axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#85)

#### pub const [NEG\_Y](#associatedconstant.NEG_Y): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the negative Y axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#88)

#### pub const [NEG\_Z](#associatedconstant.NEG_Z): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the negative Z axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#91)

#### pub const [NEG\_W](#associatedconstant.NEG_W): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

A unit vector pointing along the negative W axis.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#94)

#### pub const [AXES](#associatedconstant.AXES): \[[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

The unit axes.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#97)

#### pub const [USES\_CORE\_SIMD](#associatedconstant.USES_CORE_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec4 uses Rust Portable SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#99)

#### pub const [USES\_NEON](#associatedconstant.USES_NEON): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec4 uses Arm NEON

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#101)

#### pub const [USES\_SCALAR\_MATH](#associatedconstant.USES_SCALAR_MATH): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec4 uses scalar math

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#103)

#### pub const [USES\_SSE2](#associatedconstant.USES_SSE2): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

Vec4 uses Intel SSE2

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#105)

#### pub const [USES\_WASM\_SIMD](#associatedconstant.USES_WASM_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

Vec4 uses WebAssembly 128-bit SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#107)

#### pub const [USES\_WASM32\_SIMD](#associatedconstant.USES_WASM32_SIMD): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

👎Deprecated since 0.31.0:

Renamed to USES\_WASM\_SIMD

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#112)

#### pub const fn [new](#method.new)(x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), w: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a new vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#119)

#### pub const fn [splat](#method.splat)(v: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a vector with all elements set to `v`.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/ui/ui\_material.rs ([line 49](../../src/ui_material/ui_material.rs.html#49))

```rust
20fn setup(
21    mut commands: Commands,
22    mut ui_materials: ResMut<Assets<CustomUiMaterial>>,
23    asset_server: Res<AssetServer>,
24) {
25    // Camera so we can see UI
26    commands.spawn(Camera2d);
27
28    commands
29        .spawn(Node {
30            width: percent(100),
31            height: percent(100),
32            align_items: AlignItems::Center,
33            justify_content: JustifyContent::Center,
34            ..default()
35        })
36        .with_children(|parent| {
37            let banner_scale_factor = 0.5;
38            parent.spawn((
39                Node {
40                    position_type: PositionType::Absolute,
41                    width: px(905.0 * banner_scale_factor),
42                    height: px(363.0 * banner_scale_factor),
43                    border: UiRect::all(px(20)),
44                    border_radius: BorderRadius::all(px(20)),
45                    ..default()
46                },
47                MaterialNode(ui_materials.add(CustomUiMaterial {
48                    color: LinearRgba::WHITE.to_f32_array().into(),
49                    slider: Vec4::splat(0.5),
50                    color_texture: asset_server.load("branding/banner.png"),
51                    border_color: LinearRgba::WHITE.to_f32_array().into(),
52                })),
53                // UI material nodes can have outlines and shadows like any other UI node
54                Outline {
55                    width: px(2),
56                    offset: px(100),
57                    color: DARK_BLUE.into(),
58                },
59            ));
60        });
61}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#126-128)

#### pub fn [map](#method.map)<F>(self, f: F) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html),

Returns a vector containing each element of `self` modified by a mapping function `f`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#140)

#### pub fn [select](#method.select)(mask: [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A"), if\_true: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), if\_false: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a vector from the elements in `if_true` and `if_false`, selecting which to use for each element of `self`.

A true element in the mask uses the corresponding element from `if_true`, and false uses the element from `if_false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#152)

#### pub const fn [from\_array](#method.from_array)(a: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a new vector from an array.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#159)

#### pub const fn [to\_array](#method.to_array)(&self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts `self` to `[x, y, z, w]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#170)

#### pub const fn [from\_slice](#method.from_slice)(slice: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a vector from the first 4 values in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#181)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\])

Writes the elements of `self` to the first 4 elements in `slice`.

##### Panics

Panics if `slice` is less than 4 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#195)

#### pub fn [truncate](#method.truncate)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.

Truncation to [`Vec3`](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") may also be performed by using [`self.xyz()`](../prelude/trait.Vec4Swizzles.html#tymethod.xyz "method bevy::prelude::Vec4Swizzles::xyz").

To truncate to [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") use [`Vec3A::from_vec4()`](../prelude/struct.Vec3A.html#method.from_vec4 "associated function bevy::prelude::Vec3A::from_vec4").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#209)

#### pub fn [project](#method.project)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Projects a homogeneous coordinate to 3D space by performing perspective divide.

To project to [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") use [`Vec3A::from_homogeneous()`](../prelude/struct.Vec3A.html#method.from_homogeneous "associated function bevy::prelude::Vec3A::from_homogeneous").

##### Panics

Will panic if `self.w` is `0` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#216)

#### pub fn [with\_x](#method.with_x)(self, x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a 4D vector from `self` with the given value of `x`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#224)

#### pub fn [with\_y](#method.with_y)(self, y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a 4D vector from `self` with the given value of `y`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#232)

#### pub fn [with\_z](#method.with_z)(self, z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a 4D vector from `self` with the given value of `z`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#240)

#### pub fn [with\_w](#method.with_w)(self, w: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Creates a 4D vector from `self` with the given value of `w`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#248)

#### pub fn [dot](#method.dot)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#255)

#### pub fn [dot\_into\_vec](#method.dot_into_vec)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector where every component is the dot product of `self` and `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#267)

#### pub fn [min](#method.min)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the minimum values for each element of `self` and `rhs`.

In other words this computes `[min(x, rhs.x), min(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for minNum and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#279)

#### pub fn [max](#method.max)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the maximum values for each element of `self` and `rhs`.

In other words this computes `[max(self.x, rhs.x), max(self.y, rhs.y), ..]`.

NaN propogation does not follow IEEE 754-2008 semantics for maxNum and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#295)

#### pub fn [clamp](#method.clamp)(self, min: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), max: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Component-wise clamping of values, similar to [`f32::clamp`](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.clamp "method f32::clamp").

Each element in `min` must be less-or-equal to the corresponding element in `max`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

##### Panics

Will panic if `min` is greater than `max` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#308)

#### pub fn [min\_element](#method.min_element)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the horizontal minimum of `self`.

In other words this computes `min(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#325)

#### pub fn [max\_element](#method.max_element)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the horizontal maximum of `self`.

In other words this computes `max(x, y, ..)`.

NaN propogation does not follow IEEE 754-2008 semantics and may differ on different SIMD architectures.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#338)

#### pub fn [min\_position](#method.min_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first minimum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#359)

#### pub fn [max\_position](#method.max_position)(self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the index of the first maximum element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#381)

#### pub fn [element\_sum](#method.element_sum)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the sum of all elements of `self`.

In other words, this computes `self.x + self.y + ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#395)

#### pub fn [element\_product](#method.element_product)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the product of all elements of `self`.

In other words, this computes `self.x * self.y * ..`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#411)

#### pub fn [cmpeq](#method.cmpeq)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Returns a vector mask containing the result of a `==` comparison for each element of `self` and `rhs`.

In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#422)

#### pub fn [cmpne](#method.cmpne)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Returns a vector mask containing the result of a `!=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#433)

#### pub fn [cmpge](#method.cmpge)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Returns a vector mask containing the result of a `>=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#444)

#### pub fn [cmpgt](#method.cmpgt)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Returns a vector mask containing the result of a `>` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#455)

#### pub fn [cmple](#method.cmple)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Returns a vector mask containing the result of a `<=` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#466)

#### pub fn [cmplt](#method.cmplt)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Returns a vector mask containing the result of a `<` comparison for each element of `self` and `rhs`.

In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all elements.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#473)

#### pub fn [abs](#method.abs)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the absolute value of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#484)

#### pub fn [signum](#method.signum)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector with elements representing the sign of `self`.

*   `1.0` if the number is positive, `+0.0` or `INFINITY`
*   `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
*   `NAN` if the number is `NAN`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#493)

#### pub fn [copysign](#method.copysign)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector with signs of `rhs` and the magnitudes of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#507)

#### pub fn [is\_negative\_bitmask](#method.is_negative_bitmask)(self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.

A negative element results in a `1` bit and a positive element in a `0` bit. Element `x` goes into the first lowest bit, element `y` into the second, etc.

An element is negative if it has a negative sign, including -0.0, NaNs with negative sign bit and negative infinity.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#515)

#### pub fn [is\_finite](#method.is_finite)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#524)

#### pub fn [is\_finite\_mask](#method.is_finite_mask)(self) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Performs `is_finite` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_finite(), y.is_finite(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#531)

#### pub fn [is\_nan](#method.is_nan)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#540)

#### pub fn [is\_nan\_mask](#method.is_nan_mask)(self) -> [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

Performs `is_nan` on each element of self, returning a vector mask of the results.

In other words, this computes `[x.is_nan(), y.is_nan(), ...]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#548)

#### pub fn [length](#method.length)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#561)

#### pub fn [length\_squared](#method.length_squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the squared length of `self`.

This is faster than `length()` as it avoids a square root operation.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#570)

#### pub fn [length\_recip](#method.length_recip)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes `1.0 / length()`.

For valid results, `self` must _not_ be of length zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#580)

#### pub fn [distance](#method.distance)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the Euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#587)

#### pub fn [distance\_squared](#method.distance_squared)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Compute the squared euclidean distance between two points in space.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#594)

#### pub fn [div\_euclid](#method.div_euclid)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the element-wise quotient of \[Euclidean division\] of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#608)

#### pub fn [rem\_euclid](#method.rem_euclid)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the element-wise remainder of [Euclidean division](https://doc.rust-lang.org/nightly/std/primitive.f32.html#method.rem_euclid "method f32::rem_euclid") of `self` by `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#628)

#### pub fn [normalize](#method.normalize)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns `self` normalized to length 1.0.

For valid results, `self` must be finite and _not_ of length zero, nor very close to zero.

See also [`Self::try_normalize()`](../prelude/struct.Vec4.html#method.try_normalize "method bevy::prelude::Vec4::try_normalize") and [`Self::normalize_or_zero()`](../prelude/struct.Vec4.html#method.normalize_or_zero "method bevy::prelude::Vec4::normalize_or_zero").

##### Panics

Will panic if the resulting normalized vector is not finite when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#646)

#### pub fn [try\_normalize](#method.try_normalize)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>

Returns `self` normalized to length 1.0 if possible, else returns `None`.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be `None`.

See also [`Self::normalize_or_zero()`](../prelude/struct.Vec4.html#method.normalize_or_zero "method bevy::prelude::Vec4::normalize_or_zero").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#664)

#### pub fn [normalize\_or](#method.normalize_or)(self, fallback: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns `self` normalized to length 1.0 if possible, else returns a fallback value.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be the fallback value.

See also [`Self::try_normalize()`](../prelude/struct.Vec4.html#method.try_normalize "method bevy::prelude::Vec4::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#681)

#### pub fn [normalize\_or\_zero](#method.normalize_or_zero)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns `self` normalized to length 1.0 if possible, else returns zero.

In particular, if the input is zero (or very close to zero), or non-finite, the result of this operation will be zero.

See also [`Self::try_normalize()`](../prelude/struct.Vec4.html#method.try_normalize "method bevy::prelude::Vec4::try_normalize").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#690)

#### pub fn [normalize\_and\_length](#method.normalize_and_length)(self) -> ([Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Returns `self` normalized to length 1.0 and the length of `self`.

If `self` is zero length then `(Self::X, 0.0)` is returned.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#705)

#### pub fn [is\_normalized](#method.is_normalized)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether `self` is length `1.0` or not.

Uses a precision threshold of approximately `1e-4`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#718)

#### pub fn [project\_onto](#method.project_onto)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` is zero length when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#737)

#### pub fn [reject\_from](#method.reject_from)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be of non-zero length.

##### Panics

Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#750)

#### pub fn [project\_onto\_normalized](#method.project_onto_normalized)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the vector projection of `self` onto `rhs`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#768)

#### pub fn [reject\_from\_normalized](#method.reject_from_normalized)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the vector rejection of `self` from `rhs`.

The vector rejection is the vector perpendicular to the projection of `self` onto `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.

`rhs` must be normalized.

##### Panics

Will panic if `rhs` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#776)

#### pub fn [round](#method.round)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the nearest integer to a number for each element of `self`. Round half-way cases away from 0.0.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#784)

#### pub fn [floor](#method.floor)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the largest integer less than or equal to a number for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#792)

#### pub fn [ceil](#method.ceil)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the smallest integer greater than or equal to a number for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#800)

#### pub fn [trunc](#method.trunc)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the integer part each element of `self`. This means numbers are always truncated towards zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#809)

#### pub fn [step](#method.step)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing `0.0` if `rhs < self` and 1.0 otherwise.

Similar to glsl’s step(edge, x), which translates into edge.step(x)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#816)

#### pub fn [saturate](#method.saturate)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing all elements of `self` clamped to the range of `[0, 1]`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#828)

#### pub fn [fract](#method.fract)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the fractional part of the vector as `self - self.trunc()`.

Note that this differs from the GLSL implementation of `fract` which returns `self - self.floor()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#840)

#### pub fn [fract\_gl](#method.fract_gl)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the fractional part of the vector as `self - self.floor()`.

Note that this differs from the Rust implementation of `fract` which returns `self - self.trunc()`.

Note that this is fast but not precise for large numbers.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#848)

#### pub fn [exp](#method.exp)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing `e^self` (the exponential function) for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#860)

#### pub fn [exp2](#method.exp2)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing `2^self` for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#873)

#### pub fn [ln](#method.ln)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the natural logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#886)

#### pub fn [log2](#method.log2)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the base 2 logarithm for each element of `self`. This returns NaN when the element is negative and negative infinity when the element is zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#898)

#### pub fn [powf](#method.powf)(self, n: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing each element of `self` raised to the power of `n`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#911)

#### pub fn [sqrt](#method.sqrt)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the square root for each element of `self`. This returns NaN when the element is negative.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#923)

#### pub fn [cos](#method.cos)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#935)

#### pub fn [sin](#method.sin)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the sine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#947)

#### pub fn [sin\_cos](#method.sin_cos)(self) -> ([Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Returns a tuple of two vectors containing the sine and cosine for each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#962)

#### pub fn [recip](#method.recip)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#974)

#### pub fn [lerp](#method.lerp)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), s: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs a linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly extrapolated.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#984)

#### pub fn [move\_towards](#method.move_towards)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), d: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Moves towards `rhs` based on the value `d`.

When `d` is `0.0`, the result will be equal to `self`. When `d` is equal to `self.distance(rhs)`, the result will be equal to `rhs`. Will not go past `rhs`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#999)

#### pub fn [midpoint](#method.midpoint)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Calculates the midpoint between `self` and `rhs`.

The midpoint is the average of, or halfway point between, two vectors. `a.midpoint(b)` should yield the same result as `a.lerp(b, 0.5)` while being slightly cheaper to compute.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1014)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), max\_abs\_diff: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two vectors contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1025)

#### pub fn [clamp\_length](#method.clamp_length)(self, min: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), max: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector with a length no less than `min` and no more than `max`.

##### Panics

Will panic if `min` is greater than `max`, or if either `min` or `max` is negative, when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1045)

#### pub fn [clamp\_length\_max](#method.clamp_length_max)(self, max: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector with a length no more than `max`.

##### Panics

Will panic if `max` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1062)

#### pub fn [clamp\_length\_min](#method.clamp_length_min)(self, min: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a vector with a length no less than `min`.

##### Panics

Will panic if `min` is negative when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1081)

#### pub fn [mul\_add](#method.mul_add)(self, a: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), b: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding error, yielding a more accurate result than an unfused multiply-add.

Using `mul_add` _may_ be more performant than an unfused multiply-add if the target architecture has a dedicated fma CPU instruction. However, this is not always true, and will be heavily dependant on designing algorithms with specific target hardware in mind.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1105)

#### pub fn [reflect](#method.reflect)(self, normal: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the reflection vector for a given incident vector `self` and surface normal `normal`.

`normal` must be normalized.

##### Panics

Will panic if `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1121)

#### pub fn [refract](#method.refract)(self, normal: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), eta: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the refraction direction for a given incident vector `self`, surface normal `normal` and ratio of indices of refraction, `eta`. When total internal reflection occurs, a zero vector will be returned.

`self` and `normal` must be normalized.

##### Panics

Will panic if `self` or `normal` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1136)

#### pub fn [as\_dvec4](#method.as_dvec4)(self) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Casts all elements of `self` to `f64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1143)

#### pub fn [as\_i8vec4](#method.as_i8vec4)(self) -> [I8Vec4](struct.I8Vec4.html "struct bevy::math::I8Vec4")

Casts all elements of `self` to `i8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1150)

#### pub fn [as\_u8vec4](#method.as_u8vec4)(self) -> [U8Vec4](struct.U8Vec4.html "struct bevy::math::U8Vec4")

Casts all elements of `self` to `u8`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1157)

#### pub fn [as\_i16vec4](#method.as_i16vec4)(self) -> [I16Vec4](struct.I16Vec4.html "struct bevy::math::I16Vec4")

Casts all elements of `self` to `i16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1164)

#### pub fn [as\_u16vec4](#method.as_u16vec4)(self) -> [U16Vec4](struct.U16Vec4.html "struct bevy::math::U16Vec4")

Casts all elements of `self` to `u16`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1171)

#### pub fn [as\_ivec4](#method.as_ivec4)(self) -> [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

Casts all elements of `self` to `i32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1178)

#### pub fn [as\_uvec4](#method.as_uvec4)(self) -> [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

Casts all elements of `self` to `u32`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1185)

#### pub fn [as\_i64vec4](#method.as_i64vec4)(self) -> [I64Vec4](struct.I64Vec4.html "struct bevy::math::I64Vec4")

Casts all elements of `self` to `i64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1192)

#### pub fn [as\_u64vec4](#method.as_u64vec4)(self) -> [U64Vec4](struct.U64Vec4.html "struct bevy::math::U64Vec4")

Casts all elements of `self` to `u64`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1199)

#### pub fn [as\_isizevec4](#method.as_isizevec4)(self) -> [ISizeVec4](struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

Casts all elements of `self` to `isize`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1211)

#### pub fn [as\_usizevec4](#method.as_usizevec4)(self) -> [USizeVec4](struct.USizeVec4.html "struct bevy::math::USizeVec4")

Casts all elements of `self` to `usize`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1483)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1484)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1486)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1491)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1492)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1494)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1499)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1500)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1502)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1583)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1584)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1586)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1591)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1592)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1594)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1537)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1538)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1540)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1545)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1546)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1548)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1507)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1508)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1510)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1575)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1576)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1578)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1599)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1600)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1602)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1529)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1530)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1532)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1553)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1554)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1556)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1515)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1517)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1522)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1524)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1568)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1570)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1561)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1563)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#85)

### impl [Animatable](../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#85)

#### fn [interpolate](../prelude/trait.Animatable.html#tymethod.interpolate)(a: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), b: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Interpolates between `a` and `b` with an interpolation factor of `time`. [Read more](../prelude/trait.Animatable.html#tymethod.interpolate)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#85)

#### fn [blend](../prelude/trait.Animatable.html#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](../prelude/struct.BlendInput.html "struct bevy::prelude::BlendInput")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>>) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Blends one or more values together. [Read more](../prelude/trait.Animatable.html#tymethod.blend)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1865)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1867)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [AsMutVectorParts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

#### fn [as\_mut\_parts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html#tymethod.as_mut_parts)(&mut self) -> &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1858)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1860)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [AsRefVectorParts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

#### fn [as\_ref\_parts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html#tymethod.as_ref_parts)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#35)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#35)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#35)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [FromVectorParts](../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

#### fn [create\_from](../render/render_resource/encase/internal/trait.CreateFrom.html#tymethod.create_from)<B>(reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1969)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1970)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1221)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1223)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2070)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2071)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = Vec4<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>

The resulting type after dereferencing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2073)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2078)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2080)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Deserialize expects a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1955)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1956)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1235)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1236)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1238)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1243)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1244)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1246)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1251)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1252)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1254)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1335)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1336)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1338)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1343)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1344)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1346)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1289)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1290)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1292)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1297)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1298)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1300)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1259)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1260)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1262)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1327)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1328)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1330)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1351)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1352)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1354)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1281)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1282)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1284)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1305)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1306)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1308)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1267)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1269)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1274)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1276)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1320)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1322)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1313)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1315)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2063)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2065)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2056)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2058)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2042)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2044)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2028)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2030)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2049)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2051)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2035)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A"))> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2037)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(\_: ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A"))) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2014)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2016)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2085)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2087)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2098)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Available on **non-crate feature `scalar-math`** only.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2100)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1049)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Dir4](struct.Dir4.html "struct bevy::math::Dir4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1049)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Dir4](struct.Dir4.html "struct bevy::math::Dir4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1322)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1324)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(q: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2001)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#2003)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2189)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f64/dvec4.rs.html#2191)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(v: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [DVec4](struct.DVec4.html "struct bevy::math::DVec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1994)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1996)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(a: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1987)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[\_\_m128](https://doc.rust-lang.org/nightly/core/core_arch/x86/struct.__m128.html "struct core::core_arch::x86::__m128")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1989)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: [\_\_m128](https://doc.rust-lang.org/nightly/core/core_arch/x86/struct.__m128.html "struct core::core_arch::x86::__m128")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [FromVectorParts](../render/render_resource/encase/vector/trait.FromVectorParts.html "trait bevy::render::render_resource::encase::vector::FromVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

#### fn [from\_parts](../render/render_resource/encase/vector/trait.FromVectorParts.html#tymethod.from_parts)(parts: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1928)

### impl [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1929)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The returned type after indexing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1931)

#### fn [index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#tymethod.index)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1942)

### impl [IndexMut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1944)

#### fn [index\_mut](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut <[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4") as [Index](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html#associatedtype.Output "type core::ops::index::Index::Output")

Performs the mutable indexing (`container[index]`) operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html#tymethod.index_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1359)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1360)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1362)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1730)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1731)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1733)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1738)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1739)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1741)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1367)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1368)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1370)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1375)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1376)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1378)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1459)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1460)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1462)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1467)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1468)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1470)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1413)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1414)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1416)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1421)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1422)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1424)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1722)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1723)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1725)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> <[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1746)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1747)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1749)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1383)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1384)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1386)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1451)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1452)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1454)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1475)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1476)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1478)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1405)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1406)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1408)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1429)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1430)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1432)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1391)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1393)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1398)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1400)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1444)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1446)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1437)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1439)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1912)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1913)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1915)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1920)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1921)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1923)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#271)

### impl [NormedVectorSpace](trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#273)

#### fn [norm](trait.NormedVectorSpace.html#tymethod.norm)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The size of this element. The return value should always be nonnegative.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#278)

#### fn [norm\_squared](trait.NormedVectorSpace.html#method.norm_squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The squared norm of this element. Computing this is often faster than computing [`NormedVectorSpace::norm`](trait.NormedVectorSpace.html#tymethod.norm "method bevy::math::NormedVectorSpace::norm").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#259)

#### fn [distance](trait.NormedVectorSpace.html#method.distance)(self, rhs: Self) -> Self::[Scalar](trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The distance between this element and another, as determined by the norm.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#266)

#### fn [distance\_squared](trait.NormedVectorSpace.html#method.distance_squared)(self, rhs: Self) -> Self::[Scalar](trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")

The squared distance between this element and another, as determined by the norm. Note that this is often faster to compute in practice than [`NormedVectorSpace::distance`](trait.NormedVectorSpace.html#method.distance "method bevy::math::NormedVectorSpace::distance").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1228)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1230)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#277)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#277)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#277)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#36)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1892)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1894-1896)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1902)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1904-1906)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [AsMutVectorParts](../render/render_resource/encase/vector/trait.AsMutVectorParts.html "trait bevy::render::render_resource::encase::vector::AsMutVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

#### fn [read\_from](../render/render_resource/encase/internal/trait.ReadFrom.html#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1731)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1732)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1734)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1742)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1743)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1745)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1750)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1751)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1753)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1834)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1835)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1837)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1842)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1843)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1845)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1788)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1789)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1791)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1796)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1797)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1799)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1758)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1759)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1761)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1826)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1827)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1829)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1850)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1851)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1853)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1780)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1781)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1783)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1804)

### impl [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1805)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `%` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1807)

#### fn [rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `%` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#tymethod.rem)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1766)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1768)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1773)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1775)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1819)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1821)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1812)

### impl [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1814)

#### fn [rem\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `%=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html#tymethod.rem_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#639)

### impl [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_rand.rs.html#639)

#### type [Sampler](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html#associatedtype.Sampler) = UniformVec4<[UniformFloat](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/float/struct.UniformFloat.html "struct rand::distr::uniform::float::UniformFloat")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>>

The `UniformSampler` implementation supporting type `X`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Serialize as a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#232)

#### const [SHADER\_SIZE](../render/render_resource/trait.ShaderSize.html#associatedconstant.SHADER_SIZE): [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> = \_

Represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderType::min_size`](../render/render_resource/trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size"))

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#94)

#### fn [min\_size](../render/render_resource/trait.ShaderType.html#method.min_size)() -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Represents the minimum size of `Self` (equivalent to [GPUBufferBindingLayout.minBindingSize](https://gpuweb.github.io/gpuweb/#dom-gpubufferbindinglayout-minbindingsize)) [Read more](../render/render_resource/trait.ShaderType.html#method.min_size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#103)

#### fn [size](../render/render_resource/trait.ShaderType.html#method.size)(&self) -> [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns the size of `Self` at runtime [Read more](../render/render_resource/trait.ShaderType.html#method.size)

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#206)

#### fn [assert\_uniform\_compat](../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)()

Asserts that `Self` meets the requirements of the [uniform address space restrictions on stored values](https://gpuweb.github.io/gpuweb/wgsl/#address-spaces-uniform) and the [uniform address space layout constraints](https://gpuweb.github.io/gpuweb/wgsl/#address-space-layout-constraints) [Read more](../render/render_resource/trait.ShaderType.html#method.assert_uniform_compat)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1607)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1608)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1610)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1615)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1616)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1618)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1623)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1624)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1626)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1707)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1708)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1710)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1715)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1716)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1718)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1661)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1662)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1664)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1669)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1670)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1672)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1631)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1632)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1634)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1699)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1700)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1702)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1723)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1724)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1726)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1653)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1654)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1656)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1677)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1678)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1680)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1639)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1641)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1646)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1648)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1692)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1694)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1685)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1687)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1872)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1874-1876)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1882)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#1884-1886)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1168)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> for [Dir4](struct.Dir4.html "struct bevy::math::Dir4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1169)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [InvalidDirectionError](enum.InvalidDirectionError.html "enum bevy::math::InvalidDirectionError")

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1171)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Dir4](struct.Dir4.html "struct bevy::math::Dir4"), <[Dir4](struct.Dir4.html "struct bevy::math::Dir4") as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#12)

### impl [Vec4Swizzles](../prelude/trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#13)

#### type [Vec2](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec2) = [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#15)

#### type [Vec3](../prelude/trait.Vec4Swizzles.html#associatedtype.Vec3) = [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#18)

#### fn [xx](../prelude/trait.Vec4Swizzles.html#tymethod.xx)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#26)

#### fn [xy](../prelude/trait.Vec4Swizzles.html#tymethod.xy)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#34)

#### fn [with\_xy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xy)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#39)

#### fn [xz](../prelude/trait.Vec4Swizzles.html#tymethod.xz)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#47)

#### fn [with\_xz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xz)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#52)

#### fn [xw](../prelude/trait.Vec4Swizzles.html#tymethod.xw)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#60)

#### fn [with\_xw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xw)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#65)

#### fn [yx](../prelude/trait.Vec4Swizzles.html#tymethod.yx)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#73)

#### fn [with\_yx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yx)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#78)

#### fn [yy](../prelude/trait.Vec4Swizzles.html#tymethod.yy)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#86)

#### fn [yz](../prelude/trait.Vec4Swizzles.html#tymethod.yz)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#94)

#### fn [with\_yz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yz)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#99)

#### fn [yw](../prelude/trait.Vec4Swizzles.html#tymethod.yw)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#107)

#### fn [with\_yw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yw)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#112)

#### fn [zx](../prelude/trait.Vec4Swizzles.html#tymethod.zx)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#120)

#### fn [with\_zx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zx)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#125)

#### fn [zy](../prelude/trait.Vec4Swizzles.html#tymethod.zy)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#133)

#### fn [with\_zy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zy)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#138)

#### fn [zz](../prelude/trait.Vec4Swizzles.html#tymethod.zz)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#146)

#### fn [zw](../prelude/trait.Vec4Swizzles.html#tymethod.zw)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#154)

#### fn [with\_zw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zw)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#159)

#### fn [wx](../prelude/trait.Vec4Swizzles.html#tymethod.wx)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#167)

#### fn [with\_wx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wx)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#172)

#### fn [wy](../prelude/trait.Vec4Swizzles.html#tymethod.wy)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#180)

#### fn [with\_wy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wy)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#185)

#### fn [wz](../prelude/trait.Vec4Swizzles.html#tymethod.wz)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#193)

#### fn [with\_wz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wz)(self, rhs: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#198)

#### fn [ww](../prelude/trait.Vec4Swizzles.html#tymethod.ww)(self) -> [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#206)

#### fn [xxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#211)

#### fn [xxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#216)

#### fn [xxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#221)

#### fn [xxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#226)

#### fn [xyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#231)

#### fn [xyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#236)

#### fn [xyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#241)

#### fn [with\_xyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyz)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#246)

#### fn [xyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#251)

#### fn [with\_xyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xyw)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#256)

#### fn [xzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#261)

#### fn [xzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#266)

#### fn [with\_xzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzy)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#271)

#### fn [xzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#276)

#### fn [xzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#281)

#### fn [with\_xzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_xzw)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#286)

#### fn [xwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#291)

#### fn [xwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#296)

#### fn [with\_xwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwy)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#301)

#### fn [xwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#306)

#### fn [with\_xwz](../prelude/trait.Vec4Swizzles.html#tymethod.with_xwz)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#311)

#### fn [xww](../prelude/trait.Vec4Swizzles.html#tymethod.xww)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#316)

#### fn [yxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#321)

#### fn [yxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#326)

#### fn [yxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#331)

#### fn [with\_yxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxz)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#336)

#### fn [yxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#341)

#### fn [with\_yxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yxw)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#346)

#### fn [yyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#351)

#### fn [yyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#356)

#### fn [yyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#361)

#### fn [yyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#366)

#### fn [yzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#371)

#### fn [with\_yzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzx)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#376)

#### fn [yzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#381)

#### fn [yzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#386)

#### fn [yzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#391)

#### fn [with\_yzw](../prelude/trait.Vec4Swizzles.html#tymethod.with_yzw)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#396)

#### fn [ywx](../prelude/trait.Vec4Swizzles.html#tymethod.ywx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#401)

#### fn [with\_ywx](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywx)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#406)

#### fn [ywy](../prelude/trait.Vec4Swizzles.html#tymethod.ywy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#411)

#### fn [ywz](../prelude/trait.Vec4Swizzles.html#tymethod.ywz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#416)

#### fn [with\_ywz](../prelude/trait.Vec4Swizzles.html#tymethod.with_ywz)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#421)

#### fn [yww](../prelude/trait.Vec4Swizzles.html#tymethod.yww)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#426)

#### fn [zxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#431)

#### fn [zxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#436)

#### fn [with\_zxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxy)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#441)

#### fn [zxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#446)

#### fn [zxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#451)

#### fn [with\_zxw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zxw)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#456)

#### fn [zyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#461)

#### fn [with\_zyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyx)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#466)

#### fn [zyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#471)

#### fn [zyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#476)

#### fn [zyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#481)

#### fn [with\_zyw](../prelude/trait.Vec4Swizzles.html#tymethod.with_zyw)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#486)

#### fn [zzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#491)

#### fn [zzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#496)

#### fn [zzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#501)

#### fn [zzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#506)

#### fn [zwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#511)

#### fn [with\_zwx](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwx)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#516)

#### fn [zwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#521)

#### fn [with\_zwy](../prelude/trait.Vec4Swizzles.html#tymethod.with_zwy)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#526)

#### fn [zwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#531)

#### fn [zww](../prelude/trait.Vec4Swizzles.html#tymethod.zww)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#536)

#### fn [wxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#541)

#### fn [wxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#546)

#### fn [with\_wxy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxy)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#551)

#### fn [wxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#556)

#### fn [with\_wxz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wxz)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#561)

#### fn [wxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#566)

#### fn [wyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#571)

#### fn [with\_wyx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyx)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#576)

#### fn [wyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#581)

#### fn [wyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#586)

#### fn [with\_wyz](../prelude/trait.Vec4Swizzles.html#tymethod.with_wyz)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#591)

#### fn [wyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#596)

#### fn [wzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#601)

#### fn [with\_wzx](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzx)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#606)

#### fn [wzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#611)

#### fn [with\_wzy](../prelude/trait.Vec4Swizzles.html#tymethod.with_wzy)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#616)

#### fn [wzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#621)

#### fn [wzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzw)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#626)

#### fn [wwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwx)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#631)

#### fn [wwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwy)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#636)

#### fn [wwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#641)

#### fn [www](../prelude/trait.Vec4Swizzles.html#tymethod.www)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#646)

#### fn [xxxx](../prelude/trait.Vec4Swizzles.html#tymethod.xxxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#651)

#### fn [xxxy](../prelude/trait.Vec4Swizzles.html#tymethod.xxxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#656)

#### fn [xxxz](../prelude/trait.Vec4Swizzles.html#tymethod.xxxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#661)

#### fn [xxxw](../prelude/trait.Vec4Swizzles.html#tymethod.xxxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#666)

#### fn [xxyx](../prelude/trait.Vec4Swizzles.html#tymethod.xxyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#671)

#### fn [xxyy](../prelude/trait.Vec4Swizzles.html#tymethod.xxyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#676)

#### fn [xxyz](../prelude/trait.Vec4Swizzles.html#tymethod.xxyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#681)

#### fn [xxyw](../prelude/trait.Vec4Swizzles.html#tymethod.xxyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#686)

#### fn [xxzx](../prelude/trait.Vec4Swizzles.html#tymethod.xxzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#691)

#### fn [xxzy](../prelude/trait.Vec4Swizzles.html#tymethod.xxzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#696)

#### fn [xxzz](../prelude/trait.Vec4Swizzles.html#tymethod.xxzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#701)

#### fn [xxzw](../prelude/trait.Vec4Swizzles.html#tymethod.xxzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#706)

#### fn [xxwx](../prelude/trait.Vec4Swizzles.html#tymethod.xxwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#711)

#### fn [xxwy](../prelude/trait.Vec4Swizzles.html#tymethod.xxwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#716)

#### fn [xxwz](../prelude/trait.Vec4Swizzles.html#tymethod.xxwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#721)

#### fn [xxww](../prelude/trait.Vec4Swizzles.html#tymethod.xxww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#726)

#### fn [xyxx](../prelude/trait.Vec4Swizzles.html#tymethod.xyxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#731)

#### fn [xyxy](../prelude/trait.Vec4Swizzles.html#tymethod.xyxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#736)

#### fn [xyxz](../prelude/trait.Vec4Swizzles.html#tymethod.xyxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#741)

#### fn [xyxw](../prelude/trait.Vec4Swizzles.html#tymethod.xyxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#746)

#### fn [xyyx](../prelude/trait.Vec4Swizzles.html#tymethod.xyyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#751)

#### fn [xyyy](../prelude/trait.Vec4Swizzles.html#tymethod.xyyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#756)

#### fn [xyyz](../prelude/trait.Vec4Swizzles.html#tymethod.xyyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#761)

#### fn [xyyw](../prelude/trait.Vec4Swizzles.html#tymethod.xyyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#766)

#### fn [xyzx](../prelude/trait.Vec4Swizzles.html#tymethod.xyzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#771)

#### fn [xyzy](../prelude/trait.Vec4Swizzles.html#tymethod.xyzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#776)

#### fn [xyzz](../prelude/trait.Vec4Swizzles.html#tymethod.xyzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#781)

#### fn [xywx](../prelude/trait.Vec4Swizzles.html#tymethod.xywx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#786)

#### fn [xywy](../prelude/trait.Vec4Swizzles.html#tymethod.xywy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#791)

#### fn [xywz](../prelude/trait.Vec4Swizzles.html#tymethod.xywz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#796)

#### fn [xyww](../prelude/trait.Vec4Swizzles.html#tymethod.xyww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#801)

#### fn [xzxx](../prelude/trait.Vec4Swizzles.html#tymethod.xzxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#806)

#### fn [xzxy](../prelude/trait.Vec4Swizzles.html#tymethod.xzxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#811)

#### fn [xzxz](../prelude/trait.Vec4Swizzles.html#tymethod.xzxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#816)

#### fn [xzxw](../prelude/trait.Vec4Swizzles.html#tymethod.xzxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#821)

#### fn [xzyx](../prelude/trait.Vec4Swizzles.html#tymethod.xzyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#826)

#### fn [xzyy](../prelude/trait.Vec4Swizzles.html#tymethod.xzyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#831)

#### fn [xzyz](../prelude/trait.Vec4Swizzles.html#tymethod.xzyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#836)

#### fn [xzyw](../prelude/trait.Vec4Swizzles.html#tymethod.xzyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#841)

#### fn [xzzx](../prelude/trait.Vec4Swizzles.html#tymethod.xzzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#846)

#### fn [xzzy](../prelude/trait.Vec4Swizzles.html#tymethod.xzzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#851)

#### fn [xzzz](../prelude/trait.Vec4Swizzles.html#tymethod.xzzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#856)

#### fn [xzzw](../prelude/trait.Vec4Swizzles.html#tymethod.xzzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#861)

#### fn [xzwx](../prelude/trait.Vec4Swizzles.html#tymethod.xzwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#866)

#### fn [xzwy](../prelude/trait.Vec4Swizzles.html#tymethod.xzwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#871)

#### fn [xzwz](../prelude/trait.Vec4Swizzles.html#tymethod.xzwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#876)

#### fn [xzww](../prelude/trait.Vec4Swizzles.html#tymethod.xzww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#881)

#### fn [xwxx](../prelude/trait.Vec4Swizzles.html#tymethod.xwxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#886)

#### fn [xwxy](../prelude/trait.Vec4Swizzles.html#tymethod.xwxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#891)

#### fn [xwxz](../prelude/trait.Vec4Swizzles.html#tymethod.xwxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#896)

#### fn [xwxw](../prelude/trait.Vec4Swizzles.html#tymethod.xwxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#901)

#### fn [xwyx](../prelude/trait.Vec4Swizzles.html#tymethod.xwyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#906)

#### fn [xwyy](../prelude/trait.Vec4Swizzles.html#tymethod.xwyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#911)

#### fn [xwyz](../prelude/trait.Vec4Swizzles.html#tymethod.xwyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#916)

#### fn [xwyw](../prelude/trait.Vec4Swizzles.html#tymethod.xwyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#921)

#### fn [xwzx](../prelude/trait.Vec4Swizzles.html#tymethod.xwzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#926)

#### fn [xwzy](../prelude/trait.Vec4Swizzles.html#tymethod.xwzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#931)

#### fn [xwzz](../prelude/trait.Vec4Swizzles.html#tymethod.xwzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#936)

#### fn [xwzw](../prelude/trait.Vec4Swizzles.html#tymethod.xwzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#941)

#### fn [xwwx](../prelude/trait.Vec4Swizzles.html#tymethod.xwwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#946)

#### fn [xwwy](../prelude/trait.Vec4Swizzles.html#tymethod.xwwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#951)

#### fn [xwwz](../prelude/trait.Vec4Swizzles.html#tymethod.xwwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#956)

#### fn [xwww](../prelude/trait.Vec4Swizzles.html#tymethod.xwww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#961)

#### fn [yxxx](../prelude/trait.Vec4Swizzles.html#tymethod.yxxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#966)

#### fn [yxxy](../prelude/trait.Vec4Swizzles.html#tymethod.yxxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#971)

#### fn [yxxz](../prelude/trait.Vec4Swizzles.html#tymethod.yxxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#976)

#### fn [yxxw](../prelude/trait.Vec4Swizzles.html#tymethod.yxxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#981)

#### fn [yxyx](../prelude/trait.Vec4Swizzles.html#tymethod.yxyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#986)

#### fn [yxyy](../prelude/trait.Vec4Swizzles.html#tymethod.yxyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#991)

#### fn [yxyz](../prelude/trait.Vec4Swizzles.html#tymethod.yxyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#996)

#### fn [yxyw](../prelude/trait.Vec4Swizzles.html#tymethod.yxyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1001)

#### fn [yxzx](../prelude/trait.Vec4Swizzles.html#tymethod.yxzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1006)

#### fn [yxzy](../prelude/trait.Vec4Swizzles.html#tymethod.yxzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1011)

#### fn [yxzz](../prelude/trait.Vec4Swizzles.html#tymethod.yxzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1016)

#### fn [yxzw](../prelude/trait.Vec4Swizzles.html#tymethod.yxzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1021)

#### fn [yxwx](../prelude/trait.Vec4Swizzles.html#tymethod.yxwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1026)

#### fn [yxwy](../prelude/trait.Vec4Swizzles.html#tymethod.yxwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1031)

#### fn [yxwz](../prelude/trait.Vec4Swizzles.html#tymethod.yxwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1036)

#### fn [yxww](../prelude/trait.Vec4Swizzles.html#tymethod.yxww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1041)

#### fn [yyxx](../prelude/trait.Vec4Swizzles.html#tymethod.yyxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1046)

#### fn [yyxy](../prelude/trait.Vec4Swizzles.html#tymethod.yyxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1051)

#### fn [yyxz](../prelude/trait.Vec4Swizzles.html#tymethod.yyxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1056)

#### fn [yyxw](../prelude/trait.Vec4Swizzles.html#tymethod.yyxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1061)

#### fn [yyyx](../prelude/trait.Vec4Swizzles.html#tymethod.yyyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1066)

#### fn [yyyy](../prelude/trait.Vec4Swizzles.html#tymethod.yyyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1071)

#### fn [yyyz](../prelude/trait.Vec4Swizzles.html#tymethod.yyyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1076)

#### fn [yyyw](../prelude/trait.Vec4Swizzles.html#tymethod.yyyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1081)

#### fn [yyzx](../prelude/trait.Vec4Swizzles.html#tymethod.yyzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1086)

#### fn [yyzy](../prelude/trait.Vec4Swizzles.html#tymethod.yyzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1091)

#### fn [yyzz](../prelude/trait.Vec4Swizzles.html#tymethod.yyzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1096)

#### fn [yyzw](../prelude/trait.Vec4Swizzles.html#tymethod.yyzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1101)

#### fn [yywx](../prelude/trait.Vec4Swizzles.html#tymethod.yywx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1106)

#### fn [yywy](../prelude/trait.Vec4Swizzles.html#tymethod.yywy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1111)

#### fn [yywz](../prelude/trait.Vec4Swizzles.html#tymethod.yywz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1116)

#### fn [yyww](../prelude/trait.Vec4Swizzles.html#tymethod.yyww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1121)

#### fn [yzxx](../prelude/trait.Vec4Swizzles.html#tymethod.yzxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1126)

#### fn [yzxy](../prelude/trait.Vec4Swizzles.html#tymethod.yzxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1131)

#### fn [yzxz](../prelude/trait.Vec4Swizzles.html#tymethod.yzxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1136)

#### fn [yzxw](../prelude/trait.Vec4Swizzles.html#tymethod.yzxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1141)

#### fn [yzyx](../prelude/trait.Vec4Swizzles.html#tymethod.yzyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1146)

#### fn [yzyy](../prelude/trait.Vec4Swizzles.html#tymethod.yzyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1151)

#### fn [yzyz](../prelude/trait.Vec4Swizzles.html#tymethod.yzyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1156)

#### fn [yzyw](../prelude/trait.Vec4Swizzles.html#tymethod.yzyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1161)

#### fn [yzzx](../prelude/trait.Vec4Swizzles.html#tymethod.yzzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1166)

#### fn [yzzy](../prelude/trait.Vec4Swizzles.html#tymethod.yzzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1171)

#### fn [yzzz](../prelude/trait.Vec4Swizzles.html#tymethod.yzzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1176)

#### fn [yzzw](../prelude/trait.Vec4Swizzles.html#tymethod.yzzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1181)

#### fn [yzwx](../prelude/trait.Vec4Swizzles.html#tymethod.yzwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1186)

#### fn [yzwy](../prelude/trait.Vec4Swizzles.html#tymethod.yzwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1191)

#### fn [yzwz](../prelude/trait.Vec4Swizzles.html#tymethod.yzwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1196)

#### fn [yzww](../prelude/trait.Vec4Swizzles.html#tymethod.yzww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1201)

#### fn [ywxx](../prelude/trait.Vec4Swizzles.html#tymethod.ywxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1206)

#### fn [ywxy](../prelude/trait.Vec4Swizzles.html#tymethod.ywxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1211)

#### fn [ywxz](../prelude/trait.Vec4Swizzles.html#tymethod.ywxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1216)

#### fn [ywxw](../prelude/trait.Vec4Swizzles.html#tymethod.ywxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1221)

#### fn [ywyx](../prelude/trait.Vec4Swizzles.html#tymethod.ywyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1226)

#### fn [ywyy](../prelude/trait.Vec4Swizzles.html#tymethod.ywyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1231)

#### fn [ywyz](../prelude/trait.Vec4Swizzles.html#tymethod.ywyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1236)

#### fn [ywyw](../prelude/trait.Vec4Swizzles.html#tymethod.ywyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1241)

#### fn [ywzx](../prelude/trait.Vec4Swizzles.html#tymethod.ywzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1246)

#### fn [ywzy](../prelude/trait.Vec4Swizzles.html#tymethod.ywzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1251)

#### fn [ywzz](../prelude/trait.Vec4Swizzles.html#tymethod.ywzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1256)

#### fn [ywzw](../prelude/trait.Vec4Swizzles.html#tymethod.ywzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1261)

#### fn [ywwx](../prelude/trait.Vec4Swizzles.html#tymethod.ywwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1266)

#### fn [ywwy](../prelude/trait.Vec4Swizzles.html#tymethod.ywwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1271)

#### fn [ywwz](../prelude/trait.Vec4Swizzles.html#tymethod.ywwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1276)

#### fn [ywww](../prelude/trait.Vec4Swizzles.html#tymethod.ywww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1281)

#### fn [zxxx](../prelude/trait.Vec4Swizzles.html#tymethod.zxxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1286)

#### fn [zxxy](../prelude/trait.Vec4Swizzles.html#tymethod.zxxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1291)

#### fn [zxxz](../prelude/trait.Vec4Swizzles.html#tymethod.zxxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1296)

#### fn [zxxw](../prelude/trait.Vec4Swizzles.html#tymethod.zxxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1301)

#### fn [zxyx](../prelude/trait.Vec4Swizzles.html#tymethod.zxyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1306)

#### fn [zxyy](../prelude/trait.Vec4Swizzles.html#tymethod.zxyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1311)

#### fn [zxyz](../prelude/trait.Vec4Swizzles.html#tymethod.zxyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1316)

#### fn [zxyw](../prelude/trait.Vec4Swizzles.html#tymethod.zxyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1321)

#### fn [zxzx](../prelude/trait.Vec4Swizzles.html#tymethod.zxzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1326)

#### fn [zxzy](../prelude/trait.Vec4Swizzles.html#tymethod.zxzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1331)

#### fn [zxzz](../prelude/trait.Vec4Swizzles.html#tymethod.zxzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1336)

#### fn [zxzw](../prelude/trait.Vec4Swizzles.html#tymethod.zxzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1341)

#### fn [zxwx](../prelude/trait.Vec4Swizzles.html#tymethod.zxwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1346)

#### fn [zxwy](../prelude/trait.Vec4Swizzles.html#tymethod.zxwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1351)

#### fn [zxwz](../prelude/trait.Vec4Swizzles.html#tymethod.zxwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1356)

#### fn [zxww](../prelude/trait.Vec4Swizzles.html#tymethod.zxww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1361)

#### fn [zyxx](../prelude/trait.Vec4Swizzles.html#tymethod.zyxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1366)

#### fn [zyxy](../prelude/trait.Vec4Swizzles.html#tymethod.zyxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1371)

#### fn [zyxz](../prelude/trait.Vec4Swizzles.html#tymethod.zyxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1376)

#### fn [zyxw](../prelude/trait.Vec4Swizzles.html#tymethod.zyxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1381)

#### fn [zyyx](../prelude/trait.Vec4Swizzles.html#tymethod.zyyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1386)

#### fn [zyyy](../prelude/trait.Vec4Swizzles.html#tymethod.zyyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1391)

#### fn [zyyz](../prelude/trait.Vec4Swizzles.html#tymethod.zyyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1396)

#### fn [zyyw](../prelude/trait.Vec4Swizzles.html#tymethod.zyyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1401)

#### fn [zyzx](../prelude/trait.Vec4Swizzles.html#tymethod.zyzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1406)

#### fn [zyzy](../prelude/trait.Vec4Swizzles.html#tymethod.zyzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1411)

#### fn [zyzz](../prelude/trait.Vec4Swizzles.html#tymethod.zyzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1416)

#### fn [zyzw](../prelude/trait.Vec4Swizzles.html#tymethod.zyzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1421)

#### fn [zywx](../prelude/trait.Vec4Swizzles.html#tymethod.zywx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1426)

#### fn [zywy](../prelude/trait.Vec4Swizzles.html#tymethod.zywy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1431)

#### fn [zywz](../prelude/trait.Vec4Swizzles.html#tymethod.zywz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1436)

#### fn [zyww](../prelude/trait.Vec4Swizzles.html#tymethod.zyww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1441)

#### fn [zzxx](../prelude/trait.Vec4Swizzles.html#tymethod.zzxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1446)

#### fn [zzxy](../prelude/trait.Vec4Swizzles.html#tymethod.zzxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1451)

#### fn [zzxz](../prelude/trait.Vec4Swizzles.html#tymethod.zzxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1456)

#### fn [zzxw](../prelude/trait.Vec4Swizzles.html#tymethod.zzxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1461)

#### fn [zzyx](../prelude/trait.Vec4Swizzles.html#tymethod.zzyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1466)

#### fn [zzyy](../prelude/trait.Vec4Swizzles.html#tymethod.zzyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1471)

#### fn [zzyz](../prelude/trait.Vec4Swizzles.html#tymethod.zzyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1476)

#### fn [zzyw](../prelude/trait.Vec4Swizzles.html#tymethod.zzyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1481)

#### fn [zzzx](../prelude/trait.Vec4Swizzles.html#tymethod.zzzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1486)

#### fn [zzzy](../prelude/trait.Vec4Swizzles.html#tymethod.zzzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1491)

#### fn [zzzz](../prelude/trait.Vec4Swizzles.html#tymethod.zzzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1496)

#### fn [zzzw](../prelude/trait.Vec4Swizzles.html#tymethod.zzzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1501)

#### fn [zzwx](../prelude/trait.Vec4Swizzles.html#tymethod.zzwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1506)

#### fn [zzwy](../prelude/trait.Vec4Swizzles.html#tymethod.zzwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1511)

#### fn [zzwz](../prelude/trait.Vec4Swizzles.html#tymethod.zzwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1516)

#### fn [zzww](../prelude/trait.Vec4Swizzles.html#tymethod.zzww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1521)

#### fn [zwxx](../prelude/trait.Vec4Swizzles.html#tymethod.zwxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1526)

#### fn [zwxy](../prelude/trait.Vec4Swizzles.html#tymethod.zwxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1531)

#### fn [zwxz](../prelude/trait.Vec4Swizzles.html#tymethod.zwxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1536)

#### fn [zwxw](../prelude/trait.Vec4Swizzles.html#tymethod.zwxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1541)

#### fn [zwyx](../prelude/trait.Vec4Swizzles.html#tymethod.zwyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1546)

#### fn [zwyy](../prelude/trait.Vec4Swizzles.html#tymethod.zwyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1551)

#### fn [zwyz](../prelude/trait.Vec4Swizzles.html#tymethod.zwyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1556)

#### fn [zwyw](../prelude/trait.Vec4Swizzles.html#tymethod.zwyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1561)

#### fn [zwzx](../prelude/trait.Vec4Swizzles.html#tymethod.zwzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1566)

#### fn [zwzy](../prelude/trait.Vec4Swizzles.html#tymethod.zwzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1571)

#### fn [zwzz](../prelude/trait.Vec4Swizzles.html#tymethod.zwzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1576)

#### fn [zwzw](../prelude/trait.Vec4Swizzles.html#tymethod.zwzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1581)

#### fn [zwwx](../prelude/trait.Vec4Swizzles.html#tymethod.zwwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1586)

#### fn [zwwy](../prelude/trait.Vec4Swizzles.html#tymethod.zwwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1591)

#### fn [zwwz](../prelude/trait.Vec4Swizzles.html#tymethod.zwwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1596)

#### fn [zwww](../prelude/trait.Vec4Swizzles.html#tymethod.zwww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1601)

#### fn [wxxx](../prelude/trait.Vec4Swizzles.html#tymethod.wxxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1606)

#### fn [wxxy](../prelude/trait.Vec4Swizzles.html#tymethod.wxxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1611)

#### fn [wxxz](../prelude/trait.Vec4Swizzles.html#tymethod.wxxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1616)

#### fn [wxxw](../prelude/trait.Vec4Swizzles.html#tymethod.wxxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1621)

#### fn [wxyx](../prelude/trait.Vec4Swizzles.html#tymethod.wxyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1626)

#### fn [wxyy](../prelude/trait.Vec4Swizzles.html#tymethod.wxyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1631)

#### fn [wxyz](../prelude/trait.Vec4Swizzles.html#tymethod.wxyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1636)

#### fn [wxyw](../prelude/trait.Vec4Swizzles.html#tymethod.wxyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1641)

#### fn [wxzx](../prelude/trait.Vec4Swizzles.html#tymethod.wxzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1646)

#### fn [wxzy](../prelude/trait.Vec4Swizzles.html#tymethod.wxzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1651)

#### fn [wxzz](../prelude/trait.Vec4Swizzles.html#tymethod.wxzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1656)

#### fn [wxzw](../prelude/trait.Vec4Swizzles.html#tymethod.wxzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1661)

#### fn [wxwx](../prelude/trait.Vec4Swizzles.html#tymethod.wxwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1666)

#### fn [wxwy](../prelude/trait.Vec4Swizzles.html#tymethod.wxwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1671)

#### fn [wxwz](../prelude/trait.Vec4Swizzles.html#tymethod.wxwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1676)

#### fn [wxww](../prelude/trait.Vec4Swizzles.html#tymethod.wxww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1681)

#### fn [wyxx](../prelude/trait.Vec4Swizzles.html#tymethod.wyxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1686)

#### fn [wyxy](../prelude/trait.Vec4Swizzles.html#tymethod.wyxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1691)

#### fn [wyxz](../prelude/trait.Vec4Swizzles.html#tymethod.wyxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1696)

#### fn [wyxw](../prelude/trait.Vec4Swizzles.html#tymethod.wyxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1701)

#### fn [wyyx](../prelude/trait.Vec4Swizzles.html#tymethod.wyyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1706)

#### fn [wyyy](../prelude/trait.Vec4Swizzles.html#tymethod.wyyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1711)

#### fn [wyyz](../prelude/trait.Vec4Swizzles.html#tymethod.wyyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1716)

#### fn [wyyw](../prelude/trait.Vec4Swizzles.html#tymethod.wyyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1721)

#### fn [wyzx](../prelude/trait.Vec4Swizzles.html#tymethod.wyzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1726)

#### fn [wyzy](../prelude/trait.Vec4Swizzles.html#tymethod.wyzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1731)

#### fn [wyzz](../prelude/trait.Vec4Swizzles.html#tymethod.wyzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1736)

#### fn [wyzw](../prelude/trait.Vec4Swizzles.html#tymethod.wyzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1741)

#### fn [wywx](../prelude/trait.Vec4Swizzles.html#tymethod.wywx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1746)

#### fn [wywy](../prelude/trait.Vec4Swizzles.html#tymethod.wywy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1751)

#### fn [wywz](../prelude/trait.Vec4Swizzles.html#tymethod.wywz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1756)

#### fn [wyww](../prelude/trait.Vec4Swizzles.html#tymethod.wyww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1761)

#### fn [wzxx](../prelude/trait.Vec4Swizzles.html#tymethod.wzxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1766)

#### fn [wzxy](../prelude/trait.Vec4Swizzles.html#tymethod.wzxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1771)

#### fn [wzxz](../prelude/trait.Vec4Swizzles.html#tymethod.wzxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1776)

#### fn [wzxw](../prelude/trait.Vec4Swizzles.html#tymethod.wzxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1781)

#### fn [wzyx](../prelude/trait.Vec4Swizzles.html#tymethod.wzyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1786)

#### fn [wzyy](../prelude/trait.Vec4Swizzles.html#tymethod.wzyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1791)

#### fn [wzyz](../prelude/trait.Vec4Swizzles.html#tymethod.wzyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1796)

#### fn [wzyw](../prelude/trait.Vec4Swizzles.html#tymethod.wzyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1801)

#### fn [wzzx](../prelude/trait.Vec4Swizzles.html#tymethod.wzzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1806)

#### fn [wzzy](../prelude/trait.Vec4Swizzles.html#tymethod.wzzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1811)

#### fn [wzzz](../prelude/trait.Vec4Swizzles.html#tymethod.wzzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1816)

#### fn [wzzw](../prelude/trait.Vec4Swizzles.html#tymethod.wzzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1821)

#### fn [wzwx](../prelude/trait.Vec4Swizzles.html#tymethod.wzwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1826)

#### fn [wzwy](../prelude/trait.Vec4Swizzles.html#tymethod.wzwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1831)

#### fn [wzwz](../prelude/trait.Vec4Swizzles.html#tymethod.wzwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1836)

#### fn [wzww](../prelude/trait.Vec4Swizzles.html#tymethod.wzww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1841)

#### fn [wwxx](../prelude/trait.Vec4Swizzles.html#tymethod.wwxx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1846)

#### fn [wwxy](../prelude/trait.Vec4Swizzles.html#tymethod.wwxy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1851)

#### fn [wwxz](../prelude/trait.Vec4Swizzles.html#tymethod.wwxz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1856)

#### fn [wwxw](../prelude/trait.Vec4Swizzles.html#tymethod.wwxw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1861)

#### fn [wwyx](../prelude/trait.Vec4Swizzles.html#tymethod.wwyx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1866)

#### fn [wwyy](../prelude/trait.Vec4Swizzles.html#tymethod.wwyy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1871)

#### fn [wwyz](../prelude/trait.Vec4Swizzles.html#tymethod.wwyz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1876)

#### fn [wwyw](../prelude/trait.Vec4Swizzles.html#tymethod.wwyw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1881)

#### fn [wwzx](../prelude/trait.Vec4Swizzles.html#tymethod.wwzx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1886)

#### fn [wwzy](../prelude/trait.Vec4Swizzles.html#tymethod.wwzy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1891)

#### fn [wwzz](../prelude/trait.Vec4Swizzles.html#tymethod.wwzz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1896)

#### fn [wwzw](../prelude/trait.Vec4Swizzles.html#tymethod.wwzw)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1901)

#### fn [wwwx](../prelude/trait.Vec4Swizzles.html#tymethod.wwwx)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1906)

#### fn [wwwy](../prelude/trait.Vec4Swizzles.html#tymethod.wwwy)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1911)

#### fn [wwwz](../prelude/trait.Vec4Swizzles.html#tymethod.wwwz)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#1916)

#### fn [wwww](../prelude/trait.Vec4Swizzles.html#tymethod.wwww)(self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#481)

#### fn [xyzw](../prelude/trait.Vec4Swizzles.html#method.xyzw)(self) -> Self

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#60)

### impl [VectorSpace](trait.VectorSpace.html "trait bevy::math::VectorSpace") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#62)

#### const [ZERO](trait.VectorSpace.html#associatedconstant.ZERO): [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4") = Vec4::ZERO

The zero vector, which is the identity of addition for the vector space type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#61)

#### type [Scalar](trait.VectorSpace.html#associatedtype.Scalar) = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

The scalar type of this vector space.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#55)

#### fn [lerp](trait.VectorSpace.html#method.lerp)(self, rhs: Self, t: Self::[Scalar](trait.VectorSpace.html#associatedtype.Scalar "type bevy::math::VectorSpace::Scalar")) -> Self

Perform vector space linear interpolation between this element and another, based on the parameter `t`. When `t` is `0`, `self` is recovered. When `t` is `1`, `rhs` is recovered. [Read more](trait.VectorSpace.html#method.lerp)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

### impl [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

where [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"): [AsRefVectorParts](../render/render_resource/encase/vector/trait.AsRefVectorParts.html "trait bevy::render::render_resource::encase::vector::AsRefVectorParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [VectorScalar](../render/render_resource/encase/vector/trait.VectorScalar.html "trait bevy::render::render_resource::encase::vector::VectorScalar") + [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#15)

#### fn [write\_into](../render/render_resource/encase/internal/trait.WriteInto.html#tymethod.write_into)<B>(&self, writer: &mut [Writer](../render/render_resource/encase/internal/struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](../render/render_resource/encase/internal/trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#36)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#98)

### impl<V> [Ease](../prelude/trait.Ease.html "trait bevy::prelude::Ease") for V

where V: [VectorSpace](trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#99)

#### fn [interpolating\_curve\_unbounded](../prelude/trait.Ease.html#tymethod.interpolating_curve_unbounded)(start: V, end: V) -> impl [Curve](../prelude/trait.Curve.html "trait bevy::prelude::Curve")<V>

Given `start` and `end` values, produce a curve with [unlimited domain](../prelude/struct.Interval.html#associatedconstant.EVERYWHERE "associated constant bevy::prelude::Interval::EVERYWHERE") that: [Read more](../prelude/trait.Ease.html#tymethod.interpolating_curve_unbounded)

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

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#475-477)

### impl<V> [StableInterpolate](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for V

where V: [NormedVectorSpace](trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#480)

#### fn [interpolate\_stable](../prelude/trait.StableInterpolate.html#tymethod.interpolate_stable)(&self, other: [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> V

Interpolate between this value and the `other` given value using the parameter `t`. At `t = 0.0`, a value equivalent to `self` is recovered, while `t = 1.0` recovers a value equivalent to `other`, with intermediate values interpolating between the two. See the [trait-level documentation](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for details.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#438)

#### fn [interpolate\_stable\_assign](../prelude/trait.StableInterpolate.html#method.interpolate_stable_assign)(&mut self, other: &Self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

A version of [`interpolate_stable`](../prelude/trait.StableInterpolate.html#tymethod.interpolate_stable "method bevy::prelude::StableInterpolate::interpolate_stable") that assigns the result to `self` for convenience.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#467)

#### fn [smooth\_nudge](../prelude/trait.StableInterpolate.html#method.smooth_nudge)(&mut self, target: &Self, decay\_rate: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), delta: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Smoothly nudge this value towards the `target` at a given decay rate. The `decay_rate` parameter controls how fast the distance between `self` and `target` decays relative to the units of `delta`; the intended usage is for `decay_rate` to generally remain fixed, while `delta` is something like `delta_time` from an updating system. This produces a smooth following of the target that is independent of framerate. [Read more](../prelude/trait.StableInterpolate.html#method.smooth_nudge)

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#586)

### impl<T> [TryStableInterpolate](trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate") for T

where T: [StableInterpolate](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#587)

#### type [Error](trait.TryStableInterpolate.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

Error produced when the value cannot be interpolated.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#588)

#### fn [try\_interpolate\_stable](trait.TryStableInterpolate.html#tymethod.try_interpolate_stable)( &self, other: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryStableInterpolate](trait.TryStableInterpolate.html "trait bevy::math::TryStableInterpolate")\>::[Error](trait.TryStableInterpolate.html#associatedtype.Error "type bevy::math::TryStableInterpolate::Error")\>

Attempt to interpolate the value. This may fail if the two interpolation values have different units, or if the type is not interpolable.

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