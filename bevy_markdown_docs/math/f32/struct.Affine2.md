[bevy](../../index.html)::[math](../index.html)::[f32](index.html)

# Struct Affine2 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#24)

```rust
#[repr(C)]pub struct Affine2 {
    pub matrix2: Mat2,
    pub translation: Vec2,
}
```

A 2D affine transform, which can represent translation, rotation, scaling and shear.

## Fields

`matrix2: [Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")``translation: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#29)

### impl [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#34)

#### pub const [ZERO](#associatedconstant.ZERO): [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The degenerate zero transform.

This transforms any finite vector and point to zero. The zero transform is non-invertible.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#42)

#### pub const [IDENTITY](#associatedconstant.IDENTITY): [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The identity transform.

Multiplying a vector with this returns the same vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#48)

#### pub const [NAN](#associatedconstant.NAN): [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

All NAN:s.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#56)

#### pub const fn [from\_cols](#method.from_cols)(x\_axis: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), y\_axis: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), z\_axis: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from three column vectors.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#66)

#### pub fn [from\_cols\_array](#method.from_cols_array)(m: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [6](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from a `[f32; 6]` array stored in column major order.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#76)

#### pub fn [to\_cols\_array](#method.to_cols_array)(&self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [6](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Creates a `[f32; 6]` array storing data in column major order.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#89)

#### pub fn [from\_cols\_array\_2d](#method.from_cols_array_2d)(m: &\[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from a `[[f32; 2]; 3]` 2D array stored in column major order. If your data is in row major order you will need to `transpose` the returned matrix.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#101)

#### pub fn [to\_cols\_array\_2d](#method.to_cols_array_2d)(&self) -> \[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Creates a `[[f32; 2]; 3]` 2D array storing data in column major order. If you require data in row major order `transpose` the matrix first.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#116)

#### pub fn [from\_cols\_slice](#method.from_cols_slice)(slice: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from the first 6 values in `slice`.

##### Panics

Panics if `slice` is less than 6 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#129)

#### pub fn [write\_cols\_to\_slice](#method.write_cols_to_slice)(&self, slice: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\])

Writes the columns of `self` to the first 6 elements in `slice`.

##### Panics

Panics if `slice` is less than 6 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#138)

#### pub fn [from\_scale](#method.from_scale)(scale: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform that changes scale. Note that if any scale is zero the transform will be non-invertible.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/asset/repeated\_texture.rs ([line 60](../../../src/repeated_texture/repeated_texture.rs.html#60))

```rust
17fn setup(
18    mut commands: Commands,
19    asset_server: Res<AssetServer>,
20    mut meshes: ResMut<Assets<Mesh>>,
21    mut materials: ResMut<Assets<StandardMaterial>>,
22) {
23    let image_with_default_sampler =
24        asset_server.load("textures/fantasy_ui_borders/panel-border-010.png");
25
26    // central cube with not repeated texture
27    commands.spawn((
28        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
29        MeshMaterial3d(materials.add(StandardMaterial {
30            base_color_texture: Some(image_with_default_sampler.clone()),
31            ..default()
32        })),
33        Transform::from_translation(Vec3::ZERO),
34    ));
35
36    // left cube with repeated texture
37    commands.spawn((
38        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
39        MeshMaterial3d(
40            materials.add(StandardMaterial {
41                base_color_texture: Some(
42                    asset_server
43                        .load_builder()
44                        .with_settings(|s: &mut _| {
45                            *s = ImageLoaderSettings {
46                                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
47                                    // rewriting mode to repeat image,
48                                    address_mode_u: ImageAddressMode::Repeat,
49                                    address_mode_v: ImageAddressMode::Repeat,
50                                    ..default()
51                                }),
52                                ..default()
53                            }
54                        })
55                        .load("textures/fantasy_ui_borders/panel-border-010-repeated.png"),
56                ),
57
58                // uv_transform used here for proportions only, but it is full Affine2
59                // that's why you can use rotation and shift also
60                uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
61                ..default()
62            }),
63        ),
64        Transform::from_xyz(-1.5, 0.0, 0.0),
65    ));
66
67    // right cube with scaled texture, because with default sampler
68    commands.spawn((
69        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
70        MeshMaterial3d(materials.add(StandardMaterial {
71            // there is no sampler set, that's why
72            // by default you see only one small image in a row/column
73            // and other space is filled by image edge
74            base_color_texture: Some(image_with_default_sampler),
75
76            // uv_transform used here for proportions only, but it is full Affine2
77            // that's why you can use rotation and shift also
78            uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
79            ..default()
80        })),
81        Transform::from_xyz(1.5, 0.0, 0.0),
82    ));
83
84    // light
85    commands.spawn((
86        PointLight {
87            shadow_maps_enabled: true,
88            ..default()
89        },
90        Transform::from_xyz(4.0, 8.0, 4.0),
91    ));
92    // camera
93    commands.spawn((
94        Camera3d::default(),
95        Transform::from_xyz(0.0, 1.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
96    ));
97}
```

Hide additional examples

examples/2d/mesh2d\_repeated\_texture.rs ([line 71](../../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#71))

```rust
25fn setup(
26    mut commands: Commands,
27    asset_server: Res<AssetServer>,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<ColorMaterial>>,
30) {
31    // #11111: We use a duplicated image so that it can be load with and without
32    // settings
33    let image_with_default_sampler =
34        asset_server.load("textures/fantasy_ui_borders/panel-border-010.png");
35    let image_with_repeated_sampler = asset_server
36        .load_builder()
37        .with_settings(|s: &mut _| {
38            *s = ImageLoaderSettings {
39                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
40                    // rewriting mode to repeat image,
41                    address_mode_u: ImageAddressMode::Repeat,
42                    address_mode_v: ImageAddressMode::Repeat,
43                    ..default()
44                }),
45                ..default()
46            }
47        })
48        .load("textures/fantasy_ui_borders/panel-border-010-repeated.png");
49
50    // central rectangle with not repeated texture
51    commands.spawn((
52        Mesh2d(meshes.add(Rectangle::new(RECTANGLE_SIDE, RECTANGLE_SIDE))),
53        MeshMaterial2d(materials.add(ColorMaterial {
54            texture: Some(image_with_default_sampler.clone()),
55            ..default()
56        })),
57        Transform::from_translation(Vec3::ZERO),
58        children![(
59            Text2d::new("Control"),
60            Transform::from_xyz(0., LABEL_OFFSET, 0.),
61        )],
62    ));
63
64    // left rectangle with repeated texture
65    commands.spawn((
66        Mesh2d(meshes.add(Rectangle::new(RECTANGLE_SIDE, RECTANGLE_SIDE))),
67        MeshMaterial2d(materials.add(ColorMaterial {
68            texture: Some(image_with_repeated_sampler),
69            // uv_transform used here for proportions only, but it is full Affine2
70            // that's why you can use rotation and shift also
71            uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
72            ..default()
73        })),
74        Transform::from_xyz(-RECTANGLE_OFFSET, 0.0, 0.0),
75        children![(
76            Text2d::new("Repeat On"),
77            Transform::from_xyz(0., LABEL_OFFSET, 0.),
78        )],
79    ));
80
81    // right rectangle with scaled texture, but with default sampler.
82    commands.spawn((
83        Mesh2d(meshes.add(Rectangle::new(RECTANGLE_SIDE, RECTANGLE_SIDE))),
84        MeshMaterial2d(materials.add(ColorMaterial {
85            // there is no sampler set, that's why
86            // by default you see only one small image in a row/column
87            // and other space is filled by image edge
88            texture: Some(image_with_default_sampler),
89
90            // uv_transform used here for proportions only, but it is full Affine2
91            // that's why you can use rotation and shift also
92            uv_transform: Affine2::from_scale(Vec2::new(2., 3.)),
93            ..default()
94        })),
95        Transform::from_xyz(RECTANGLE_OFFSET, 0.0, 0.0),
96        children![(
97            Text2d::new("Repeat Off"),
98            Transform::from_xyz(0., LABEL_OFFSET, 0.),
99        )],
100    ));
101
102    // camera
103    commands.spawn((
104        Camera2d,
105        Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
106    ));
107}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#148)

