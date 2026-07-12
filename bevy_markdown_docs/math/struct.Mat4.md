[bevy](../index.html)::[math](index.html)

# Struct Mat4 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#65)

```rust
#[repr(C)]pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}
```

A 4x4 column major matrix.

This 4x4 matrix type features convenience methods for creating and using affine transforms and perspective projections. If you are primarily dealing with 3D affine transformations considering using [`Affine3A`](struct.Affine3A.html "struct bevy::math::Affine3A") which is faster than a 4x4 matrix for some affine operations.

Affine transformations including 3D translation, rotation and scale can be created using methods such as [`Self::from_translation()`](../prelude/struct.Mat4.html#method.from_translation "associated function bevy::prelude::Mat4::from_translation"), [`Self::from_quat()`](../prelude/struct.Mat4.html#method.from_quat "associated function bevy::prelude::Mat4::from_quat"), [`Self::from_scale()`](../prelude/struct.Mat4.html#method.from_scale "associated function bevy::prelude::Mat4::from_scale") and [`Self::from_scale_rotation_translation()`](../prelude/struct.Mat4.html#method.from_scale_rotation_translation "associated function bevy::prelude::Mat4::from_scale_rotation_translation").

Orthographic projections can be created using the methods [`Self::orthographic_lh()`](../prelude/struct.Mat4.html#method.orthographic_lh "associated function bevy::prelude::Mat4::orthographic_lh") for left-handed coordinate systems and [`Self::orthographic_rh()`](../prelude/struct.Mat4.html#method.orthographic_rh "associated function bevy::prelude::Mat4::orthographic_rh") for right-handed systems. The resulting matrix is also an affine transformation.

The [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3") convenience methods are provided for performing affine transformations on 3D vectors and points. These multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0` for vectors respectively. These methods assume that `Self` contains a valid affine transform.

Perspective projections can be created using methods such as [`Self::perspective_lh()`](../prelude/struct.Mat4.html#method.perspective_lh "associated function bevy::prelude::Mat4::perspective_lh"), [`Self::perspective_infinite_lh()`](../prelude/struct.Mat4.html#method.perspective_infinite_lh "associated function bevy::prelude::Mat4::perspective_infinite_lh") and [`Self::perspective_infinite_reverse_lh()`](../prelude/struct.Mat4.html#method.perspective_infinite_reverse_lh "associated function bevy::prelude::Mat4::perspective_infinite_reverse_lh") for left-handed co-ordinate systems and [`Self::perspective_rh()`](../prelude/struct.Mat4.html#method.perspective_rh "associated function bevy::prelude::Mat4::perspective_rh"), [`Self::perspective_infinite_rh()`](../prelude/struct.Mat4.html#method.perspective_infinite_rh "associated function bevy::prelude::Mat4::perspective_infinite_rh") and [`Self::perspective_infinite_reverse_rh()`](../prelude/struct.Mat4.html#method.perspective_infinite_reverse_rh "associated function bevy::prelude::Mat4::perspective_infinite_reverse_rh") for right-handed co-ordinate systems.

The resulting perspective project can be use to transform 3D vectors as points with perspective correction using the [`Self::project_point3()`](../prelude/struct.Mat4.html#method.project_point3 "method bevy::prelude::Mat4::project_point3") convenience method.

## Fields

`x_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")``y_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")``z_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")``w_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")`

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#72)

### impl [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#74)

#### pub const [ZERO](#associatedconstant.ZERO): [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

A 4x4 matrix with all elements set to `0.0`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#77)

#### pub const [IDENTITY](#associatedconstant.IDENTITY): [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

A 4x4 identity matrix, where all diagonal elements are `1`, and all off-diagonal elements are `0`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#80)

#### pub const [NAN](#associatedconstant.NAN): [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

All NAN:s.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#114)

#### pub const fn [from\_cols](#method.from_cols)( x\_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), y\_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), z\_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), w\_axis: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4"), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a 4x4 matrix from four column vectors.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#128)

#### pub const fn [from\_cols\_array](#method.from_cols_array)(m: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a 4x4 matrix from a `[f32; 16]` array stored in column major order. If your data is stored in row major you will need to `transpose` the returned matrix.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#139)

#### pub const fn [to\_cols\_array](#method.to_cols_array)(&self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Creates a `[f32; 16]` array storing data in column major order. If you require data in row major order `transpose` the matrix first.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#156)

#### pub const fn [from\_cols\_array\_2d](#method.from_cols_array_2d)(m: &\[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a 4x4 matrix from a `[[f32; 4]; 4]` 4D array stored in column major order. If your data is in row major order you will need to `transpose` the returned matrix.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/irradiance\_volumes.rs ([lines 59-64](../../src/irradiance_volumes/irradiance_volumes.rs.html#59-64))

```rust
59static VOXEL_FROM_WORLD: Mat4 = Mat4::from_cols_array_2d(&[
60    [-42.317566, 0.0, 0.0, 0.0],
61    [0.0, 0.0, 44.601563, 0.0],
62    [0.0, 16.73776, 0.0, 0.0],
63    [0.0, 6.544792, 0.0, 1.0],
64]);
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#169)

#### pub const fn [to\_cols\_array\_2d](#method.to_cols_array_2d)(&self) -> \[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Creates a `[[f32; 4]; 4]` 4D array storing data in column major order. If you require data in row major order `transpose` the matrix first.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#182)

#### pub const fn [from\_diagonal](#method.from_diagonal)(diagonal: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#226)

#### pub fn [from\_scale\_rotation\_translation](#method.from_scale_rotation_translation)( scale: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), rotation: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), translation: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix from the given 3D `scale`, `rotation` and `translation`.

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

##### Panics

Will panic if `rotation` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#246)

#### pub fn [from\_rotation\_translation](#method.from_rotation_translation)(rotation: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), translation: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix from the given 3D `translation`.

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

##### Panics

Will panic if `rotation` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#260)

#### pub fn [to\_scale\_rotation\_translation](#method.to_scale_rotation_translation)(&self) -> ([Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"))

Extracts `scale`, `rotation` and `translation` from `self`. The input matrix is expected to be a 3D affine transformation matrix otherwise the output will be invalid.

##### Panics

Will panic if the determinant of `self` is zero or if the resulting scale vector contains any zero elements when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#295)

#### pub fn [from\_quat](#method.from_quat)(rotation: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix from the given `rotation` quaternion.

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

##### Panics

Will panic if `rotation` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#307)

#### pub fn [from\_mat3](#method.from_mat3)(m: [Mat3](../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix from the given 3x3 linear transformation matrix.

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#322)

#### pub fn [from\_mat3\_translation](#method.from_mat3_translation)(mat3: [Mat3](../prelude/struct.Mat3.html "struct bevy::prelude::Mat3"), translation: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrics from a 3x3 matrix (expressing scale, shear and rotation) and a translation vector.

Equivalent to `Mat4::from_translation(translation) * Mat4::from_mat3(mat3)`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#338)

#### pub fn [from\_mat3a](#method.from_mat3a)(m: [Mat3A](../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix from the given 3x3 linear transformation matrix.

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 355](../../src/mirror/mirror.rs.html#355))

```rust
341fn calculate_mirror_camera_transform_and_projection(
342    main_camera_transform: &Transform,
343    main_camera_projection: &PerspectiveProjection,
344    mirror_transform: &Transform,
345) -> (Transform, PerspectiveProjection) {
346    // Calculate the reflection matrix (a.k.a. Householder matrix) that will
347    // reflect the scene across the mirror plane.
348    //
349    // Note that you must calculate this in *matrix* form and only *afterward*
350    // convert to a `Transform` instead of composing `Transform`s. This is
351    // because the reflection matrix has non-uniform scale, and composing
352    // transforms can't always handle composition of matrices with non-uniform
353    // scales.
354    let mirror_camera_transform = Transform::from_matrix(
355        Mat4::from_mat3a(reflection_matrix(Vec3::NEG_Z)) * main_camera_transform.to_matrix(),
356    );
357
358    // Compute the distance from the camera to the mirror plane. This will be
359    // used to calculate the distance to the near clip plane for the mirror
360    // world.
361    let distance_from_camera_to_mirror = InfinitePlane3d::new(mirror_transform.rotation * Vec3::Y)
362        .signed_distance(
363            Isometry3d::IDENTITY,
364            mirror_transform.translation - main_camera_transform.translation,
365        );
366
367    // Compute the normal of the mirror plane in view space.
368    let view_from_world = main_camera_transform.compute_affine().matrix3.inverse();
369    let mirror_projection_plane_normal =
370        (view_from_world * (mirror_transform.rotation * Vec3::NEG_Y)).normalize();
371
372    // Compute the final projection. It should match the main camera projection,
373    // except that `near` and `near_normal` should be set to the updated near
374    // plane and near normal plane as above.
375    let mirror_camera_projection = PerspectiveProjection {
376        near_clip_plane: mirror_projection_plane_normal.extend(distance_from_camera_to_mirror),
377        ..*main_camera_projection
378    };
379
380    (mirror_camera_transform, mirror_camera_projection)
381}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#353)

#### pub fn [from\_translation](#method.from_translation)(translation: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix from the given 3D `translation`.

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

tests/3d/test\_invalid\_skinned\_mesh.rs ([line 145](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#145))

```rust
92fn setup_meshes(
93    mut commands: Commands,
94    mut mesh_assets: ResMut<Assets<Mesh>>,
95    mut material_assets: ResMut<Assets<StandardMaterial>>,
96    mut inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
97) {
98    // Create a mesh with two rectangles.
99    let unskinned_mesh = Mesh::new(
100        PrimitiveTopology::TriangleList,
101        RenderAssetUsages::default(),
102    )
103    .with_inserted_attribute(
104        Mesh::ATTRIBUTE_POSITION,
105        vec![
106            [-0.3, -0.3, 0.0],
107            [0.3, -0.3, 0.0],
108            [-0.3, 0.3, 0.0],
109            [0.3, 0.3, 0.0],
110            [-0.4, 0.8, 0.0],
111            [0.4, 0.8, 0.0],
112            [-0.4, 1.8, 0.0],
113            [0.4, 1.8, 0.0],
114        ],
115    )
116    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 8])
117    .with_inserted_indices(Indices::U16(vec![0, 1, 3, 0, 3, 2, 4, 5, 7, 4, 7, 6]));
118
119    // Copy the mesh and add skinning attributes that bind each rectangle to a joint.
120    let skinned_mesh = unskinned_mesh
121        .clone()
122        .with_inserted_attribute(
123            Mesh::ATTRIBUTE_JOINT_INDEX,
124            VertexAttributeValues::Uint16x4(vec![
125                [0, 0, 0, 0],
126                [0, 0, 0, 0],
127                [0, 0, 0, 0],
128                [0, 0, 0, 0],
129                [1, 0, 0, 0],
130                [1, 0, 0, 0],
131                [1, 0, 0, 0],
132                [1, 0, 0, 0],
133            ]),
134        )
135        .with_inserted_attribute(
136            Mesh::ATTRIBUTE_JOINT_WEIGHT,
137            vec![[1.00, 0.00, 0.0, 0.0]; 8],
138        );
139
140    let unskinned_mesh_handle = mesh_assets.add(unskinned_mesh);
141    let skinned_mesh_handle = mesh_assets.add(skinned_mesh);
142
143    let inverse_bindposes_handle = inverse_bindposes_assets.add(vec![
144        Mat4::IDENTITY,
145        Mat4::from_translation(Vec3::new(0.0, -1.3, 0.0)),
146    ]);
147
148    let mesh_material_handle = material_assets.add(StandardMaterial::default());
149
150    let background_material_handle = material_assets.add(StandardMaterial {
151        base_color: Color::srgb(0.05, 0.15, 0.05),
152        reflectance: 0.2,
153        ..default()
154    });
155
156    #[derive(PartialEq)]
157    enum Variation {
158        Normal,
159        MissingMeshAttributes,
160        MissingJointEntity,
161        MissingSkinnedMeshComponent,
162    }
163
164    for (index, variation) in [
165        Variation::Normal,
166        Variation::MissingMeshAttributes,
167        Variation::MissingJointEntity,
168        Variation::MissingSkinnedMeshComponent,
169    ]
170    .into_iter()
171    .enumerate()
172    {
173        // Skip variations that are currently broken. See https://github.com/bevyengine/bevy/issues/16929,
174        // https://github.com/bevyengine/bevy/pull/18074.
175        if (variation == Variation::MissingSkinnedMeshComponent)
176            || (variation == Variation::MissingMeshAttributes)
177        {
178            continue;
179        }
180
181        let transform = Transform::from_xyz(((index as f32) - 1.5) * 4.5, 0.0, 0.0);
182
183        let joint_0 = commands.spawn(transform).id();
184
185        let joint_1 = commands
186            .spawn((ChildOf(joint_0), AnimatedJoint, Transform::IDENTITY))
187            .id();
188
189        if variation == Variation::MissingJointEntity {
190            commands.entity(joint_1).despawn();
191        }
192
193        let mesh_handle = match variation {
194            Variation::MissingMeshAttributes => &unskinned_mesh_handle,
195            _ => &skinned_mesh_handle,
196        };
197
198        let mut entity_commands = commands.spawn((
199            Mesh3d(mesh_handle.clone()),
200            MeshMaterial3d(mesh_material_handle.clone()),
201            transform,
202        ));
203
204        if variation != Variation::MissingSkinnedMeshComponent {
205            entity_commands.insert(SkinnedMesh {
206                inverse_bindposes: inverse_bindposes_handle.clone(),
207                joints: vec![joint_0, joint_1],
208            });
209        }
210
211        // Add a square behind the mesh to distinguish it from the other meshes.
212        commands.spawn((
213            Transform::from_xyz(transform.translation.x, transform.translation.y, -0.8),
214            Mesh3d(mesh_assets.add(Plane3d::default().mesh().size(4.3, 4.3).normal(Dir3::Z))),
215            MeshMaterial3d(background_material_handle.clone()),
216        ));
217    }
218}
```

Hide additional examples

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 188](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#188))

```rust
124fn spawn_custom_meshes(
125    mut commands: Commands,
126    mut mesh_assets: ResMut<Assets<Mesh>>,
127    mut material_assets: ResMut<Assets<StandardMaterial>>,
128    mut inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
129) {
130    let mesh_handle = mesh_assets.add(
131        Mesh::new(
132            PrimitiveTopology::TriangleStrip,
133            // Test that skinned mesh bounds work even if the mesh is render
134            // world only.
135            RenderAssetUsages::RENDER_WORLD,
136        )
137        .with_inserted_attribute(
138            Mesh::ATTRIBUTE_POSITION,
139            vec![
140                [-0.5, 0.0, 0.0],
141                [0.5, 0.0, 0.0],
142                [-0.5, 0.5, 0.0],
143                [0.5, 0.5, 0.0],
144                [-0.5, 1.0, 0.0],
145                [0.5, 1.0, 0.0],
146                [-0.5, 1.5, 0.0],
147                [0.5, 1.5, 0.0],
148                [-0.5, 2.0, 0.0],
149                [0.5, 2.0, 0.0],
150            ],
151        )
152        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 10])
153        .with_inserted_attribute(
154            Mesh::ATTRIBUTE_JOINT_INDEX,
155            VertexAttributeValues::Uint16x4(vec![
156                [1, 0, 0, 0],
157                [1, 0, 0, 0],
158                [1, 2, 0, 0],
159                [1, 2, 0, 0],
160                [1, 2, 0, 0],
161                [1, 2, 0, 0],
162                [2, 1, 0, 0],
163                [2, 1, 0, 0],
164                [2, 0, 0, 0],
165                [2, 0, 0, 0],
166            ]),
167        )
168        .with_inserted_attribute(
169            Mesh::ATTRIBUTE_JOINT_WEIGHT,
170            vec![
171                [1.00, 0.00, 0.0, 0.0],
172                [1.00, 0.00, 0.0, 0.0],
173                [0.75, 0.25, 0.0, 0.0],
174                [0.75, 0.25, 0.0, 0.0],
175                [0.50, 0.50, 0.0, 0.0],
176                [0.50, 0.50, 0.0, 0.0],
177                [0.75, 0.25, 0.0, 0.0],
178                [0.75, 0.25, 0.0, 0.0],
179                [1.00, 0.00, 0.0, 0.0],
180                [1.00, 0.00, 0.0, 0.0],
181            ],
182        )
183        .with_generated_skinned_mesh_bounds()
184        .unwrap(),
185    );
186
187    let inverse_bindposes_handle = inverse_bindposes_assets.add(vec![
188        Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)),
189        Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)),
190        Mat4::from_translation(Vec3::new(0.0, -1.0, 0.0)),
191    ]);
192
193    struct MeshInstance {
194        animations: [CustomAnimationId; 2],
195    }
196
197    let mesh_instances = [
198        // Simple cases. First joint is still, second joint is all rotation/translation/scale variations.
199        MeshInstance { animations: [0, 1] },
200        MeshInstance { animations: [0, 2] },
201        MeshInstance { animations: [0, 3] },
202        MeshInstance { animations: [0, 4] },
203        MeshInstance { animations: [0, 5] },
204        MeshInstance { animations: [0, 6] },
205        MeshInstance { animations: [0, 7] },
206        MeshInstance { animations: [0, 8] },
207        // Skewed cases. First joint is non-uniform scaling, second joint is rotation/translation variations.
208        MeshInstance { animations: [9, 1] },
209        MeshInstance { animations: [9, 2] },
210        MeshInstance { animations: [9, 3] },
211        MeshInstance { animations: [9, 4] },
212        MeshInstance { animations: [9, 5] },
213    ];
214
215    for (i, mesh_instance) in mesh_instances.iter().enumerate() {
216        let x = ((i as f32) * 2.0) - ((mesh_instances.len() - 1) as f32);
217
218        let base_entity = commands
219            .spawn((Transform::from_xyz(x, 0.0, 0.0), Visibility::default()))
220            .id();
221
222        let joints = vec![
223            commands.spawn((Transform::IDENTITY,)).id(),
224            commands
225                .spawn((
226                    CustomAnimation(mesh_instance.animations[0]),
227                    Transform::IDENTITY,
228                ))
229                .id(),
230            commands
231                .spawn((
232                    CustomAnimation(mesh_instance.animations[1]),
233                    Transform::IDENTITY,
234                ))
235                .id(),
236        ];
237
238        commands.entity(joints[0]).insert(ChildOf(base_entity));
239
240        commands.entity(joints[1]).insert(ChildOf(joints[0]));
241        commands.entity(joints[2]).insert(ChildOf(joints[1]));
242
243        let mesh_entity = commands
244            .spawn((
245                Transform::IDENTITY,
246                Mesh3d(mesh_handle.clone()),
247                MeshMaterial3d(material_assets.add(StandardMaterial {
248                    base_color: Color::WHITE,
249                    cull_mode: None,
250                    ..default()
251                })),
252                SkinnedMesh {
253                    inverse_bindposes: inverse_bindposes_handle.clone(),
254                    joints: joints.clone(),
255                },
256                DynamicSkinnedMeshBounds,
257            ))
258            .id();
259
260        commands.entity(mesh_entity).insert(ChildOf(base_entity));
261    }
262}
```

examples/animation/custom\_skinned\_mesh.rs ([line 53](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#53))

```rust
38fn setup(
39    mut commands: Commands,
40    asset_server: Res<AssetServer>,
41    mut meshes: ResMut<Assets<Mesh>>,
42    mut materials: ResMut<Assets<StandardMaterial>>,
43    mut skinned_mesh_inverse_bindposes_assets: ResMut<Assets<SkinnedMeshInverseBindposes>>,
44) {
45    // Create a camera
46    commands.spawn((
47        Camera3d::default(),
48        Transform::from_xyz(2.5, 2.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
49    ));
50
51    // Create inverse bindpose matrices for a skeleton consists of 2 joints
52    let inverse_bindposes = skinned_mesh_inverse_bindposes_assets.add(vec![
53        Mat4::from_translation(Vec3::new(-0.5, -1.0, 0.0)),
54        Mat4::from_translation(Vec3::new(-0.5, -1.0, 0.0)),
55    ]);
56
57    // Create a mesh
58    let mesh = Mesh::new(
59        PrimitiveTopology::TriangleList,
60        RenderAssetUsages::RENDER_WORLD,
61    )
62    // Set mesh vertex positions
63    .with_inserted_attribute(
64        Mesh::ATTRIBUTE_POSITION,
65        vec![
66            [0.0, 0.0, 0.0],
67            [1.0, 0.0, 0.0],
68            [0.0, 0.5, 0.0],
69            [1.0, 0.5, 0.0],
70            [0.0, 1.0, 0.0],
71            [1.0, 1.0, 0.0],
72            [0.0, 1.5, 0.0],
73            [1.0, 1.5, 0.0],
74            [0.0, 2.0, 0.0],
75            [1.0, 2.0, 0.0],
76        ],
77    )
78    // Add UV coordinates that map the left half of the texture since its a 1 x
79    // 2 rectangle.
80    .with_inserted_attribute(
81        Mesh::ATTRIBUTE_UV_0,
82        vec![
83            [0.0, 0.00],
84            [0.5, 0.00],
85            [0.0, 0.25],
86            [0.5, 0.25],
87            [0.0, 0.50],
88            [0.5, 0.50],
89            [0.0, 0.75],
90            [0.5, 0.75],
91            [0.0, 1.00],
92            [0.5, 1.00],
93        ],
94    )
95    // Set mesh vertex normals
96    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 10])
97    // Set mesh vertex joint indices for mesh skinning.
98    // Each vertex gets 4 indices used to address the `JointTransforms` array in the vertex shader
99    //  as well as `SkinnedMeshJoint` array in the `SkinnedMesh` component.
100    // This means that a maximum of 4 joints can affect a single vertex.
101    .with_inserted_attribute(
102        Mesh::ATTRIBUTE_JOINT_INDEX,
103        // Need to be explicit here as [u16; 4] could be either Uint16x4 or Unorm16x4.
104        VertexAttributeValues::Uint16x4(vec![
105            [0, 0, 0, 0],
106            [0, 0, 0, 0],
107            [0, 1, 0, 0],
108            [0, 1, 0, 0],
109            [0, 1, 0, 0],
110            [0, 1, 0, 0],
111            [0, 1, 0, 0],
112            [0, 1, 0, 0],
113            [0, 1, 0, 0],
114            [0, 1, 0, 0],
115        ]),
116    )
117    // Set mesh vertex joint weights for mesh skinning.
118    // Each vertex gets 4 joint weights corresponding to the 4 joint indices assigned to it.
119    // The sum of these weights should equal to 1.
120    .with_inserted_attribute(
121        Mesh::ATTRIBUTE_JOINT_WEIGHT,
122        vec![
123            [1.00, 0.00, 0.0, 0.0],
124            [1.00, 0.00, 0.0, 0.0],
125            [0.75, 0.25, 0.0, 0.0],
126            [0.75, 0.25, 0.0, 0.0],
127            [0.50, 0.50, 0.0, 0.0],
128            [0.50, 0.50, 0.0, 0.0],
129            [0.25, 0.75, 0.0, 0.0],
130            [0.25, 0.75, 0.0, 0.0],
131            [0.00, 1.00, 0.0, 0.0],
132            [0.00, 1.00, 0.0, 0.0],
133        ],
134    )
135    // Tell bevy to construct triangles from a list of vertex indices,
136    // where each 3 vertex indices form a triangle.
137    .with_inserted_indices(Indices::U16(vec![
138        0, 1, 3, 0, 3, 2, 2, 3, 5, 2, 5, 4, 4, 5, 7, 4, 7, 6, 6, 7, 9, 6, 9, 8,
139    ]))
140    // Create skinned mesh bounds. Together with the `DynamicSkinnedMeshBounds`
141    // component, this will ensure the mesh is correctly frustum culled.
142    .with_generated_skinned_mesh_bounds()
143    .unwrap();
144
145    let mesh = meshes.add(mesh);
146
147    // We're seeding the PRNG here to make this example deterministic for testing purposes.
148    // This isn't strictly required in practical use unless you need your app to be deterministic.
149    let mut rng = ChaCha8Rng::seed_from_u64(42);
150
151    for i in -5..5 {
152        // Create joint entities
153        let joint_0 = commands
154            .spawn(Transform::from_xyz(
155                i as f32 * 1.5,
156                0.0,
157                // Move quads back a small amount to avoid Z-fighting and not
158                // obscure the transform gizmos.
159                -(i as f32 * 0.01).abs(),
160            ))
161            .id();
162        let joint_1 = commands.spawn((AnimatedJoint(i), Transform::IDENTITY)).id();
163
164        // Set joint_1 as a child of joint_0.
165        commands.entity(joint_0).add_children(&[joint_1]);
166
167        // Each joint in this vector corresponds to each inverse bindpose matrix in `SkinnedMeshInverseBindposes`.
168        let joint_entities = vec![joint_0, joint_1];
169
170        // Create skinned mesh renderer. Note that its transform doesn't affect the position of the mesh.
171        commands.spawn((
172            Mesh3d(mesh.clone()),
173            MeshMaterial3d(materials.add(StandardMaterial {
174                base_color: Color::srgb(
175                    rng.random_range(0.0..1.0),
176                    rng.random_range(0.0..1.0),
177                    rng.random_range(0.0..1.0),
178                ),
179                base_color_texture: Some(asset_server.load("textures/uv_checker_bw.png")),
180                ..default()
181            })),
182            SkinnedMesh {
183                inverse_bindposes: inverse_bindposes.clone(),
184                joints: joint_entities,
185            },
186            DynamicSkinnedMeshBounds,
187        ));
188    }
189}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#373)

#### pub fn [from\_axis\_angle](#method.from_axis_angle)(axis: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix containing a 3D rotation around a normalized rotation `axis` of `angle` (in radians).

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

##### Panics

Will panic if `axis` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#413)

#### pub fn [from\_euler](#method.from_euler)(order: [EulerRot](../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot"), a: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), c: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a affine transformation matrix containing a rotation from the given euler rotation sequence and angles (in radians).

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#428)

#### pub fn [to\_euler](#method.to_euler)(&self, order: [EulerRot](../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")) -> ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Extract Euler angles with the given Euler rotation order.

Note if the upper 3x3 matrix contain scales, shears, or other non-rotation transformations then the resulting Euler angles will be ill-defined.

##### Panics

Will panic if any column of the upper 3x3 rotation matrix is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#444)

#### pub fn [from\_rotation\_x](#method.from_rotation_x)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix containing a 3D rotation around the x axis of `angle` (in radians).

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#461)

#### pub fn [from\_rotation\_y](#method.from_rotation_y)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix containing a 3D rotation around the y axis of `angle` (in radians).

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#478)

#### pub fn [from\_rotation\_z](#method.from_rotation_z)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix containing a 3D rotation around the z axis of `angle` (in radians).

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#498)

#### pub fn [from\_scale](#method.from_scale)(scale: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an affine transformation matrix containing the given 3D non-uniform `scale`.

The resulting matrix can be used to transform 3D points and vectors. See [`Self::transform_point3()`](../prelude/struct.Mat4.html#method.transform_point3 "method bevy::prelude::Mat4::transform_point3") and [`Self::transform_vector3()`](../prelude/struct.Mat4.html#method.transform_vector3 "method bevy::prelude::Mat4::transform_vector3").

##### Panics

Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#517)

#### pub const fn [from\_cols\_slice](#method.from_cols_slice)(slice: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a 4x4 matrix from the first 16 values in `slice`.

##### Panics

Panics if `slice` is less than 16 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#530)

#### pub fn [write\_cols\_to\_slice](#method.write_cols_to_slice)(&self, slice: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\])

Writes the columns of `self` to the first 16 elements in `slice`.

##### Panics

Panics if `slice` is less than 16 elements long.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#556)

#### pub fn [col](#method.col)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the matrix column for the given `index`.

##### Panics

Panics if `index` is greater than 3.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#572)

#### pub fn [col\_mut](#method.col_mut)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &mut [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns a mutable reference to the matrix column for the given `index`.

##### Panics

Panics if `index` is greater than 3.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/camera/custom\_projection.rs ([line 25](../../src/custom_projection/custom_projection.rs.html#25))

```rust
23    fn get_clip_from_view(&self) -> Mat4 {
24        let mut mat = self.perspective.get_clip_from_view();
25        mat.col_mut(2)[0] = self.horizontal_obliqueness;
26        mat.col_mut(2)[1] = self.vertical_obliqueness;
27        mat
28    }
29
30    fn get_clip_from_view_for_sub(&self, sub_view: &bevy::camera::SubCameraView) -> Mat4 {
31        let mut mat = self.perspective.get_clip_from_view_for_sub(sub_view);
32        mat.col_mut(2)[0] = self.horizontal_obliqueness;
33        mat.col_mut(2)[1] = self.vertical_obliqueness;
34        mat
35    }
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#589)

#### pub fn [row](#method.row)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the matrix row for the given `index`.

##### Panics

Panics if `index` is greater than 3.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#603)

#### pub fn [is\_finite](#method.is_finite)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#613)

#### pub fn [is\_nan](#method.is_nan)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NaN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#620)

#### pub fn [transpose](#method.transpose)(&self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Returns the transpose of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#640)

#### pub fn [diagonal](#method.diagonal)(&self) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Returns the diagonal of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#646)

#### pub fn [determinant](#method.determinant)(&self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the determinant of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#857)

#### pub fn [inverse](#method.inverse)(&self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Returns the inverse of `self`.

If the matrix is not invertible the returned matrix will be invalid.

##### Panics

Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/3d/irradiance\_volumes.rs ([line 557](../../src/irradiance_volumes/irradiance_volumes.rs.html#557))

```rust
528fn create_cubes(
529    image_assets: Res<Assets<Image>>,
530    mut commands: Commands,
531    irradiance_volumes: Query<(&IrradianceVolume, &GlobalTransform)>,
532    voxel_cube_parents: Query<Entity, With<VoxelCubeParent>>,
533    voxel_cubes: Query<Entity, With<VoxelCube>>,
534    example_assets: Res<ExampleAssets>,
535    mut voxel_visualization_material_assets: ResMut<Assets<VoxelVisualizationMaterial>>,
536) {
537    // If voxel cubes have already been spawned, don't do anything.
538    if !voxel_cubes.is_empty() {
539        return;
540    }
541
542    let Some(voxel_cube_parent) = voxel_cube_parents.iter().next() else {
543        return;
544    };
545
546    for (irradiance_volume, global_transform) in irradiance_volumes.iter() {
547        let Some(image) = image_assets.get(&irradiance_volume.voxels) else {
548            continue;
549        };
550
551        let resolution = image.texture_descriptor.size;
552
553        let voxel_cube_material = voxel_visualization_material_assets.add(ExtendedMaterial {
554            base: StandardMaterial::from(Color::from(RED)),
555            extension: VoxelVisualizationExtension {
556                irradiance_volume_info: VoxelVisualizationIrradianceVolumeInfo {
557                    world_from_voxel: VOXEL_FROM_WORLD.inverse(),
558                    voxel_from_world: VOXEL_FROM_WORLD,
559                    resolution: uvec3(
560                        resolution.width,
561                        resolution.height,
562                        resolution.depth_or_array_layers,
563                    ),
564                    intensity: IRRADIANCE_VOLUME_INTENSITY,
565                },
566            },
567        });
568
569        let scale = vec3(
570            1.0 / resolution.width as f32,
571            1.0 / resolution.height as f32,
572            1.0 / resolution.depth_or_array_layers as f32,
573        );
574
575        // Spawn a cube for each voxel.
576        for z in 0..resolution.depth_or_array_layers {
577            for y in 0..resolution.height {
578                for x in 0..resolution.width {
579                    let uvw = (uvec3(x, y, z).as_vec3() + 0.5) * scale - 0.5;
580                    let pos = global_transform.transform_point(uvw);
581                    let voxel_cube = commands
582                        .spawn((
583                            Mesh3d(example_assets.voxel_cube.clone()),
584                            MeshMaterial3d(voxel_cube_material.clone()),
585                            Transform::from_scale(Vec3::splat(VOXEL_CUBE_SCALE))
586                                .with_translation(pos),
587                        ))
588                        .insert(VoxelCube)
589                        .insert(NotShadowCaster)
590                        .id();
591
592                    commands.entity(voxel_cube_parent).add_child(voxel_cube);
593                }
594            }
595        }
596    }
597}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#863)

#### pub fn [try\_inverse](#method.try_inverse)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>

Returns the inverse of `self` or `None` if the matrix is not invertible.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#874)

#### pub fn [inverse\_or\_zero](#method.inverse_or_zero)(&self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Returns the inverse of `self` or `Mat4::ZERO` if the matrix is not invertible.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#888)

#### pub fn [look\_to\_lh](#method.look_to_lh)(eye: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), dir: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a left-handed view matrix using a camera position, a facing direction and an up direction

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

##### Panics

Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#902)

#### pub fn [look\_to\_rh](#method.look_to_rh)(eye: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), dir: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed view matrix using a camera position, a facing direction, and an up direction.

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

##### Panics

Will panic if `dir` or `up` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#927)

#### pub fn [look\_at\_lh](#method.look_at_lh)(eye: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), center: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a left-handed view matrix using a camera position, a focal points and an up direction.

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

##### Panics

Will panic if `up` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#940)

#### pub fn [look\_at\_rh](#method.look_at_rh)(eye: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), center: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed view matrix using a camera position, a focal point, and an up direction.

For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

##### Panics

Will panic if `up` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#951-958)

#### pub fn [frustum\_rh\_gl](#method.frustum_rh_gl)( left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed perspective projection matrix with \[-1,1\] depth range.

This is the same as the OpenGL `glFrustum` function.

See [https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml](https://registry.khronos.org/OpenGL-Refpages/gl2.1/xhtml/glFrustum.xml)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#983-990)

#### pub fn [frustum\_lh](#method.frustum_lh)( left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a left-handed perspective projection matrix with `[0,1]` depth range.

##### Panics

Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1016-1023)

#### pub fn [frustum\_rh](#method.frustum_rh)( left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed perspective projection matrix with `[0,1]` depth range.

##### Panics

Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1049-1054)

#### pub fn [perspective\_rh\_gl](#method.perspective_rh_gl)( fov\_y\_radians: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), aspect\_ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed perspective projection matrix with `[-1,1]` depth range.

Useful to map the standard right-handed coordinate system into what OpenGL expects.

This is the same as the OpenGL `gluPerspective` function. See [https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml](https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1078)

#### pub fn [perspective\_lh](#method.perspective_lh)( fov\_y\_radians: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), aspect\_ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a left-handed perspective projection matrix with `[0,1]` depth range.

Useful to map the standard left-handed coordinate system into what WebGPU/Metal/Direct3D expect.

##### Panics

Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1102)

#### pub fn [perspective\_rh](#method.perspective_rh)( fov\_y\_radians: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), aspect\_ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed perspective projection matrix with `[0,1]` depth range.

Useful to map the standard right-handed coordinate system into what WebGPU/Metal/Direct3D expect.

##### Panics

Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1127)

#### pub fn [perspective\_infinite\_lh](#method.perspective_infinite_lh)( fov\_y\_radians: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), aspect\_ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.

Like `perspective_lh`, but with an infinite value for `z_far`. The result is that points near `z_near` are mapped to depth `0`, and as they move towards infinity the depth approaches `1`.

##### Panics

Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1149-1153)

#### pub fn [perspective\_infinite\_reverse\_lh](#method.perspective_infinite_reverse_lh)( fov\_y\_radians: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), aspect\_ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an infinite reverse left-handed perspective projection matrix with `[0,1]` depth range.

Similar to `perspective_infinite_lh`, but maps `Z = z_near` to a depth of `1` and `Z = infinity` to a depth of `0`.

##### Panics

Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1177)

#### pub fn [perspective\_infinite\_rh](#method.perspective_infinite_rh)( fov\_y\_radians: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), aspect\_ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an infinite right-handed perspective projection matrix with `[0,1]` depth range.

Like `perspective_rh`, but with an infinite value for `z_far`. The result is that points near `z_near` are mapped to depth `0`, and as they move towards infinity the depth approaches `1`.

##### Panics

Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1197-1201)

#### pub fn [perspective\_infinite\_reverse\_rh](#method.perspective_infinite_reverse_rh)( fov\_y\_radians: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), aspect\_ratio: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z\_near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates an infinite reverse right-handed perspective projection matrix with `[0,1]` depth range.

Similar to `perspective_infinite_rh`, but maps `Z = z_near` to a depth of `1` and `Z = infinity` to a depth of `0`.

##### Panics

Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1220-1227)

#### pub fn [orthographic\_rh\_gl](#method.orthographic_rh_gl)( left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed orthographic projection matrix with `[-1,1]` depth range. This is the same as the OpenGL `glOrtho` function in OpenGL. See [https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml](https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml)

Useful to map a right-handed coordinate system to the normalized device coordinates that OpenGL expects.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1248-1255)

#### pub fn [orthographic\_lh](#method.orthographic_lh)( left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a left-handed orthographic projection matrix with `[0,1]` depth range.

Useful to map a left-handed coordinate system to the normalized device coordinates that WebGPU/Direct3D/Metal expect.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1277-1284)

#### pub fn [orthographic\_rh](#method.orthographic_rh)( left: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), right: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), bottom: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), top: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), near: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), far: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), ) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Creates a right-handed orthographic projection matrix with `[0,1]` depth range.

Useful to map a right-handed coordinate system to the normalized device coordinates that WebGPU/Direct3D/Metal expect.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1309)

#### pub fn [project\_point3](#method.project_point3)(&self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Transforms the given 3D vector as a point, applying perspective correction.

This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`. The perspective divide is performed meaning the resulting 3D vector is divided by `w`.

This method assumes that `self` contains a projective transform.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1332)

#### pub fn [transform\_point3](#method.transform_point3)(&self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Transforms the given 3D vector as a point.

This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.

This method assumes that `self` contains a valid affine transform. It does not perform a perspective divide, if `self` contains a perspective transform, or if you are unsure, the [`Self::project_point3()`](../prelude/struct.Mat4.html#method.project_point3 "method bevy::prelude::Mat4::project_point3") method should be used instead.

##### Panics

Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1353)

#### pub fn [transform\_vector3](#method.transform_vector3)(&self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Transforms the give 3D vector as a direction.

This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `0.0`.

This method assumes that `self` contains a valid affine transform.

##### Panics

Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1369)

#### pub fn [project\_point3a](#method.project_point3a)(&self, rhs: [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Transforms the given [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") as a 3D point, applying perspective correction.

This is the equivalent of multiplying the [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") as a 4D vector where `w` is `1.0`. The perspective divide is performed meaning the resulting 3D vector is divided by `w`.

This method assumes that `self` contains a projective transform.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1383)

#### pub fn [transform\_point3a](#method.transform_point3a)(&self, rhs: [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Transforms the given [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") as 3D point.

This is the equivalent of multiplying the [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") as a 4D vector where `w` is `1.0`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1397)

#### pub fn [transform\_vector3a](#method.transform_vector3a)(&self, rhs: [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Transforms the give [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") as 3D vector.

This is the equivalent of multiplying the [`Vec3A`](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A") as a 4D vector where `w` is `0.0`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1408)

#### pub fn [mul\_vec4](#method.mul_vec4)(&self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Transforms a 4D vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1419)

#### pub fn [mul\_transpose\_vec4](#method.mul_transpose_vec4)(&self, rhs: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Transforms a 4D vector by the transpose of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1431)

#### pub fn [mul\_mat4](#method.mul_mat4)(&self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Multiplies two 4x4 matrices.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1438)

#### pub fn [add\_mat4](#method.add_mat4)(&self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Adds two 4x4 matrices.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1445)

#### pub fn [sub\_mat4](#method.sub_mat4)(&self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Subtracts two 4x4 matrices.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1452)

#### pub fn [mul\_scalar](#method.mul_scalar)(&self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Multiplies a 4x4 matrix by a scalar.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1466)

#### pub fn [mul\_diagonal\_scale](#method.mul_diagonal_scale)(&self, scale: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Multiply `self` by a scaling vector `scale`. This is faster than creating a whole diagonal scaling matrix and then multiplying that. This operation is commutative.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1478)

#### pub fn [div\_scalar](#method.div_scalar)(&self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Divides a 4x4 matrix by a scalar.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1491)

#### pub fn [recip](#method.recip)(&self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Returns a matrix containing the reciprocal `1.0/n` of each element of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1511)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(&self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"), max\_abs\_diff: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two matrices contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1521)

#### pub fn [abs](#method.abs)(&self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Takes the absolute value of each element in `self`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1531)

#### pub fn [as\_dmat4](#method.as_dmat4)(&self) -> [DMat4](struct.DMat4.html "struct bevy::math::DMat4")

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1548)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1549)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1551)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1561)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1562)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1564)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1569)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1570)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1572)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1577)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1578)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1580)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1585)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1587)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1592)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1594)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1968)

