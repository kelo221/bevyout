[bevy](../index.html)::[math](index.html)

# Function vec2 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec2.rs.html#15)

```rust
pub const fn vec2(x: f32, y: f32) -> Vec2
```

Creates a 2-dimensional vector.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/2d/dynamic\_mip\_generation.rs ([line 716](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#716))

```rust
714    fn max_mip_slice_size(&self, window_size: Vec2) -> Vec2 {
715        let spacing = self.vertical_mip_slice_spacing(window_size);
716        vec2(window_size.x * MIP_SLICES_WIDTH, spacing)
717    }
718
719    /// Returns the horizontal center point of each mip slice image in the
720    /// column at the right side of the window.
721    fn horizontal_mip_slice_origin(&self, window_size: Vec2) -> f32 {
722        let max_slice_size = self.max_mip_slice_size(window_size);
723        window_size.x * 0.5 - max_slice_size.x * 0.5 - MIP_SLICES_MARGIN_RIGHT
724    }
725
726    /// Calculates and returns the area reserved for the animated image on the
727    /// left side of the window.
728    ///
729    /// Note that this isn't necessarily equal to the final size of the animated
730    /// image, because that size preserves the image's aspect ratio.
731    fn animated_mesh_area_size(&self, window_size: Vec2) -> Vec2 {
732        vec2(
733            self.horizontal_mip_slice_origin(window_size) * 2.0 - MIP_SLICES_MARGIN_LEFT * 2.0,
734            window_size.y,
735        )
736    }
737
738    /// Calculates and returns the actual maximum size of the animated image on
739    /// the left side of the window.
740    ///
741    /// This is equal to the maximum portion of the
742    /// [`Self::animated_mesh_area_size`] that the image can occupy while
743    /// preserving its aspect ratio.
744    fn animated_mesh_size(&self, window_size: Vec2) -> Vec2 {
745        let max_image_size = self.animated_mesh_area_size(window_size);
746        let image_size = self.image_size_f32();
747        let ratios = max_image_size / image_size;
748        let image_scale = ratios.x.min(ratios.y);
749        image_size * image_scale
750    }
751
752    /// Returns the size of the image as a [`UVec2`].
753    fn image_size_u32(&self) -> UVec2 {
754        uvec2(self.image_width as u32, self.image_height as u32)
755    }
756
757    /// Returns the size of the image as a [`Vec2`].
758    fn image_size_f32(&self) -> Vec2 {
759        vec2(
760            self.image_width as u32 as f32,
761            self.image_height as u32 as f32,
762        )
763    }
764
765    /// Regenerates the main image based on the image size selected by the user.
766    fn regenerate_mipmap_source_image(
767        &mut self,
768        commands: &mut Commands,
769        images: &mut Assets<Image>,
770    ) -> Handle<Image> {
771        let image_data = self.generate_image_data();
772
773        let mut image = Image::new_uninit(
774            Extent3d {
775                width: self.image_width as u32,
776                height: self.image_height as u32,
777                depth_or_array_layers: 1,
778            },
779            TextureDimension::D2,
780            TextureFormat::Rgba8Unorm,
781            RenderAssetUsages::all(),
782        );
783        image.texture_descriptor.mip_level_count = self.image_mip_level_count();
784        image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
785        image.data = Some(image_data);
786
787        let image_handle = images.add(image);
788        commands.insert_resource(MipmapSourceImage(image_handle.clone()));
789
790        image_handle
791    }
792
793    /// Draws the concentric ellipses that make up the image.
794    ///
795    /// Returns the RGBA8 image data.
796    fn generate_image_data(&mut self) -> Vec<u8> {
797        // Select random colors for the inner and outer ellipses.
798        let outer_color: [u8; 3] = array::from_fn(|_| self.rng.random());
799        let inner_color: [u8; 3] = array::from_fn(|_| self.rng.random());
800
801        let image_byte_size = 4usize
802            * MipmapSizeIterator::new(self)
803                .map(|size| size.x as usize * size.y as usize)
804                .sum::<usize>();
805        let mut image_data = vec![0u8; image_byte_size];
806
807        let center = self.image_size_f32() * 0.5;
808
809        let inner_ellipse_radii = self.inner_ellipse_radii();
810        let outer_ellipse_radii = self.outer_ellipse_radii();
811
812        for y in 0..(self.image_height as u32) {
813            for x in 0..(self.image_width as u32) {
814                let p = vec2(x as f32, y as f32);
815                let (color, alpha) = if point_in_ellipse(p, center, inner_ellipse_radii) {
816                    (inner_color, 255)
817                } else if point_in_ellipse(p, center, outer_ellipse_radii) {
818                    (outer_color, 255)
819                } else {
820                    ([0; 3], 0)
821                };
822                let start = (4 * (x + y * (self.image_width as u32))) as usize;
823                image_data[start..(start + 3)].copy_from_slice(&color);
824                image_data[start + 3] = alpha;
825            }
826        }
827
828        image_data
829    }
```

Hide additional examples

examples/3d/mixed\_lighting.rs ([line 93](../../src/mixed_lighting/mixed_lighting.rs.html#93))

```rust
86static LIGHTMAPS: [(&str, Rect); 5] = [
87    (
88        "Plane",
89        uv_rect_opengl(Vec2::splat(0.026), Vec2::splat(0.710)),
90    ),
91    (
92        "SheenChair_fabric",
93        uv_rect_opengl(vec2(0.7864, 0.02377), vec2(0.1910, 0.1912)),
94    ),
95    (
96        "SheenChair_label",
97        uv_rect_opengl(vec2(0.275, -0.016), vec2(0.858, 0.486)),
98    ),
99    (
100        "SheenChair_metal",
101        uv_rect_opengl(vec2(0.998, 0.506), vec2(-0.029, -0.067)),
102    ),
103    (
104        "SheenChair_wood",
105        uv_rect_opengl(vec2(0.787, 0.257), vec2(0.179, 0.177)),
106    ),
107];
108
109static SPHERE_UV_RECT: Rect = uv_rect_opengl(vec2(0.788, 0.484), Vec2::splat(0.062));
110
111/// The initial position of the sphere.
112///
113/// When the user sets the light mode to [`LightingMode::Baked`], we reset the
114/// position to this point.
115const INITIAL_SPHERE_POSITION: Vec3 = vec3(0.0, 0.5233223, 0.0);
116
117fn main() {
118    App::new()
119        .add_plugins(DefaultPlugins.set(WindowPlugin {
120            primary_window: Some(Window {
121                title: "Bevy Mixed Lighting Example".into(),
122                ..default()
123            }),
124            ..default()
125        }))
126        .add_plugins(MeshPickingPlugin)
127        .insert_resource(GlobalAmbientLight {
128            color: ClearColor::default().0,
129            brightness: 10000.0,
130            affects_lightmapped_meshes: true,
131        })
132        .init_resource::<AppStatus>()
133        .add_message::<WidgetClickEvent<LightingMode>>()
134        .add_message::<LightingModeChanged>()
135        .add_systems(Startup, setup)
136        .add_systems(Update, update_lightmaps)
137        .add_systems(Update, update_directional_light)
138        .add_systems(Update, make_sphere_nonpickable)
139        .add_systems(Update, update_radio_buttons)
140        .add_systems(Update, handle_lighting_mode_change)
141        .add_systems(Update, widgets::handle_ui_interactions::<LightingMode>)
142        .add_systems(Update, reset_sphere_position)
143        .add_systems(Update, move_sphere)
144        .add_systems(Update, adjust_help_text)
145        .run();
146}
147
148/// Creates the scene.
149fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_status: Res<AppStatus>) {
150    spawn_camera(&mut commands);
151    spawn_scene(&mut commands, &asset_server);
152    spawn_buttons(&mut commands);
153    spawn_help_text(&mut commands, &app_status);
154}
155
156/// Spawns the 3D camera.
157fn spawn_camera(commands: &mut Commands) {
158    commands
159        .spawn(Camera3d::default())
160        .insert(Transform::from_xyz(-0.7, 0.7, 1.0).looking_at(vec3(0.0, 0.3, 0.0), Vec3::Y));
161}
162
163/// Spawns the scene.
164///
165/// The scene is loaded from a glTF file.
166fn spawn_scene(commands: &mut Commands, asset_server: &AssetServer) {
167    commands
168        .spawn(WorldAssetRoot(
169            asset_server.load(
170                GltfAssetLabel::Scene(0)
171                    .from_asset("models/MixedLightingExample/MixedLightingExample.gltf"),
172            ),
173        ))
174        .observe(
175            |_: On<WorldInstanceReady>,
176             mut lighting_mode_changed_writer: MessageWriter<LightingModeChanged>| {
177                // When the scene loads, send a `LightingModeChanged` event so
178                // that we set up the lightmaps.
179                lighting_mode_changed_writer.write(LightingModeChanged);
180            },
181        );
182}
183
184/// Spawns the buttons that allow the user to change the lighting mode.
185fn spawn_buttons(commands: &mut Commands) {
186    commands.spawn((
187        widgets::main_ui_node(),
188        children![widgets::option_buttons(
189            "Lighting",
190            &[
191                (LightingMode::Baked, "Baked"),
192                (LightingMode::MixedDirect, "Mixed (Direct)"),
193                (LightingMode::MixedIndirect, "Mixed (Indirect)"),
194                (LightingMode::RealTime, "Real-Time"),
195            ],
196        )],
197    ));
198}
199
200/// Spawns the help text at the top of the window.
201fn spawn_help_text(commands: &mut Commands, app_status: &AppStatus) {
202    commands.spawn((
203        create_help_text(app_status),
204        Node {
205            position_type: PositionType::Absolute,
206            top: px(12),
207            left: px(12),
208            ..default()
209        },
210        HelpText,
211    ));
212}
213
214/// Adds lightmaps to and/or removes lightmaps from objects in the scene when
215/// the lighting mode changes.
216///
217/// This is also called right after the scene loads in order to set up the
218/// lightmaps.
219fn update_lightmaps(
220    mut commands: Commands,
221    asset_server: Res<AssetServer>,
222    mut materials: ResMut<Assets<StandardMaterial>>,
223    meshes: Query<(Entity, &GltfMeshName, &MeshMaterial3d<StandardMaterial>), With<Mesh3d>>,
224    mut lighting_mode_changed_reader: MessageReader<LightingModeChanged>,
225    app_status: Res<AppStatus>,
226) {
227    // Only run if the lighting mode changed. (Note that a change event is fired
228    // when the scene first loads.)
229    if lighting_mode_changed_reader.read().next().is_none() {
230        return;
231    }
232
233    // Select the lightmap to use, based on the lighting mode.
234    let lightmap: Option<Handle<Image>> = match app_status.lighting_mode {
235        LightingMode::Baked => {
236            Some(asset_server.load("lightmaps/MixedLightingExample-Baked.zstd.ktx2"))
237        }
238        LightingMode::MixedDirect => {
239            Some(asset_server.load("lightmaps/MixedLightingExample-MixedDirect.zstd.ktx2"))
240        }
241        LightingMode::MixedIndirect => {
242            Some(asset_server.load("lightmaps/MixedLightingExample-MixedIndirect.zstd.ktx2"))
243        }
244        LightingMode::RealTime => None,
245    };
246
247    'outer: for (entity, name, material) in &meshes {
248        // Add lightmaps to or remove lightmaps from the scenery objects in the
249        // scene (all objects but the sphere).
250        //
251        // Note that doing a linear search through the `LIGHTMAPS` array is
252        // inefficient, but we do it anyway in this example to improve clarity.
253        for (lightmap_name, uv_rect) in LIGHTMAPS {
254            if &**name != lightmap_name {
255                continue;
256            }
257
258            // Lightmap exposure defaults to zero, so we need to set it.
259            if let Some(ref mut material) = materials.get_mut(material) {
260                material.lightmap_exposure = LIGHTMAP_EXPOSURE;
261            }
262
263            // Add or remove the lightmap.
264            match lightmap {
265                Some(ref lightmap) => {
266                    commands.entity(entity).insert(Lightmap {
267                        image: (*lightmap).clone(),
268                        uv_rect,
269                        bicubic_sampling: false,
270                    });
271                }
272                None => {
273                    commands.entity(entity).remove::<Lightmap>();
274                }
275            }
276            continue 'outer;
277        }
278
279        // Add lightmaps to or remove lightmaps from the sphere.
280        if &**name == "Sphere" {
281            // Lightmap exposure defaults to zero, so we need to set it.
282            if let Some(ref mut material) = materials.get_mut(material) {
283                material.lightmap_exposure = LIGHTMAP_EXPOSURE;
284            }
285
286            // Add or remove the lightmap from the sphere. We only apply the
287            // lightmap in fully-baked mode.
288            match (&lightmap, app_status.lighting_mode) {
289                (Some(lightmap), LightingMode::Baked) => {
290                    commands.entity(entity).insert(Lightmap {
291                        image: (*lightmap).clone(),
292                        uv_rect: SPHERE_UV_RECT,
293                        bicubic_sampling: false,
294                    });
295                }
296                _ => {
297                    commands.entity(entity).remove::<Lightmap>();
298                }
299            }
300        }
301    }
302}
303
304/// Converts a uv rectangle from the OpenGL coordinate system (origin in the
305/// lower left) to the Vulkan coordinate system (origin in the upper left) that
306/// Bevy uses.
307///
308/// For this particular example, the baking tool happened to use the OpenGL
309/// coordinate system, so it was more convenient to do the conversion at compile
310/// time than to pre-calculate and hard-code the values.
311const fn uv_rect_opengl(gl_min: Vec2, size: Vec2) -> Rect {
312    let min = vec2(gl_min.x, 1.0 - gl_min.y - size.y);
313    Rect {
314        min,
315        max: vec2(min.x + size.x, min.y + size.y),
316    }
317}
```

examples/gizmos/anchored\_text\_gizmos.rs ([line 24](../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#24))

```rust
21fn anchors(mut text_gizmos: Gizmos, time: Res<Time>) {
22    let t = time.elapsed_secs();
23    for (label, anchor, color) in [
24        ("left", vec2(-0.5, 0.0), RED),
25        ("right", vec2(0.5, 0.0), ORANGE),
26        ("center", Vec2::ZERO, YELLOW),
27        ("top", vec2(0.0, 0.5), GREEN),
28        ("bottom", vec2(0.0, -0.5), BLUE),
29    ] {
30        let position = Vec2::splat(350.0) * anchor;
31        text_gizmos.text_2d(
32            Isometry2d::from_translation(position),
33            "+",
34            12.,
35            Vec2::ZERO,
36            Color::WHITE,
37        );
38        text_gizmos.text_2d(
39            Isometry2d::new(position, Rot2::radians(t)),
40            label,
41            25.,
42            anchor,
43            color,
44        );
45    }
46}
```

examples/math/cubic\_splines.rs ([line 44](../../src/cubic_splines/cubic_splines.rs.html#44))

```rust
35fn setup(mut commands: Commands) {
36    // Initialize the modes with their defaults:
37    let spline_mode = SplineMode::default();
38    commands.insert_resource(spline_mode);
39    let cycling_mode = CyclingMode::default();
40    commands.insert_resource(cycling_mode);
41
42    // Starting data for [`ControlPoints`]:
43    let default_points = vec![
44        vec2(-500., -200.),
45        vec2(-250., 250.),
46        vec2(250., 250.),
47        vec2(500., -200.),
48    ];
49
50    let default_tangents = vec![
51        vec2(0., 200.),
52        vec2(200., 0.),
53        vec2(0., -200.),
54        vec2(-200., 0.),
55    ];
56
57    let default_control_data = ControlPoints {
58        points_and_tangents: default_points.into_iter().zip(default_tangents).collect(),
59    };
60
61    let curve = form_curve(&default_control_data, spline_mode, cycling_mode);
62    commands.insert_resource(curve);
63    commands.insert_resource(default_control_data);
64
65    // Mouse tracking information:
66    commands.insert_resource(MousePosition::default());
67    commands.insert_resource(MouseEditMove::default());
68
69    commands.spawn(Camera2d);
70
71    // The instructions and modes are rendered on the left-hand side in a column.
72    let instructions_text = "Click and drag to add control points and their tangents\n\
73        R: Remove the last control point\n\
74        S: Cycle the spline construction being used\n\
75        C: Toggle cyclic curve construction";
76    let spline_mode_text = format!("Spline: {spline_mode}");
77    let cycling_mode_text = format!("{cycling_mode}");
78    let style = TextFont::default();
79
80    commands
81        .spawn(Node {
82            position_type: PositionType::Absolute,
83            top: px(12),
84            left: px(12),
85            flex_direction: FlexDirection::Column,
86            row_gap: px(20),
87            ..default()
88        })
89        .with_children(|parent| {
90            parent.spawn((Text::new(instructions_text), style.clone()));
91            parent.spawn((SplineModeText, Text(spline_mode_text), style.clone()));
92            parent.spawn((CyclingModeText, Text(cycling_mode_text), style.clone()));
93        });
94}
```

examples/3d/auto\_exposure.rs ([line 58](../../src/auto_exposure/auto_exposure.rs.html#58))

```rust
32fn setup(
33    mut commands: Commands,
34    mut meshes: ResMut<Assets<Mesh>>,
35    mut materials: ResMut<Assets<StandardMaterial>>,
36    mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
37    asset_server: Res<AssetServer>,
38) {
39    let metering_mask = asset_server.load("textures/basic_metering_mask.png");
40
41    commands.spawn((
42        Camera3d::default(),
43        Transform::from_xyz(1.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
44        AutoExposure {
45            metering_mask: metering_mask.clone(),
46            ..default()
47        },
48        Skybox {
49            image: Some(asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2")),
50            brightness: light_consts::lux::DIRECT_SUNLIGHT,
51            ..default()
52        },
53    ));
54
55    commands.insert_resource(ExampleResources {
56        basic_compensation_curve: compensation_curves.add(
57            AutoExposureCompensationCurve::from_curve(LinearSpline::new([
58                vec2(-4.0, -2.0),
59                vec2(0.0, 0.0),
60                vec2(2.0, 0.0),
61                vec2(4.0, 2.0),
62            ]))
63            .unwrap(),
64        ),
65        basic_metering_mask: metering_mask.clone(),
66    });
67
68    let plane = meshes.add(Mesh::from(
69        Plane3d {
70            normal: -Dir3::Z,
71            half_size: Vec2::new(2.0, 0.5),
72        }
73        .mesh(),
74    ));
75
76    // Build a dimly lit box around the camera, with a slot to see the bright skybox.
77    for level in -1..=1 {
78        for side in [-Vec3::X, Vec3::X, -Vec3::Z, Vec3::Z] {
79            if level == 0 && Vec3::Z == side {
80                continue;
81            }
82
83            let height = Vec3::Y * level as f32;
84
85            commands.spawn((
86                Mesh3d(plane.clone()),
87                MeshMaterial3d(materials.add(StandardMaterial {
88                    base_color: Color::srgb(
89                        0.5 + side.x * 0.5,
90                        0.75 - level as f32 * 0.25,
91                        0.5 + side.z * 0.5,
92                    ),
93                    ..default()
94                })),
95                Transform::from_translation(side * 2.0 + height).looking_at(height, Vec3::Y),
96            ));
97        }
98    }
99
100    commands.insert_resource(GlobalAmbientLight {
101        color: Color::WHITE,
102        brightness: 0.0,
103        ..default()
104    });
105
106    commands.spawn((
107        PointLight {
108            intensity: 2000.0,
109            ..default()
110        },
111        Transform::from_xyz(0.0, 0.0, 0.0),
112    ));
113
114    commands.spawn((
115        ImageNode {
116            image: metering_mask,
117            ..default()
118        },
119        Node {
120            width: percent(100),
121            height: percent(100),
122            ..default()
123        },
124    ));
125
126    let text_font = TextFont::default();
127
128    commands.spawn((Text::new("Left / Right - Rotate Camera\nC - Toggle Compensation Curve\nM - Toggle Metering Mask\nV - Visualize Metering Mask"),
129            text_font.clone(), Node {
130            position_type: PositionType::Absolute,
131            top: px(12),
132            left: px(12),
133            ..default()
134        })
135    );
136
137    commands.spawn((
138        Text::default(),
139        text_font,
140        Node {
141            position_type: PositionType::Absolute,
142            top: px(12),
143            right: px(12),
144            ..default()
145        },
146        ExampleDisplay,
147    ));
148}
```

examples/stress\_tests/many\_cubes.rs ([line 258](../../src/many_cubes/many_cubes.rs.html#258))

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