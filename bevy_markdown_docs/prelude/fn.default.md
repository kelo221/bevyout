[bevy](../index.html)::[prelude](index.html)

# Function default 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/default.rs.html#29)

```rust
pub fn default<T>() -> Twhere
    T: Default,
```

An ergonomic abbreviation for [`Default::default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") to make initializing structs easier.

This is especially helpful when combined with [“struct update syntax”](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax).

```rust
use bevy_utils::default;

#[derive(Default)]
struct Foo {
  a: usize,
  b: usize,
  c: usize,
}

// Normally you would initialize a struct with defaults using "struct update syntax"
// combined with `Default::default()`. This example sets `Foo::a` to 10 and the remaining
// values to their defaults.
let foo = Foo {
  a: 10,
  ..Default::default()
};

// But now you can do this, which is equivalent:
let foo = Foo {
  a: 10,
  ..default()
};
```

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/color\_grading.rs ([line 74](../../src/color_grading/color_grading.rs.html#74))

```rust
73    fn default() -> Self {
74        Self::Global(default())
75    }
76}
77
78/// Buttons consist of three parts: the button itself, a label child, and a
79/// value child. This specifies one of the three entities.
80#[derive(Clone, Copy, PartialEq, Component)]
81enum ColorGradingOptionWidgetType {
82    /// The parent button.
83    Button,
84    /// The label of the button.
85    Label,
86    /// The numerical value that the button displays.
87    Value,
88}
89
90#[derive(Clone, Copy, Component)]
91struct ColorGradingOptionWidget {
92    widget_type: ColorGradingOptionWidgetType,
93    option: SelectedColorGradingOption,
94}
95
96/// A marker component for the help text at the top left of the screen.
97#[derive(Clone, Copy, Component)]
98struct HelpText;
99
100fn main() {
101    App::new()
102        .add_plugins(DefaultPlugins)
103        .init_resource::<SelectedColorGradingOption>()
104        .add_systems(Startup, setup)
105        .add_systems(
106            Update,
107            (
108                handle_button_presses,
109                adjust_color_grading_option,
110                update_ui_state,
111            )
112                .chain(),
113        )
114        .run();
115}
116
117fn setup(
118    mut commands: Commands,
119    currently_selected_option: Res<SelectedColorGradingOption>,
120    asset_server: Res<AssetServer>,
121) {
122    // Create the scene.
123    add_basic_scene(&mut commands, &asset_server);
124
125    // Create the root UI element.
126    let font = asset_server.load(FONT_PATH);
127    let color_grading = ColorGrading::default();
128    add_buttons(&mut commands, &font, &color_grading);
129
130    // Spawn help text.
131    add_help_text(&mut commands, &font, &currently_selected_option);
132
133    // Spawn the camera.
134    add_camera(&mut commands, &asset_server, color_grading);
135}
136
137/// Adds all the buttons on the bottom of the scene.
138fn add_buttons(commands: &mut Commands, font: &Handle<Font>, color_grading: &ColorGrading) {
139    commands.spawn((
140        // Spawn the parent node that contains all the buttons.
141        Node {
142            flex_direction: FlexDirection::Column,
143            position_type: PositionType::Absolute,
144            row_gap: px(6),
145            left: px(12),
146            bottom: px(12),
147            ..default()
148        },
149        children![
150            // Create the first row, which contains the global controls.
151            buttons_for_global_controls(color_grading, font),
152            // Create the rows for individual controls.
153            buttons_for_section(SelectedColorGradingSection::Highlights, color_grading, font),
154            buttons_for_section(SelectedColorGradingSection::Midtones, color_grading, font),
155            buttons_for_section(SelectedColorGradingSection::Shadows, color_grading, font),
156        ],
157    ));
158}
159
160/// Adds the buttons for the global controls (those that control the scene as a
161/// whole as opposed to shadows, midtones, or highlights).
162fn buttons_for_global_controls(color_grading: &ColorGrading, font: &Handle<Font>) -> impl Bundle {
163    let make_button = |option: SelectedGlobalColorGradingOption| {
164        button_for_value(
165            SelectedColorGradingOption::Global(option),
166            color_grading,
167            font,
168        )
169    };
170
171    // Add the parent node for the row.
172    (
173        Node::default(),
174        children![
175            Node {
176                width: px(125),
177                ..default()
178            },
179            make_button(SelectedGlobalColorGradingOption::Exposure),
180            make_button(SelectedGlobalColorGradingOption::Temperature),
181            make_button(SelectedGlobalColorGradingOption::Tint),
182            make_button(SelectedGlobalColorGradingOption::Hue),
183        ],
184    )
185}
186
187/// Adds the buttons that control color grading for individual sections
188/// (highlights, midtones, shadows).
189fn buttons_for_section(
190    section: SelectedColorGradingSection,
191    color_grading: &ColorGrading,
192    font: &Handle<Font>,
193) -> impl Bundle {
194    let make_button = |option| {
195        button_for_value(
196            SelectedColorGradingOption::Section(section, option),
197            color_grading,
198            font,
199        )
200    };
201
202    // Spawn the row container.
203    (
204        Node {
205            align_items: AlignItems::Center,
206            ..default()
207        },
208        children![
209            // Spawn the label ("Highlights", etc.)
210            (
211                text(&section.to_string(), font, Color::WHITE),
212                Node {
213                    width: px(125),
214                    ..default()
215                }
216            ),
217            // Spawn the buttons.
218            make_button(SelectedSectionColorGradingOption::Saturation),
219            make_button(SelectedSectionColorGradingOption::Contrast),
220            make_button(SelectedSectionColorGradingOption::Gamma),
221            make_button(SelectedSectionColorGradingOption::Gain),
222            make_button(SelectedSectionColorGradingOption::Lift),
223        ],
224    )
225}
226
227/// Adds a button that controls one of the color grading values.
228fn button_for_value(
229    option: SelectedColorGradingOption,
230    color_grading: &ColorGrading,
231    font: &Handle<Font>,
232) -> impl Bundle {
233    let label = match option {
234        SelectedColorGradingOption::Global(option) => option.to_string(),
235        SelectedColorGradingOption::Section(_, option) => option.to_string(),
236    };
237
238    // Add the button node.
239    (
240        Button,
241        Node {
242            border: UiRect::all(px(1)),
243            width: px(200),
244            justify_content: JustifyContent::Center,
245            align_items: AlignItems::Center,
246            padding: UiRect::axes(px(12), px(6)),
247            margin: UiRect::right(px(12)),
248            border_radius: BorderRadius::MAX,
249            ..default()
250        },
251        BorderColor::all(Color::WHITE),
252        BackgroundColor(Color::BLACK),
253        ColorGradingOptionWidget {
254            widget_type: ColorGradingOptionWidgetType::Button,
255            option,
256        },
257        children![
258            // Add the button label.
259            (
260                text(&label, font, Color::WHITE),
261                ColorGradingOptionWidget {
262                    widget_type: ColorGradingOptionWidgetType::Label,
263                    option,
264                },
265            ),
266            // Add a spacer.
267            Node {
268                flex_grow: 1.0,
269                ..default()
270            },
271            // Add the value text.
272            (
273                text(
274                    &format!("{:.3}", option.get(color_grading)),
275                    font,
276                    Color::WHITE,
277                ),
278                ColorGradingOptionWidget {
279                    widget_type: ColorGradingOptionWidgetType::Value,
280                    option,
281                },
282            ),
283        ],
284    )
285}
286
287/// Creates the help text at the top of the screen.
288fn add_help_text(
289    commands: &mut Commands,
290    font: &Handle<Font>,
291    currently_selected_option: &SelectedColorGradingOption,
292) {
293    commands.spawn((
294        Text::new(create_help_text(currently_selected_option)),
295        TextFont {
296            font: FontSource::from(font),
297            ..default()
298        },
299        Node {
300            position_type: PositionType::Absolute,
301            left: px(12),
302            top: px(12),
303            ..default()
304        },
305        HelpText,
306    ));
307}
308
309/// Adds some text to the scene.
310fn text(label: &str, font: &Handle<Font>, color: Color) -> impl Bundle + use<> {
311    (
312        Text::new(label),
313        TextFont {
314            font: font.into(),
315            font_size: FontSize::Px(15.0),
316            ..default()
317        },
318        TextColor(color),
319    )
320}
321
322fn add_camera(commands: &mut Commands, asset_server: &AssetServer, color_grading: ColorGrading) {
323    commands.spawn((
324        Camera3d::default(),
325        Hdr,
326        Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
327        color_grading,
328        DistanceFog {
329            color: Color::srgb_u8(43, 44, 47),
330            falloff: FogFalloff::Linear {
331                start: 1.0,
332                end: 8.0,
333            },
334            ..default()
335        },
336        EnvironmentMapLight {
337            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
338            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
339            intensity: 2000.0,
340            ..default()
341        },
342    ));
343}
344
345fn add_basic_scene(commands: &mut Commands, asset_server: &AssetServer) {
346    // Spawn the main scene.
347    commands.spawn(WorldAssetRoot(asset_server.load(
348        GltfAssetLabel::Scene(0).from_asset("models/TonemappingTest/TonemappingTest.gltf"),
349    )));
350
351    // Spawn the flight helmet.
352    commands.spawn((
353        WorldAssetRoot(
354            asset_server
355                .load(GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf")),
356        ),
357        Transform::from_xyz(0.5, 0.0, -0.5).with_rotation(Quat::from_rotation_y(-0.15 * PI)),
358    ));
359
360    // Spawn the light.
361    commands.spawn((
362        DirectionalLight {
363            illuminance: 15000.0,
364            shadow_maps_enabled: true,
365            ..default()
366        },
367        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
368        CascadeShadowConfigBuilder {
369            maximum_distance: 3.0,
370            first_cascade_far_bound: 0.9,
371            ..default()
372        }
373        .build(),
374    ));
375}
```

Hide additional examples

examples/shader/extended\_material.rs ([line 95](../../src/extended_material/extended_material.rs.html#95))

```rust
92    fn new(quantize_steps: u32) -> Self {
93        Self {
94            quantize_steps,
95            ..default()
96        }
97    }
```

examples/3d/pcss.rs ([line 66](../../src/pcss/pcss.rs.html#66))

```rust
64    fn default() -> Self {
65        Self {
66            light_type: default(),
67            shadow_filter: default(),
68            soft_shadows: true,
69        }
70    }
71}
72
73/// The type of light presently in the scene: directional, point, or spot.
74#[derive(Clone, Copy, Default, PartialEq)]
75enum LightType {
76    /// A directional light, with a cascaded shadow map.
77    #[default]
78    Directional,
79    /// A point light, with a cube shadow map.
80    Point,
81    /// A spot light, with a cube shadow map.
82    Spot,
83}
84
85/// The type of shadow filter.
86///
87/// Generally, `Gaussian` is preferred when temporal antialiasing isn't in use,
88/// while `Temporal` is preferred when TAA is in use. In this example, this
89/// setting also turns TAA on and off.
90#[derive(Clone, Copy, Default, PartialEq)]
91enum ShadowFilter {
92    /// The non-temporal Gaussian filter (Castano '13 for directional lights, an
93    /// analogous alternative for point and spot lights).
94    #[default]
95    NonTemporal,
96    /// The temporal Gaussian filter (Jimenez '14 for directional lights, an
97    /// analogous alternative for point and spot lights).
98    Temporal,
99}
100
101/// Each example setting that can be toggled in the UI.
102#[derive(Clone, Copy, PartialEq)]
103enum AppSetting {
104    /// The type of light presently in the scene: directional, point, or spot.
105    LightType(LightType),
106    /// The type of shadow filter.
107    ShadowFilter(ShadowFilter),
108    /// Whether PCSS is enabled or disabled.
109    SoftShadows(bool),
110}
111
112/// The example application entry point.
113fn main() {
114    #[cfg(not(feature = "free_camera"))]
115    println!("Enable feature free_camera to add a free camera to this example");
116
117    App::new()
118        .init_resource::<AppStatus>()
119        .add_plugins((
120            DefaultPlugins.set(WindowPlugin {
121                primary_window: Some(Window {
122                    title: "Bevy Percentage Closer Soft Shadows Example".into(),
123                    ..default()
124                }),
125                ..default()
126            }),
127            #[cfg(feature = "free_camera")]
128            FreeCameraPlugin,
129        ))
130        .add_message::<WidgetClickEvent<AppSetting>>()
131        .add_systems(Startup, setup)
132        .add_systems(Update, widgets::handle_ui_interactions::<AppSetting>)
133        .add_systems(
134            Update,
135            update_radio_buttons.after(widgets::handle_ui_interactions::<AppSetting>),
136        )
137        .add_systems(
138            Update,
139            (
140                handle_light_type_change,
141                handle_shadow_filter_change,
142                handle_pcss_toggle,
143            )
144                .after(widgets::handle_ui_interactions::<AppSetting>),
145        )
146        .run();
147}
148
149/// Creates all the objects in the scene.
150fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_status: Res<AppStatus>) {
151    spawn_camera(&mut commands, &asset_server);
152    spawn_light(&mut commands, &app_status);
153    spawn_gltf_scene(&mut commands, &asset_server);
154    spawn_buttons(&mut commands);
155}
156
157/// Spawns the camera, with the initial shadow filtering method.
158fn spawn_camera(commands: &mut Commands, asset_server: &AssetServer) {
159    commands
160        .spawn((
161            Camera3d::default(),
162            Transform::from_xyz(-12.912 * 0.7, 4.466 * 0.7, -10.624 * 0.7).with_rotation(
163                Quat::from_euler(EulerRot::YXZ, -134.76 / 180.0 * PI, -0.175, 0.0),
164            ),
165            #[cfg(feature = "free_camera")]
166            FreeCamera::default(),
167        ))
168        .insert(ShadowFilteringMethod::Gaussian)
169        // `TemporalJitter` is needed for TAA. Note that it does nothing without
170        // `TemporalAntiAliasSettings`.
171        .insert(TemporalJitter::default())
172        // We want MSAA off for TAA to work properly.
173        .insert(Msaa::Off)
174        // The depth prepass is needed for TAA.
175        .insert(DepthPrepass)
176        // The motion vector prepass is needed for TAA.
177        .insert(MotionVectorPrepass)
178        // Add a nice skybox.
179        .insert(Skybox {
180            image: Some(asset_server.load("environment_maps/sky_skybox.ktx2")),
181            brightness: 500.0,
182            rotation: Quat::IDENTITY,
183        });
184}
185
186/// Spawns the initial light.
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
209
210/// Loads and spawns the glTF palm tree scene.
211fn spawn_gltf_scene(commands: &mut Commands, asset_server: &AssetServer) {
212    commands.spawn(WorldAssetRoot(
213        asset_server.load("models/PalmTree/PalmTree.gltf#Scene0"),
214    ));
215}
216
217/// Spawns all the buttons at the bottom of the screen.
218fn spawn_buttons(commands: &mut Commands) {
219    commands.spawn((
220        widgets::main_ui_node(),
221        children![
222            widgets::option_buttons(
223                "Light Type",
224                &[
225                    (AppSetting::LightType(LightType::Directional), "Directional"),
226                    (AppSetting::LightType(LightType::Point), "Point"),
227                    (AppSetting::LightType(LightType::Spot), "Spot"),
228                ],
229            ),
230            widgets::option_buttons(
231                "Shadow Filter",
232                &[
233                    (AppSetting::ShadowFilter(ShadowFilter::Temporal), "Temporal"),
234                    (
235                        AppSetting::ShadowFilter(ShadowFilter::NonTemporal),
236                        "Non-Temporal",
237                    ),
238                ],
239            ),
240            widgets::option_buttons(
241                "Soft Shadows",
242                &[
243                    (AppSetting::SoftShadows(true), "On"),
244                    (AppSetting::SoftShadows(false), "Off"),
245                ],
246            ),
247        ],
248    ));
249}
250
251/// Updates the style of the radio buttons that enable and disable soft shadows
252/// to reflect whether PCSS is enabled.
253fn update_radio_buttons(
254    mut widgets: Query<
255        (
256            Entity,
257            Option<&mut BackgroundColor>,
258            Has<Text>,
259            &WidgetClickSender<AppSetting>,
260        ),
261        Or<(With<RadioButton>, With<RadioButtonText>)>,
262    >,
263    app_status: Res<AppStatus>,
264    mut writer: TextUiWriter,
265) {
266    for (entity, image, has_text, sender) in widgets.iter_mut() {
267        let selected = match **sender {
268            AppSetting::LightType(light_type) => light_type == app_status.light_type,
269            AppSetting::ShadowFilter(shadow_filter) => shadow_filter == app_status.shadow_filter,
270            AppSetting::SoftShadows(soft_shadows) => soft_shadows == app_status.soft_shadows,
271        };
272
273        if let Some(mut bg_color) = image {
274            widgets::update_ui_radio_button(&mut bg_color, selected);
275        }
276        if has_text {
277            widgets::update_ui_radio_button_text(entity, &mut writer, selected);
278        }
279    }
280}
281
282/// Handles requests from the user to change the type of light.
283fn handle_light_type_change(
284    mut commands: Commands,
285    mut lights: Query<Entity, Or<(With<DirectionalLight>, With<PointLight>, With<SpotLight>)>>,
286    mut events: MessageReader<WidgetClickEvent<AppSetting>>,
287    mut app_status: ResMut<AppStatus>,
288) {
289    for event in events.read() {
290        let AppSetting::LightType(light_type) = **event else {
291            continue;
292        };
293        app_status.light_type = light_type;
294
295        for light in lights.iter_mut() {
296            let mut light_commands = commands.entity(light);
297            light_commands
298                .remove::<DirectionalLight>()
299                .remove::<PointLight>()
300                .remove::<SpotLight>();
301            match light_type {
302                LightType::Point => {
303                    light_commands.insert(create_point_light(&app_status));
304                }
305                LightType::Spot => {
306                    light_commands.insert(create_spot_light(&app_status));
307                }
308                LightType::Directional => {
309                    light_commands.insert(create_directional_light(&app_status));
310                }
311            }
312        }
313    }
314}
315
316/// Handles requests from the user to change the shadow filter method.
317///
318/// This system is also responsible for enabling and disabling TAA as
319/// appropriate.
320fn handle_shadow_filter_change(
321    mut commands: Commands,
322    mut cameras: Query<(Entity, &mut ShadowFilteringMethod)>,
323    mut events: MessageReader<WidgetClickEvent<AppSetting>>,
324    mut app_status: ResMut<AppStatus>,
325) {
326    for event in events.read() {
327        let AppSetting::ShadowFilter(shadow_filter) = **event else {
328            continue;
329        };
330        app_status.shadow_filter = shadow_filter;
331
332        for (camera, mut shadow_filtering_method) in cameras.iter_mut() {
333            match shadow_filter {
334                ShadowFilter::NonTemporal => {
335                    *shadow_filtering_method = ShadowFilteringMethod::Gaussian;
336                    commands.entity(camera).remove::<TemporalAntiAliasing>();
337                }
338                ShadowFilter::Temporal => {
339                    *shadow_filtering_method = ShadowFilteringMethod::Temporal;
340                    commands
341                        .entity(camera)
342                        .insert(TemporalAntiAliasing::default());
343                }
344            }
345        }
346    }
347}
348
349/// Handles requests from the user to toggle soft shadows on and off.
350fn handle_pcss_toggle(
351    mut lights: Query<AnyOf<(&mut DirectionalLight, &mut PointLight, &mut SpotLight)>>,
352    mut events: MessageReader<WidgetClickEvent<AppSetting>>,
353    mut app_status: ResMut<AppStatus>,
354) {
355    for event in events.read() {
356        let AppSetting::SoftShadows(value) = **event else {
357            continue;
358        };
359        app_status.soft_shadows = value;
360
361        // Recreating the lights is the simplest way to toggle soft shadows.
362        for (directional_light, point_light, spot_light) in lights.iter_mut() {
363            if let Some(mut directional_light) = directional_light {
364                *directional_light = create_directional_light(&app_status);
365            }
366            if let Some(mut point_light) = point_light {
367                *point_light = create_point_light(&app_status);
368            }
369            if let Some(mut spot_light) = spot_light {
370                *spot_light = create_spot_light(&app_status);
371            }
372        }
373    }
374}
375
376/// Creates the [`DirectionalLight`] component with the appropriate settings.
377fn create_directional_light(app_status: &AppStatus) -> DirectionalLight {
378    DirectionalLight {
379        shadow_maps_enabled: true,
380        soft_shadow_size: if app_status.soft_shadows {
381            Some(LIGHT_RADIUS)
382        } else {
383            None
384        },
385        shadow_depth_bias: DIRECTIONAL_SHADOW_DEPTH_BIAS,
386        ..default()
387    }
388}
389
390/// Creates the [`PointLight`] component with the appropriate settings.
391fn create_point_light(app_status: &AppStatus) -> PointLight {
392    PointLight {
393        intensity: POINT_LIGHT_INTENSITY,
394        range: POINT_LIGHT_RANGE,
395        shadow_maps_enabled: true,
396        radius: LIGHT_RADIUS,
397        soft_shadows_enabled: app_status.soft_shadows,
398        shadow_depth_bias: POINT_SHADOW_DEPTH_BIAS,
399        shadow_map_near_z: SHADOW_MAP_NEAR_Z,
400        ..default()
401    }
402}
403
404/// Creates the [`SpotLight`] component with the appropriate settings.
405fn create_spot_light(app_status: &AppStatus) -> SpotLight {
406    SpotLight {
407        intensity: POINT_LIGHT_INTENSITY,
408        range: POINT_LIGHT_RANGE,
409        radius: LIGHT_RADIUS,
410        shadow_maps_enabled: true,
411        soft_shadows_enabled: app_status.soft_shadows,
412        shadow_depth_bias: DIRECTIONAL_SHADOW_DEPTH_BIAS,
413        shadow_map_near_z: SHADOW_MAP_NEAR_Z,
414        ..default()
415    }
416}
```

tests/3d/no\_prepass.rs ([line 9](../../src/no_prepass/no_prepass.rs.html#9))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins.set(PbrPlugin {
8            prepass_enabled: false,
9            ..default()
10        }))
11        .run();
12}
```