### impl [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1970)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

### impl [AsMutMatrixParts](../render/render_resource/encase/matrix/trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../render/render_resource/encase/matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

#### fn [as\_mut\_parts](../render/render_resource/encase/matrix/trait.AsMutMatrixParts.html#tymethod.as_mut_parts)(&mut self) -> &mut \[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1961)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1963)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

### impl [AsRefMatrixParts](../render/render_resource/encase/matrix/trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [16](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../render/render_resource/encase/matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

#### fn [as\_ref\_parts](../render/render_resource/encase/matrix/trait.AsRefMatrixParts.html#tymethod.as_ref_parts)(&self) -> &\[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#58)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#58)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#58)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [FromMatrixParts](../render/render_resource/encase/matrix/trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../render/render_resource/encase/matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [CreateFrom](../render/render_resource/encase/internal/trait.CreateFrom.html "trait bevy::render::render_resource::encase::internal::CreateFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

#### fn [create\_from](../render/render_resource/encase/internal/trait.CreateFrom.html#tymethod.create_from)<B>(reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1975)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1976)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1541)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1543)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Deserialize expects a sequence of 16 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1986)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1987)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1845)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1846)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1848)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1853)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1854)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1856)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1877)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1878)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1880)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1885)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1886)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1888)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1832)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1833)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1835)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> <[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) as [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output "type core::ops::arith::Div::Output")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1861)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1862)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1864)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1869)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1870)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1872)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1893)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1894)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1896)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1908)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1910)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1901)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1903)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#658)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Affine3](struct.Affine3.html "struct bevy::math::Affine3")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#660)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(m: [Affine3](struct.Affine3.html "struct bevy::math::Affine3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#691)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#693)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(m: [Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#314)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [GlobalTransform](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#315)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(world\_from\_local: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [GlobalTransform](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

