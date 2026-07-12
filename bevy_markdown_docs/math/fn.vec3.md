[bevy](../index.html)::[math](index.html)

# Function vec3 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/f32/vec3.rs.html#15)

```rust
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3
```

Creates a 3-dimensional vector.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/mirror.rs ([line 89](../../src/mirror/mirror.rs.html#89))

```rust
89const CAMERA_TARGET: Vec3 = vec3(-25.0, 20.0, 0.0);
90/// The camera stays this distance in meters from the camera target.
91const CAMERA_ORBIT_DISTANCE: f32 = 500.0;
92/// The speed at which the user can move the camera vertically, in radians per
93/// mouse input unit.
94const CAMERA_PITCH_SPEED: f32 = 0.003;
95/// The speed at which the user can move the camera horizontally, in radians per
96/// mouse input unit.
97const CAMERA_YAW_SPEED: f32 = 0.004;
98// Limiting pitch stops some unexpected rotation past 90° up or down.
99const CAMERA_PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
100
101/// The angle that the mirror faces.
102///
103/// The mirror is rotated across the X axis in this many radians.
104const MIRROR_ROTATION_ANGLE: f32 = -FRAC_PI_2;
105const MIRROR_POSITION: Vec3 = vec3(-25.0, 75.0, 0.0);
106
107/// The path to the animated fox model.
108static FOX_ASSET_PATH: &str = "models/animated/Fox.glb";
109
110/// The app entry point.
111fn main() {
112    App::new()
113        .add_plugins(DefaultPlugins.set(WindowPlugin {
114            primary_window: Some(Window {
115                title: "Bevy Mirror Example".into(),
116                ..default()
117            }),
118            ..default()
119        }))
120        .add_plugins(MaterialPlugin::<
121            ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>,
122        >::default())
123        .init_resource::<AppStatus>()
124        .add_message::<WidgetClickEvent<DragAction>>()
125        .add_systems(Startup, setup)
126        .add_systems(Update, handle_window_resize_messages)
127        .add_systems(Update, (move_camera_on_mouse_down, move_fox_on_mouse_down))
128        .add_systems(Update, widgets::handle_ui_interactions::<DragAction>)
129        .add_systems(
130            Update,
131            (handle_mouse_action_change, update_radio_buttons)
132                .after(widgets::handle_ui_interactions::<DragAction>),
133        )
134        .add_systems(
135            Update,
136            update_mirror_camera_on_main_camera_transform_change.after(move_camera_on_mouse_down),
137        )
138        .add_systems(Update, play_fox_animation)
139        .add_systems(Update, update_help_text)
140        .run();
141}
142
143/// A startup system that spawns the scene and sets up the mirror render target.
144fn setup(
145    mut commands: Commands,
146    windows_query: Query<&Window>,
147    asset_server: Res<AssetServer>,
148    mut meshes: ResMut<Assets<Mesh>>,
149    mut standard_materials: ResMut<Assets<StandardMaterial>>,
150    mut screen_space_texture_materials: ResMut<
151        Assets<ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>>,
152    >,
153    mut images: ResMut<Assets<Image>>,
154    app_status: Res<AppStatus>,
155) {
156    // Spawn the main camera.
157    let camera_projection = PerspectiveProjection::default();
158    let camera_transform = spawn_main_camera(&mut commands, &camera_projection);
159
160    // Spawn the light.
161    spawn_light(&mut commands);
162
163    // Spawn the objects reflected in the mirror.
164    spawn_ground_plane(&mut commands, &mut meshes, &mut standard_materials);
165    spawn_fox(&mut commands, &asset_server);
166
167    // Spawn the mirror and associated camera.
168    let mirror_render_target_image =
169        create_mirror_texture_resource(&mut commands, &windows_query, &mut images);
170    let mirror_transform = spawn_mirror(
171        &mut commands,
172        &mut meshes,
173        &mut screen_space_texture_materials,
174        mirror_render_target_image.clone(),
175    );
176    spawn_mirror_camera(
177        &mut commands,
178        &camera_transform,
179        &camera_projection,
180        &mirror_transform,
181        mirror_render_target_image,
182    );
183
184    // Spawn the UI.
185    spawn_buttons(&mut commands);
186    spawn_help_text(&mut commands, &app_status);
187}
188
189/// Spawns the main camera (not the mirror camera).
190fn spawn_main_camera(
191    commands: &mut Commands,
192    camera_projection: &PerspectiveProjection,
193) -> Transform {
194    let camera_transform = Transform::from_translation(
195        vec3(-2.0, 1.0, -2.0).normalize_or_zero() * CAMERA_ORBIT_DISTANCE,
196    )
197    .looking_at(CAMERA_TARGET, Vec3::Y);
198
199    commands.spawn((
200        Camera3d::default(),
201        camera_transform,
202        Projection::Perspective(camera_projection.clone()),
203    ));
204
205    camera_transform
206}
207
208/// Spawns a directional light to illuminate the scene.
209fn spawn_light(commands: &mut Commands) {
210    commands.spawn((
211        DirectionalLight {
212            illuminance: 5000.0,
213            ..default()
214        },
215        Transform::from_xyz(-85.0, 16.0, -200.0).looking_at(vec3(-50.0, 0.0, 100.0), Vec3::Y),
216    ));
217}
218
219/// Spawns the circular ground plane object.
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

examples/3d/visibility\_range.rs ([line 17](../../src/visibility_range/visibility_range.rs.html#17))

```rust
17const CAMERA_FOCAL_POINT: Vec3 = vec3(0.0, 0.3, 0.0);
```

examples/3d/anisotropy.rs ([line 14](../../src/anisotropy/anisotropy.rs.html#14))

```rust
14const CAMERA_INITIAL_POSITION: Vec3 = vec3(-0.4, 0.0, 0.0);
15
16/// The current settings of the app, as chosen by the user.
17#[derive(Resource)]
18struct AppStatus {
19    /// Which type of light is in the scene.
20    light_mode: LightMode,
21    /// Whether anisotropy is enabled.
22    anisotropy_enabled: bool,
23    /// Which mesh is visible
24    visible_scene: Scene,
25}
26
27/// Which type of light we're using: a directional light, a point light, or an
28/// environment map.
29#[derive(Clone, Copy, PartialEq, Default)]
30enum LightMode {
31    /// A rotating directional light.
32    #[default]
33    Directional,
34    /// A rotating point light.
35    Point,
36    /// An environment map (image-based lighting, including skybox).
37    EnvironmentMap,
38}
39
40/// A component that stores the version of the material with anisotropy and the
41/// version of the material without it.
42///
43/// This is placed on each mesh with a material. It exists so that the
44/// appropriate system can replace the materials when the user presses Enter to
45/// turn anisotropy on and off.
46#[derive(Component)]
47struct MaterialVariants {
48    /// The version of the material in the glTF file, with anisotropy.
49    anisotropic: Handle<StandardMaterial>,
50    /// The version of the material with anisotropy removed.
51    isotropic: Handle<StandardMaterial>,
52}
53
54#[derive(Default, Clone, Copy, PartialEq, Eq, Component)]
55enum Scene {
56    #[default]
57    BarnLamp,
58    Sphere,
59}
60
61impl Scene {
62    fn next(&self) -> Self {
63        match self {
64            Self::BarnLamp => Self::Sphere,
65            Self::Sphere => Self::BarnLamp,
66        }
67    }
68}
69
70impl Display for Scene {
71    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
72        let scene_name = match self {
73            Self::BarnLamp => "Barn Lamp",
74            Self::Sphere => "Sphere",
75        };
76        write!(f, "{scene_name}")
77    }
78}
79
80/// The application entry point.
81fn main() {
82    App::new()
83        .init_resource::<AppStatus>()
84        .add_plugins(DefaultPlugins.set(WindowPlugin {
85            primary_window: Some(Window {
86                title: "Bevy Anisotropy Example".into(),
87                ..default()
88            }),
89            ..default()
90        }))
91        .add_systems(Startup, setup)
92        .add_systems(Update, create_material_variants)
93        .add_systems(Update, animate_light)
94        .add_systems(Update, rotate_camera)
95        .add_systems(Update, (handle_input, update_help_text).chain())
96        .run();
97}
98
99/// Creates the initial scene.
100fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_status: Res<AppStatus>) {
101    commands.spawn((
102        Camera3d::default(),
103        Transform::from_translation(CAMERA_INITIAL_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
104    ));
105
106    spawn_directional_light(&mut commands);
107
108    commands.spawn((
109        WorldAssetRoot(
110            asset_server.load("models/AnisotropyBarnLamp/AnisotropyBarnLamp.gltf#Scene0"),
111        ),
112        Transform::from_xyz(0.0, 0.07, -0.13),
113        Scene::BarnLamp,
114    ));
115
116    commands.spawn((
117        Mesh3d(
118            asset_server.add(
119                Mesh::from(Sphere::new(0.1))
120                    .with_generated_tangents()
121                    .unwrap(),
122            ),
123        ),
124        MeshMaterial3d(asset_server.add(StandardMaterial {
125            base_color: palettes::tailwind::GRAY_300.into(),
126            anisotropy_rotation: 0.5,
127            anisotropy_strength: 1.,
128            ..default()
129        })),
130        Scene::Sphere,
131        Visibility::Hidden,
132    ));
133
134    spawn_text(&mut commands, &app_status);
135}
136
137/// Spawns the help text.
138fn spawn_text(commands: &mut Commands, app_status: &AppStatus) {
139    commands.spawn((
140        app_status.create_help_text(),
141        Node {
142            position_type: PositionType::Absolute,
143            bottom: px(12),
144            left: px(12),
145            ..default()
146        },
147    ));
148}
149
150/// For each material, creates a version with the anisotropy removed.
151///
152/// This allows the user to press Enter to toggle anisotropy on and off.
153fn create_material_variants(
154    mut commands: Commands,
155    mut materials: ResMut<Assets<StandardMaterial>>,
156    new_meshes: Query<
157        (Entity, &MeshMaterial3d<StandardMaterial>),
158        (
159            Added<MeshMaterial3d<StandardMaterial>>,
160            Without<MaterialVariants>,
161        ),
162    >,
163) {
164    for (entity, anisotropic_material_handle) in new_meshes.iter() {
165        let Some(anisotropic_material) = materials.get(anisotropic_material_handle).cloned() else {
166            continue;
167        };
168
169        commands.entity(entity).insert(MaterialVariants {
170            anisotropic: anisotropic_material_handle.0.clone(),
171            isotropic: materials.add(StandardMaterial {
172                anisotropy_texture: None,
173                anisotropy_strength: 0.0,
174                anisotropy_rotation: 0.0,
175                ..anisotropic_material
176            }),
177        });
178    }
179}
180
181/// A system that animates the light every frame, if there is one.
182fn animate_light(
183    mut lights: Query<&mut Transform, Or<(With<DirectionalLight>, With<PointLight>)>>,
184    time: Res<Time>,
185) {
186    let now = time.elapsed_secs();
187    for mut transform in lights.iter_mut() {
188        transform.translation = vec3(ops::cos(now), 1.0, ops::sin(now)) * vec3(3.0, 4.0, 3.0);
189        transform.look_at(Vec3::ZERO, Vec3::Y);
190    }
191}
```

examples/3d/mixed\_lighting.rs ([line 115](../../src/mixed_lighting/mixed_lighting.rs.html#115))

```rust
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
318
319/// Ensures that clicking on the scene to move the sphere doesn't result in a
320/// hit on the sphere itself.
321fn make_sphere_nonpickable(
322    mut commands: Commands,
323    mut query: Query<(Entity, &Name), (With<Mesh3d>, Without<Pickable>)>,
324) {
325    for (sphere, name) in &mut query {
326        if &**name == "Sphere" {
327            commands.entity(sphere).insert(Pickable::IGNORE);
328        }
329    }
330}
331
332/// Updates the directional light settings as necessary when the lighting mode
333/// changes.
334fn update_directional_light(
335    mut lights: Query<&mut DirectionalLight>,
336    mut lighting_mode_changed_reader: MessageReader<LightingModeChanged>,
337    app_status: Res<AppStatus>,
338) {
339    // Only run if the lighting mode changed. (Note that a change event is fired
340    // when the scene first loads.)
341    if lighting_mode_changed_reader.read().next().is_none() {
342        return;
343    }
344
345    // Real-time direct light is used on the scenery if we're using mixed
346    // indirect or real-time mode.
347    let scenery_is_lit_in_real_time = matches!(
348        app_status.lighting_mode,
349        LightingMode::MixedIndirect | LightingMode::RealTime
350    );
351
352    for mut light in &mut lights {
353        light.affects_lightmapped_mesh_diffuse = scenery_is_lit_in_real_time;
354        // Don't bother enabling shadows if they won't show up on the scenery.
355        light.shadow_maps_enabled = scenery_is_lit_in_real_time;
356    }
357}
358
359/// Updates the state of the selection widgets at the bottom of the window when
360/// the lighting mode changes.
361fn update_radio_buttons(
362    mut widgets: Query<
363        (
364            Entity,
365            Option<&mut BackgroundColor>,
366            Has<Text>,
367            &WidgetClickSender<LightingMode>,
368        ),
369        Or<(With<RadioButton>, With<RadioButtonText>)>,
370    >,
371    app_status: Res<AppStatus>,
372    mut writer: TextUiWriter,
373) {
374    for (entity, image, has_text, sender) in &mut widgets {
375        let selected = **sender == app_status.lighting_mode;
376
377        if let Some(mut bg_color) = image {
378            widgets::update_ui_radio_button(&mut bg_color, selected);
379        }
380        if has_text {
381            widgets::update_ui_radio_button_text(entity, &mut writer, selected);
382        }
383    }
384}
385
386/// Handles clicks on the widgets at the bottom of the screen and fires
387/// [`LightingModeChanged`] events.
388fn handle_lighting_mode_change(
389    mut widget_click_event_reader: MessageReader<WidgetClickEvent<LightingMode>>,
390    mut lighting_mode_changed_writer: MessageWriter<LightingModeChanged>,
391    mut app_status: ResMut<AppStatus>,
392) {
393    for event in widget_click_event_reader.read() {
394        app_status.lighting_mode = **event;
395        lighting_mode_changed_writer.write(LightingModeChanged);
396    }
397}
398
399/// Moves the sphere to its original position when the user selects the baked
400/// lighting mode.
401///
402/// As the light from the sphere is precomputed and depends on the sphere's
403/// original position, the sphere must be placed there in order for the lighting
404/// to be correct.
405fn reset_sphere_position(
406    mut objects: Query<(&Name, &mut Transform)>,
407    mut lighting_mode_changed_reader: MessageReader<LightingModeChanged>,
408    app_status: Res<AppStatus>,
409) {
410    // Only run if the lighting mode changed and if the lighting mode is
411    // `LightingMode::Baked`. (Note that a change event is fired when the scene
412    // first loads.)
413    if lighting_mode_changed_reader.read().next().is_none()
414        || app_status.lighting_mode != LightingMode::Baked
415    {
416        return;
417    }
418
419    for (name, mut transform) in &mut objects {
420        if &**name == "Sphere" {
421            transform.translation = INITIAL_SPHERE_POSITION;
422            break;
423        }
424    }
425}
426
427/// Updates the position of the sphere when the user clicks on a spot in the
428/// scene.
429///
430/// Note that the position of the sphere is locked in baked lighting mode.
431fn move_sphere(
432    mouse_button_input: Res<ButtonInput<MouseButton>>,
433    pointers: Query<&PointerInteraction>,
434    mut meshes: Query<(&GltfMeshName, &ChildOf), With<Mesh3d>>,
435    mut transforms: Query<&mut Transform>,
436    app_status: Res<AppStatus>,
437) {
438    // Only run when the left button is clicked and we're not in baked lighting
439    // mode.
440    if app_status.lighting_mode == LightingMode::Baked
441        || !mouse_button_input.pressed(MouseButton::Left)
442    {
443        return;
444    }
445
446    // Find the sphere.
447    let Some(child_of) = meshes
448        .iter_mut()
449        .filter_map(|(name, child_of)| {
450            if &**name == "Sphere" {
451                Some(child_of)
452            } else {
453                None
454            }
455        })
456        .next()
457    else {
458        return;
459    };
460
461    // Grab its transform.
462    let Ok(mut transform) = transforms.get_mut(child_of.parent()) else {
463        return;
464    };
465
466    // Set its transform to the appropriate position, as determined by the
467    // picking subsystem.
468    for interaction in pointers.iter() {
469        if let Some(&(
470            _,
471            HitData {
472                position: Some(position),
473                ..
474            },
475        )) = interaction.get_nearest_hit()
476        {
477            transform.translation = position + vec3(0.0, SPHERE_OFFSET, 0.0);
478        }
479    }
480}
```

examples/stress\_tests/many\_cubes.rs ([line 609](../../src/many_cubes/many_cubes.rs.html#609))

```rust
608fn fast_hue_to_rgb(hue: f32) -> Vec3 {
609    (hue * 6.0 - vec3(3.0, 2.0, 4.0)).abs() * vec3(1.0, -1.0, -1.0) + vec3(-1.0, 2.0, 2.0)
610}
```

examples/shader\_advanced/custom\_phase\_item.rs ([line 158](../../src/custom_phase_item/custom_phase_item.rs.html#158))

```rust
157static VERTICES: [Vertex; 3] = [
158    Vertex::new(vec3(-0.866, -0.5, 0.5), vec3(1.0, 0.0, 0.0)),
159    Vertex::new(vec3(0.866, -0.5, 0.5), vec3(0.0, 1.0, 0.0)),
160    Vertex::new(vec3(0.0, 1.0, 0.5), vec3(0.0, 0.0, 1.0)),
161];
```

Additional examples can be found in:  

*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#272)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#249-253)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#297)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#228)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#200)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#479-483)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#48)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#419)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#88)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#65)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#66)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#43)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#507)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#71)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#299-303)