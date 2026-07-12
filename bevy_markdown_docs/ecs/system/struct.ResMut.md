[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Struct ResMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#535)

```rust
pub struct ResMut<'w, T>where
    T: Resource<Mutability = Mutable> + ?Sized,{ /* private fields */ }
```

Unique mutable borrow of a [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource").

See the [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") documentation for usage.

If you need a shared borrow, use [`Res`](../../prelude/struct.Res.html "struct bevy::prelude::Res") instead.

This [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") fails validation if resource doesn’t exist. This will cause a panic, but can be configured to do nothing or warn once.

Use [`Option<ResMut<T>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") instead if the resource might not always exist.

## Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#567)

### impl<'w, T> [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#567)

#### pub fn [into\_inner](#method.into_inner)(self) -> [&'w mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Consume `self` and return a mutable reference to the contained value while marking `self` as “changed”.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/stress\_tests/bevymark\_3d.rs ([line 154](../../../src/bevymark_3d/bevymark_3d.rs.html#154))

```rust
146fn scheduled_spawner(
147    mut commands: Commands,
148    args: Res<Args>,
149    mut scheduled: ResMut<CubeScheduled>,
150    mut counter: ResMut<BevyCounter>,
151    cube_resources: ResMut<CubeResources>,
152) {
153    if scheduled.waves > 0 {
154        let cube_resources = cube_resources.into_inner();
155        spawn_cubes(
156            &mut commands,
157            args.into_inner(),
158            &mut counter,
159            scheduled.per_wave,
160            cube_resources,
161            None,
162            scheduled.waves - 1,
163        );
164
165        scheduled.waves -= 1;
166    }
167}
168
169#[derive(Resource)]
170struct CubeResources {
171    _textures: Vec<Handle<Image>>,
172    materials: Vec<Handle<StandardMaterial>>,
173    cube_mesh: Handle<Mesh>,
174    color_rng: ChaCha8Rng,
175    material_rng: ChaCha8Rng,
176    velocity_rng: ChaCha8Rng,
177    transform_rng: ChaCha8Rng,
178}
179
180#[derive(Component)]
181struct StatsText;
182
183fn setup(
184    mut commands: Commands,
185    args: Res<Args>,
186    asset_server: Res<AssetServer>,
187    mut meshes: ResMut<Assets<Mesh>>,
188    material_assets: ResMut<Assets<StandardMaterial>>,
189    images: ResMut<Assets<Image>>,
190    counter: ResMut<BevyCounter>,
191) {
192    let args = args.into_inner();
193    let images = images.into_inner();
194
195    let mut textures = Vec::with_capacity(args.material_texture_count.max(1));
196    if args.material_texture_count > 0 {
197        textures.push(asset_server.load("branding/icon.png"));
198    }
199    init_textures(&mut textures, args, images);
200
201    let material_assets = material_assets.into_inner();
202    let materials = init_materials(args, &textures, material_assets);
203
204    let mut cube_resources = CubeResources {
205        _textures: textures,
206        materials,
207        cube_mesh: meshes.add(Cuboid::from_size(Vec3::splat(CUBE_SCALE))),
208        color_rng: ChaCha8Rng::seed_from_u64(42),
209        material_rng: ChaCha8Rng::seed_from_u64(12),
210        velocity_rng: ChaCha8Rng::seed_from_u64(97),
211        transform_rng: ChaCha8Rng::seed_from_u64(26),
212    };
213
214    let font = TextFont {
215        font_size: FontSize::Px(40.0),
216        ..Default::default()
217    };
218
219    commands.spawn((
220        Camera3d::default(),
221        Transform::from_translation(VOLUME_SIZE * 1.3).looking_at(Vec3::ZERO, Vec3::Y),
222    ));
223
224    commands.spawn((
225        DirectionalLight {
226            illuminance: 10000.0,
227            shadow_maps_enabled: false,
228            ..default()
229        },
230        Transform::from_xyz(1.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
231    ));
232
233    commands.spawn((
234        Node {
235            position_type: PositionType::Absolute,
236            padding: UiRect::all(px(5)),
237            ..default()
238        },
239        BackgroundColor(Color::BLACK.with_alpha(0.75)),
240        GlobalZIndex(i32::MAX),
241        children![(
242            Text::default(),
243            StatsText,
244            children![
245                (
246                    TextSpan::new("Cube Count: "),
247                    font.clone(),
248                    TextColor(LIME.into()),
249                ),
250                (TextSpan::new(""), font.clone(), TextColor(AQUA.into())),
251                (
252                    TextSpan::new("\nFPS (raw): "),
253                    font.clone(),
254                    TextColor(LIME.into()),
255                ),
256                (TextSpan::new(""), font.clone(), TextColor(AQUA.into())),
257                (
258                    TextSpan::new("\nFPS (SMA): "),
259                    font.clone(),
260                    TextColor(LIME.into()),
261                ),
262                (TextSpan::new(""), font.clone(), TextColor(AQUA.into())),
263                (
264                    TextSpan::new("\nFPS (EMA): "),
265                    font.clone(),
266                    TextColor(LIME.into()),
267                ),
268                (TextSpan::new(""), font.clone(), TextColor(AQUA.into()))
269            ]
270        )],
271    ));
272
273    let mut scheduled = CubeScheduled {
274        per_wave: args.per_wave,
275        waves: args.waves,
276    };
277
278    if args.benchmark {
279        let counter = counter.into_inner();
280        for wave in (0..scheduled.waves).rev() {
281            spawn_cubes(
282                &mut commands,
283                args,
284                counter,
285                scheduled.per_wave,
286                &mut cube_resources,
287                Some(wave),
288                wave,
289            );
290        }
291        scheduled.waves = 0;
292    }
293    commands.insert_resource(cube_resources);
294    commands.insert_resource(scheduled);
295}
296
297fn mouse_handler(
298    mut commands: Commands,
299    args: Res<Args>,
300    time: Res<Time>,
301    mouse_button_input: Res<ButtonInput<MouseButton>>,
302    cube_resources: ResMut<CubeResources>,
303    mut counter: ResMut<BevyCounter>,
304    mut rng: Local<Option<ChaCha8Rng>>,
305    mut wave: Local<usize>,
306) {
307    if rng.is_none() {
308        *rng = Some(ChaCha8Rng::seed_from_u64(42));
309    }
310    let rng = rng.as_mut().unwrap();
311
312    if mouse_button_input.just_released(MouseButton::Left) {
313        counter.color = Color::linear_rgb(rng.random(), rng.random(), rng.random());
314    }
315
316    if mouse_button_input.pressed(MouseButton::Left) {
317        let spawn_count = (CUBES_PER_SECOND as f64 * time.delta_secs_f64()) as usize;
318        spawn_cubes(
319            &mut commands,
320            args.into_inner(),
321            &mut counter,
322            spawn_count,
323            cube_resources.into_inner(),
324            None,
325            *wave,
326        );
327        *wave += 1;
328    }
329}
```

Hide additional examples

examples/stress\_tests/bevymark.rs ([line 187](../../../src/bevymark/bevymark.rs.html#187))

```rust
178fn scheduled_spawner(
179    mut commands: Commands,
180    args: Res<Args>,
181    window: Single<&Window>,
182    mut scheduled: ResMut<BirdScheduled>,
183    mut counter: ResMut<BevyCounter>,
184    bird_resources: ResMut<BirdResources>,
185) {
186    if scheduled.waves > 0 {
187        let bird_resources = bird_resources.into_inner();
188        spawn_birds(
189            &mut commands,
190            args.into_inner(),
191            &window.resolution,
192            &mut counter,
193            scheduled.per_wave,
194            bird_resources,
195            None,
196            scheduled.waves - 1,
197        );
198
199        scheduled.waves -= 1;
200    }
201}
202
203#[derive(Resource)]
204struct BirdResources {
205    textures: Vec<Handle<Image>>,
206    materials: Vec<Handle<ColorMaterial>>,
207    quad: Handle<Mesh>,
208    color_rng: ChaCha8Rng,
209    material_rng: ChaCha8Rng,
210    velocity_rng: ChaCha8Rng,
211    transform_rng: ChaCha8Rng,
212}
213
214#[derive(Component)]
215struct StatsText;
216
217fn setup(
218    mut commands: Commands,
219    args: Res<Args>,
220    asset_server: Res<AssetServer>,
221    mut meshes: ResMut<Assets<Mesh>>,
222    material_assets: ResMut<Assets<ColorMaterial>>,
223    images: ResMut<Assets<Image>>,
224    window: Single<&Window>,
225    counter: ResMut<BevyCounter>,
226) {
227    warn!(include_str!("warning_string.txt"));
228
229    let args = args.into_inner();
230    let images = images.into_inner();
231
232    let mut textures = Vec::with_capacity(args.material_texture_count.max(1));
233    if matches!(args.mode, Mode::Sprite) || args.material_texture_count > 0 {
234        textures.push(asset_server.load("branding/icon.png"));
235    }
236    init_textures(&mut textures, args, images);
237
238    let material_assets = material_assets.into_inner();
239    let materials = init_materials(args, &textures, material_assets);
240
241    let mut bird_resources = BirdResources {
242        textures,
243        materials,
244        quad: meshes.add(Rectangle::from_size(Vec2::splat(BIRD_TEXTURE_SIZE as f32))),
245        // We're seeding the PRNG here to make this example deterministic for testing purposes.
246        // This isn't strictly required in practical use unless you need your app to be deterministic.
247        color_rng: ChaCha8Rng::seed_from_u64(42),
248        material_rng: ChaCha8Rng::seed_from_u64(42),
249        velocity_rng: ChaCha8Rng::seed_from_u64(42),
250        transform_rng: ChaCha8Rng::seed_from_u64(42),
251    };
252
253    let font = TextFont {
254        font_size: FontSize::Px(40.0),
255        ..Default::default()
256    };
257
258    commands.spawn(Camera2d);
259    commands
260        .spawn((
261            Node {
262                position_type: PositionType::Absolute,
263                padding: UiRect::all(px(5)),
264                ..default()
265            },
266            BackgroundColor(Color::BLACK.with_alpha(0.75)),
267            GlobalZIndex(i32::MAX),
268        ))
269        .with_children(|p| {
270            p.spawn((Text::default(), StatsText)).with_children(|p| {
271                p.spawn((
272                    TextSpan::new("Bird Count: "),
273                    font.clone(),
274                    TextColor(LIME.into()),
275                ));
276                p.spawn((TextSpan::new(""), font.clone(), TextColor(AQUA.into())));
277                p.spawn((
278                    TextSpan::new("\nFPS (raw): "),
279                    font.clone(),
280                    TextColor(LIME.into()),
281                ));
282                p.spawn((TextSpan::new(""), font.clone(), TextColor(AQUA.into())));
283                p.spawn((
284                    TextSpan::new("\nFPS (SMA): "),
285                    font.clone(),
286                    TextColor(LIME.into()),
287                ));
288                p.spawn((TextSpan::new(""), font.clone(), TextColor(AQUA.into())));
289                p.spawn((
290                    TextSpan::new("\nFPS (EMA): "),
291                    font.clone(),
292                    TextColor(LIME.into()),
293                ));
294                p.spawn((TextSpan::new(""), font.clone(), TextColor(AQUA.into())));
295            });
296        });
297
298    let mut scheduled = BirdScheduled {
299        per_wave: args.per_wave,
300        waves: args.waves,
301    };
302
303    if args.benchmark {
304        let counter = counter.into_inner();
305        for wave in (0..scheduled.waves).rev() {
306            spawn_birds(
307                &mut commands,
308                args,
309                &window.resolution,
310                counter,
311                scheduled.per_wave,
312                &mut bird_resources,
313                Some(wave),
314                wave,
315            );
316        }
317        scheduled.waves = 0;
318    }
319    commands.insert_resource(bird_resources);
320    commands.insert_resource(scheduled);
321}
322
323fn mouse_handler(
324    mut commands: Commands,
325    args: Res<Args>,
326    time: Res<Time>,
327    mouse_button_input: Res<ButtonInput<MouseButton>>,
328    window: Query<&Window>,
329    bird_resources: ResMut<BirdResources>,
330    mut counter: ResMut<BevyCounter>,
331    mut rng: Local<Option<ChaCha8Rng>>,
332    mut wave: Local<usize>,
333) {
334    let Ok(window) = window.single() else {
335        return;
336    };
337
338    if rng.is_none() {
339        // We're seeding the PRNG here to make this example deterministic for testing purposes.
340        // This isn't strictly required in practical use unless you need your app to be deterministic.
341        *rng = Some(ChaCha8Rng::seed_from_u64(42));
342    }
343    let rng = rng.as_mut().unwrap();
344
345    if mouse_button_input.just_released(MouseButton::Left) {
346        counter.color = Color::linear_rgb(rng.random(), rng.random(), rng.random());
347    }
348
349    if mouse_button_input.pressed(MouseButton::Left) {
350        let spawn_count = (BIRDS_PER_SECOND as f64 * time.delta_secs_f64()) as usize;
351        spawn_birds(
352            &mut commands,
353            args.into_inner(),
354            &window.resolution,
355            &mut counter,
356            spawn_count,
357            bird_resources.into_inner(),
358            None,
359            *wave,
360        );
361        *wave += 1;
362    }
363}
```

examples/stress\_tests/many\_materials.rs ([line 54](../../../src/many_materials/many_materials.rs.html#54))

```rust
47fn setup(
48    mut commands: Commands,
49    args: Res<Args>,
50    mesh_assets: ResMut<Assets<Mesh>>,
51    material_assets: ResMut<Assets<StandardMaterial>>,
52) {
53    let args = args.into_inner();
54    let material_assets = material_assets.into_inner();
55    let mesh_assets = mesh_assets.into_inner();
56    let n = args.grid_size;
57
58    // Camera
59    let w = n as f32;
60    commands.spawn((
61        Camera3d::default(),
62        Transform::from_xyz(w * 1.25, w + 1.0, w * 1.25)
63            .looking_at(Vec3::new(0.0, (w * -1.1) + 1.0, 0.0), Vec3::Y),
64    ));
65
66    // Light
67    commands.spawn((
68        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
69        DirectionalLight {
70            illuminance: 3000.0,
71            shadow_maps_enabled: true,
72            ..default()
73        },
74    ));
75
76    // Cubes
77    let mesh_handle = mesh_assets.add(Cuboid::from_size(Vec3::ONE));
78    for x in 0..n {
79        for z in 0..n {
80            commands.spawn((
81                Mesh3d(mesh_handle.clone()),
82                MeshMaterial3d(material_assets.add(Color::WHITE)),
83                Transform::from_translation(Vec3::new(x as f32, 0.0, z as f32)),
84            ));
85        }
86    }
87}
```

examples/stress\_tests/many\_cubes.rs ([line 171](../../../src/many_cubes/many_cubes.rs.html#171))

```rust
161fn setup(
162    mut commands: Commands,
163    args: Res<Args>,
164    mesh_assets: ResMut<Assets<Mesh>>,
165    material_assets: ResMut<Assets<StandardMaterial>>,
166    images: ResMut<Assets<Image>>,
167) {
168    warn!(include_str!("warning_string.txt"));
169
170    let args = args.into_inner();
171    let images = images.into_inner();
172    let material_assets = material_assets.into_inner();
173    let mesh_assets = mesh_assets.into_inner();
174
175    let meshes = init_meshes(args, mesh_assets);
176
177    let material_textures = init_textures(args, images);
178    let materials = init_materials(args, &material_textures, material_assets);
179
180    // We're seeding the PRNG here to make this example deterministic for testing purposes.
181    // This isn't strictly required in practical use unless you need your app to be deterministic.
182    let mut material_rng = ChaCha8Rng::seed_from_u64(42);
183    match args.layout {
184        Layout::Sphere => {
185            // NOTE: This pattern is good for testing performance of culling as it provides roughly
186            // the same number of visible meshes regardless of the viewing angle.
187            let n_points: usize = args.instance_count;
188            // NOTE: f64 is used to avoid precision issues that produce visual artifacts in the distribution
189            let radius = WIDTH as f64 * 2.5;
190            let golden_ratio = 0.5f64 * (1.0f64 + 5.0f64.sqrt());
191            for i in 0..n_points {
192                let spherical_polar_theta_phi =
193                    fibonacci_spiral_on_sphere(golden_ratio, i, n_points);
194                let unit_sphere_p = spherical_polar_to_cartesian(spherical_polar_theta_phi);
195                let (mesh, transform) = meshes.choose(&mut material_rng).unwrap();
196                commands
197                    .spawn((
198                        Mesh3d(mesh.clone()),
199                        MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
200                        Transform::from_translation((radius * unit_sphere_p).as_vec3())
201                            .looking_at(Vec3::ZERO, Vec3::Y)
202                            .mul_transform(*transform),
203                    ))
204                    .insert_if(NoFrustumCulling, || args.no_frustum_culling)
205                    .insert_if(NoAutomaticBatching, || args.no_automatic_batching)
206                    .insert_if(NoCpuCulling, || args.no_cpu_culling);
207            }
208
209            // camera
210            let mut camera = commands.spawn(Camera3d::default());
211            if args.no_indirect_drawing {
212                camera.insert(NoIndirectDrawing);
213            }
214            if args.no_cpu_culling {
215                camera.insert(NoCpuCulling);
216            }
217            if args.motion_blur {
218                camera.insert((
219                    MotionBlur {
220                        // Use an unrealistically large shutter angle so that motion blur is clearly visible.
221                        shutter_angle: 3.0,
222                        ..Default::default()
223                    },
224                    // MSAA and MotionBlur are not compatible on WebGL.
225                    #[cfg(all(
226                        feature = "webgl2",
227                        target_arch = "wasm32",
228                        not(feature = "webgpu")
229                    ))]
230                    Msaa::Off,
231                ));
232            }
233
234            // Inside-out box around the meshes onto which shadows are cast (though you cannot see them...)
235            commands.spawn((
236                Mesh3d(mesh_assets.add(Cuboid::from_size(Vec3::splat(radius as f32 * 2.2)))),
237                MeshMaterial3d(material_assets.add(StandardMaterial::from(Color::WHITE))),
238                Transform::from_scale(-Vec3::ONE),
239                NotShadowCaster,
240            ));
241        }
242        Layout::Cube => {
243            // NOTE: This pattern is good for demonstrating that frustum culling is working correctly
244            // as the number of visible meshes rises and falls depending on the viewing angle.
245            let scale = 2.5;
246
247            // Scale the width and height by the same factor so that we have the
248            // right number of instances.
249            // Because of the moiré pattern check and the fact that we're
250            // spawning 4 instances per trip around the inner loop below, we're
251            // solving the following equation for the factor variable:
252            //
253            //      4 * (9/10 * factor * width * 9/10 * factor * height) = count
254            //
255            // The solution is the value below.
256            let factor = (5.0 / 9.0) * sqrt(args.instance_count as f32)
257                / (sqrt(HEIGHT as f32) * sqrt(WIDTH as f32));
258            let dimensions = (vec2(WIDTH as f32, HEIGHT as f32) * factor)
259                .ceil()
260                .as_uvec2();
261
262            for x in 0..dimensions.x {
263                for y in 0..dimensions.y {
264                    // introduce spaces to break any kind of moiré pattern
265                    if x % 10 == 0 || y % 10 == 0 {
266                        continue;
267                    }
268                    // cube
269                    commands
270                        .spawn((
271                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
272                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
273                            Transform::from_xyz((x as f32) * scale, (y as f32) * scale, 0.0),
274                        ))
275                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
276                    commands
277                        .spawn((
278                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
279                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
280                            Transform::from_xyz(
281                                (x as f32) * scale,
282                                dimensions.y as f32 * scale,
283                                (y as f32) * scale,
284                            ),
285                        ))
286                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
287                    commands
288                        .spawn((
289                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
290                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
291                            Transform::from_xyz((x as f32) * scale, 0.0, (y as f32) * scale),
292                        ))
293                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
294                    commands
295                        .spawn((
296                            Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
297                            MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
298                            Transform::from_xyz(0.0, (x as f32) * scale, (y as f32) * scale),
299                        ))
300                        .insert_if(NoCpuCulling, || args.no_cpu_culling);
301                }
302            }
303            // camera
304            let center = 0.5
305                * scale
306                * Vec3::new(
307                    dimensions.x as f32,
308                    dimensions.y as f32,
309                    dimensions.x as f32,
310                );
311            commands.spawn((Camera3d::default(), Transform::from_translation(center)));
312            // Inside-out box around the meshes onto which shadows are cast (though you cannot see them...)
313            commands.spawn((
314                Mesh3d(mesh_assets.add(Cuboid::from_size(2.0 * 1.1 * center))),
315                MeshMaterial3d(material_assets.add(StandardMaterial::from(Color::WHITE))),
316                Transform::from_scale(-Vec3::ONE).with_translation(center),
317                NotShadowCaster,
318            ));
319        }
320        Layout::Dense => {
321            // NOTE: This pattern is good for demonstrating a dense configuration of cubes
322            // overlapping each other, all within the camera frustum.
323            let count = args.instance_count;
324            let size = cbrt(count as f32).round();
325            let gap = 1.25;
326
327            for i in 0..count {
328                let x = i as f32 % size;
329                let y = (i as f32 / size) % size;
330                let z = i as f32 / (size * size);
331                let pos = Vec3::new(x * gap, y * gap, z * gap);
332                commands
333                    .spawn((
334                        Mesh3d(meshes.choose(&mut material_rng).unwrap().0.clone()),
335                        MeshMaterial3d(materials.choose(&mut material_rng).unwrap().clone()),
336                        Transform::from_translation(pos),
337                    ))
338                    .insert_if(NoCpuCulling, || args.no_cpu_culling);
339            }
340
341            // camera
342            commands.spawn((
343                Camera3d::default(),
344                Transform::from_xyz(100.0, 90.0, 100.0)
345                    .looking_at(Vec3::new(0.0, -10.0, 0.0), Vec3::Y),
346            ));
347        }
348    }
349
350    commands.spawn((
351        DirectionalLight {
352            shadow_maps_enabled: args.shadows,
353            ..default()
354        },
355        Transform::IDENTITY.looking_at(Vec3::new(0.0, -1.0, -1.0), Vec3::Y),
356    ));
357}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#567)

#### pub fn [reborrow](#method.reborrow)(&mut self) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>

Returns a `Mut<>` with a smaller lifetime. This is useful if you have `&mut ResMut <T>`, but you need a `Mut<T>`.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#567)

#### pub fn [map\_unchanged](#method.map_unchanged)<U>(self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [&mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, U>

where U: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Maps to an inner value by applying a function to the contained reference, without flagging a change.

You should never modify the argument passed to the closure – if you want to modify the data without flagging a change, consider using [`DetectChangesMut::bypass_change_detection`](../../prelude/trait.DetectChangesMut.html#tymethod.bypass_change_detection "method bevy::prelude::DetectChangesMut::bypass_change_detection") to make your intent explicit.

```rust
// When run, zeroes the translation of every entity.
fn reset_positions(mut transforms: Query<&mut Transform>) {
    for transform in &mut transforms {
        // We pinky promise not to modify `t` within the closure.
        // Breaking this promise will result in logic errors, but will never cause undefined behavior.
        let mut translation = transform.map_unchanged(|t| &mut t.translation);
        // Only reset the translation if it isn't already zero;
        translation.set_if_neq(Vec2::ZERO);
    }
}
```

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#567)

#### pub fn [filter\_map\_unchanged](#method.filter_map_unchanged)<U>( self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, U>>

where U: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Optionally maps to an inner value by applying a function to the contained reference. This is useful in a situation where you need to convert a `Mut<T>` to a `Mut<U>`, but only if `T` contains `U`.

As with `map_unchanged`, you should never modify the argument passed to the closure.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#567)

#### pub fn [try\_map\_unchanged](#method.try_map_unchanged)<U, E>( self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html), E>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, U>, E>

where U: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Optionally maps to an inner value by applying a function to the contained reference, returns an error on failure. This is useful in a situation where you need to convert a `Mut<T>` to a `Mut<U>`, but only if `T` contains `U`.

As with `map_unchanged`, you should never modify the argument passed to the closure.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#567)

#### pub fn [as\_deref\_mut](#method.as_deref_mut)(&mut self) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")\>

where T: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"),

Allows you access to the dereferenced value of this pointer without immediately triggering change detection.

## Trait Implementations

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

### impl<'w, T> [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<T> for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### fn [as\_mut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html#tymethod.as_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Converts this type into a mutable reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

### impl<'w, T> [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<T> for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [as\_ref](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html#tymethod.as_ref)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Converts this type into a shared reference of the (usually inferred) input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#568)

### impl<'w, T> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#568)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

### impl<'w, T> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = T

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

### impl<'w, T> [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

### impl<'w, T> [DetectChanges](../../prelude/trait.DetectChanges.html "trait bevy::prelude::DetectChanges") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [is\_added](../../prelude/trait.DetectChanges.html#tymethod.is_added)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added after the system last ran.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [is\_changed](../../prelude/trait.DetectChanges.html#tymethod.is_changed)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added or mutably dereferenced either since the last time the system ran or, if the system never ran, since the beginning of the program. [Read more](../../prelude/trait.DetectChanges.html#tymethod.is_changed)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [is\_added\_after](../../prelude/trait.DetectChanges.html#tymethod.is_added_after)(&self, other: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added after the `other` tick.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [is\_changed\_after](../../prelude/trait.DetectChanges.html#tymethod.is_changed_after)(&self, other: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this value was added or mutably dereferenced after the `other` tick. [Read more](../../prelude/trait.DetectChanges.html#tymethod.is_changed_after)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [last\_changed](../../prelude/trait.DetectChanges.html#tymethod.last_changed)(&self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Returns the change tick recording the time this data was most recently changed. [Read more](../../prelude/trait.DetectChanges.html#tymethod.last_changed)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [added](../../prelude/trait.DetectChanges.html#tymethod.added)(&self) -> [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

Returns the change tick recording the time this data was added.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#565)

#### fn [changed\_by](../../prelude/trait.DetectChanges.html#tymethod.changed_by)(&self) -> [MaybeLocation](../change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")

The location that last caused this to change.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

### impl<'w, T> [DetectChangesMut](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### type [Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner) = T

The type contained within this smart pointer [Read more](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### fn [set\_changed](../../prelude/trait.DetectChangesMut.html#tymethod.set_changed)(&mut self)

Flags this value as having been changed. [Read more](../../prelude/trait.DetectChangesMut.html#tymethod.set_changed)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### fn [set\_added](../../prelude/trait.DetectChangesMut.html#tymethod.set_added)(&mut self)

Flags this value as having been added. [Read more](../../prelude/trait.DetectChangesMut.html#tymethod.set_added)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### fn [set\_last\_changed](../../prelude/trait.DetectChangesMut.html#tymethod.set_last_changed)(&mut self, last\_changed: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"))

Manually sets the change tick recording the time when this data was last mutated. [Read more](../../prelude/trait.DetectChangesMut.html#tymethod.set_last_changed)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### fn [set\_last\_added](../../prelude/trait.DetectChangesMut.html#tymethod.set_last_added)(&mut self, last\_added: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"))

Manually sets the added tick recording the time when this data was last added. [Read more](../../prelude/trait.DetectChangesMut.html#tymethod.set_last_added)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#566)

#### fn [bypass\_change\_detection](../../prelude/trait.DetectChangesMut.html#tymethod.bypass_change_detection)( &mut self, ) -> &mut <[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T> as [DetectChangesMut](../../prelude/trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut")\>::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")

Manually bypasses change detection, allowing you to mutate the underlying value without updating the change tick. [Read more](../../prelude/trait.DetectChangesMut.html#tymethod.bypass_change_detection)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#215-217)

#### fn [set\_if\_neq](../../prelude/trait.DetectChangesMut.html#method.set_if_neq)(&mut self, value: Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Overwrites this smart pointer with the given value, if and only if `*self != value`. Returns `true` if the value was overwritten, and returns `false` if it was not. [Read more](../../prelude/trait.DetectChangesMut.html#method.set_if_neq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#297-299)

#### fn [replace\_if\_neq](../../prelude/trait.DetectChangesMut.html#method.replace_if_neq)(&mut self, value: Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")\>

where Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner"): [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"),

Overwrites this smart pointer with the given value, if and only if `*self != value`, returning the previous value if this occurs. [Read more](../../prelude/trait.DetectChangesMut.html#method.replace_if_neq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/traits.rs.html#345-348)

#### fn [clone\_from\_if\_neq](../../prelude/trait.DetectChangesMut.html#method.clone_from_if_neq)<T>(&mut self, value: [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned")<Owned = Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), Self::[Inner](../../prelude/trait.DetectChangesMut.html#associatedtype.Inner "type bevy::prelude::DetectChangesMut::Inner"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")<T>,

Overwrites this smart pointer with a clone of the given value, if and only if `*self != value`. Returns `true` if the value was overwritten, and returns `false` if it was not. [Read more](../../prelude/trait.DetectChangesMut.html#method.clone_from_if_neq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#491)

### impl<'w, T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>> for [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#492)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(res: [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>) -> [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'w, T>

Converts to this type from the input type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#570)

### impl<'w, T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>> for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#573)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(other: [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>) -> [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'w, T>

Convert this `ResMut` into a `Mut`. This allows keeping the change-detection feature of `Mut` while losing the specificity of `ResMut` for resources.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#540-542)

### impl<'w, 'a, T> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>, [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#544)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = <[&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#545)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = <[&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#547)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#552-554)

### impl<'w, 'a, T> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a mut [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>, [&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#556)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = <[&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item "type core::iter::traits::collect::IntoIterator::Item")

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#557)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = <[&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/params.rs.html#559)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a mut [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T> as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#729)

### impl<'a, T> [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'a, T>

where T: [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#730)

#### type [State](trait.SystemParam.html#associatedtype.State) = [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

Used to store data which persists across invocations of a system.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#731)

#### type [Item](trait.SystemParam.html#associatedtype.Item)<'w, 's> = [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

The item type returned when constructing this system param. The value of this associated type should be `Self`, instantiated with new lifetimes. [Read more](trait.SystemParam.html#associatedtype.Item)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#733)

#### fn [init\_state](trait.SystemParam.html#tymethod.init_state)(world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> <[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'a, T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State")

Creates a new instance of this param’s [`State`](trait.SystemParam.html#associatedtype.State "associated type bevy::ecs::system::SystemParam::State").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#737-742)

#### fn [init\_access](trait.SystemParam.html#tymethod.init_access)( \_: &<[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'a, T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &mut [SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), component\_access\_set: &mut [FilteredAccessSet](../query/struct.FilteredAccessSet.html "struct bevy::ecs::query::FilteredAccessSet"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), )

Registers any [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") access used by this [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam"). [Read more](trait.SystemParam.html#tymethod.init_access)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#762-767)

#### unsafe fn [get\_param](trait.SystemParam.html#tymethod.get_param)<'w, 's>( \_: &'s mut <[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'a, T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [UnsafeWorldCell](../world/unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'w>, change\_tick: [Tick](../change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'a, T> as [SystemParam](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, 's>, [SystemParamValidationError](struct.SystemParamValidationError.html "struct bevy::ecs::system::SystemParamValidationError")\>

Creates a parameter to be passed into a [`SystemParamFunction`](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction"). [Read more](trait.SystemParam.html#tymethod.get_param)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#250)

#### fn [apply](trait.SystemParam.html#method.apply)(state: &mut Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Applies any deferred mutations stored in this [`SystemParam`](trait.SystemParam.html "trait bevy::ecs::system::SystemParam")’s state. This is used to apply [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") during [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#258)

#### fn [queue](trait.SystemParam.html#method.queue)( state: &mut Self::[State](trait.SystemParam.html#associatedtype.State "type bevy::ecs::system::SystemParam::State"), system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>, )

Queues any deferred mutations to be applied at the next [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

## Auto Trait Implementations

### impl<'w, T> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

### impl<'w, T> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'w, T> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'w, T> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'w, T> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'w, T> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

### impl<'w, T> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'w, T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

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

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#97)

### impl<R> [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng") for R

where R: [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng")<Error = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#206)

### impl<T> [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng") for T

where T: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#355-358)

### impl<T, C, D> [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> for D

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = C>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#360)

#### fn [domain](../../prelude/trait.Curve.html#tymethod.domain)(&self) -> [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval")

The interval over which this curve is parametrized. [Read more](../../prelude/trait.Curve.html#tymethod.domain)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#364)

#### fn [sample\_unchecked](../../prelude/trait.Curve.html#tymethod.sample_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

Sample a point on this curve at the parameter value `t`, extracting the associated value. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain. [Read more](../../prelude/trait.Curve.html#tymethod.sample_unchecked)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#340)

#### fn [sample](../../prelude/trait.Curve.html#method.sample)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Sample a point on this curve at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#349)

#### fn [sample\_clamped](../../prelude/trait.Curve.html#method.sample_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

Sample a point on this curve at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#764)

### impl<C, T> [CurveExt](../../prelude/trait.CurveExt.html "trait bevy::prelude::CurveExt")<T> for C

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#387)

#### fn [sample\_iter](../../prelude/trait.CurveExt.html#method.sample_iter)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, returning `None` if the point is outside of the curve’s domain. [Read more](../../prelude/trait.CurveExt.html#method.sample_iter)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#402-405)

#### fn [sample\_iter\_unchecked](../../prelude/trait.CurveExt.html#method.sample_iter_unchecked)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, extracting the associated values. This is the unchecked version of sampling, which should only be used if the sample times `t_n` are already known to lie within the curve’s domain. [Read more](../../prelude/trait.CurveExt.html#method.sample_iter_unchecked)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#415)

#### fn [sample\_iter\_clamped](../../prelude/trait.CurveExt.html#method.sample_iter_clamped)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, clamping `t_n` to lie inside the domain of the curve. [Read more](../../prelude/trait.CurveExt.html#method.sample_iter_clamped)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#423-425)

#### fn [map](../../prelude/trait.CurveExt.html#method.map)<S, F>(self, f: F) -> [MapCurve](../../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<T, S, Self, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(T) -> S,

Create a new curve by mapping the values of this curve via a function `f`; i.e., if the sample at time `t` for this curve is `x`, the value at time `t` on the new curve will be `f(x)`.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#465-467)

#### fn [reparametrize](../../prelude/trait.CurveExt.html#method.reparametrize)<F>(self, domain: [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval"), f: F) -> [ReparamCurve](../../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, Self, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html),

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") whose parameter space is related to the parameter space of this curve by `f`. For each time `t`, the sample from the new curve at time `t` is the sample from this curve at time `f(t)`. The given `domain` will be the domain of the new curve. The function `f` is expected to take `domain` into `self.domain()`. [Read more](../../prelude/trait.CurveExt.html#method.reparametrize)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#484-487)

#### fn [reparametrize\_linear](../../prelude/trait.CurveExt.html#method.reparametrize_linear)( self, domain: [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[LinearReparamCurve](../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, Self>, [LinearReparamError](../../prelude/enum.LinearReparamError.html "enum bevy::prelude::LinearReparamError")\>

Linearly reparametrize this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), producing a new curve whose domain is the given `domain` instead of the current one. This operation is only valid for curves with bounded domains. [Read more](../../prelude/trait.CurveExt.html#method.reparametrize_linear)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#509-511)

#### fn [reparametrize\_by\_curve](../../prelude/trait.CurveExt.html#method.reparametrize_by_curve)<C>(self, other: C) -> [CurveReparamCurve](../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, Self, C>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Reparametrize this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by sampling from another curve. [Read more](../../prelude/trait.CurveExt.html#method.reparametrize_by_curve)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#527)

#### fn [graph](../../prelude/trait.CurveExt.html#method.graph)(self) -> [GraphCurve](../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, Self>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") which is the graph of this one; that is, its output echoes the sample time as part of a tuple. [Read more](../../prelude/trait.CurveExt.html#method.graph)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#543-545)

#### fn [zip](../../prelude/trait.CurveExt.html#method.zip)<S, C>( self, other: C, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ZipCurve](../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<T, S, Self, C>, [InvalidIntervalError](../../prelude/interval/struct.InvalidIntervalError.html "struct bevy::prelude::interval::InvalidIntervalError")\>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<S>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by zipping this curve together with another. [Read more](../../prelude/trait.CurveExt.html#method.zip)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#564-566)

#### fn [chain](../../prelude/trait.CurveExt.html#method.chain)<C>(self, other: C) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ChainCurve](../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, Self, C>, [ChainError](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError")\>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by composing this curve end-to-start with another, producing another curve with outputs of the same type. The domain of the other curve is translated so that its start coincides with where this curve ends. [Read more](../../prelude/trait.CurveExt.html#method.chain)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#589)

#### fn [reverse](../../prelude/trait.CurveExt.html#method.reverse)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ReverseCurve](../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, Self>, [ReverseError](../../prelude/enum.ReverseError.html "enum bevy::prelude::ReverseError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") inverting this curve on the x-axis, producing another curve with outputs of the same type, effectively playing backwards starting at `self.domain().end()` and transitioning over to `self.domain().start()`. The domain of the new curve is still the same. [Read more](../../prelude/trait.CurveExt.html#method.reverse)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#613)

#### fn [repeat](../../prelude/trait.CurveExt.html#method.repeat)(self, count: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RepeatCurve](../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, Self>, [RepeatError](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") repeating this curve `N` times, producing another curve with outputs of the same type. The domain of the new curve will be bigger by a factor of `n + 1`. [Read more](../../prelude/trait.CurveExt.html#method.repeat)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#646)

#### fn [forever](../../prelude/trait.CurveExt.html#method.forever)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ForeverCurve](../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, Self>, [RepeatError](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") repeating this curve forever, producing another curve with outputs of the same type. The domain of the new curve will be unbounded. [Read more](../../prelude/trait.CurveExt.html#method.forever)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#663)

#### fn [ping\_pong](../../prelude/trait.CurveExt.html#method.ping_pong)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[PingPongCurve](../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, Self>, [PingPongError](../../prelude/enum.PingPongError.html "enum bevy::prelude::PingPongError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") chaining the original curve with its inverse, producing another curve with outputs of the same type. The domain of the new curve will be twice as long. The transition point is guaranteed to not make any jumps. [Read more](../../prelude/trait.CurveExt.html#method.ping_pong)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#688-691)

#### fn [chain\_continue](../../prelude/trait.CurveExt.html#method.chain_continue)<C>( self, other: C, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ContinuationCurve](../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, Self, C>, [ChainError](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError")\>

where T: [VectorSpace](../../math/trait.VectorSpace.html "trait bevy::math::VectorSpace"), C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by composing this curve end-to-start with another, producing another curve with outputs of the same type. The domain of the other curve is translated so that its start coincides with where this curve ends. [Read more](../../prelude/trait.CurveExt.html#method.chain_continue)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#717)

#### fn [samples](../../prelude/trait.CurveExt.html#method.samples)( &self, samples: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

Extract an iterator over evenly-spaced samples from this curve. [Read more](../../prelude/trait.CurveExt.html#method.samples)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#750)

#### fn [by\_ref](../../prelude/trait.CurveExt.html#method.by_ref)(&self) -> &Self

Borrow this curve rather than taking ownership of it. This is essentially an alias for a prefix `&`; the point is that intermediate operations can be performed while retaining access to the original curve. [Read more](../../prelude/trait.CurveExt.html#method.by_ref)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#756-758)

#### fn [flip](../../prelude/trait.CurveExt.html#method.flip)<U, V>(self) -> impl [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[(V, U)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Self: [CurveExt](../../prelude/trait.CurveExt.html "trait bevy::prelude::CurveExt")<[(U, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Flip this curve so that its tuple output is arranged the other way.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#930)

### impl<C, T> [CurveResampleExt](../../prelude/trait.CurveResampleExt.html "trait bevy::prelude::CurveResampleExt")<T> for C

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#801-807)

#### fn [resample](../../prelude/trait.CurveResampleExt.html#method.resample)<I>( &self, segments: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), interpolation: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[SampleCurve](../../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over equally spaced sample values, using the provided `interpolation` to interpolate between adjacent samples. The curve is interpolated on `segments` segments between samples. For example, if `segments` is 1, only the start and end points of the curve are used as samples; if `segments` is 2, a sample at the midpoint is taken as well, and so on. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#830-832)

#### fn [resample\_auto](../../prelude/trait.CurveResampleExt.html#method.resample_auto)( &self, segments: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[SampleAutoCurve](../../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where T: [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over equally spaced sample values, using [automatic interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") to interpolate between adjacent samples. The curve is interpolated on `segments` segments between samples. For example, if `segments` is 1, only the start and end points of the curve are used as samples; if `segments` is 2, a sample at the midpoint is taken as well, and so on. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample_auto)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#863-869)

#### fn [resample\_uneven](../../prelude/trait.CurveResampleExt.html#method.resample_uneven)<I>( &self, sample\_times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, interpolation: I, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UnevenSampleCurve](../../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by interpolation over samples taken at a given set of times. The given `interpolation` is used to interpolate adjacent samples, and the `sample_times` are expected to contain at least two valid times within the curve’s domain interval. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample_uneven)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#905-910)

#### fn [resample\_uneven\_auto](../../prelude/trait.CurveResampleExt.html#method.resample_uneven_auto)( &self, sample\_times: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UnevenSampleAutoCurve](../../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

where T: [StableInterpolate](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

Resample this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") to produce a new one that is defined by [automatic interpolation](../../prelude/trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") over samples taken at the given set of times. The given `sample_times` are expected to contain at least two valid times within the curve’s domain interval. [Read more](../../prelude/trait.CurveResampleExt.html#method.resample_uneven_auto)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#212-215)

### impl<T, C> [CurveWithDerivative](../../prelude/derivatives/trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative")<T> for C

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](../../prelude/derivatives/trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#217)

#### fn [with\_derivative](../../prelude/derivatives/trait.CurveWithDerivative.html#tymethod.with_derivative)(self) -> [SampleDerivativeWrapper](../../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

This curve, but with its first derivative included in sampling. [Read more](../../prelude/derivatives/trait.CurveWithDerivative.html#tymethod.with_derivative)

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

### impl<T> [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

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

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#65-67)

### impl<R> [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") for R

where R: [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")<Error = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")\> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#70)

#### fn [next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.next_u32)(&mut self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Return the next random `u32`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#77)

#### fn [next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.next_u64)(&mut self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Return the next random `u64`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#84)

#### fn [fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.fill_bytes)(&mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Fill `dest` with random data. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html#tymethod.fill_bytes)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#357)

### impl<R> [Rng](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html "trait rand::rng::Rng") for R

where R: [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#95-97)

#### fn [random](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random)<T>(&mut self) -> T

where [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return a random value via the [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#120-123)

#### fn [random\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_iter)<T>(self) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<[StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"), Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return an iterator over [`random`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random "method rand::rng::Rng::random") variates [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#161-164)

#### fn [random\_range](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_range)<T, R>(&mut self, range: R) -> T

where T: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"), R: [SampleRange](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html "trait rand::distr::uniform::SampleRange")<T>,

Generate a random value in the given range. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_range)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#191)

#### fn [random\_bool](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_bool)(&mut self, p: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability `p` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_bool)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#225)

#### fn [random\_ratio](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_ratio)(&mut self, numerator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), denominator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability of `numerator/denominator` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_ratio)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#249)

#### fn [sample](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample)<T, D>(&mut self, distr: D) -> T

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Sample a new value, using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#286-289)

#### fn [sample\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample_iter)<T, D>(self, distr: D) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<D, Self, T> [ⓘ](#)

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator that generates values using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.sample_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#314)

#### fn [fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.fill)<T>(&mut self, dest: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where T: [Fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Fill any type implementing [`Fill`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill") with random data [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.fill)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#324-326)

#### fn [gen](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen)<T>(&mut self) -> T

where [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

👎Deprecated since 0.9.0:

Renamed to `random` to avoid conflict with the new `gen` keyword in Rust 2024.

Alias for [`Rng::random`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random "method rand::rng::Rng::random").

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#334-337)

#### fn [gen\_range](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen_range)<T, R>(&mut self, range: R) -> T

where T: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"), R: [SampleRange](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html "trait rand::distr::uniform::SampleRange")<T>,

👎Deprecated since 0.9.0:

Renamed to `random_range`

Alias for [`Rng::random_range`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_range "method rand::rng::Rng::random_range").

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#345)

#### fn [gen\_bool](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen_bool)(&mut self, p: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

👎Deprecated since 0.9.0:

Renamed to `random_bool`

Alias for [`Rng::random_bool`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_bool "method rand::rng::Rng::random_bool").

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#352)

#### fn [gen\_ratio](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.gen_ratio)(&mut self, numerator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), denominator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

👎Deprecated since 0.9.0:

Renamed to `random_ratio`

Alias for [`Rng::random_ratio`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Rng.html#method.random_ratio "method rand::rng::Rng::random_ratio").

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#259)

### impl<R> [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") for R

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#158-160)

### impl<T> [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") for T

where T: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#163)

#### fn [next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u32)(&mut self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Return the next random `u32`. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u32)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#168)

#### fn [next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u64)(&mut self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Return the next random `u64`. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.next_u64)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#173)

#### fn [fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.fill_bytes)(&mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Fill `dest` with random data. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html#tymethod.fill_bytes)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#317)

### impl<R> [RngExt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") for R

where R: [Rng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.Rng.html "trait rand_core::Rng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#93-95)

#### fn [random](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random)<T>(&mut self) -> T

where [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return a random value via the [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#118-121)

#### fn [random\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_iter)<T>(self) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<[StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"), Self, T> [ⓘ](#)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [StandardUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform"): [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Return an iterator over [`random`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random "method rand::rng::RngExt::random") variates [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#159-162)

#### fn [random\_range](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_range)<T, R>(&mut self, range: R) -> T

where T: [SampleUniform](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleUniform.html "trait rand::distr::uniform::SampleUniform"), R: [SampleRange](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/uniform/trait.SampleRange.html "trait rand::distr::uniform::SampleRange")<T>,

Generate a random value in the given range. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_range)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#189)

#### fn [random\_bool](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_bool)(&mut self, p: [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability `p` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_bool)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#223)

#### fn [random\_ratio](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_ratio)(&mut self, numerator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), denominator: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Return a bool with a probability of `numerator/denominator` of being true. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.random_ratio)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#247)

#### fn [sample](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample)<T, D>(&mut self, distr: D) -> T

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>,

Sample a new value, using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#284-287)

#### fn [sample\_iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample_iter)<T, D>(self, distr: D) -> [Iter](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html "struct rand::distr::distribution::Iter")<D, Self, T> [ⓘ](#)

where D: [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<T>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Create an iterator that generates values using the given distribution. [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.sample_iter)

[Source](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/src/rand/rng.rs.html#312)

#### fn [fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.fill)<T>(&mut self, dest: &mut [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html))

where T: [Fill](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill"),

Fill any type implementing [`Fill`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.Fill.html "trait rand::rng::Fill") with random data [Read more](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html#method.fill)

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#100-104)

### impl<T, C, D> [SampleDerivative](../../prelude/derivatives/trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> for D

where T: [HasTangent](../../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](../../prelude/derivatives/trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = C>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#106)

#### fn [sample\_with\_derivative\_unchecked](../../prelude/derivatives/trait.SampleDerivative.html#tymethod.sample_with_derivative_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

Sample this curve at the parameter value `t`, extracting the associated value in addition to its derivative. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain. [Read more](../../prelude/derivatives/trait.SampleDerivative.html#tymethod.sample_with_derivative_unchecked)

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#85)

#### fn [sample\_with\_derivative](../../prelude/derivatives/trait.SampleDerivative.html#method.sample_with_derivative)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>>

Sample this curve’s value and derivative at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#94)

#### fn [sample\_with\_derivative\_clamped](../../prelude/derivatives/trait.SampleDerivative.html#method.sample_with_derivative_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [WithDerivative](../../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

Sample this curve’s value and derivative at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#203-206)

### impl<T> [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source") for T

where T: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#208)

#### type [Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice)<'a> = <<T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'a> where T: 'a

A type this `Source` can be sliced into.

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#213)

#### fn [len](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Length of the source

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#217-219)

#### fn [read](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)<'a, Chunk>(&'a self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Chunk>

where Chunk: [Chunk](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Chunk.html "trait logos::source::Chunk")<'a>,

Read a chunk of bytes into an array. Returns `None` when reading out of bounds would occur. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#224)

#### fn [slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)(&self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>>

Get a slice of the source at given range. This is analogous to `slice::get(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#229)

#### unsafe fn [slice\_unchecked](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)( &self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> <T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>

Available on **non-crate feature `forbid_unsafe`** only.

Get a slice of the source at given range. This is analogous to `slice::get_unchecked(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#233)

#### fn [is\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Check if `index` is valid for this `Source`, that is: [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#237)

#### fn [find\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

For `&str` sources attempts to find the closest `char` boundary at which source can be sliced, starting from `index`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)

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

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#251)

### impl<R> [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng") for R

where R: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <R as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#293)

### impl<R> [TryCryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryCryptoRng.html "trait rand_core::TryCryptoRng") for R

where R: [CryptoRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.CryptoRng.html "trait rand_core::CryptoRng") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

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

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#203-205)

### impl<R> [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng") for R

where R: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut"), <R as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#207)

#### type [Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error) = <<R as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")

The type returned in the event of a RNG error. [Read more](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error)

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#210)

#### fn [try\_next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#tymethod.try_next_u32)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")\>

Return the next random `u32`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#215)

#### fn [try\_next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#tymethod.try_next_u64)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")\>

Return the next random `u64`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#220)

#### fn [try\_fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#tymethod.try_fill_bytes)(&mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")\>

Fill `dst` entirely with random data.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#270)

### impl<R> [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore") for R

where R: [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#271)

#### type [Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error) = <R as [TryRng](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html "trait rand_core::TryRng")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRng.html#associatedtype.Error "type rand_core::TryRng::Error")

👎Deprecated since 0.10.0:

use `TryRng` instead

Error type.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#257)

### impl<R> [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore") for R

where R: [RngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#258)

#### type [Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a RNG error.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#261)

#### fn [try\_next\_u32](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#tymethod.try_next_u32)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), <R as [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error "type rand_core::TryRngCore::Error")\>

Return the next random `u32`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#266)

#### fn [try\_next\_u64](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#tymethod.try_next_u64)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html), <R as [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error "type rand_core::TryRngCore::Error")\>

Return the next random `u64`.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#271)

#### fn [try\_fill\_bytes](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#tymethod.try_fill_bytes)( &mut self, dst: &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), <R as [TryRngCore](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html "trait rand_core::TryRngCore")\>::[Error](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#associatedtype.Error "type rand_core::TryRngCore::Error")\>

Fill `dest` entirely with random data.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#232-234)

#### fn [unwrap\_err](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#method.unwrap_err)(self) -> [UnwrapErr](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapErr.html "struct rand_core::UnwrapErr")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wrap RNG with the [`UnwrapErr`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapErr.html "struct rand_core::UnwrapErr") wrapper.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#240)

#### fn [unwrap\_mut](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#method.unwrap_mut)(&mut self) -> [UnwrapMut](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapMut.html "struct rand_core::UnwrapMut")<'\_, Self>

Wrap RNG with the [`UnwrapMut`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.UnwrapMut.html "struct rand_core::UnwrapMut") wrapper.

[Source](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/src/rand_core/lib.rs.html#246-248)

#### fn [read\_adapter](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.TryRngCore.html#method.read_adapter)(&mut self) -> [RngReadAdapter](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.RngReadAdapter.html "struct rand_core::RngReadAdapter")<'\_, Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **crate feature `std`** only.

Convert an [`RngCore`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/trait.RngCore.html "trait rand_core::RngCore") to a [`RngReadAdapter`](https://docs.rs/rand_core/0.9.5/x86_64-unknown-linux-gnu/rand_core/struct.RngReadAdapter.html "struct rand_core::RngReadAdapter").

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<D, Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;D, R, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html\\" title=\\"trait rand::distr::distribution::Distribution\\">Distribution</a>&lt;T&gt;,\\n R: <a class=\\"trait\\" href=\\"https://docs.rs/rand\_core/0.9.5/x86\_64-unknown-linux-gnu/rand\_core/trait.Rng.html\\" title=\\"trait rand\_core::Rng\\">Rng</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","Iter<StandardUniform, Self, T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;D, R, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/struct.Iter.html\\" title=\\"struct rand::distr::distribution::Iter\\">Iter</a>&lt;D, R, T&gt;<div class=\\"where\\">where\\n D: <a class=\\"trait\\" href=\\"https://docs.rs/rand/0.9.4/x86\_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html\\" title=\\"trait rand::distr::distribution::Distribution\\">Distribution</a>&lt;T&gt;,\\n R: <a class=\\"trait\\" href=\\"https://docs.rs/rand\_core/0.9.5/x86\_64-unknown-linux-gnu/rand\_core/trait.Rng.html\\" title=\\"trait rand\_core::Rng\\">Rng</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = T;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}