### impl [FromMatrixParts](../render/render_resource/encase/matrix/trait.FromMatrixParts.html "trait bevy::render::render_resource::encase::matrix::FromMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#57)

#### fn [from\_parts](../render/render_resource/encase/matrix/trait.FromMatrixParts.html#tymethod.from_parts)(parts: \[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]; [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1671)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1672)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1674)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#620)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine3](struct.Affine3.html "struct bevy::math::Affine3")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#621)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#623)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine3](struct.Affine3.html "struct bevy::math::Affine3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#628)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine3](struct.Affine3.html "struct bevy::math::Affine3")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#629)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#631)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine3](struct.Affine3.html "struct bevy::math::Affine3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#653)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#654)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#656)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#661)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#662)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#664)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#587)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Affine3](struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#588)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#590)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#595)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Affine3](struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#596)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#598)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#620)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#621)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#623)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#628)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#629)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#631)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1684)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1685)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1687)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1692)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1693)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1695)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1762)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1763)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1765)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1770)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1771)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1773)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1794)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1795)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1797)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1802)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1803)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1805)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#611)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine3](struct.Affine3.html "struct bevy::math::Affine3")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#612)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#615)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine3](struct.Affine3.html "struct bevy::math::Affine3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#636)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine3](struct.Affine3.html "struct bevy::math::Affine3")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#637)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#639)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine3](struct.Affine3.html "struct bevy::math::Affine3")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#644)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#645)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#648)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#669)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#670)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#672)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#578)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Affine3](struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#579)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#582)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> <[Affine3](struct.Affine3.html "struct bevy::math::Affine3") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#603)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Affine3](struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#604)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#606)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#611)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#612)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#615)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> <[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#636)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#637)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#639)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1700)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1701)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1703)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1754)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1755)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1757)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> <[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html) as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1778)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1779)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1781)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1786)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1787)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1789)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1810)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1811)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1813)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1708)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1710)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#651)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Affine3](struct.Affine3.html "struct bevy::math::Affine3")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#653)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Affine3](struct.Affine3.html "struct bevy::math::Affine3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#684)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#686)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1715)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1717)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1825)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1827)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#644)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[Affine3](struct.Affine3.html "struct bevy::math::Affine3")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3.rs.html#646)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Affine3](struct.Affine3.html "struct bevy::math::Affine3"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#677)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/affine3a.rs.html#679)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1818)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1820)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1650)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1651)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1653)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> <[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4") as [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output "type core::ops::arith::Neg::Output")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1663)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1664)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1666)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1951)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1953)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#370)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#370)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#370)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#59)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1933)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1934-1936)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1942)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1943-1945)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsMutMatrixParts](../render/render_resource/encase/matrix/trait.AsMutMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsMutMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../render/render_resource/encase/matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [ReadFrom](../render/render_resource/encase/internal/trait.ReadFrom.html "trait bevy::render::render_resource::encase::internal::ReadFrom"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

