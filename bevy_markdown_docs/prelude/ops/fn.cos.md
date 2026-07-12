[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function cos 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#318)

```rust
pub fn cos(x: f32) -> f32
```

Computes the cosine of a number (in radians).

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/stress\_tests/transform\_hierarchy.rs ([line 267](../../../src/transform_hierarchy/transform_hierarchy.rs.html#267))

```rust
266fn set_translation(translation: &mut Vec3, a: f32) {
267    translation.x = ops::cos(a) * 32.0;
268    translation.y = ops::sin(a) * 32.0;
269}
```

Hide additional examples

examples/remote/server.rs ([line 81](../../../src/server/server.rs.html#81))

```rust
79fn move_cube(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
80    for mut transform in &mut query {
81        transform.translation.y = -cos(time.elapsed_secs()) + 1.5;
82    }
83}
```

examples/gizmos/axes.rs ([line 204](../../../src/axes/axes.rs.html#204))

```rust
201fn build_direction(height: f32, theta: f32) -> Vec3 {
202    let z = height;
203    let m = ops::sin(ops::acos(z));
204    let x = ops::cos(theta) * m;
205    let y = ops::sin(theta) * m;
206
207    Vec3::new(x, y, z)
208}
```

examples/3d/meshlet.rs ([line 124](../../../src/meshlet/meshlet.rs.html#124))

```rust
122fn bunny_wiggler(mut bunny: Query<&mut Transform, With<BunnyWiggler>>, time: Res<Time>) {
123    bunny.single_mut().as_deref_mut().unwrap().translation.z +=
124        ops::cos(time.elapsed_secs() * 10.0) * 0.003;
125}
```

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 53](../../../src/overflow_debug/overflow_debug.rs.html#53))

```rust
51    fn update(&self, t: f32, transform: &mut UiTransform) {
52        transform.translation.x = percent(ops::sin(t * TAU - FRAC_PI_2) * 50.);
53        transform.translation.y = percent(-ops::cos(t * TAU - FRAC_PI_2) * 50.);
54    }
55}
56
57#[derive(Component)]
58struct Scale;
59
60impl UpdateTransform for Scale {
61    fn update(&self, t: f32, transform: &mut UiTransform) {
62        transform.scale.x = 1.0 + 0.5 * ops::cos(t * TAU).max(0.0);
63        transform.scale.y = 1.0 + 0.5 * ops::cos(t * TAU + PI).max(0.0);
64    }
65}
66
67#[derive(Component)]
68struct Rotate;
69
70impl UpdateTransform for Rotate {
71    fn update(&self, t: f32, transform: &mut UiTransform) {
72        transform.rotation = Rot2::radians(ops::cos(t * TAU) * 45.0);
73    }
```

examples/2d/text2d.rs ([line 182](../../../src/text2d/text2d.rs.html#182))

```rust
176fn animate_translation(
177    time: Res<Time>,
178    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateTranslation>)>,
179) {
180    for mut transform in &mut query {
181        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs()) - 400.0;
182        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
183    }
184}
185
186fn animate_rotation(
187    time: Res<Time>,
188    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
189) {
190    for mut transform in &mut query {
191        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
192    }
193}
```

Additional examples can be found in:  

*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#188)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#225)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../../src/async_channel_pattern/async_channel_pattern.rs.html#163)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#291)
*   [examples/3d/clearcoat.rs](../../../src/clearcoat/clearcoat.rs.html#251)
*   [examples/math/bounding\_2d.rs](../../../src/bounding_2d/bounding_2d.rs.html#286)
*   [examples/math/render\_primitives.rs](../../../src/render_primitives/render_primitives.rs.html#659)
*   [examples/gltf/update\_gltf\_scene.rs](../../../src/update_gltf_scene/update_gltf_scene.rs.html#70)
*   [examples/audio/spatial\_audio\_3d.rs](../../../src/spatial_audio_3d/spatial_audio_3d.rs.html#107)
*   [examples/3d/mesh\_ray\_cast.rs](../../../src/mesh_ray_cast/mesh_ray_cast.rs.html#31)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#600)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#270)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#219)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#439)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#425)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#507)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../../src/many_cameras_lights/many_cameras_lights.rs.html#64)
*   [examples/gizmos/2d\_gizmos.rs](../../../src/2d_gizmos/2d_gizmos.rs.html#83)
*   [examples/2d/mesh2d\_manual.rs](../../../src/mesh2d_manual/mesh2d_manual.rs.html#85)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#140)
*   [examples/3d/fog.rs](../../../src/fog/fog.rs.html#151)