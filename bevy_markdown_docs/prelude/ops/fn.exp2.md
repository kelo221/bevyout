[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function exp2 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#259)

```rust
pub fn exp2(x: f32) -> f32
```

Returns `2^(self)`.

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/gizmos/axes.rs ([line 169](../../../src/axes/axes.rs.html#169))

```rust
160fn random_scale(rng: &mut impl RngExt) -> Vec3 {
161    let x_factor_log = rng.random::<f32>() * (SCALING_BOUND_UPPER_LOG - SCALING_BOUND_LOWER_LOG)
162        + SCALING_BOUND_LOWER_LOG;
163    let y_factor_log = rng.random::<f32>() * (SCALING_BOUND_UPPER_LOG - SCALING_BOUND_LOWER_LOG)
164        + SCALING_BOUND_LOWER_LOG;
165    let z_factor_log = rng.random::<f32>() * (SCALING_BOUND_UPPER_LOG - SCALING_BOUND_LOWER_LOG)
166        + SCALING_BOUND_LOWER_LOG;
167
168    Vec3::new(
169        ops::exp2(x_factor_log),
170        ops::exp2(y_factor_log),
171        ops::exp2(z_factor_log),
172    )
173}
174
175fn elerp(v1: Vec3, v2: Vec3, t: f32) -> Vec3 {
176    let x_factor_log = (1. - t) * ops::log2(v1.x) + t * ops::log2(v2.x);
177    let y_factor_log = (1. - t) * ops::log2(v1.y) + t * ops::log2(v2.y);
178    let z_factor_log = (1. - t) * ops::log2(v1.z) + t * ops::log2(v2.z);
179
180    Vec3::new(
181        ops::exp2(x_factor_log),
182        ops::exp2(y_factor_log),
183        ops::exp2(z_factor_log),
184    )
185}
```

Hide additional examples

examples/3d/parallax\_mapping.rs ([line 146](../../../src/parallax_mapping/parallax_mapping.rs.html#146))

```rust
131fn update_parallax_layers(
132    input: Res<ButtonInput<KeyCode>>,
133    mut materials: ResMut<Assets<StandardMaterial>>,
134    mut target_layers: Local<TargetLayers>,
135    text: Single<Entity, With<Text>>,
136    mut writer: TextUiWriter,
137) {
138    if input.just_pressed(KeyCode::Digit3) {
139        target_layers.0 -= 1.0;
140        target_layers.0 = target_layers.0.max(0.0);
141    } else if input.just_pressed(KeyCode::Digit4) {
142        target_layers.0 += 1.0;
143    } else {
144        return;
145    }
146    let layer_count = ops::exp2(target_layers.0);
147    let text_entity = *text;
148    *writer.text(text_entity, 2) = format!("Layers: {layer_count:.0}\n");
149
150    for (_, mat) in materials.iter_mut() {
151        mat.max_parallax_layer_count = layer_count;
152    }
153}
154
155fn spin(time: Res<Time>, mut query: Query<(&mut Transform, &Spin)>) {
156    for (mut transform, spin) in query.iter_mut() {
157        transform.rotate_local_y(spin.speed * time.delta_secs());
158        transform.rotate_local_x(spin.speed * time.delta_secs());
159        transform.rotate_local_z(-spin.speed * time.delta_secs());
160    }
161}
162
163// Camera positions to cycle through when left-clicking.
164const CAMERA_POSITIONS: &[Transform] = &[
165    Transform {
166        translation: Vec3::new(1.5, 1.5, 1.5),
167        rotation: Quat::from_xyzw(-0.279, 0.364, 0.115, 0.880),
168        scale: Vec3::ONE,
169    },
170    Transform {
171        translation: Vec3::new(2.4, 0.0, 0.2),
172        rotation: Quat::from_xyzw(0.094, 0.676, 0.116, 0.721),
173        scale: Vec3::ONE,
174    },
175    Transform {
176        translation: Vec3::new(2.4, 2.6, -4.3),
177        rotation: Quat::from_xyzw(0.170, 0.908, 0.308, 0.225),
178        scale: Vec3::ONE,
179    },
180    Transform {
181        translation: Vec3::new(-1.0, 0.8, -1.2),
182        rotation: Quat::from_xyzw(-0.004, 0.909, 0.247, -0.335),
183        scale: Vec3::ONE,
184    },
185];
186
187fn move_camera(
188    mut camera: Single<&mut Transform, With<FreeCameraController>>,
189    mut current_view: Local<usize>,
190    button: Res<ButtonInput<MouseButton>>,
191) {
192    if button.just_pressed(MouseButton::Left) {
193        *current_view = (*current_view + 1) % CAMERA_POSITIONS.len();
194    }
195    let target = CAMERA_POSITIONS[*current_view];
196    camera.translation = camera.translation.lerp(target.translation, 0.2);
197    camera.rotation = camera.rotation.slerp(target.rotation, 0.2);
198}
199
200fn setup(
201    mut commands: Commands,
202    mut materials: ResMut<Assets<StandardMaterial>>,
203    mut meshes: ResMut<Assets<Mesh>>,
204    asset_server: Res<AssetServer>,
205) {
206    // The normal map. Note that to generate it in the GIMP image editor, you should
207    // open the depth map, and do Filters → Generic → Normal Map
208    // You should enable the "flip X" checkbox.
209    let normal_handle = asset_server
210        .load_builder()
211        .with_settings(
212            // The normal map texture is in linear color space. Lighting won't look correct
213            // if `is_srgb` is `true`, which is the default.
214            |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
215        )
216        .load("textures/parallax_example/cube_normal.png");
217
218    // Camera
219    commands.spawn((
220        Camera3d::default(),
221        Transform::from_xyz(1.5, 1.5, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
222        FreeCameraController,
223    ));
224
225    // represent the light source as a sphere
226    let mesh = meshes.add(Sphere::new(0.05).mesh().ico(3).unwrap());
227
228    // light
229    commands.spawn((
230        PointLight {
231            shadow_maps_enabled: true,
232            ..default()
233        },
234        Transform::from_xyz(2.0, 1.0, -1.1),
235        children![(Mesh3d(mesh), MeshMaterial3d(materials.add(Color::WHITE)))],
236    ));
237
238    // Plane
239    commands.spawn((
240        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
241        MeshMaterial3d(materials.add(StandardMaterial {
242            // standard material derived from dark green, but
243            // with roughness and reflectance set.
244            perceptual_roughness: 0.45,
245            reflectance: 0.18,
246            ..Color::srgb_u8(0, 80, 0).into()
247        })),
248        Transform::from_xyz(0.0, -1.0, 0.0),
249    ));
250
251    let parallax_depth_scale = TargetDepth::default().0;
252    let max_parallax_layer_count = ops::exp2(TargetLayers::default().0);
253    let parallax_mapping_method = CurrentMethod::default();
254    let parallax_material = materials.add(StandardMaterial {
255        perceptual_roughness: 0.4,
256        base_color_texture: Some(asset_server.load("textures/parallax_example/cube_color.png")),
257        normal_map_texture: Some(normal_handle),
258        // The depth map is a grayscale texture where black is the highest level and
259        // white the lowest.
260        depth_map: Some(asset_server.load("textures/parallax_example/cube_depth.png")),
261        parallax_depth_scale,
262        parallax_mapping_method: parallax_mapping_method.0,
263        max_parallax_layer_count,
264        ..default()
265    });
266    commands.spawn((
267        Mesh3d(
268            meshes.add(
269                // NOTE: for normal maps and depth maps to work, the mesh
270                // needs tangents generated.
271                Mesh::from(Cuboid::default())
272                    .with_generated_tangents()
273                    .unwrap(),
274            ),
275        ),
276        MeshMaterial3d(parallax_material.clone()),
277        Spin { speed: 0.3 },
278    ));
279
280    let background_cube = meshes.add(
281        Mesh::from(Cuboid::new(40.0, 40.0, 40.0))
282            .with_generated_tangents()
283            .unwrap(),
284    );
285
286    let background_cube_bundle = |translation| {
287        (
288            Mesh3d(background_cube.clone()),
289            MeshMaterial3d(parallax_material.clone()),
290            Transform::from_translation(translation),
291            Spin { speed: -0.1 },
292        )
293    };
294    commands.spawn(background_cube_bundle(Vec3::new(45., 0., 0.)));
295    commands.spawn(background_cube_bundle(Vec3::new(-45., 0., 0.)));
296    commands.spawn(background_cube_bundle(Vec3::new(0., 0., 45.)));
297    commands.spawn(background_cube_bundle(Vec3::new(0., 0., -45.)));
298
299    // example instructions
300    commands.spawn((
301        Text::default(),
302        Node {
303            position_type: PositionType::Absolute,
304            top: px(12),
305            left: px(12),
306            ..default()
307        },
308        children![
309            (TextSpan(format!("Parallax depth scale: {parallax_depth_scale:.5}\n"))),
310            (TextSpan(format!("Layers: {max_parallax_layer_count:.0}\n"))),
311            (TextSpan(format!("{parallax_mapping_method}\n"))),
312            (TextSpan::new("\n\n")),
313            (TextSpan::new("Controls:\n")),
314            (TextSpan::new("Left click - Change view angle\n")),
315            (TextSpan::new("1/2 - Decrease/Increase parallax depth scale\n",)),
316            (TextSpan::new("3/4 - Decrease/Increase layer count\n")),
317            (TextSpan::new("Space - Switch parallaxing algorithm\n")),
318        ],
319    ));
320}
```

examples/3d/deferred\_rendering.rs ([line 245](../../../src/deferred_rendering/deferred_rendering.rs.html#245))

```rust
212fn setup_parallax(
213    mut commands: Commands,
214    mut materials: ResMut<Assets<StandardMaterial>>,
215    mut meshes: ResMut<Assets<Mesh>>,
216    asset_server: Res<AssetServer>,
217) {
218    // The normal map. Note that to generate it in the GIMP image editor, you should
219    // open the depth map, and do Filters → Generic → Normal Map
220    // You should enable the "flip X" checkbox.
221    let normal_handle = asset_server
222        .load_builder()
223        .with_settings(
224            // The normal map texture is in linear color space. Lighting won't look correct
225            // if `is_srgb` is `true`, which is the default.
226            |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
227        )
228        .load("textures/parallax_example/cube_normal.png");
229
230    let mut cube = Mesh::from(Cuboid::new(0.15, 0.15, 0.15));
231
232    // NOTE: for normal maps and depth maps to work, the mesh
233    // needs tangents generated.
234    cube.generate_tangents().unwrap();
235
236    let parallax_material = materials.add(StandardMaterial {
237        perceptual_roughness: 0.4,
238        base_color_texture: Some(asset_server.load("textures/parallax_example/cube_color.png")),
239        normal_map_texture: Some(normal_handle),
240        // The depth map is a grayscale texture where black is the highest level and
241        // white the lowest.
242        depth_map: Some(asset_server.load("textures/parallax_example/cube_depth.png")),
243        parallax_depth_scale: 0.09,
244        parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 4 },
245        max_parallax_layer_count: ops::exp2(5.0f32),
246        ..default()
247    });
248    commands.spawn((
249        Mesh3d(meshes.add(cube)),
250        MeshMaterial3d(parallax_material),
251        Transform::from_xyz(0.4, 0.2, -0.8),
252        Spin { speed: 0.3 },
253    ));
254}
```