#### fn [read\_from](../render/render_resource/encase/internal/trait.ReadFrom.html#tymethod.read_from)<B>(&mut self, reader: &mut [Reader](../render/render_resource/encase/internal/struct.Reader.html "struct bevy::render::render_resource::encase::internal::Reader")<B>)

where B: [BufferRef](../render/render_resource/encase/internal/trait.BufferRef.html "trait bevy::render::render_resource::encase::internal::BufferRef"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Serialize as a sequence of 16 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [ShaderSize](../render/render_resource/trait.ShaderSize.html "trait bevy::render::render_resource::ShaderSize"),

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/traits.rs.html#232)

#### const [SHADER\_SIZE](../render/render_resource/trait.ShaderSize.html#associatedconstant.SHADER_SIZE): [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> = \_

Represents [WGSL Size](https://gpuweb.github.io/gpuweb/wgsl/#alignment-and-size) (equivalent to [`ShaderType::min_size`](../render/render_resource/trait.ShaderType.html#method.min_size "associated function bevy::render::render_resource::ShaderType::min_size"))

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1599)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1600)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1602)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1612)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1613)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1615)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1620)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1621)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1623)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1628)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1629)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1631)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1636)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1638)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1643)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1645)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1915)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1916-1918)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1924)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\> for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#1925-1927)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#25)

### impl [TransformPoint](../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#27)

#### fn [transform\_point](../prelude/trait.TransformPoint.html#tymethod.transform_point)(&self, point: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Transform a point.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

### impl [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

where [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4"): [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType")<ExtraMetadata = MatrixMetadata> + [AsRefMatrixParts](../render/render_resource/encase/matrix/trait.AsRefMatrixParts.html "trait bevy::render::render_resource::encase::matrix::AsRefMatrixParts")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), 4, 4>, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [MatrixScalar](../render/render_resource/encase/matrix/trait.MatrixScalar.html "trait bevy::render::render_resource::encase::matrix::MatrixScalar") + [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_encase.rs.html#61)

#### fn [write\_into](../render/render_resource/encase/internal/trait.WriteInto.html#tymethod.write_into)<B>(&self, writer: &mut [Writer](../render/render_resource/encase/internal/struct.Writer.html "struct bevy::render::render_resource::encase::internal::Writer")<B>)

where B: [BufferMut](../render/render_resource/encase/internal/trait.BufferMut.html "trait bevy::render::render_resource::encase::internal::BufferMut"),

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/mat4.rs.html#59)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

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