[bevy](../../index.html)::[math](../index.html)::[prelude](index.html)

# Function vec4 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/sse2/vec4.rs.html#26)

```rust
pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4
```

Creates a 4-dimensional vector.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 73](../../../src/custom_primitives/custom_primitives.rs.html#73))

```rust
68const PROJECTION_3D: Projection = Projection::Perspective(PerspectiveProjection {
69    fov: PI / 4.0,
70    near: 0.1,
71    far: 1000.0,
72    aspect_ratio: 1.0,
73    near_clip_plane: vec4(0.0, 0.0, -1.0, -0.1),
74});
```

Hide additional examples

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([line 74](../../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#74))

```rust
54fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
55    // Build a custom triangle mesh with colors
56    // We define a custom mesh because the examples only uses a limited
57    // set of vertex attributes for simplicity
58    let mesh = Mesh::new(
59        PrimitiveTopology::TriangleList,
60        RenderAssetUsages::default(),
61    )
62    .with_inserted_indices(Indices::U32(vec![0, 1, 2]))
63    .with_inserted_attribute(
64        Mesh::ATTRIBUTE_POSITION,
65        vec![
66            vec3(-0.5, -0.5, 0.0),
67            vec3(0.5, -0.5, 0.0),
68            vec3(0.0, 0.25, 0.0),
69        ],
70    )
71    .with_inserted_attribute(
72        Mesh::ATTRIBUTE_COLOR,
73        vec![
74            vec4(1.0, 0.0, 0.0, 1.0),
75            vec4(0.0, 1.0, 0.0, 1.0),
76            vec4(0.0, 0.0, 1.0, 1.0),
77        ],
78    );
79
80    // spawn 3 triangles to show that batching works
81    for (x, y) in [-0.5, 0.0, 0.5].into_iter().zip([-0.25, 0.5, -0.25]) {
82        // Spawn an entity with all the required components for it to be rendered with our custom pipeline
83        commands.spawn((
84            // We use a marker component to identify the mesh that will be rendered
85            // with our specialized pipeline
86            CustomRenderedEntity,
87            // We need to add the mesh handle to the entity
88            Mesh3d(meshes.add(mesh.clone())),
89            Transform::from_xyz(x, y, 0.0),
90        ));
91    }
92
93    // Spawn the camera.
94    commands.spawn((
95        Camera3d::default(),
96        // Move the camera back a bit to see all the triangles
97        Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
98    ));
99}
```

examples/3d/atmosphere.rs ([line 293](../../../src/atmosphere/atmosphere.rs.html#293))

```rust
260fn spawn_water(
261    commands: &mut Commands,
262    asset_server: &AssetServer,
263    meshes: &mut Assets<Mesh>,
264    water_materials: &mut Assets<ExtendedMaterial<StandardMaterial, Water>>,
265) {
266    commands.spawn((
267        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
268        MeshMaterial3d(
269            water_materials.add(ExtendedMaterial {
270                base: StandardMaterial {
271                    base_color: BLACK.into(),
272                    perceptual_roughness: 0.0,
273                    ..default()
274                },
275                extension: Water {
276                    normals: asset_server
277                        .load_builder()
278                        .with_settings(|settings: &mut ImageLoaderSettings| {
279                            settings.is_srgb = false;
280                            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
281                                address_mode_u: ImageAddressMode::Repeat,
282                                address_mode_v: ImageAddressMode::Repeat,
283                                mag_filter: ImageFilterMode::Linear,
284                                min_filter: ImageFilterMode::Linear,
285                                ..default()
286                            });
287                        })
288                        .load("textures/water_normals.png"),
289                    // These water settings are just random values to create some
290                    // variety.
291                    settings: WaterSettings {
292                        octave_vectors: [
293                            vec4(0.080, 0.059, 0.073, -0.062),
294                            vec4(0.153, 0.138, -0.149, -0.195),
295                        ],
296                        octave_scales: vec4(1.0, 2.1, 7.9, 14.9) * 500.0,
297                        octave_strengths: vec4(0.16, 0.18, 0.093, 0.044) * 0.2,
298                    },
299                },
300            }),
301        ),
302        Transform::from_scale(Vec3::splat(100.0)),
303    ));
304}
```

examples/3d/ssr.rs ([line 397](../../../src/ssr/ssr.rs.html#397))

```rust
364fn spawn_water(
365    commands: &mut Commands,
366    asset_server: &AssetServer,
367    meshes: &mut Assets<Mesh>,
368    water_materials: &mut Assets<ExtendedMaterial<StandardMaterial, Water>>,
369) {
370    commands.spawn((
371        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1.0)))),
372        MeshMaterial3d(
373            water_materials.add(ExtendedMaterial {
374                base: StandardMaterial {
375                    base_color: BLACK.into(),
376                    perceptual_roughness: 0.09,
377                    ..default()
378                },
379                extension: Water {
380                    normals: asset_server
381                        .load_builder()
382                        .with_settings::<ImageLoaderSettings>(|settings| {
383                            settings.is_srgb = false;
384                            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
385                                address_mode_u: ImageAddressMode::Repeat,
386                                address_mode_v: ImageAddressMode::Repeat,
387                                mag_filter: ImageFilterMode::Linear,
388                                min_filter: ImageFilterMode::Linear,
389                                ..default()
390                            });
391                        })
392                        .load("textures/water_normals.png"),
393                    // These water settings are just random values to create some
394                    // variety.
395                    settings: WaterSettings {
396                        octave_vectors: [
397                            vec4(0.080, 0.059, 0.073, -0.062),
398                            vec4(0.153, 0.138, -0.149, -0.195),
399                        ],
400                        octave_scales: vec4(1.0, 2.1, 7.9, 14.9) * 5.0,
401                        octave_strengths: vec4(0.16, 0.18, 0.093, 0.044),
402                    },
403                },
404            }),
405        ),
406        Transform::from_scale(Vec3::splat(100.0)),
407        WaterModel,
408    ));
409}
```