#### pub fn [from\_angle](#method.from_angle)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from the given rotation `angle`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#158)

#### pub fn [from\_translation](#method.from_translation)(translation: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transformation from the given 2D `translation`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#168)

#### pub fn [from\_mat2](#method.from_mat2)(matrix2: [Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#182)

#### pub fn [from\_mat2\_translation](#method.from_mat2_translation)(matrix2: [Mat2](../../prelude/struct.Mat2.html "struct bevy::prelude::Mat2"), translation: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a translation vector.

Equivalent to `Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#196)

#### pub fn [from\_scale\_angle\_translation](#method.from_scale_angle_translation)( scale: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), translation: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), ) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and `translation`.

Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle) * Affine2::from_scale(scale)`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#210)

#### pub fn [from\_angle\_translation](#method.from_angle_translation)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), translation: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Creates an affine transform from the given 2D rotation `angle` (in radians) and `translation`.

Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#220)

#### pub fn [from\_mat3](#method.from_mat3)(m: [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The given `Mat3` must be an affine transform,

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#231)

#### pub fn [from\_mat3a](#method.from_mat3a)(m: [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The given [`Mat3A`](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A") must be an affine transform,

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#250)

#### pub fn [to\_scale\_angle\_translation](#method.to_scale_angle_translation)(&self) -> ([Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"))

