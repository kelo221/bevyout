[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function sqrt 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#562)

```rust
pub fn sqrt(x: f32) -> f32
```

Returns the square root of a number.

The result of this operation is guaranteed to be the rounded infinite-precision result. It is specified by IEEE 754 as `squareRoot` and guaranteed not to change.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_cubes.rs ([line 256](../../../src/many_cubes/many_cubes.rs.html#256))

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