examples/3d/spherical\_area\_lights.rs ([line 9](../../src/spherical_area_lights/spherical_area_lights.rs.html#9))

```rust
5fn main() {
6    App::new()
7        .insert_resource(GlobalAmbientLight {
8            brightness: 60.0,
9            ..default()
10        })
11        .add_plugins(DefaultPlugins)
12        .add_systems(Startup, setup)
13        .run();
14}
15
16fn setup(
17    mut commands: Commands,
18    mut meshes: ResMut<Assets<Mesh>>,
19    mut materials: ResMut<Assets<StandardMaterial>>,
20) {
21    // camera
22    commands.spawn((
23        Camera3d::default(),
24        Transform::from_xyz(0.2, 1.5, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
25    ));
26
27    // plane
28    commands.spawn((
29        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
30        MeshMaterial3d(materials.add(StandardMaterial {
31            base_color: Color::srgb(0.2, 0.2, 0.2),
32            perceptual_roughness: 0.08,
33            ..default()
34        })),
35    ));
36
37    const COUNT: usize = 6;
38    let position_range = -2.0..2.0;
39    let radius_range = 0.0..0.4;
40    let pos_len = position_range.end - position_range.start;
41    let radius_len = radius_range.end - radius_range.start;
42    let mesh = meshes.add(Sphere::new(1.0).mesh().uv(120, 64));
43
44    for i in 0..COUNT {
45        let percent = i as f32 / COUNT as f32;
46        let radius = radius_range.start + percent * radius_len;
47
48        // sphere light
49        commands
50            .spawn((
51                Mesh3d(mesh.clone()),
52                MeshMaterial3d(materials.add(StandardMaterial {
53                    base_color: Color::srgb(0.5, 0.5, 1.0),
54                    unlit: true,
55                    ..default()
56                })),
57                Transform::from_xyz(position_range.start + percent * pos_len, 0.3, 0.0)
58                    .with_scale(Vec3::splat(radius)),
59            ))
60            .with_child(PointLight {
61                radius,
62                color: Color::srgb(0.2, 0.2, 1.0),
63                ..default()
64            });
65    }
66}
```

examples/3d/../helpers/widgets.rs ([line 54](../../src/clustered_decal_maps/helpers/widgets.rs.html#54))

```rust
47pub fn main_ui_node() -> Node {
48    Node {
49        flex_direction: FlexDirection::Column,
50        position_type: PositionType::Absolute,
51        row_gap: px(6),
52        left: px(10),
53        bottom: px(10),
54        ..default()
55    }
56}
57
58/// Spawns a single radio button that allows configuration of a setting.
59///
60/// The type parameter specifies the value that will be packaged up and sent in
61/// a [`WidgetClickEvent`] when the radio button is clicked.
62pub fn option_button<T>(
63    option_value: T,
64    option_name: &str,
65    is_selected: bool,
66    is_first: bool,
67    is_last: bool,
68) -> impl Bundle
69where
70    T: Clone + Send + Sync + 'static,
71{
72    let (bg_color, fg_color) = if is_selected {
73        (Color::WHITE, Color::BLACK)
74    } else {
75        (Color::BLACK, Color::WHITE)
76    };
77
78    // Add the button node.
79    (
80        Button,
81        Node {
82            border: BUTTON_BORDER.with_left(if is_first { px(1) } else { px(0) }),
83            justify_content: JustifyContent::Center,
84            align_items: AlignItems::Center,
85            padding: BUTTON_PADDING,
86            border_radius: BorderRadius::ZERO
87                .with_left(if is_first {
88                    BUTTON_BORDER_RADIUS_SIZE
89                } else {
90                    px(0)
91                })
92                .with_right(if is_last {
93                    BUTTON_BORDER_RADIUS_SIZE
94                } else {
95                    px(0)
96                }),
97            ..default()
98        },
99        BUTTON_BORDER_COLOR,
100        BackgroundColor(bg_color),
101        RadioButton,
102        WidgetClickSender(option_value.clone()),
103        children![(
104            ui_text(option_name, fg_color),
105            RadioButtonText,
106            WidgetClickSender(option_value),
107        )],
108    )
109}
110
111/// Spawns the buttons that allow configuration of a setting.
112///
113/// The user may change the setting to any one of the labeled `options`. The
114/// value of the given type parameter will be packaged up and sent as a
115/// [`WidgetClickEvent`] when one of the radio buttons is clicked.
116pub fn option_buttons<T>(title: &str, options: &[(T, &str)]) -> impl Bundle
117where
118    T: Clone + Send + Sync + 'static,
119{
120    let buttons = options
121        .iter()
122        .cloned()
123        .enumerate()
124        .map(|(option_index, (option_value, option_name))| {
125            option_button(
126                option_value,
127                option_name,
128                option_index == 0,
129                option_index == 0,
130                option_index == options.len() - 1,
131            )
132        })
133        .collect::<Vec<_>>();
134    // Add the parent node for the row.
135    (
136        Node {
137            align_items: AlignItems::Center,
138            ..default()
139        },
140        Children::spawn((
141            Spawn((
142                ui_text(title, Color::WHITE),
143                Node {
144                    width: px(150),
145                    ..default()
146                },
147            )),
148            SpawnIter(buttons.into_iter()),
149        )),
150    )
151}
152
153/// Creates a text bundle for the UI.
154pub fn ui_text(label: &str, color: Color) -> impl Bundle + use<> {
155    (
156        Text::new(label),
157        TextFont {
158            font_size: FontSize::Px(18.0),
159            ..default()
160        },
161        TextColor(color),
162    )
163}
```

Additional examples can be found in:  

*   [examples/app/log\_layers.rs](../../src/log_layers/log_layers.rs.html#60)
*   [examples/asset/asset\_decompression.rs](../../src/asset_decompression/asset_decompression.rs.html#111)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#16)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#17)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#15)
*   [examples/window/monitor\_info.rs](../../src/monitor_info/monitor_info.rs.html#14)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#13)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#24)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#26)
*   [examples/audio/decodable.rs](../../src/decodable/decodable.rs.html#88)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#21)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#318)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#29)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#226)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#51)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#58)
*   [examples/app/no\_renderer.rs](../../src/no_renderer/no_renderer.rs.html#18)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#218)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#11)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#18)
*   [examples/audio/play\_sound\_effect.rs](../../src/play_sound_effect/play_sound_effect.rs.html#39)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#17)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#23)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#10)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#21)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#53)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#30)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#104)
*   [examples/app/return\_after\_run.rs](../../src/return_after_run/return_after_run.rs.html#16)
*   [examples/window/window\_resizing.rs](../../src/window_resizing/window_resizing.rs.html#40)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#47)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#12)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#19)
*   [examples/window/scale\_factor\_override.rs](../../src/scale_factor_override/scale_factor_override.rs.html#14)
*   [examples/app/log\_layers\_ecs.rs](../../src/log_layers_ecs/log_layers_ecs.rs.html#33)
*   [tests/window/minimizing.rs](../../src/minimizing/minimizing.rs.html#12)
*   [examples/app/logs.rs](../../src/logs/logs.rs.html#11)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#21)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#82)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#19)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#17)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#47)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#26)
*   [examples/2d/sprite\_tile.rs](../../src/sprite_tile/sprite_tile.rs.html#37)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#227)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#58)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#206)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#87)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#27)
*   [examples/app/settings.rs](../../src/settings/settings.rs.html#27)
*   [examples/ecs/one\_shot\_systems.rs](../../src/one_shot_systems/one_shot_systems.rs.html#100)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#45)
*   [examples/ecs/hotpatching\_systems.rs](../../src/hotpatching_systems/hotpatching_systems.rs.html#68)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#163)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#88)
*   [examples/window/persisting\_window\_settings.rs](../../src/persisting_window_settings/persisting_window_settings.rs.html#25)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#21)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../src/2d_text_gizmos/2d_text_gizmos.rs.html#24)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#100)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#36)
*   [examples/window/multi\_window\_text.rs](../../src/multi_window_text/multi_window_text.rs.html#19)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#158)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#30)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#56)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#16)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#28)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#42)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#27)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#231)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#28)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#29)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#59)
*   [examples/2d/transparency\_2d.rs](../../src/transparency_2d/transparency_2d.rs.html#27)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#83)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#91)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#147)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#320)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#81)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#59)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#22)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#80)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#27)
*   [examples/window/transparent\_window.rs](../../src/transparent_window/transparent_window.rs.html#23)
*   [examples/stress\_tests/many\_glyphs.rs](../../src/many_glyphs/many_glyphs.rs.html#47)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#31)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#29)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#87)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#34)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#33)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#42)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#40)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#80)
*   [examples/3d/parenting.rs](../../src/parenting/parenting.rs.html#34)
*   [examples/ecs/system\_piping.rs](../../src/system_piping/system_piping.rs.html#16)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#214)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#90)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#85)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#11)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#85)
*   [examples/ui/text/editable\_text\_filter.rs](../../src/editable_text_filter/editable_text_filter.rs.html#25)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#49)
*   [examples/ui/images/image\_node.rs](../../src/image_node/image_node.rs.html#25)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#23)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#64)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#20)
*   [examples/remote/server.rs](../../src/server/server.rs.html#56)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#89)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#173)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#70)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#26)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#132)
*   [examples/stress\_tests/many\_gradients.rs](../../src/many_gradients/many_gradients.rs.html#67)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#118)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#80)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#23)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#109)
*   [tests/ecs/ambiguity\_detection.rs](../../src/ambiguity_detection/ambiguity_detection.rs.html#85)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#122)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#26)
*   [examples/3d/two\_passes.rs](../../src/two_passes/two_passes.rs.html#35)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#95)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#126)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#140)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#29)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#116)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#253)
*   [examples/state/states.rs](../../src/states/states.rs.html#66)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#84)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#79)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#27)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#126)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#29)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#194)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#27)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#101)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#44)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#23)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#28)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#53)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#55)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#66)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#43)
*   [examples/asset/processing/asset\_processing.rs](../../src/asset_processing/asset_processing.rs.html#37)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#166)
*   [examples/dev\_tools/fps\_overlay.rs](../../src/fps_overlay/fps_overlay.rs.html#26)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#110)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#112)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#44)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#37)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#42)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#141)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#98)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#24)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#74)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#56)
*   [examples/ui/relative\_cursor\_position.rs](../../src/relative_cursor_position/relative_cursor_position.rs.html#21)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#79)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#34)
*   [examples/3d/vertex\_colors.rs](../../src/vertex_colors/vertex_colors.rs.html#48)
*   [examples/animation/animation\_events.rs](../../src/animation_events/animation_events.rs.html#61)
*   [examples/ui/text/font\_weights.rs](../../src/font_weights/font_weights.rs.html#23)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#127)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#79)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#35)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#30)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#95)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#265)
*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#88)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#141)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#155)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#38)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#151)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#214)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../src/ui_texture_atlas/ui_texture_atlas.rs.html#42)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#27)
*   [examples/ui/text/font\_variations.rs](../../src/font_variations/font_variations.rs.html#24)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#118)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#110)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#53)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#63)
*   [examples/window/window\_settings.rs](../../src/window_settings/window_settings.rs.html#37)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#155)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#45)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#95)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#165)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#55)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#80)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#60)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#73)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#57)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#78)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#74)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#66)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#61)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#37)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#24)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#99)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#87)
*   [examples/ui/text/ime\_support.rs](../../src/ime_support/ime_support.rs.html#38)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#78)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#153)
*   [examples/ecs/nondeterministic\_system\_order.rs](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#28)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#34)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#347)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#71)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#219)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#33)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#38)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#31)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#61)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#427)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#64)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#162)
*   [examples/ui/text/generic\_font\_families.rs](../../src/generic_font_families/generic_font_families.rs.html#52)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#45)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#43)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#72)
*   [examples/ui/styling/transparency\_ui.rs](../../src/transparency_ui/transparency_ui.rs.html#25)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#34)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#78)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#139)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#71)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#178)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#75)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#124)
*   [examples/2d/mesh2d\_alpha\_mode.rs](../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#37)
*   [examples/2d/mesh2d\_arcs.rs](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#46)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#347)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#87)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#125)
*   [examples/ui/text/text\_wrap\_debug.rs](../../src/text_wrap_debug/text_wrap_debug.rs.html#50)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#45)
*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#43)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#129)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#45)
*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#38)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#31)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#83)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#43)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#103)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#48)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#37)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#35)
*   [examples/ui/ui\_target\_camera.rs](../../src/ui_target_camera/ui_target_camera.rs.html#27)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#250)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#225)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#46)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#26)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#72)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#71)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#26)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#72)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#43)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#49)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#79)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#43)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#80)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../src/drag_to_scroll/drag_to_scroll.rs.html#71)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#39)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#62)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#156)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#124)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#136)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#124)
*   [examples/ui/layout/z\_index.rs](../../src/z_index/z_index.rs.html#31)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#44)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#24)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#232)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#122)
*   [examples/2d/texture\_atlas.rs](../../src/texture_atlas/texture_atlas.rs.html#106)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#121)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#49)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#46)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#154)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#42)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#41)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#65)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#43)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#55)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#49)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#39)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#50)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#172)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#113)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#27)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#37)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#89)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#41)