Extracts `scale`, `angle` and `translation` from `self`.

The transform is expected to be non-degenerate and without shearing, or the output will be invalid.

##### Panics

Will panic if the determinant `self.matrix2` is zero or if the resulting scale vector contains any zero elements when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#270)

#### pub fn [transform\_point2](#method.transform_point2)(&self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Transforms the given 2D point, applying shear, scale, rotation and translation.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#279)

#### pub fn [transform\_vector2](#method.transform_vector2)(&self, rhs: [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Transforms the given 2D vector, applying shear, scale and rotation (but NOT translation).

To also apply translation, use [`Self::transform_point2()`](../struct.Affine2.html#method.transform_point2 "method bevy::math::Affine2::transform_point2") instead.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#289)

#### pub fn [is\_finite](#method.is_finite)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite.

If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#296)

#### pub fn [is\_nan](#method.is_nan)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#311)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(&self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2"), max\_abs\_diff: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two 3x4 matrices contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#321)

#### pub fn [inverse](#method.inverse)(&self) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Return the inverse of this transform.

Note that if the transform is not invertible the result will be invalid.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#335)

#### pub fn [as\_daffine2](#method.as_daffine2)(&self) -> [DAffine2](../struct.DAffine2.html "struct bevy::math::DAffine2")

Casts all elements of `self` to `f64`.

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#13)

### impl [AnyBitPattern](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/anybitpattern/trait.AnyBitPattern.html "trait bytemuck::anybitpattern::AnyBitPattern") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#10)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#10)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#10)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#369)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#370)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#340)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#342)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#347)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#348)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = Cols3<[Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>

The resulting type after dereferencing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#350)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#355)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#357)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[Affine2](../struct.Affine2.html "struct bevy::math::Affine2") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Deserialize expects a sequence of 6 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#378)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#379)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#272)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#273)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: &[UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#455)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#457)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(m: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#626)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#628)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(m: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#260)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#261)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#185)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#187)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(iso: [Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#266)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#267)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Affine2](../struct.Affine2.html "struct bevy::math::Affine2"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#405)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#406)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#409)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#417)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#418)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#420)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#425)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#426)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#428)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#508)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#509)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#511)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#516)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for &[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#517)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#519)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#588)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#589)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#591)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#596)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for &[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#597)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#599)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#475)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#476)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#478)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#483)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")\> for &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#484)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#486)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#555)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#556)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#558)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#563)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")\> for &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#564)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#566)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#433)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#434)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#436)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#499)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#500)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#503)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#524)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for &[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#525)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#527)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#579)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#580)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#583)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#604)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for &[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#605)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#607)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#287)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#288)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#291)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, affine2: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> <[UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#466)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#467)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#470)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")) -> <[Affine2](../struct.Affine2.html "struct bevy::math::Affine2") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#491)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")\> for &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#492)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#494)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")) -> [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#546)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#547)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#550)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")) -> <[Affine2](../struct.Affine2.html "struct bevy::math::Affine2") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#571)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")\> for &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#572)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#574)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")) -> [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#296)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#297)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#300)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)( self, transform: [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"), ) -> <[Affine2](../struct.Affine2.html "struct bevy::math::Affine2") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#441)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#443)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#448)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#450)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#539)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#541)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#619)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#621)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#532)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3](../../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#534)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#612)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Mat3A](../../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#614)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Affine2](../struct.Affine2.html "struct bevy::math::Affine2"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#362)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#364)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, rhs: &[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#409)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#409)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#409)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#396)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\> for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#397-399)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Affine2](../struct.Affine2.html "struct bevy::math::Affine2")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

Serialize as a sequence of 6 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [Struct](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [field](../../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [field\_mut](../../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [field\_at](../../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [field\_at\_mut](../../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [name\_at](../../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [index\_of\_name](../../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [field\_len](../../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [iter\_fields](../../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [to\_dynamic\_struct](../../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine2.rs.html#13)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Affine2](../struct.Affine2.html "struct bevy::math::Affine2")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

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

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

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

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}