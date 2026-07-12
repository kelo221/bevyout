[bevy](../index.html)::[math](index.html)

# Struct Quat 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#56)

```rust
pub struct Quat(/* private fields */);
```

A quaternion representing an orientation.

This quaternion is intended to be of unit length but may denormalize due to floating point “error creep” which can occur when successive quaternion operations are applied.

SIMD vector types are used for storage on supported platforms.

This type is 16 byte aligned.

## Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#58)

### impl [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#63)

#### pub const [IDENTITY](#associatedconstant.IDENTITY): [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The identity quaternion. Corresponds to no rotation.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#66)

#### pub const [NAN](#associatedconstant.NAN): [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

All NANs.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#81)

#### pub const fn [from\_xyzw](#method.from_xyzw)(x: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), y: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), z: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), w: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a new rotation quaternion.

This should generally not be called manually unless you know what you are doing. Use one of the other constructors instead such as `identity` or `from_axis_angle`.

`from_xyzw` is mostly used by unit tests and `serde` deserialization.

##### Preconditions

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 64](../../src/custom_primitives/custom_primitives.rs.html#64))

```rust
61const TRANSFORM_3D: Transform = Transform {
62    translation: Vec3::ZERO,
63    // The camera is pointing at the 3D shape
64    rotation: Quat::from_xyzw(-0.2669336, -0.0, -0.0, 0.96371484),
65    scale: Vec3::ONE,
66};
```

Hide additional examples

examples/3d/parallax\_mapping.rs ([line 167](../../src/parallax_mapping/parallax_mapping.rs.html#167))

```rust
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
```

examples/3d/solari.rs ([lines 126-131](../../src/solari/solari.rs.html#126-131))

```rust
77fn setup_pica_pica(
78    mut commands: Commands,
79    asset_server: Res<AssetServer>,
80    args: Res<Args>,
81    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))] dlss_rr_supported: Option<
82        Res<DlssRayReconstructionSupported>,
83    >,
84) {
85    commands
86        .spawn((
87            WorldAssetRoot(
88                asset_server.load(
89                    GltfAssetLabel::Scene(0)
90                        .from_asset("https://github.com/bevyengine/bevy_asset_files/raw/2a5950295a8b6d9d051d59c0df69e87abcda58c3/pica_pica/mini_diorama_01.glb")
91                ),
92            ),
93            Transform::from_scale(Vec3::splat(10.0)),
94        ))
95        .observe(add_raytracing_meshes_on_scene_load);
96
97    commands
98        .spawn((
99            WorldAssetRoot(asset_server.load(
100                GltfAssetLabel::Scene(0).from_asset("https://github.com/bevyengine/bevy_asset_files/raw/2a5950295a8b6d9d051d59c0df69e87abcda58c3/pica_pica/robot_01.glb")
101            )),
102            Transform::from_scale(Vec3::splat(2.0))
103                .with_translation(Vec3::new(-2.0, 0.05, -2.1))
104                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
105            PatrolPath {
106                path: vec![
107                    (Vec3::new(-2.0, 0.05, -2.1), Quat::from_rotation_y(PI / 2.0)),
108                    (Vec3::new(2.2, 0.05, -2.1), Quat::from_rotation_y(0.0)),
109                    (
110                        Vec3::new(2.2, 0.05, 2.1),
111                        Quat::from_rotation_y(3.0 * PI / 2.0),
112                    ),
113                    (Vec3::new(-2.0, 0.05, 2.1), Quat::from_rotation_y(PI)),
114                ],
115                i: 0,
116            },
117        ))
118        .observe(add_raytracing_meshes_on_scene_load);
119
120    commands.spawn((
121        DirectionalLight {
122            illuminance: light_consts::lux::FULL_DAYLIGHT,
123            shadow_maps_enabled: false, // Solari replaces shadow mapping
124            ..default()
125        },
126        Transform::from_rotation(Quat::from_xyzw(
127            -0.13334629,
128            -0.86597735,
129            -0.3586996,
130            0.3219264,
131        )),
132    ));
133
134    let mut camera = commands.spawn((
135        Camera3d::default(),
136        Camera {
137            clear_color: ClearColorConfig::Custom(Color::BLACK),
138            ..default()
139        },
140        FreeCamera {
141            walk_speed: 3.0,
142            run_speed: 10.0,
143            ..Default::default()
144        },
145        Transform::from_translation(Vec3::new(0.219417, 2.5764852, 6.9718704)).with_rotation(
146            Quat::from_xyzw(-0.1466768, 0.013738206, 0.002037309, 0.989087),
147        ),
148        // Msaa::Off and CameraMainTextureUsages with STORAGE_BINDING are required for Solari
149        CameraMainTextureUsages::default().with(TextureUsages::STORAGE_BINDING),
150        Msaa::Off,
151    ));
152
153    if args.pathtracer == Some(true) {
154        camera.insert(Pathtracer::default());
155    } else {
156        camera.insert(SolariLighting::default());
157    }
158
159    // Using DLSS Ray Reconstruction for denoising (and cheaper rendering via upscaling) is _highly_ recommended when using Solari
160    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))]
161    if dlss_rr_supported.is_some() {
162        camera.insert(Dlss::<DlssRayReconstructionFeature> {
163            perf_quality_mode: Default::default(),
164            reset: Default::default(),
165            _phantom_data: Default::default(),
166        });
167    }
168
169    commands.spawn((
170        ControlText,
171        Text::default(),
172        Node {
173            position_type: PositionType::Absolute,
174            bottom: px(12.0),
175            left: px(12.0),
176            ..default()
177        },
178    ));
179
180    commands.spawn((
181        Node {
182            position_type: PositionType::Absolute,
183            right: px(0.0),
184            padding: px(4.0).all(),
185            border_radius: BorderRadius::bottom_left(px(4.0)),
186            ..default()
187        },
188        BackgroundColor(Color::srgba(0.10, 0.10, 0.10, 0.8)),
189        children![(
190            PerformanceText,
191            Text::default(),
192            TextFont {
193                font_size: FontSize::Px(8.0),
194                ..default()
195            },
196        )],
197    ));
198}
199
200fn setup_many_lights(
201    mut commands: Commands,
202    asset_server: Res<AssetServer>,
203    mut meshes: ResMut<Assets<Mesh>>,
204    mut materials: ResMut<Assets<StandardMaterial>>,
205    args: Res<Args>,
206    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))] dlss_rr_supported: Option<
207        Res<DlssRayReconstructionSupported>,
208    >,
209) {
210    let mut rng = ChaCha8Rng::seed_from_u64(42);
211
212    let mut plane_mesh = Plane3d::default()
213        .mesh()
214        .size(400.0, 400.0)
215        .build()
216        .with_generated_tangents()
217        .unwrap();
218    match plane_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap() {
219        VertexAttributeValues::Float32x2(items) => {
220            items.iter_mut().flatten().for_each(|x| *x *= 3.0);
221        }
222        _ => unreachable!(),
223    }
224    let plane_mesh = meshes.add(plane_mesh);
225    let cube_mesh = meshes.add(
226        Cuboid::default()
227            .mesh()
228            .build()
229            .with_generated_tangents()
230            .unwrap(),
231    );
232    let sphere_mesh = meshes.add(
233        Sphere::new(1.0)
234            .mesh()
235            .build()
236            .with_generated_tangents()
237            .unwrap(),
238    );
239
240    commands
241        .spawn((
242            RaytracingMesh3d(plane_mesh.clone()),
243            MeshMaterial3d(
244                materials.add(StandardMaterial {
245                    base_color_texture: Some(
246                        asset_server
247                            .load_builder()
248                            .with_settings::<ImageLoaderSettings>(|settings| {
249                                settings
250                                    .sampler
251                                    .get_or_init_descriptor()
252                                    .set_address_mode(ImageAddressMode::Repeat);
253                            })
254                            .load("textures/uv_checker_bw.png"),
255                    ),
256                    perceptual_roughness: 0.0,
257                    ..default()
258                }),
259            ),
260        ))
261        .insert_if(Mesh3d(plane_mesh), || args.pathtracer != Some(true));
262
263    for _ in 0..8000 {
264        commands
265            .spawn((
266                RaytracingMesh3d(cube_mesh.clone()),
267                MeshMaterial3d(materials.add(StandardMaterial {
268                    base_color: Color::srgb(rng.random(), rng.random(), rng.random()),
269                    perceptual_roughness: rng.random(),
270                    ..default()
271                })),
272                Transform::default()
273                    .with_scale(Vec3 {
274                        x: rng.random_range(0.2..=2.0),
275                        y: rng.random_range(0.2..=2.0),
276                        z: rng.random_range(0.2..=2.0),
277                    })
278                    .with_translation(Vec3::new(
279                        rng.random_range(-180.0..=180.0),
280                        0.2,
281                        rng.random_range(-180.0..=180.0),
282                    )),
283            ))
284            .insert_if(Mesh3d(cube_mesh.clone()), || args.pathtracer != Some(true));
285    }
286
287    for x in -10..=10 {
288        for y in -10..=10 {
289            commands
290                .spawn((
291                    RaytracingMesh3d(sphere_mesh.clone()),
292                    MeshMaterial3d(
293                        materials.add(StandardMaterial {
294                            emissive: Color::linear_rgb(
295                                rng.random::<f32>() * 60000.0,
296                                rng.random::<f32>() * 60000.0,
297                                rng.random::<f32>() * 60000.0,
298                            )
299                            .into(),
300                            ..default()
301                        }),
302                    ),
303                    Transform::default().with_translation(Vec3::new(
304                        (x * 20) as f32,
305                        7.0,
306                        (y * 20) as f32,
307                    )),
308                ))
309                .insert_if(Mesh3d(sphere_mesh.clone()), || {
310                    args.pathtracer != Some(true)
311                });
312        }
313    }
314
315    let mut camera = commands.spawn((
316        Camera3d::default(),
317        Camera {
318            clear_color: ClearColorConfig::Custom(Color::BLACK),
319            ..default()
320        },
321        FreeCamera {
322            walk_speed: 3.0,
323            run_speed: 10.0,
324            ..Default::default()
325        },
326        Transform::from_translation(Vec3::new(6.11329, 166.74896, 451.8226)).with_rotation(
327            Quat::from_xyzw(-0.183938, 0.009093744, 0.0017017953, 0.9828943),
328        ),
329        // Msaa::Off and CameraMainTextureUsages with STORAGE_BINDING are required for Solari
330        CameraMainTextureUsages::default().with(TextureUsages::STORAGE_BINDING),
331        Msaa::Off,
332        Bloom {
333            intensity: 0.1,
334            ..Bloom::NATURAL
335        },
336    ));
337
338    if args.pathtracer == Some(true) {
339        camera.insert(Pathtracer::default());
340    } else {
341        camera.insert(SolariLighting::default());
342    }
343
344    // Using DLSS Ray Reconstruction for denoising (and cheaper rendering via upscaling) is _highly_ recommended when using Solari
345    #[cfg(all(feature = "dlss", not(feature = "force_disable_dlss")))]
346    if dlss_rr_supported.is_some() {
347        camera.insert(Dlss::<DlssRayReconstructionFeature> {
348            perf_quality_mode: Default::default(),
349            reset: Default::default(),
350            _phantom_data: Default::default(),
351        });
352    }
353
354    commands.spawn((
355        Node {
356            position_type: PositionType::Absolute,
357            right: px(0.0),
358            padding: px(4.0).all(),
359            border_radius: BorderRadius::bottom_left(px(4.0)),
360            ..default()
361        },
362        BackgroundColor(Color::srgba(0.10, 0.10, 0.10, 0.8)),
363        children![(
364            PerformanceText,
365            Text::default(),
366            TextFont {
367                font_size: FontSize::Px(8.0),
368                ..default()
369            },
370        )],
371    ));
372}
373
374fn add_raytracing_meshes_on_scene_load(
375    scene_ready: On<WorldInstanceReady>,
376    children: Query<&Children>,
377    mesh_query: Query<(
378        &Mesh3d,
379        &MeshMaterial3d<StandardMaterial>,
380        Option<&GltfMaterialName>,
381    )>,
382    mut meshes: ResMut<Assets<Mesh>>,
383    mut materials: ResMut<Assets<StandardMaterial>>,
384    mut commands: Commands,
385    args: Res<Args>,
386) {
387    for descendant in children.iter_descendants(scene_ready.entity) {
388        if let Ok((Mesh3d(mesh_handle), MeshMaterial3d(material_handle), material_name)) =
389            mesh_query.get(descendant)
390        {
391            // Add raytracing mesh component
392            commands
393                .entity(descendant)
394                .insert(RaytracingMesh3d(mesh_handle.clone()));
395
396            // Ensure meshes are Solari compatible
397            let mut mesh = meshes.get_mut(mesh_handle).unwrap();
398            if !mesh.contains_attribute(Mesh::ATTRIBUTE_UV_0) {
399                let vertex_count = mesh.count_vertices();
400                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; vertex_count]);
401                mesh.insert_attribute(
402                    Mesh::ATTRIBUTE_TANGENT,
403                    vec![[0.0, 0.0, 0.0, 0.0]; vertex_count],
404                );
405            }
406            if !mesh.contains_attribute(Mesh::ATTRIBUTE_TANGENT) {
407                mesh.generate_tangents().unwrap();
408            }
409            if mesh.contains_attribute(Mesh::ATTRIBUTE_UV_1) {
410                mesh.remove_attribute(Mesh::ATTRIBUTE_UV_1);
411            }
412            if let Some(indices) = mesh.indices_mut()
413                && let Indices::U16(_) = indices
414            {
415                *indices = Indices::U32(indices.iter().map(|i| i as u32).collect());
416            }
417
418            // Prevent rasterization if using pathtracer
419            if args.pathtracer == Some(true) {
420                commands.entity(descendant).remove::<Mesh3d>();
421            }
422
423            // Adjust scene materials to better demo Solari features
424            if material_name.map(|s| s.0.as_str()) == Some("material") {
425                let mut material = materials.get_mut(material_handle).unwrap();
426                material.emissive = LinearRgba::BLACK;
427            }
428            if material_name.map(|s| s.0.as_str()) == Some("Lights") {
429                let mut material = materials.get_mut(material_handle).unwrap();
430                material.emissive =
431                    LinearRgba::from(Color::srgb(0.941, 0.714, 0.043)) * 1_000_000.0;
432                material.alpha_mode = AlphaMode::Opaque;
433                material.specular_transmission = 0.0;
434
435                commands.insert_resource(RobotLightMaterial(material_handle.clone()));
436            }
437            if material_name.map(|s| s.0.as_str()) == Some("Glass_Dark_01") {
438                let mut material = materials.get_mut(material_handle).unwrap();
439                material.alpha_mode = AlphaMode::Opaque;
440                material.specular_transmission = 0.0;
441            }
442        }
443    }
444}
445
446fn pause_scene(mut time: ResMut<Time<Virtual>>, key_input: Res<ButtonInput<KeyCode>>) {
447    if key_input.just_pressed(KeyCode::Space) {
448        time.toggle();
449    }
450}
451
452#[derive(Resource)]
453struct RobotLightMaterial(Handle<StandardMaterial>);
454
455fn toggle_lights(
456    key_input: Res<ButtonInput<KeyCode>>,
457    robot_light_material: Option<Res<RobotLightMaterial>>,
458    mut materials: ResMut<Assets<StandardMaterial>>,
459    directional_light: Query<Entity, With<DirectionalLight>>,
460    mut commands: Commands,
461) {
462    if key_input.just_pressed(KeyCode::Digit1) {
463        if let Ok(directional_light) = directional_light.single() {
464            commands.entity(directional_light).despawn();
465        } else {
466            commands.spawn((
467                DirectionalLight {
468                    illuminance: light_consts::lux::FULL_DAYLIGHT,
469                    shadow_maps_enabled: false, // Solari replaces shadow mapping
470                    ..default()
471                },
472                Transform::from_rotation(Quat::from_xyzw(
473                    -0.13334629,
474                    -0.86597735,
475                    -0.3586996,
476                    0.3219264,
477                )),
478            ));
479        }
480    }
481
482    if key_input.just_pressed(KeyCode::Digit2)
483        && let Some(robot_light_material) = robot_light_material
484    {
485        let mut material = materials.get_mut(&robot_light_material.0).unwrap();
486        if material.emissive == LinearRgba::BLACK {
487            material.emissive = LinearRgba::from(Color::srgb(0.941, 0.714, 0.043)) * 1_000_000.0;
488        } else {
489            material.emissive = LinearRgba::BLACK;
490        }
491    }
492}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#93)

#### pub const fn [from\_array](#method.from_array)(a: \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a rotation quaternion from an array.

##### Preconditions

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/3d/pcss.rs ([lines 194-199](../../src/pcss/pcss.rs.html#194-199))

```rust
187fn spawn_light(commands: &mut Commands, app_status: &AppStatus) {
188    // Because this light can become a directional light, point light, or spot
189    // light depending on the settings, we add the union of the components
190    // necessary for this light to behave as all three of those.
191    commands
192        .spawn((
193            create_directional_light(app_status),
194            Transform::from_rotation(Quat::from_array([
195                0.6539259,
196                -0.34646285,
197                0.36505926,
198                -0.5648683,
199            ]))
200            .with_translation(vec3(57.693, 34.334, -6.422)),
201        ))
202        // These two are needed for point lights.
203        .insert(CubemapVisibleEntities::default())
204        .insert(CubemapFrusta::default())
205        // These two are needed for spot lights.
206        .insert(VisibleMeshEntities::default())
207        .insert(Frustum::default());
208}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#105)

#### pub const fn [from\_vec4](#method.from_vec4)(v: [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a new rotation quaternion from a 4D vector.

##### Preconditions

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#121)

#### pub fn [from\_slice](#method.from_slice)(slice: &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\]) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a rotation quaternion from a slice.

##### Preconditions

This function does not check if the input is normalized, it is up to the user to provide normalized input or to normalized the resulting quaternion.

##### Panics

Panics if `slice` length is less than 4.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#132)

#### pub fn [write\_to\_slice](#method.write_to_slice)(self, slice: &mut \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\])

Writes the quaternion to an unaligned slice.

##### Panics

Panics if `slice` length is less than 4.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#146)

#### pub fn [from\_axis\_angle](#method.from_axis_angle)(axis: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Create a quaternion for a normalized rotation `axis` and `angle` (in radians).

The axis must be a unit vector.

##### Panics

Will panic if `axis` is not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/gizmos/axes.rs ([line 191](../../src/axes/axes.rs.html#191))

```rust
187fn random_rotation(rng: &mut impl RngExt) -> Quat {
188    let dir = random_direction(rng);
189    let angle = rng.random::<f32>() * 2. * PI;
190
191    Quat::from_axis_angle(dir, angle)
192}
```

Hide additional examples

examples/shader/shader\_material\_screenspace\_texture.rs ([line 55](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#55))

```rust
52fn rotate_camera(mut cam_transform: Single<&mut Transform, With<MainCamera>>, time: Res<Time>) {
53    cam_transform.rotate_around(
54        Vec3::ZERO,
55        Quat::from_axis_angle(Vec3::Y, 45f32.to_radians() * time.delta_secs()),
56    );
57    cam_transform.look_at(Vec3::ZERO, Vec3::Y);
58}
```

examples/3d/split\_screen.rs ([line 200](../../src/split_screen/split_screen.rs.html#200))

```rust
181fn button_system(
182    interaction_query: Query<
183        (&Interaction, &ComputedUiTargetCamera, &RotateCamera),
184        (Changed<Interaction>, With<Button>),
185    >,
186    mut camera_query: Query<&mut Transform, With<Camera>>,
187) {
188    for (interaction, computed_target, RotateCamera(direction)) in &interaction_query {
189        if let Interaction::Pressed = *interaction {
190            // Since TargetCamera propagates to the children, we can use it to find
191            // which side of the screen the button is on.
192            if let Some(mut camera_transform) = computed_target
193                .get()
194                .and_then(|camera| camera_query.get_mut(camera).ok())
195            {
196                let angle = match direction {
197                    Direction::Left => -0.1,
198                    Direction::Right => 0.1,
199                };
200                camera_transform.rotate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, angle));
201            }
202        }
203    }
204}
```

examples/3d/ssr.rs ([line 670](../../src/ssr/ssr.rs.html#670))

```rust
634fn move_camera(
635    keyboard_input: Res<ButtonInput<KeyCode>>,
636    mut mouse_wheel_reader: MessageReader<MouseWheel>,
637    mut cameras: Query<&mut Transform, With<Camera>>,
638) {
639    let (mut distance_delta, mut theta_delta) = (0.0, 0.0);
640
641    // Handle keyboard events.
642    if keyboard_input.pressed(KeyCode::KeyW) {
643        distance_delta -= CAMERA_KEYBOARD_ZOOM_SPEED;
644    }
645    if keyboard_input.pressed(KeyCode::KeyS) {
646        distance_delta += CAMERA_KEYBOARD_ZOOM_SPEED;
647    }
648    if keyboard_input.pressed(KeyCode::KeyA) {
649        theta_delta += CAMERA_KEYBOARD_ORBIT_SPEED;
650    }
651    if keyboard_input.pressed(KeyCode::KeyD) {
652        theta_delta -= CAMERA_KEYBOARD_ORBIT_SPEED;
653    }
654
655    // Handle mouse events.
656    for mouse_wheel in mouse_wheel_reader.read() {
657        distance_delta -= mouse_wheel.y * CAMERA_MOUSE_WHEEL_ZOOM_SPEED;
658    }
659
660    // Update transforms.
661    for mut camera_transform in cameras.iter_mut() {
662        let local_z = camera_transform.local_z().as_vec3().normalize_or_zero();
663        if distance_delta != 0.0 {
664            camera_transform.translation = (camera_transform.translation.length() + distance_delta)
665                .clamp(CAMERA_ZOOM_RANGE.start, CAMERA_ZOOM_RANGE.end)
666                * local_z;
667        }
668        if theta_delta != 0.0 {
669            camera_transform
670                .translate_around(Vec3::ZERO, Quat::from_axis_angle(Vec3::Y, theta_delta));
671            camera_transform.look_at(Vec3::ZERO, Vec3::Y);
672        }
673    }
674}
```

examples/3d/decal.rs ([lines 74-77](../../src/decal/decal.rs.html#74-77))

```rust
20fn setup(
21    mut commands: Commands,
22    mut meshes: ResMut<Assets<Mesh>>,
23    mut standard_materials: ResMut<Assets<StandardMaterial>>,
24    mut decal_standard_materials: ResMut<Assets<ForwardDecalMaterial<StandardMaterial>>>,
25    asset_server: Res<AssetServer>,
26) {
27    // Spawn the forward decal
28    commands.spawn((
29        Name::new("Decal"),
30        ForwardDecal,
31        MeshMaterial3d(decal_standard_materials.add(ForwardDecalMaterial {
32            base: StandardMaterial {
33                base_color_texture: Some(asset_server.load("textures/uv_checker_bw.png")),
34                ..default()
35            },
36            extension: ForwardDecalMaterialExt {
37                depth_fade_factor: 1.0,
38            },
39        })),
40        Transform::from_scale(Vec3::splat(4.0)),
41    ));
42
43    commands.spawn((
44        Name::new("Camera"),
45        Camera3d::default(),
46        FreeCamera::default(),
47        // Must enable the depth prepass to render forward decals
48        DepthPrepass,
49        Transform::from_xyz(2.0, 9.5, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
50    ));
51
52    let white_material = standard_materials.add(Color::WHITE);
53
54    commands.spawn((
55        Name::new("Floor"),
56        Mesh3d(meshes.add(Rectangle::from_length(10.0))),
57        MeshMaterial3d(white_material.clone()),
58        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
59    ));
60
61    // Spawn a few cube with random rotations to showcase how the decals behave with non-flat geometry
62    let num_obs = 10;
63    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
64    for i in 0..num_obs {
65        for j in 0..num_obs {
66            let rotation_axis: [f32; 3] = rng.random();
67            let rotation_vec: Vec3 = rotation_axis.into();
68            let rotation: u32 = rng.random_range(0..360);
69            let transform = Transform::from_xyz(
70                (-num_obs + 1) as f32 / 2.0 + i as f32,
71                -0.2,
72                (-num_obs + 1) as f32 / 2.0 + j as f32,
73            )
74            .with_rotation(Quat::from_axis_angle(
75                rotation_vec.normalize_or_zero(),
76                (rotation as f32).to_radians(),
77            ));
78
79            commands.spawn((
80                Mesh3d(meshes.add(Cuboid::from_length(0.6))),
81                MeshMaterial3d(white_material.clone()),
82                transform,
83            ));
84        }
85    }
86
87    commands.spawn((
88        Name::new("Light"),
89        PointLight {
90            shadow_maps_enabled: true,
91            ..default()
92        },
93        Transform::from_xyz(4.0, 8.0, 4.0),
94    ));
95}
```

examples/3d/contact\_shadows.rs ([line 237](../../src/contact_shadows/contact_shadows.rs.html#237))

```rust
109fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
110    commands.spawn((
111        Camera3d::default(),
112        Transform::from_xyz(-0.8, 0.6, -0.8).looking_at(Vec3::new(0.0, 0.35, 0.0), Vec3::Y),
113        ContactShadows::default(),
114        TemporalAntiAliasing::default(), // Contact shadows and AO benefit from TAA
115        // Everything past this point is extra to look pretty.
116        Bloom::default(),
117        Hdr,
118        Skybox {
119            brightness: 1000.0,
120            image: Some(asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2")),
121            ..default()
122        },
123        EnvironmentMapLight {
124            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
125            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
126            intensity: 1000.0,
127            ..default()
128        },
129        ScreenSpaceAmbientOcclusion::default(),
130        Msaa::Off,
131        Tonemapping::AcesFitted,
132        MotionBlur {
133            shutter_angle: 2.0, // This is really just for fun when spinning the model
134            ..default()
135        },
136    ));
137
138    let directional_light = commands
139        .spawn((
140            DirectionalLight {
141                shadow_maps_enabled: true,
142                contact_shadows_enabled: true,
143                ..default()
144            },
145            Visibility::Hidden,
146        ))
147        .id();
148
149    let point_light = commands
150        .spawn((
151            PointLight {
152                intensity: light_consts::lumens::VERY_LARGE_CINEMA_LIGHT * 0.4,
153                shadow_maps_enabled: true,
154                contact_shadows_enabled: true,
155                ..default()
156            },
157            Visibility::Visible,
158        ))
159        .id();
160
161    let spot_light = commands
162        .spawn((
163            SpotLight {
164                intensity: light_consts::lumens::VERY_LARGE_CINEMA_LIGHT * 0.4,
165                shadow_maps_enabled: true,
166                contact_shadows_enabled: true,
167                ..default()
168            },
169            Visibility::Hidden,
170        ))
171        .id();
172
173    commands
174        .spawn((
175            Transform::from_xyz(-0.8, 1.5, 1.2).looking_at(Vec3::ZERO, Vec3::Y),
176            Visibility::default(),
177            LightContainer,
178        ))
179        .add_child(directional_light)
180        .add_child(point_light)
181        .add_child(spot_light);
182
183    commands
184        .spawn((
185            WorldAssetRoot(asset_server.load(
186                GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"),
187            )),
188            Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
189        ))
190        .observe(
191            |event: On<Pointer<Drag>>,
192             mut query: Query<&mut Transform, With<WorldAssetRoot>>,
193             mut commands: Commands,
194             mut window: Query<Entity, With<PrimaryWindow>>| {
195                for mut transform in query.iter_mut() {
196                    transform.rotate_y(event.delta.x * 0.01);
197                }
198                commands
199                    .entity(window.single_mut().unwrap())
200                    .insert(CursorIcon::System(SystemCursorIcon::Grabbing));
201            },
202        )
203        .observe(
204            |_: On<Pointer<Over>>,
205             mut commands: Commands,
206             mut window: Query<Entity, With<PrimaryWindow>>| {
207                commands
208                    .entity(window.single_mut().unwrap())
209                    .insert(CursorIcon::System(SystemCursorIcon::Grab));
210            },
211        )
212        .observe(
213            |_: On<Pointer<Out>>,
214             mut commands: Commands,
215             mut window: Query<Entity, With<PrimaryWindow>>| {
216                commands
217                    .entity(window.single_mut().unwrap())
218                    .insert(CursorIcon::System(SystemCursorIcon::Default));
219            },
220        )
221        .observe(
222            |_: On<Pointer<DragEnd>>,
223             mut commands: Commands,
224             mut window: Query<Entity, With<PrimaryWindow>>| {
225                commands
226                    .entity(window.single_mut().unwrap())
227                    .insert(CursorIcon::System(SystemCursorIcon::Default));
228            },
229        );
230
231    commands.spawn((
232        Mesh3d(asset_server.add(Circle::default().mesh().into())),
233        MeshMaterial3d(asset_server.add(StandardMaterial {
234            base_color: Color::srgb(0.06, 0.06, 0.06),
235            ..default()
236        })),
237        Transform::from_rotation(Quat::from_axis_angle(Vec3::X, -std::f32::consts::FRAC_PI_2)),
238        GroundPlane,
239    ));
240
241    spawn_buttons(&mut commands);
242
243    commands.spawn((
244        Node {
245            position_type: PositionType::Absolute,
246            top: px(12.0),
247            left: px(0.0),
248            right: px(0.0),
249            justify_content: JustifyContent::Center,
250            ..default()
251        },
252        children![(
253            Text::new("Drag model to spin"),
254            TextFont {
255                font_size: FontSize::Px(18.0),
256                ..default()
257            },
258        )],
259    ));
260}
```

Additional examples can be found in:  

*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#81)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#158)

#### pub fn [from\_scaled\_axis](#method.from_scaled_axis)(v: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Create a quaternion that rotates `v.length()` radians around `v.normalize()`.

`from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/3d/mesh\_ray\_cast.rs ([line 82](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#82))

```rust
71fn setup(
72    mut commands: Commands,
73    mut meshes: ResMut<Assets<Mesh>>,
74    mut materials: ResMut<Assets<StandardMaterial>>,
75) {
76    // Make a box of planes facing inward so the laser gets trapped inside
77    let plane_mesh = meshes.add(Plane3d::default());
78    let plane_material = materials.add(Color::from(css::GRAY).with_alpha(0.01));
79    let create_plane = move |translation, rotation| {
80        (
81            Transform::from_translation(translation)
82                .with_rotation(Quat::from_scaled_axis(rotation)),
83            Mesh3d(plane_mesh.clone()),
84            MeshMaterial3d(plane_material.clone()),
85        )
86    };
87
88    commands.spawn(create_plane(vec3(0.0, 0.5, 0.0), Vec3::X * PI));
89    commands.spawn(create_plane(vec3(0.0, -0.5, 0.0), Vec3::ZERO));
90    commands.spawn(create_plane(vec3(0.5, 0.0, 0.0), Vec3::Z * FRAC_PI_2));
91    commands.spawn(create_plane(vec3(-0.5, 0.0, 0.0), Vec3::Z * -FRAC_PI_2));
92    commands.spawn(create_plane(vec3(0.0, 0.0, 0.5), Vec3::X * -FRAC_PI_2));
93    commands.spawn(create_plane(vec3(0.0, 0.0, -0.5), Vec3::X * FRAC_PI_2));
94
95    // Light
96    commands.spawn((
97        DirectionalLight::default(),
98        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.1, 0.2, 0.0)),
99    ));
100
101    // Camera
102    commands.spawn((
103        Camera3d::default(),
104        Transform::from_xyz(1.5, 1.5, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
105        Tonemapping::TonyMcMapface,
106        Bloom::default(),
107    ));
108}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#170)

#### pub fn [from\_rotation\_x](#method.from_rotation_x)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from the `angle` (in radians) around the x axis.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 228](../../src/mirror/mirror.rs.html#228))

```rust
220fn spawn_ground_plane(
221    commands: &mut Commands,
222    meshes: &mut Assets<Mesh>,
223    standard_materials: &mut Assets<StandardMaterial>,
224) {
225    commands.spawn((
226        Mesh3d(meshes.add(Circle::new(200.0))),
227        MeshMaterial3d(standard_materials.add(Color::from(GREEN))),
228        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2))
229            .with_translation(vec3(-25.0, 0.0, 0.0)),
230    ));
231}
232
233/// Creates the initial image that the mirror camera will render the mirror
234/// world to.
235fn create_mirror_texture_resource(
236    commands: &mut Commands,
237    windows_query: &Query<&Window>,
238    images: &mut Assets<Image>,
239) -> Handle<Image> {
240    let window = windows_query.iter().next().expect("No window found");
241    let window_size = uvec2(window.physical_width(), window.physical_height());
242    let image = create_mirror_texture_image(images, window_size);
243    commands.insert_resource(MirrorImage(image.clone()));
244    image
245}
246
247/// Spawns the camera that renders the mirror world.
248fn spawn_mirror_camera(
249    commands: &mut Commands,
250    camera_transform: &Transform,
251    camera_projection: &PerspectiveProjection,
252    mirror_transform: &Transform,
253    mirror_render_target: Handle<Image>,
254) {
255    let (mirror_camera_transform, mirror_camera_projection) =
256        calculate_mirror_camera_transform_and_projection(
257            camera_transform,
258            camera_projection,
259            mirror_transform,
260        );
261
262    commands.spawn((
263        Camera3d::default(),
264        Camera {
265            order: -1,
266            // Reflecting the model across the mirror will flip the winding of
267            // all the polygons. Therefore, in order to properly backface cull,
268            // we need to turn on `invert_culling`.
269            invert_culling: true,
270            ..default()
271        },
272        RenderTarget::Image(mirror_render_target.clone().into()),
273        mirror_camera_transform,
274        Projection::Perspective(mirror_camera_projection),
275        MirrorCamera,
276    ));
277}
278
279/// Spawns the animated fox.
280///
281/// Note that this doesn't play the animation; that's handled in
282/// [`play_fox_animation`].
283fn spawn_fox(commands: &mut Commands, asset_server: &AssetServer) {
284    commands.spawn((
285        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(FOX_ASSET_PATH))),
286        Transform::from_xyz(-50.0, 0.0, -100.0),
287    ));
288}
289
290/// Spawns the mirror plane mesh and returns its transform.
291fn spawn_mirror(
292    commands: &mut Commands,
293    meshes: &mut Assets<Mesh>,
294    screen_space_texture_materials: &mut Assets<
295        ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>,
296    >,
297    mirror_render_target: Handle<Image>,
298) -> Transform {
299    let mirror_transform = Transform::from_scale(vec3(300.0, 1.0, 150.0))
300        .with_rotation(Quat::from_rotation_x(MIRROR_ROTATION_ANGLE))
301        .with_translation(MIRROR_POSITION);
302
303    commands.spawn((
304        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
305        MeshMaterial3d(screen_space_texture_materials.add(ExtendedMaterial {
306            base: StandardMaterial {
307                base_color: Color::BLACK,
308                emissive: Color::WHITE.into(),
309                emissive_texture: Some(mirror_render_target),
310                perceptual_roughness: 0.0,
311                metallic: 1.0,
312                ..default()
313            },
314            extension: ScreenSpaceTextureExtension { dummy: 0.0 },
315        })),
316        mirror_transform,
317        Mirror,
318    ));
319
320    mirror_transform
321}
```

Hide additional examples

examples/3d/light\_textures.rs ([line 364](../../src/light_textures/light_textures.rs.html#364))

```rust
357fn draw_gizmos(mut gizmos: Gizmos, spotlight: Query<(&GlobalTransform, &SpotLight, &Visibility)>) {
358    if let Ok((global_transform, spotlight, visibility)) = spotlight.single()
359        && visibility != Visibility::Hidden
360    {
361        gizmos.primitive_3d(
362            &Cone::new(7.0 * spotlight.outer_angle, 7.0),
363            Isometry3d {
364                rotation: global_transform.rotation() * Quat::from_rotation_x(FRAC_PI_2),
365                translation: global_transform.translation_vec3a() * 0.5,
366            },
367            YELLOW,
368        );
369    }
370}
```

examples/app/externally\_driven\_headless\_renderer.rs ([line 138](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#138))

```rust
130fn spawn_test_scene(
131    mut commands: Commands,
132    mut meshes: ResMut<Assets<Mesh>>,
133    mut materials: ResMut<Assets<StandardMaterial>>,
134) {
135    commands.spawn((
136        Mesh3d(meshes.add(Circle::new(4.0))),
137        MeshMaterial3d(materials.add(Color::WHITE)),
138        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
139    ));
140    commands.spawn((
141        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
142        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
143        Transform::from_xyz(0.0, 1.0, 0.0),
144    ));
145    commands.spawn((
146        PointLight {
147            shadow_maps_enabled: true,
148            ..default()
149        },
150        Transform::from_xyz(4.0, 8.0, 4.0),
151    ));
152}
```

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 364](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#364))

```rust
346fn draw_camera_gizmo(cameras: Query<(&Camera, &GlobalTransform)>, mut gizmos: Gizmos) {
347    for (camera, transform) in &cameras {
348        // As above, we use the order as a cheap tag to tell the depth texture
349        // apart from the main texture.
350        if camera.order >= 0 {
351            continue;
352        }
353
354        // Draw a cone representing the camera.
355        gizmos.primitive_3d(
356            &Cone {
357                radius: 1.0,
358                height: 3.0,
359            },
360            Isometry3d::new(
361                transform.translation(),
362                // We have to rotate here because `Cone` primitives are oriented
363                // along +Y and cameras point along +Z.
364                transform.rotation() * Quat::from_rotation_x(FRAC_PI_2),
365            ),
366            LIME,
367        );
368    }
369}
```

examples/async\_tasks/async\_channel\_pattern.rs ([line 140](../../src/async_channel_pattern/async_channel_pattern.rs.html#140))

```rust
131fn setup_env(
132    mut commands: Commands,
133    mut meshes: ResMut<Assets<Mesh>>,
134    mut materials: ResMut<Assets<StandardMaterial>>,
135) {
136    // Spawn a circular ground plane
137    commands.spawn((
138        Mesh3d(meshes.add(Circle::new(1.618 * NUM_CUBES as f32))),
139        MeshMaterial3d(materials.add(Color::WHITE)),
140        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
141    ));
142
143    // Spawn a point light with shadows enabled
144    commands.spawn((
145        PointLight {
146            shadow_maps_enabled: true,
147            ..default()
148        },
149        Transform::from_xyz(0.0, LIGHT_RADIUS, 4.0),
150    ));
151
152    // Spawn a camera looking at the origin
153    commands.spawn((
154        Camera3d::default(),
155        Transform::from_xyz(-6.5, 5.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
156    ));
157}
```

examples/3d/3d\_scene.rs ([line 19](../../src/3d_scene/3d_scene.rs.html#19))

```rust
13fn scene() -> impl SceneList {
14    bsn_list! [
15        (
16            #CircularBase
17            Mesh3d(asset_value(Circle::new(4.0)))
18            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
19            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
20        ),
21        (
22            #Cube
23            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
24            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
25            Transform::from_xyz(0.0, 0.5, 0.0)
26        ),
27        (
28            PointLight {
29                shadow_maps_enabled: true,
30            }
31            Transform::from_xyz(4.0, 8.0, 4.0)
32        ),
33        (
34            Camera3d
35            template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
36        )
37    ]
38}
```

Additional examples can be found in:  

*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#253)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#278)
*   [examples/remote/server.rs](../../src/server/server.rs.html#35)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#153)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#70)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#67)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#84)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#68)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#330)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#43)
*   [examples/asset/generated\_assets.rs](../../src/generated_assets/generated_assets.rs.html#45)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#67)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#201)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#262)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#177)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#43)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#104)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#44)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#59)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#58)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#186)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#146)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#98)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#105)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#217)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#49)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#89)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#147)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#143)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#245)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#166)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#178)

#### pub fn [from\_rotation\_y](#method.from_rotation_y)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from the `angle` (in radians) around the y axis.

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/gizmos/light\_gizmos.rs ([line 151](../../src/light_gizmos/light_gizmos.rs.html#151))

```rust
150fn rotate_camera(mut transform: Single<&mut Transform, With<Camera>>, time: Res<Time>) {
151    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs() / 2.));
152}
```

Hide additional examples

examples/stress\_tests/many\_cameras\_lights.rs ([line 101](../../src/many_cameras_lights/many_cameras_lights.rs.html#101))

```rust
99fn rotate_cameras(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
100    for mut transform in query.iter_mut() {
101        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs()));
102    }
103}
```

examples/3d/clearcoat.rs ([line 262](../../src/clearcoat/clearcoat.rs.html#262))

```rust
259fn animate_spheres(mut spheres: Query<&mut Transform, With<ExampleSphere>>, time: Res<Time>) {
260    let now = time.elapsed_secs();
261    for mut transform in spheres.iter_mut() {
262        transform.rotation = Quat::from_rotation_y(SPHERE_ROTATION_SPEED * now);
263    }
264}
```

examples/3d/ssr.rs ([line 629](../../src/ssr/ssr.rs.html#629))

```rust
623fn rotate_model(
624    mut query: Query<&mut Transform, Or<(With<CubeModel>, With<FlightHelmetModel>)>>,
625    time: Res<Time>,
626) {
627    for mut transform in query.iter_mut() {
628        // Models rotate on the Y axis.
629        transform.rotation = Quat::from_rotation_y(time.elapsed_secs());
630    }
631}
```

examples/3d/specular\_tint.rs ([line 138](../../src/specular_tint/specular_tint.rs.html#138))

```rust
135fn rotate_camera(mut cameras: Query<&mut Transform, With<Camera3d>>) {
136    for mut camera_transform in cameras.iter_mut() {
137        camera_transform.translation =
138            Quat::from_rotation_y(ROTATION_SPEED) * camera_transform.translation;
139        camera_transform.look_at(Vec3::ZERO, Vec3::Y);
140    }
141}
```

examples/camera/2d\_on\_ui.rs ([line 69](../../src/2d_on_ui/2d_on_ui.rs.html#69))

```rust
66fn rotate_sprite(time: Res<Time>, mut sprite: Single<&mut Transform, With<Sprite>>) {
67    // Use any of the regular 2D rendering features, for example rotating a sprite via its `Transform`.
68    sprite.rotation *=
69        Quat::from_rotation_z(time.delta_secs() * 0.5) * Quat::from_rotation_y(time.delta_secs());
70}
```

Additional examples can be found in:  

*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#77)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#42)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#196)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#207)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#87)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#29)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#46)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#211)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#241)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#123)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#357)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#279)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#115)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#248)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#64)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#205)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#177)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#191)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#76)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#123)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#318)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#164)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#141)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#104)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#104)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#147)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#188)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#186)

#### pub fn [from\_rotation\_z](#method.from_rotation_z)(angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from the `angle` (in radians) around the z axis.

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/math/bounding\_2d.rs ([line 41](../../src/bounding_2d/bounding_2d.rs.html#41))

```rust
39fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
40    for mut transform in query.iter_mut() {
41        transform.rotation *= Quat::from_rotation_z(time.delta_secs() / 5.);
42    }
43}
```

Hide additional examples

examples/2d/text2d.rs ([line 191](../../src/text2d/text2d.rs.html#191))

```rust
186fn animate_rotation(
187    time: Res<Time>,
188    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
189) {
190    for mut transform in &mut query {
191        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
192    }
193}
```

examples/shader/shader\_prepass.rs ([line 176](../../src/shader_prepass/shader_prepass.rs.html#176))

```rust
173fn rotate(mut q: Query<&mut Transform, With<Rotates>>, time: Res<Time>) {
174    for mut t in q.iter_mut() {
175        let rot = (ops::sin(time.elapsed_secs()) * 0.5 + 0.5) * std::f32::consts::PI * 2.0;
176        t.rotation = Quat::from_rotation_z(rot);
177    }
178}
```

examples/math/custom\_primitives.rs ([line 278](../../src/custom_primitives/custom_primitives.rs.html#278))

```rust
274fn rotate_2d_shapes(mut shapes: Query<&mut Transform, With<Shape2d>>, time: Res<Time>) {
275    let elapsed_seconds = time.elapsed_secs();
276
277    for mut transform in shapes.iter_mut() {
278        transform.rotation = Quat::from_rotation_z(elapsed_seconds);
279    }
280}
```

examples/camera/2d\_on\_ui.rs ([line 69](../../src/2d_on_ui/2d_on_ui.rs.html#69))

```rust
66fn rotate_sprite(time: Res<Time>, mut sprite: Single<&mut Transform, With<Sprite>>) {
67    // Use any of the regular 2D rendering features, for example rotating a sprite via its `Transform`.
68    sprite.rotation *=
69        Quat::from_rotation_z(time.delta_secs() * 0.5) * Quat::from_rotation_y(time.delta_secs());
70}
```

tests/3d/test\_invalid\_skinned\_mesh.rs ([line 226](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#226))

```rust
223fn update_animated_joints(time: Res<Time>, query: Query<&mut Transform, With<AnimatedJoint>>) {
224    for mut transform in query {
225        let angle = TAU * 4.0 * ops::cos((time.elapsed_secs() / 8.0) * TAU);
226        let rotation = Quat::from_rotation_z(angle);
227
228        transform.rotation = rotation;
229        transform.translation = rotation.mul_vec3(Vec3::new(0.0, 1.3, 0.0));
230    }
231}
```

Additional examples can be found in:  

*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#127)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#49)
*   [examples/2d/rotate\_to\_cursor.rs](../../src/rotate_to_cursor/rotate_to_cursor.rs.html#55-57)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#280)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#69)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#77)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#79)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#226)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#201)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#75)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#77)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#128)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#209)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#145)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#81)
*   [examples/2d/mesh2d\_arcs.rs](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#72)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#197)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#194)

#### pub fn [from\_euler](#method.from_euler)(euler: [EulerRot](../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot"), a: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), b: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), c: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/shader/extended\_material\_bindless.rs ([line 152](../../src/extended_material_bindless/extended_material_bindless.rs.html#152))

```rust
149fn rotate_sphere(mut meshes: Query<&mut Transform, With<Mesh3d>>, time: Res<Time>) {
150    for mut transform in &mut meshes {
151        transform.rotation =
152            Quat::from_euler(EulerRot::YXZ, -time.elapsed_secs(), FRAC_PI_2 * 3.0, 0.0);
153    }
154}
```

Hide additional examples

examples/3d/occlusion\_culling.rs ([lines 345-350](../../src/occlusion_culling/occlusion_culling.rs.html#345-350))

```rust
343fn spin_large_cube(mut large_cubes: Query<&mut Transform, With<LargeCube>>) {
344    for mut transform in &mut large_cubes {
345        transform.rotate(Quat::from_euler(
346            EulerRot::XYZ,
347            0.13 * ROTATION_SPEED,
348            0.29 * ROTATION_SPEED,
349            0.35 * ROTATION_SPEED,
350        ));
351    }
352}
353
354/// Spawns a directional light to illuminate the scene.
355fn spawn_light(commands: &mut Commands) {
356    commands
357        .spawn(DirectionalLight::default())
358        .insert(Transform::from_rotation(Quat::from_euler(
359            EulerRot::ZYX,
360            0.0,
361            PI * -0.15,
362            PI * -0.15,
363        )));
364}
```

examples/gltf/load\_gltf.rs ([lines 56-61](../../src/load_gltf/load_gltf.rs.html#56-61))

```rust
51fn animate_light_direction(
52    time: Res<Time>,
53    mut query: Query<&mut Transform, With<DirectionalLight>>,
54) {
55    for mut transform in &mut query {
56        transform.rotation = Quat::from_euler(
57            EulerRot::ZYX,
58            0.0,
59            time.elapsed_secs() * PI / 5.0,
60            -FRAC_PI_4,
61        );
62    }
63}
```

examples/gltf/query\_gltf\_primitives.rs ([line 61](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#61))

```rust
54fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
55    commands.spawn((
56        Camera3d::default(),
57        Transform::from_xyz(4.0, 4.0, 12.0).looking_at(Vec3::new(0.0, 0.0, 0.5), Vec3::Y),
58    ));
59
60    commands.spawn((
61        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
62        DirectionalLight::default(),
63    ));
64
65    commands.spawn(WorldAssetRoot(asset_server.load(
66        GltfAssetLabel::Scene(0).from_asset("models/GltfPrimitives/gltf_primitives.glb"),
67    )));
68}
```

examples/3d/spotlight.rs ([lines 139-144](../../src/spotlight/spotlight.rs.html#139-144))

```rust
137fn light_sway(time: Res<Time>, mut query: Query<(&mut Transform, &mut SpotLight)>) {
138    for (mut transform, mut angles) in query.iter_mut() {
139        transform.rotation = Quat::from_euler(
140            EulerRot::XYZ,
141            -FRAC_PI_2 + ops::sin(time.elapsed_secs() * 0.67 * 3.0) * 0.5,
142            ops::sin(time.elapsed_secs() * 3.0) * 0.5,
143            0.0,
144        );
145        let angle = (ops::sin(time.elapsed_secs() * 1.2) + 1.0) * (FRAC_PI_4 - 0.1);
146        angles.inner_angle = angle * 0.8;
147        angles.outer_angle = angle;
148    }
149}
```

examples/asset/multi\_asset\_sync.rs ([line 205](../../src/multi_asset_sync/multi_asset_sync.rs.html#205))

```rust
188fn setup_scene(
189    mut commands: Commands,
190    mut meshes: ResMut<Assets<Mesh>>,
191    mut materials: ResMut<Assets<StandardMaterial>>,
192) {
193    // Camera
194    commands.spawn((
195        Camera3d::default(),
196        Transform::from_xyz(10.0, 10.0, 15.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
197    ));
198
199    // Light
200    commands.spawn((
201        DirectionalLight {
202            shadow_maps_enabled: true,
203            ..default()
204        },
205        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
206    ));
207
208    // Plane
209    commands.spawn((
210        Mesh3d(meshes.add(Plane3d::default().mesh().size(50000.0, 50000.0))),
211        MeshMaterial3d(materials.add(Color::srgb(0.7, 0.2, 0.2))),
212        Loading,
213    ));
214}
```

Additional examples can be found in:  

*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#120)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#190)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#133)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#367)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#163)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#126)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#68)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#328)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#108)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#98)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#65)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#271)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#288)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#135)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#76)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#114)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#431)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#235)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#563)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#513)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#152)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#91)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#451)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#110)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#65)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#36)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#230)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#72)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#76-81)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#208)

#### pub fn [from\_rotation\_axes](#method.from_rotation_axes)(x\_axis: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), y\_axis: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), z\_axis: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

From the columns of a 3x3 rotation matrix.

Note if the input axes contain scales, shears, or other non-rotation transformations then the output of this function is ill-defined.

##### Panics

Will panic if any axis is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#277)

#### pub fn [from\_mat3](#method.from_mat3)(mat: &[Mat3](../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from a 3x3 rotation matrix.

Note if the input matrix contain scales, shears, or other non-rotation transformations then the resulting quaternion will be ill-defined.

##### Panics

Will panic if any input matrix column is not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/math/render\_primitives.rs ([line 639](../../src/render_primitives/render_primitives.rs.html#639))

```rust
632fn rotate_primitive_2d_meshes(
633    mut primitives_2d: Query<
634        (&mut Transform, &ViewVisibility),
635        (With<PrimitiveData>, With<MeshDim2>),
636    >,
637    time: Res<Time>,
638) {
639    let rotation_2d = Quat::from_mat3(&Mat3::from_angle(time.elapsed_secs()));
640    primitives_2d
641        .iter_mut()
642        .filter(|(_, vis)| vis.get())
643        .for_each(|(mut transform, _)| {
644            transform.rotation = rotation_2d;
645        });
646}
```

Hide additional examples

examples/ecs/fallible\_params.rs ([line 151](../../src/fallible_params/fallible_params.rs.html#151))

```rust
136fn track_targets(
137    // `Single` ensures the system runs ONLY when exactly one matching entity exists.
138    mut player: Single<(&mut Transform, &Player)>,
139    // `Option<Single>` never prevents the system from running, but will be `None` if there is not exactly one matching entity.
140    enemy: Option<Single<&Transform, (With<Enemy>, Without<Player>)>>,
141    time: Res<Time>,
142) {
143    let (player_transform, player) = &mut *player;
144    if let Some(enemy_transform) = enemy {
145        // Enemy found, rotate and move towards it.
146        let delta = enemy_transform.translation - player_transform.translation;
147        let distance = delta.length();
148        let front = delta / distance;
149        let up = Vec3::Z;
150        let side = front.cross(up);
151        player_transform.rotation = Quat::from_mat3(&Mat3::from_cols(side, front, up));
152        let max_step = distance - player.min_follow_radius;
153        if 0.0 < max_step {
154            let velocity = (player.speed * time.delta_secs()).min(max_step);
155            player_transform.translation += front * velocity;
156        }
157    } else {
158        // 0 or multiple enemies found, keep searching.
159        player_transform.rotate_axis(Dir3::Z, player.rotation_speed * time.delta_secs());
160    }
161}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#291)

#### pub fn [from\_mat3a](#method.from_mat3a)(mat: &[Mat3A](../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from a 3x3 SIMD aligned rotation matrix.

Note if the input matrix contain scales, shears, or other non-rotation transformations then the resulting quaternion will be ill-defined.

##### Panics

Will panic if any input matrix column is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#306)

#### pub fn [from\_mat4](#method.from_mat4)(mat: &[Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from the upper 3x3 rotation matrix inside a homogeneous 4x4 matrix.

Note if the upper 3x3 matrix contain scales, shears, or other non-rotation transformations then the resulting quaternion will be ill-defined.

##### Panics

Will panic if any column of the upper 3x3 rotation matrix is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#328)

#### pub fn [from\_rotation\_arc](#method.from_rotation_arc)(from: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), to: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Gets the minimal rotation for transforming `from` to `to`. The rotation is in the plane spanned by the two vectors. Will rotate at most 180 degrees.

The inputs must be unit vectors.

`from_rotation_arc(from, to) * from ≈ to`.

For near-singular cases (from≈to and from≈-to) the current implementation is only accurate to about 0.001 (for `f32`).

##### Panics

Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/math/render\_primitives.rs ([lines 655-664](../../src/render_primitives/render_primitives.rs.html#655-664))

```rust
648fn rotate_primitive_3d_meshes(
649    mut primitives_3d: Query<
650        (&mut Transform, &ViewVisibility),
651        (With<PrimitiveData>, With<MeshDim3>),
652    >,
653    time: Res<Time>,
654) {
655    let rotation_3d = Quat::from_rotation_arc(
656        Vec3::Z,
657        Vec3::new(
658            ops::sin(time.elapsed_secs()),
659            ops::cos(time.elapsed_secs()),
660            ops::sin(time.elapsed_secs()) * 0.5,
661        )
662        .try_normalize()
663        .unwrap_or(Vec3::Z),
664    );
665    primitives_3d
666        .iter_mut()
667        .filter(|(_, vis)| vis.get())
668        .for_each(|(mut transform, _)| {
669            transform.rotation = rotation_3d;
670        });
671}
672
673fn draw_gizmos_3d(mut gizmos: Gizmos, state: Res<State<PrimitiveSelected>>, time: Res<Time>) {
674    const POSITION: Vec3 = Vec3::new(LEFT_RIGHT_OFFSET_3D, 0.0, 0.0);
675    let rotation = Quat::from_rotation_arc(
676        Vec3::Z,
677        Vec3::new(
678            ops::sin(time.elapsed_secs()),
679            ops::cos(time.elapsed_secs()),
680            ops::sin(time.elapsed_secs()) * 0.5,
681        )
682        .try_normalize()
683        .unwrap_or(Vec3::Z),
684    );
685    let isometry = Isometry3d::new(POSITION, rotation);
686    let color = Color::WHITE;
687    let resolution = 10;
688
689    #[expect(
690        clippy::match_same_arms,
691        reason = "Certain primitives don't have any 3D rendering support yet."
692    )]
693    match state.get() {
694        PrimitiveSelected::RectangleAndCuboid => {
695            gizmos.primitive_3d(&CUBOID, isometry, color);
696        }
697        PrimitiveSelected::CircleAndSphere => drop(
698            gizmos
699                .primitive_3d(&SPHERE, isometry, color)
700                .resolution(resolution),
701        ),
702        PrimitiveSelected::Ellipse => {}
703        PrimitiveSelected::Triangle => gizmos.primitive_3d(&TRIANGLE_3D, isometry, color),
704        PrimitiveSelected::Plane => drop(gizmos.primitive_3d(&PLANE_3D, isometry, color)),
705        PrimitiveSelected::Line => gizmos.primitive_3d(&LINE_3D, isometry, color),
706        PrimitiveSelected::Segment => gizmos.primitive_3d(&SEGMENT_3D, isometry, color),
707        PrimitiveSelected::Polyline => gizmos.primitive_3d(
708            &Polyline3d {
709                vertices: POLYLINE_3D_VERTICES.to_vec(),
710            },
711            isometry,
712            color,
713        ),
714        PrimitiveSelected::Polygon => {}
715        PrimitiveSelected::ConvexPolygon => {}
716        PrimitiveSelected::RegularPolygon => {}
717        PrimitiveSelected::Capsule => drop(
718            gizmos
719                .primitive_3d(&CAPSULE_3D, isometry, color)
720                .resolution(resolution),
721        ),
722        PrimitiveSelected::Cylinder => drop(
723            gizmos
724                .primitive_3d(&CYLINDER, isometry, color)
725                .resolution(resolution),
726        ),
727        PrimitiveSelected::Cone => drop(
728            gizmos
729                .primitive_3d(&CONE, isometry, color)
730                .resolution(resolution),
731        ),
732        PrimitiveSelected::ConicalFrustum => {
733            gizmos.primitive_3d(&CONICAL_FRUSTUM, isometry, color);
734        }
735
736        PrimitiveSelected::Torus => drop(
737            gizmos
738                .primitive_3d(&TORUS, isometry, color)
739                .minor_resolution(resolution)
740                .major_resolution(resolution),
741        ),
742        PrimitiveSelected::Tetrahedron => {
743            gizmos.primitive_3d(&TETRAHEDRON, isometry, color);
744        }
745
746        PrimitiveSelected::Arc => {}
747        PrimitiveSelected::CircularSector => {}
748        PrimitiveSelected::CircularSegment => {}
749    }
750}
```

Hide additional examples

examples/2d/rotation.rs ([line 167](../../src/rotation/rotation.rs.html#167))

```rust
154fn snap_to_player_system(
155    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
156    player_transform: Single<&Transform, With<Player>>,
157) {
158    // Get the player translation in 2D
159    let player_translation = player_transform.translation.xy();
160
161    for mut enemy_transform in &mut query {
162        // Get the vector from the enemy ship to the player ship in 2D and normalize it.
163        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
164
165        // Get the quaternion to rotate from the initial enemy facing direction to the direction
166        // facing the player
167        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
168
169        // Rotate the enemy to face the player
170        enemy_transform.rotation = rotate_to_player;
171    }
172}
```

examples/3d/3d\_viewport\_to\_world.rs ([line 31](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#31))

```rust
13fn draw_cursor(
14    camera_query: Single<(&Camera, &GlobalTransform)>,
15    ground: Single<&GlobalTransform, With<Ground>>,
16    window: Single<&Window>,
17    mut gizmos: Gizmos,
18) {
19    let (camera, camera_transform) = *camera_query;
20
21    if let Some(cursor_position) = window.cursor_position()
22        // Calculate a ray pointing from the camera into the world based on the cursor's position.
23        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
24        // Calculate if and where the ray is hitting the ground plane.
25        && let Some(point) = ray.plane_intersection_point(ground.translation(), InfinitePlane3d::new(ground.up()))
26    {
27        // Draw a circle just above the ground plane at that position.
28        gizmos.circle(
29            Isometry3d::new(
30                point + ground.up() * 0.01,
31                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
32            ),
33            0.2,
34            Color::WHITE,
35        );
36    }
37}
```

examples/gizmos/3d\_gizmos.rs ([line 181](../../src/3d_gizmos/3d_gizmos.rs.html#181))

```rust
99fn draw_example_collection(
100    mut gizmos: Gizmos,
101    mut my_gizmos: Gizmos<MyRoundGizmos>,
102    time: Res<Time>,
103) {
104    gizmos.grid(
105        Quat::from_rotation_x(PI / 2.),
106        UVec2::splat(20),
107        Vec2::new(2., 2.),
108        // Light gray
109        LinearRgba::gray(0.65),
110    );
111    gizmos.grid(
112        Isometry3d::new(Vec3::splat(10.0), Quat::from_rotation_x(PI / 3. * 2.)),
113        UVec2::splat(20),
114        Vec2::new(2., 2.),
115        PURPLE,
116    );
117    gizmos.sphere(Vec3::splat(10.0), 1.0, PURPLE);
118
119    gizmos
120        .primitive_3d(
121            &Plane3d {
122                normal: Dir3::Y,
123                half_size: Vec2::splat(1.0),
124            },
125            Isometry3d::new(
126                Vec3::splat(4.0) + Vec2::from(ops::sin_cos(time.elapsed_secs())).extend(0.0),
127                Quat::from_rotation_x(PI / 2. + time.elapsed_secs()),
128            ),
129            GREEN,
130        )
131        .cell_count(UVec2::new(5, 10))
132        .spacing(Vec2::new(0.2, 0.1));
133
134    gizmos.cube(
135        Transform::from_translation(Vec3::Y * 0.5).with_scale(Vec3::splat(1.25)),
136        BLACK,
137    );
138    gizmos.rect(
139        Isometry3d::new(
140            Vec3::new(ops::cos(time.elapsed_secs()) * 2.5, 1., 0.),
141            Quat::from_rotation_y(PI / 2.),
142        ),
143        Vec2::splat(2.),
144        LIME,
145    );
146
147    gizmos.cross(Vec3::new(-1., 1., 1.), 0.5, FUCHSIA);
148
149    let domain = Interval::EVERYWHERE;
150    let curve = FunctionCurve::new(domain, |t| {
151        (Vec2::from(ops::sin_cos(t * 10.0))).extend(t - 6.0)
152    });
153    let resolution = ((ops::sin(time.elapsed_secs()) + 1.0) * 100.0) as usize;
154    let times_and_colors = (0..=resolution)
155        .map(|n| n as f32 / resolution as f32)
156        .map(|t| t * 5.0)
157        .map(|t| (t, TEAL.mix(&HOT_PINK, t / 5.0)));
158    gizmos.curve_gradient_3d(curve, times_and_colors);
159
160    my_gizmos.sphere(Vec3::new(1., 0.5, 0.), 0.5, RED);
161
162    my_gizmos
163        .rounded_cuboid(Vec3::new(-2.0, 0.75, -0.75), Vec3::splat(0.9), TURQUOISE)
164        .edge_radius(0.1)
165        .arc_resolution(4);
166
167    for y in [0., 0.5, 1.] {
168        gizmos.ray(
169            Vec3::new(1., y, 0.),
170            Vec3::new(-3., ops::sin(time.elapsed_secs() * 3.), 0.),
171            BLUE,
172        );
173    }
174
175    my_gizmos
176        .arc_3d(
177            180.0_f32.to_radians(),
178            0.2,
179            Isometry3d::new(
180                Vec3::ONE,
181                Quat::from_rotation_arc(Vec3::Y, Vec3::ONE.normalize()),
182            ),
183            ORANGE,
184        )
185        .resolution(10);
186
187    // Circles have 32 line-segments by default.
188    my_gizmos.circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3., BLACK);
189
190    // You may want to increase this for larger circles or spheres.
191    my_gizmos
192        .circle(Quat::from_rotation_arc(Vec3::Z, Vec3::Y), 3.1, NAVY)
193        .resolution(64);
194    my_gizmos
195        .sphere(Isometry3d::IDENTITY, 3.2, BLACK)
196        .resolution(64);
197
198    gizmos.arrow(Vec3::ZERO, Vec3::splat(1.5), YELLOW);
199
200    // You can create more complex arrows using the arrow builder.
201    gizmos
202        .arrow(Vec3::new(2., 0., 2.), Vec3::new(2., 2., 2.), ORANGE_RED)
203        .with_double_end()
204        .with_tip_length(0.5);
205}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#362)

#### pub fn [from\_rotation\_arc\_colinear](#method.from_rotation_arc_colinear)(from: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), to: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Gets the minimal rotation for transforming `from` to either `to` or `-to`. This means that the resulting quaternion will rotate `from` so that it is colinear with `to`.

The rotation is in the plane spanned by the two vectors. Will rotate at most 90 degrees.

The inputs must be unit vectors.

`to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.

##### Panics

Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#384)

#### pub fn [from\_rotation\_arc\_2d](#method.from_rotation_arc_2d)(from: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2"), to: [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Gets the minimal rotation for transforming `from` to `to`. The resulting rotation is around the z axis. Will rotate at most 180 degrees.

The inputs must be unit vectors.

`from_rotation_arc_2d(from, to) * from ≈ to`.

For near-singular cases (from≈to and from≈-to) the current implementation is only accurate to about 0.001 (for `f32`).

##### Panics

Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#418)

#### pub fn [look\_to\_lh](#method.look_to_lh)(dir: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaterion rotation from a facing direction and an up direction.

For a left-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

##### Panics

Will panic if `up` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#431)

#### pub fn [look\_to\_rh](#method.look_to_rh)(dir: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaterion rotation from facing direction and an up direction.

For a right-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

##### Panics

Will panic if `dir` and `up` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#455)

#### pub fn [look\_at\_lh](#method.look_at_lh)(eye: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), center: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a left-handed view matrix using a camera position, a focal point, and an up direction.

For a left-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

##### Panics

Will panic if `up` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#469)

#### pub fn [look\_at\_rh](#method.look_at_rh)(eye: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), center: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), up: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a right-handed view matrix using a camera position, an up direction, and a focal point.

For a right-handed view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

##### Panics

Will panic if `up` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#476)

#### pub fn [to\_axis\_angle](#method.to_axis_angle)(self) -> ([Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Returns the rotation axis (normalized) and angle (in radians) of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#492)

#### pub fn [to\_scaled\_axis](#method.to_scaled_axis)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the rotation axis scaled by the rotation in radians.

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 290](../../src/custom_primitives/custom_primitives.rs.html#290))

```rust
283fn bounding_shapes_2d(
284    shapes: Query<&Transform, With<Shape2d>>,
285    mut gizmos: Gizmos,
286    bounding_shape: Res<State<BoundingShape>>,
287) {
288    for transform in shapes.iter() {
289        // Get the rotation angle from the 3D rotation.
290        let rotation = transform.rotation.to_scaled_axis().z;
291        let rotation = Rot2::radians(rotation);
292        let isometry = Isometry2d::new(transform.translation.xy(), rotation);
293
294        match bounding_shape.get() {
295            BoundingShape::None => (),
296            BoundingShape::BoundingBox => {
297                // Get the AABB of the primitive with the rotation and translation of the mesh.
298                let aabb = HEART.aabb_2d(isometry);
299                gizmos.rect_2d(aabb.center(), aabb.half_size() * 2., WHITE);
300            }
301            BoundingShape::BoundingSphere => {
302                // Get the bounding sphere of the primitive with the rotation and translation of the mesh.
303                let bounding_circle = HEART.bounding_circle(isometry);
304                gizmos
305                    .circle_2d(bounding_circle.center(), bounding_circle.radius(), WHITE)
306                    .resolution(64);
307            }
308        }
309    }
310}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#500)

#### pub fn [to\_euler](#method.to_euler)(self, order: [EulerRot](../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")) -> ([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Returns the rotation angles for the given euler rotation sequence.

##### [Examples found in repository](#scraped-examples-11)[?](../../scrape-examples-help.html)

examples/usage/debug\_frustum\_culling.rs ([line 362](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#362))

```rust
351fn move_free_camera(
352    new_transform: Transform,
353    mut free_camera_query: Query<
354        (&mut Transform, &mut FreeCameraState),
355        (With<Camera3d>, Without<MyCamera>),
356    >,
357) -> Result {
358    let (mut transform, mut state) = free_camera_query.single_mut()?;
359    *transform = new_transform;
360
361    // Update the yaw and pitch so that free camera orientation is updated correctly upon mouse grab
362    let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
363    state.yaw = yaw;
364    state.pitch = pitch;
365
366    Ok(())
367}
```

Hide additional examples

examples/2d/mesh2d\_arcs.rs ([line 111](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#111))

```rust
104fn draw_bounds<Shape: Bounded2d + Send + Sync + 'static>(
105    q: Query<(&DrawBounds<Shape>, &GlobalTransform)>,
106    mut gizmos: Gizmos,
107) {
108    for (shape, transform) in &q {
109        let (_, rotation, translation) = transform.to_scale_rotation_translation();
110        let translation = translation.truncate();
111        let rotation = rotation.to_euler(EulerRot::XYZ).2;
112        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
113
114        let aabb = shape.0.aabb_2d(isometry);
115        gizmos.rect_2d(aabb.center(), aabb.half_size() * 2.0, RED);
116
117        let bounding_circle = shape.0.bounding_circle(isometry);
118        gizmos.circle_2d(bounding_circle.center, bounding_circle.radius(), BLUE);
119    }
120}
```

examples/math/bounding\_2d.rs ([line 105](../../src/bounding_2d/bounding_2d.rs.html#105))

```rust
101fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
102    let color = GRAY;
103    for (shape, transform) in query.iter() {
104        let translation = transform.translation.xy();
105        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
106        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
107        match shape {
108            Shape::Rectangle(r) => {
109                gizmos.primitive_2d(r, isometry, color);
110            }
111            Shape::Circle(c) => {
112                gizmos.primitive_2d(c, isometry, color);
113            }
114            Shape::Triangle(t) => {
115                gizmos.primitive_2d(t, isometry, color);
116            }
117            Shape::Line(l) => {
118                gizmos.primitive_2d(l, isometry, color);
119            }
120            Shape::Capsule(c) => {
121                gizmos.primitive_2d(c, isometry, color);
122            }
123            Shape::Polygon(p) => {
124                gizmos.primitive_2d(p, isometry, color);
125            }
126        }
127    }
128}
129
130#[derive(Component)]
131enum DesiredVolume {
132    Aabb,
133    Circle,
134}
135
136#[derive(Component, Debug)]
137enum CurrentVolume {
138    Aabb(Aabb2d),
139    Circle(BoundingCircle),
140}
141
142fn update_volumes(
143    mut commands: Commands,
144    query: Query<
145        (Entity, &DesiredVolume, &Shape, &Transform),
146        Or<(Changed<DesiredVolume>, Changed<Shape>, Changed<Transform>)>,
147    >,
148) {
149    for (entity, desired_volume, shape, transform) in query.iter() {
150        let translation = transform.translation.xy();
151        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
152        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));
153        match desired_volume {
154            DesiredVolume::Aabb => {
155                let aabb = match shape {
156                    Shape::Rectangle(r) => r.aabb_2d(isometry),
157                    Shape::Circle(c) => c.aabb_2d(isometry),
158                    Shape::Triangle(t) => t.aabb_2d(isometry),
159                    Shape::Line(l) => l.aabb_2d(isometry),
160                    Shape::Capsule(c) => c.aabb_2d(isometry),
161                    Shape::Polygon(p) => p.aabb_2d(isometry),
162                };
163                commands.entity(entity).insert(CurrentVolume::Aabb(aabb));
164            }
165            DesiredVolume::Circle => {
166                let circle = match shape {
167                    Shape::Rectangle(r) => r.bounding_circle(isometry),
168                    Shape::Circle(c) => c.bounding_circle(isometry),
169                    Shape::Triangle(t) => t.bounding_circle(isometry),
170                    Shape::Line(l) => l.bounding_circle(isometry),
171                    Shape::Capsule(c) => c.bounding_circle(isometry),
172                    Shape::Polygon(p) => p.bounding_circle(isometry),
173                };
174                commands
175                    .entity(entity)
176                    .insert(CurrentVolume::Circle(circle));
177            }
178        }
179    }
180}
```

examples/movement/physics\_in\_fixed\_timestep.rs ([line 259](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#259))

```rust
244fn rotate_camera(
245    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
246    player: Single<(&mut Transform, &CameraSensitivity), With<Camera>>,
247) {
248    let (mut transform, camera_sensitivity) = player.into_inner();
249
250    let delta = accumulated_mouse_motion.delta;
251
252    if delta != Vec2::ZERO {
253        // Note that we are not multiplying by delta time here.
254        // The reason is that for mouse movement, we already get the full movement that happened since the last frame.
255        // This means that if we multiply by delta time, we will get a smaller rotation than intended by the user.
256        let delta_yaw = -delta.x * camera_sensitivity.x;
257        let delta_pitch = -delta.y * camera_sensitivity.y;
258
259        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
260        let yaw = yaw + delta_yaw;
261
262        // If the pitch was ±¹⁄₂ π, the camera would look straight up or down.
263        // When the user wants to move the camera back to the horizon, which way should the camera face?
264        // The camera has no way of knowing what direction was "forward" before landing in that extreme position,
265        // so the direction picked will for all intents and purposes be arbitrary.
266        // Another issue is that for mathematical reasons, the yaw will effectively be flipped when the pitch is at the extremes.
267        // To not run into these issues, we clamp the pitch to a safe range.
268        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
269        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
270
271        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
272    }
273}
```

examples/camera/camera\_orbit.rs ([line 126](../../src/camera_orbit/camera_orbit.rs.html#126))

```rust
99fn orbit(
100    mut camera: Single<&mut Transform, With<Camera>>,
101    camera_settings: Res<CameraSettings>,
102    mouse_buttons: Res<ButtonInput<MouseButton>>,
103    mouse_motion: Res<AccumulatedMouseMotion>,
104    time: Res<Time>,
105) {
106    let delta = mouse_motion.delta;
107    let mut delta_roll = 0.0;
108
109    if mouse_buttons.pressed(MouseButton::Left) {
110        delta_roll -= 1.0;
111    }
112    if mouse_buttons.pressed(MouseButton::Right) {
113        delta_roll += 1.0;
114    }
115
116    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
117    // as we are already receiving the full movement since the last frame was rendered. Multiplying
118    // by delta time here would make the movement slower that it should be.
119    let delta_pitch = delta.y * camera_settings.pitch_speed;
120    let delta_yaw = delta.x * camera_settings.yaw_speed;
121
122    // Conversely, we DO need to factor in delta time for mouse button inputs.
123    delta_roll *= camera_settings.roll_speed * time.delta_secs();
124
125    // Obtain the existing pitch, yaw, and roll values from the transform.
126    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
127
128    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
129    let pitch = (pitch + delta_pitch).clamp(
130        camera_settings.pitch_range.start,
131        camera_settings.pitch_range.end,
132    );
133    let roll = roll + delta_roll;
134    let yaw = yaw + delta_yaw;
135    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
136
137    // Adjust the translation to maintain the correct orientation toward the orbit target.
138    // In our example it's a static target, but this could easily be customized.
139    let target = Vec3::ZERO;
140    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
141}
```

examples/3d/clustered\_decals.rs ([line 428](../../src/clustered_decals/clustered_decals.rs.html#428))

```rust
386fn process_move_input(
387    mut selections: Query<(&mut Transform, &Selection)>,
388    mouse_buttons: Res<ButtonInput<MouseButton>>,
389    mouse_motion: Res<AccumulatedMouseMotion>,
390    app_status: Res<AppStatus>,
391) {
392    // Only process drags when movement is selected.
393    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Move {
394        return;
395    }
396
397    for (mut transform, selection) in &mut selections {
398        if app_status.selection != *selection {
399            continue;
400        }
401
402        let position = transform.translation;
403
404        // Convert to spherical coordinates.
405        let radius = position.length();
406        let mut theta = acos(position.y / radius);
407        let mut phi = position.z.signum() * acos(position.x * position.xz().length_recip());
408
409        // Camera movement is the inverse of object movement.
410        let (phi_factor, theta_factor) = match *selection {
411            Selection::Camera => (1.0, -1.0),
412            Selection::DecalA | Selection::DecalB => (-1.0, 1.0),
413        };
414
415        // Adjust the spherical coordinates. Clamp the inclination to (0, π).
416        phi += phi_factor * mouse_motion.delta.x * MOVE_SPEED;
417        theta = f32::clamp(
418            theta + theta_factor * mouse_motion.delta.y * MOVE_SPEED,
419            0.001,
420            PI - 0.001,
421        );
422
423        // Convert spherical coordinates back to Cartesian coordinates.
424        transform.translation =
425            radius * vec3(sin(theta) * cos(phi), cos(theta), sin(theta) * sin(phi));
426
427        // Look at the center, but preserve the previous roll angle.
428        let roll = transform.rotation.to_euler(EulerRot::YXZ).2;
429        transform.look_at(Vec3::ZERO, Vec3::Y);
430        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
431        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
432    }
433}
434
435/// Processes a drag event that scales the selected target.
436fn process_scale_input(
437    mut selections: Query<(&mut Transform, &Selection)>,
438    mouse_buttons: Res<ButtonInput<MouseButton>>,
439    mouse_motion: Res<AccumulatedMouseMotion>,
440    app_status: Res<AppStatus>,
441) {
442    // Only process drags when the scaling operation is selected.
443    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Scale {
444        return;
445    }
446
447    for (mut transform, selection) in &mut selections {
448        if app_status.selection == *selection {
449            transform.scale *= 1.0 + mouse_motion.delta.x * SCALE_SPEED;
450        }
451    }
452}
453
454/// Processes a drag event that rotates the selected target along its local Z
455/// axis.
456fn process_roll_input(
457    mut selections: Query<(&mut Transform, &Selection)>,
458    mouse_buttons: Res<ButtonInput<MouseButton>>,
459    mouse_motion: Res<AccumulatedMouseMotion>,
460    app_status: Res<AppStatus>,
461) {
462    // Only process drags when the rolling operation is selected.
463    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Roll {
464        return;
465    }
466
467    for (mut transform, selection) in &mut selections {
468        if app_status.selection != *selection {
469            continue;
470        }
471
472        let (yaw, pitch, mut roll) = transform.rotation.to_euler(EulerRot::YXZ);
473        roll += mouse_motion.delta.x * ROLL_SPEED;
474        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
475    }
476}
```

Additional examples can be found in:  

*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#223)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#558)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#510)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#507)

#### pub fn [to\_array](#method.to_array)(self) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

`[x, y, z, w]`

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#514)

#### pub fn [xyz](#method.xyz)(self) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Returns the vector part of the quaternion.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#522)

#### pub fn [conjugate](#method.conjugate)(self) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Returns the quaternion conjugate of `self`. For a unit quaternion the conjugate is also the inverse.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#538)

#### pub fn [inverse](#method.inverse)(self) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Returns the inverse of a normalized quaternion.

Typically quaternion inverse returns the conjugate of a normalized quaternion. Because `self` is assumed to already be unit length this method _does not_ normalize before returning the conjugate.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#547)

#### pub fn [dot](#method.dot)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the dot product of `self` and `rhs`. The dot product is equal to the cosine of the angle between two quaternion rotations.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#555)

#### pub fn [length](#method.length)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the length of `self`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#566)

#### pub fn [length\_squared](#method.length_squared)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes the squared length of `self`.

This is generally faster than `length()` as it avoids a square root operation.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#575)

#### pub fn [length\_recip](#method.length_recip)(self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Computes `1.0 / length()`.

For valid results, `self` must _not_ be of length zero.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#588)

#### pub fn [normalize](#method.normalize)(self) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Returns `self` normalized to length 1.0.

For valid results, `self` must _not_ be of length zero.

Panics

Will panic if `self` is zero length when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#596)

#### pub fn [is\_finite](#method.is_finite)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if, and only if, all elements are finite. If any element is either `NaN`, positive or negative infinity, this will return `false`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#603)

#### pub fn [is\_nan](#method.is_nan)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if any elements are `NAN`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#612)

#### pub fn [is\_normalized](#method.is_normalized)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns whether `self` of length `1.0` or not.

Uses a precision threshold of `1e-6`.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#618)

#### pub fn [is\_near\_identity](#method.is_near_identity)(self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#651)

#### pub fn [angle\_between](#method.angle_between)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Returns the angle (in radians) for the minimal rotation for transforming this quaternion into another.

Both quaternions must be normalized.

##### Panics

Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-12)[?](../../scrape-examples-help.html)

examples/transforms/align.rs ([line 153](../../src/align/align.rs.html#153))

```rust
140fn rotate_ship(ship: Single<(&mut Ship, &mut Transform)>, time: Res<Time>) {
141    let (mut ship, mut ship_transform) = ship.into_inner();
142
143    if !ship.in_motion {
144        return;
145    }
146
147    let target_rotation = ship.target_transform.rotation;
148
149    ship_transform
150        .rotation
151        .smooth_nudge(&target_rotation, 3.0, time.delta_secs());
152
153    if ship_transform.rotation.angle_between(target_rotation) <= f32::EPSILON {
154        ship.in_motion = false;
155    }
156}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#669)

#### pub fn [rotate\_towards](#method.rotate_towards)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), max\_angle: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Rotates towards `rhs` up to `max_angle` (in radians).

When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to `self.angle_between(rhs)`, the result will be equal to `rhs`. If `max_angle` is negative, rotates towards the exact opposite of `rhs`. Will not go past the target.

Both quaternions must be normalized.

##### Panics

Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#690)

#### pub fn [abs\_diff\_eq](#method.abs_diff_eq)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), max\_abs\_diff: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the absolute difference of all elements between `self` and `rhs` is less than or equal to `max_abs_diff`.

This can be used to compare if two quaternions contain similar elements. It works best when comparing with a known value. The `max_abs_diff` that should be used used depends on the values being compared against.

For more see [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#712)

#### pub fn [lerp](#method.lerp)(self, end: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), s: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs a linear interpolation between `self` and `rhs` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `rhs`.

##### Panics

Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-13)[?](../../scrape-examples-help.html)

examples/ecs/iter\_combinations.rs ([line 157](../../src/iter_combinations/iter_combinations.rs.html#157))

```rust
150fn look_at_star(
151    mut camera: Single<&mut Transform, (With<Camera>, Without<Star>)>,
152    star: Single<&Transform, With<Star>>,
153) {
154    let new_rotation = camera
155        .looking_at(star.translation, Vec3::Y)
156        .rotation
157        .lerp(camera.rotation, 0.1);
158    camera.rotation = new_rotation;
159}
```

Hide additional examples

examples/transforms/transform.rs ([line 120](../../src/transform/transform.rs.html#120))

```rust
101fn rotate_cube(
102    mut cubes: Query<(&mut Transform, &mut CubeState), Without<Center>>,
103    center_spheres: Query<&Transform, With<Center>>,
104    timer: Res<Time>,
105) {
106    // Calculate the point to circle around. (The position of the center_sphere)
107    let mut center: Vec3 = Vec3::ZERO;
108    for sphere in &center_spheres {
109        center += sphere.translation;
110    }
111    // Update the rotation of the cube(s).
112    for (mut transform, cube) in &mut cubes {
113        // Calculate the rotation of the cube if it would be looking at the sphere in the center.
114        let look_at_sphere = transform.looking_at(center, *transform.local_y());
115        // Interpolate between the current rotation and the fully turned rotation
116        // when looking at the sphere, with a given turn speed to get a smooth motion.
117        // With higher speed the curvature of the orbit would be smaller.
118        let incremental_turn_weight = cube.turn_speed * timer.delta_secs();
119        let old_rotation = transform.rotation;
120        transform.rotation = old_rotation.lerp(look_at_sphere.rotation, incremental_turn_weight);
121    }
122}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#737)

#### pub fn [slerp](#method.slerp)(self, end: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), s: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs a spherical linear interpolation between `self` and `end` based on the value `s`.

When `s` is `0.0`, the result will be equal to `self`. When `s` is `1.0`, the result will be equal to `end`.

##### Panics

Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-14)[?](../../scrape-examples-help.html)

examples/gizmos/axes.rs ([line 212](../../src/axes/axes.rs.html#212))

```rust
210fn interpolate_transforms(t1: Transform, t2: Transform, t: f32) -> Transform {
211    let translation = t1.translation.lerp(t2.translation, t);
212    let rotation = t1.rotation.slerp(t2.rotation, t);
213    let scale = elerp(t1.scale, t2.scale, t);
214
215    Transform {
216        translation,
217        rotation,
218        scale,
219    }
220}
```

Hide additional examples

examples/3d/parallax\_mapping.rs ([line 197](../../src/parallax_mapping/parallax_mapping.rs.html#197))

```rust
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
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#788)

#### pub fn [mul\_vec3](#method.mul_vec3)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Multiplies a quaternion and a 3D vector, returning the rotated vector.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

##### [Examples found in repository](#scraped-examples-15)[?](../../scrape-examples-help.html)

tests/3d/test\_invalid\_skinned\_mesh.rs ([line 229](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#229))

```rust
223fn update_animated_joints(time: Res<Time>, query: Query<&mut Transform, With<AnimatedJoint>>) {
224    for mut transform in query {
225        let angle = TAU * 4.0 * ops::cos((time.elapsed_secs() / 8.0) * TAU);
226        let rotation = Quat::from_rotation_z(angle);
227
228        transform.rotation = rotation;
229        transform.translation = rotation.mul_vec3(Vec3::new(0.0, 1.3, 0.0));
230    }
231}
```

Hide additional examples

examples/3d/anisotropy.rs ([line 207](../../src/anisotropy/anisotropy.rs.html#207))

```rust
194fn rotate_camera(
195    mut camera: Query<&mut Transform, With<Camera>>,
196    app_status: Res<AppStatus>,
197    time: Res<Time>,
198    mut stopwatch: Local<Stopwatch>,
199) {
200    if app_status.light_mode == LightMode::EnvironmentMap {
201        stopwatch.tick(time.delta());
202    }
203
204    let now = stopwatch.elapsed_secs();
205    for mut transform in camera.iter_mut() {
206        *transform = Transform::from_translation(
207            Quat::from_rotation_y(now).mul_vec3(CAMERA_INITIAL_POSITION),
208        )
209        .looking_at(Vec3::ZERO, Vec3::Y);
210    }
211}
```

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#804)

#### pub fn [mul\_quat](#method.mul_quat)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Multiplies two quaternions. If they each represent a rotation, the result will represent the combined rotation.

Note that due to floating point rounding the result may not be perfectly normalized.

##### Panics

Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#853)

#### pub fn [from\_affine3](#method.from_affine3)(a: &[Affine3](struct.Affine3.html "struct bevy::math::Affine3")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

Note if the input affine matrix contain scales, shears, or other non-rotation transformations then the resulting quaternion will be ill-defined.

##### Panics

Will panic if any input affine matrix column is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#868)

#### pub fn [from\_affine3a](#method.from_affine3a)(a: &[Affine3A](struct.Affine3A.html "struct bevy::math::Affine3A")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

Note if the input affine matrix contain scales, shears, or other non-rotation transformations then the resulting quaternion will be ill-defined.

##### Panics

Will panic if any input affine matrix column is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#879)

#### pub fn [mul\_vec3a](#method.mul_vec3a)(self, rhs: [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Multiplies a quaternion and a 3D vector, returning the rotated vector.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#897)

#### pub fn [as\_dquat](#method.as_dquat)(self) -> [DQuat](struct.DQuat.html "struct bevy::math::DQuat")

## Trait Implementations

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#927)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#936)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Adds two quaternions.

The sum is not guaranteed to be normalized.

Note that addition is not the same as combining the rotations represented by the two quaternions! That corresponds to multiplication.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#928)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#941)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#942)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#944)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#949)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#950)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#952)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#957)

### impl [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#958)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `+` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#960)

#### fn [add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `+` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#tymethod.add)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#965)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#967)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#972)

### impl [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#974)

#### fn [add\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)(&mut self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"))

Performs the `+=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html#tymethod.add_assign)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#172)

### impl [Animatable](../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#175)

#### fn [interpolate](../prelude/trait.Animatable.html#tymethod.interpolate)(a: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), b: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs a slerp to smoothly interpolate between quaternions.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animatable.rs.html#182)

#### fn [blend](../prelude/trait.Animatable.html#tymethod.blend)(inputs: impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [BlendInput](../prelude/struct.BlendInput.html "struct bevy::prelude::BlendInput")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>>) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Blends one or more values together. [Read more](../prelude/trait.Animatable.html#tymethod.blend)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1279)

### impl [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1281)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> &\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#49)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#49)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#49)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#120)

### impl [Curve](../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [CubicRotationCurve](../animation/gltf_curves/struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#122)

#### fn [domain](../prelude/trait.Curve.html#tymethod.domain)(&self) -> [Interval](../prelude/struct.Interval.html "struct bevy::prelude::Interval")

The interval over which this curve is parametrized. [Read more](../prelude/trait.Curve.html#tymethod.domain)

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#127)

#### fn [sample\_clamped](../prelude/trait.Curve.html#method.sample_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Sample a point on this curve at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#142)

#### fn [sample\_unchecked](../prelude/trait.Curve.html#tymethod.sample_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Sample a point on this curve at the parameter value `t`, extracting the associated value. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain. [Read more](../prelude/trait.Curve.html#tymethod.sample_unchecked)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#340)

#### fn [sample](../prelude/trait.Curve.html#method.sample)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Sample a point on this curve at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#902)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#903)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, fmt: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1265)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1267)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1350)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1351)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = Vec4<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>

The resulting type after dereferencing.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1353)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1358)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1360)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Deserialize expects a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#913)

### impl [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#914)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1087)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1088)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1090)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1095)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1096)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1098)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1077)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1082)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Divides a quaternion by a scalar value. The quotient is not guaranteed to be normalized.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1078)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1103)

### impl [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1104)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `/` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1106)

#### fn [div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `/` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#tymethod.div)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1118)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1120)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1111)

### impl [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1113)

#### fn [div\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `/=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html#tymethod.div_assign)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#110)

### impl [Ease](../prelude/trait.Ease.html "trait bevy::prelude::Ease") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#111)

#### fn [interpolating\_curve\_unbounded](../prelude/trait.Ease.html#tymethod.interpolating_curve_unbounded)(start: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), end: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> impl [Curve](../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>

Given `start` and `end` values, produce a curve with [unlimited domain](../prelude/struct.Interval.html#associatedconstant.EVERYWHERE "associated constant bevy::prelude::Interval::EVERYWHERE") that: [Read more](../prelude/trait.Ease.html#tymethod.interpolating_curve_unbounded)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1322)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1324)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(q: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

Converts to this type from the input type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1336)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1338)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(q: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> \[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

Converts to this type from the input type.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#497)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#499)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(rotation: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### type [This](../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The type to convert into. [Read more](../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [from\_arg](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [FromArg](../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [from\_reflect](../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#99)

### impl [FromRng](../prelude/trait.FromRng.html "trait bevy::prelude::FromRng") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/standard.rs.html#51)

#### fn [from\_rng](../prelude/trait.FromRng.html#method.from_rng)<R>(rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> Self

where R: [RngExt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Construct a value of this type uniformly at random using `rng` as the source of randomness.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [GetOwnership](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [ownership](../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [get\_type\_registration](../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [register\_type\_dependencies](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [IntoReturn](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [into\_return](../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"): 'into\_return,

Converts [`Self`](../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1125)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1137)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Multiplies two quaternions. If they each represent a rotation, the result will represent the combined rotation.

Note that due to floating point rounding the result may not be perfectly normalized.

##### Panics

Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1126)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1142)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1143)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1145)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1150)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1151)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1153)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1193)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1194)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1196)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1201)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1202)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1204)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1225)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1226)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1228)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1233)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1234)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1236)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1039)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1040)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1042)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1047)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1048)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1050)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#740)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Dir3](../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#744)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, direction: [Dir3](../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")) -> <[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Dir3](../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Rotates the [`Dir3`](../prelude/struct.Dir3.html "struct bevy::prelude::Dir3") using a [`Quat`](../prelude/struct.Quat.html "struct bevy::prelude::Quat").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#741)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Dir3](../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#993)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Dir3A](../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#997)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, direction: [Dir3A](../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")) -> <[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Dir3A](../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Rotates the [`Dir3A`](../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A") using a [`Quat`](../prelude/struct.Quat.html "struct bevy::prelude::Quat").

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#994)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Dir3A](../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1158)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1159)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1161)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1180)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1188)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> <[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Multiplies a quaternion and a 3D vector, returning the rotated vector.

##### Panics

Will panic if `self` is not normalized when `glam_assert` is enabled.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1181)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1209)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1210)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1212)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")) -> [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1217)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1218)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1220)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> <[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat") as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\>>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1241)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1242)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1244)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")) -> [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1028)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1034)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Multiplies a quaternion by a scalar value.

The product is not guaranteed to be normalized.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1029)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1055)

### impl [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1056)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `*` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1058)

#### fn [mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)(self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `*` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#tymethod.mul)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1166)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1168)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1173)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1175)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1070)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<&[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1072)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: &[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1063)

### impl [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1065)

#### fn [mul\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)(&mut self, rhs: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Performs the `*=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html#tymethod.mul_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1249)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1250)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1252)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1257)

### impl [Neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html "trait core::ops::arith::Neg") for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1258)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1260)

#### fn [neg](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)(self) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the unary `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Neg.html#tymethod.neg)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1272)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1274)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [get\_represented\_type\_info](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [try\_apply](../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [reflect\_kind](../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [reflect\_ref](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [reflect\_owned](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>) -> [ReflectOwned](../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [try\_into\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [try\_as\_reflect](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [try\_as\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [into\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [as\_partial\_reflect](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [as\_partial\_reflect\_mut](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#451)

#### fn [reflect\_partial\_eq](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [reflect\_partial\_cmp](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#451)

#### fn [debug](../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#451)

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

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#50)

### impl [Pod](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/pod/trait.Pod.html "trait bytemuck::pod::Pod") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1304)

### impl [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1305-1307)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1313)

### impl<'a> [Product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html "trait core::iter::traits::accum::Product")<&'a [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1314-1316)

#### fn [product](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Product.html#tymethod.product)<I>(iter: I) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>,

Takes an iterator and generates `Self` from the elements by multiplying the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [into\_any](../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [as\_any](../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [as\_any\_mut](../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [into\_reflect](../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [as\_reflect](../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [as\_reflect\_mut](../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [set](../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Serialize as a sequence of 4 values.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/features/impl_serde.rs.html#1058)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#492)

### impl [StableInterpolate](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#494)

#### fn [interpolate\_stable](../prelude/trait.StableInterpolate.html#tymethod.interpolate_stable)(&self, other: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Interpolate between this value and the `other` given value using the parameter `t`. At `t = 0.0`, a value equivalent to `self` is recovered, while `t = 1.0` recovers a value equivalent to `other`, with intermediate values interpolating between the two. See the [trait-level documentation](../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for details.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#438)

#### fn [interpolate\_stable\_assign](../prelude/trait.StableInterpolate.html#method.interpolate_stable_assign)(&mut self, other: &Self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

A version of [`interpolate_stable`](../prelude/trait.StableInterpolate.html#tymethod.interpolate_stable "method bevy::prelude::StableInterpolate::interpolate_stable") that assigns the result to `self` for convenience.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#467)

#### fn [smooth\_nudge](../prelude/trait.StableInterpolate.html#method.smooth_nudge)(&mut self, target: &Self, decay\_rate: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), delta: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Smoothly nudge this value towards the `target` at a given decay rate. The `decay_rate` parameter controls how fast the distance between `self` and `target` decays relative to the units of `delta`; the intended usage is for `decay_rate` to generally remain fixed, while `delta` is something like `delta_time` from an updating system. This produces a smooth following of the target that is independent of framerate. [Read more](../prelude/trait.StableInterpolate.html#method.smooth_nudge)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [Struct](../prelude/trait.Struct.html "trait bevy::prelude::Struct") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [field](../prelude/trait.Struct.html#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [field\_mut](../prelude/trait.Struct.html#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [field\_at](../prelude/trait.Struct.html#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [field\_at\_mut](../prelude/trait.Struct.html#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [name\_at](../prelude/trait.Struct.html#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [index\_of\_name](../prelude/trait.Struct.html#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [field\_len](../prelude/trait.Struct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [iter\_fields](../prelude/trait.Struct.html#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [to\_dynamic\_struct](../prelude/trait.Struct.html#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](../prelude/trait.Struct.html#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#979)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#985)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Subtracts the `rhs` quaternion from `self`.

The difference is not guaranteed to be normalized.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#980)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#990)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#991)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#993)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#998)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#999)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1001)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1006)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")<[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1007)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1009)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

Performs the `-` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1014)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1016)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1021)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign")<&[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1023)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, rhs: &[Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat"))

Performs the `-=` operation. [Read more](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1286)

### impl [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1287-1289)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1295)

### impl<'a> [Sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html "trait core::iter::traits::accum::Sum")<&'a [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\> for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#1296-1298)

#### fn [sum](https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html#tymethod.sum)<I>(iter: I) -> [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &'a [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")\>,

Takes an iterator and generates `Self` from the elements by “summing up” the items.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

#### fn [type\_info](../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/quat.rs.html#50)

### impl [Zeroable](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html "trait bytemuck::zeroable::Zeroable") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/src/bytemuck/zeroable.rs.html#32)

#### fn [zeroed](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)() -> Self

Calls [`zeroed`](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html "fn core::mem::zeroed"). [Read more](https://docs.rs/bytemuck/1.25.0/x86_64-unknown-linux-gnu/bytemuck/zeroable/trait.Zeroable.html#method.zeroed)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

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

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

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