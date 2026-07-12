[bevy](../index.html)::[asset](index.html)

# Struct LoadBuilder 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1873)

```rust
pub struct LoadBuilder<'a> { /* private fields */ }
```

A builder for initiating a more complex load than the one provided by [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load").

For example, a load may look like:

[ⓘ](# "This example is not tested")

```rust
asset_server
    .load_builder()
    .with_settings(settings)
    .override_unapproved()
    .load("my.path")
```

## Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1885)

### impl<'a> [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1905-1908)

#### pub fn [with\_settings](#method.with_settings)<S>( self, settings: impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

where S: [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings"),

Use the given `settings` function to override the asset’s [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") settings.

The type `S` must match the configured [`AssetLoader::Settings`](trait.AssetLoader.html#associatedtype.Settings "associated type bevy::asset::AssetLoader::Settings") or `settings` changes will be ignored and an error will be printed to the log.

Repeatedly calling this method will “chain” the operations (matching the order of these calls).

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/clustered\_decal\_maps.rs ([line 55](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#55))

```rust
48    fn from_world(world: &mut World) -> Self {
49        // Load all the decal textures.
50        let asset_server = world.resource::<AssetServer>();
51        AppTextures {
52            decal_base_color_texture: asset_server.load("branding/bevy_bird_dark.png"),
53            decal_normal_map_texture: asset_server
54                .load_builder()
55                .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
56                .load(get_web_asset_url("BevyLogo-Normal.png")),
57            decal_metallic_roughness_map_texture: asset_server
58                .load_builder()
59                .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
60                .load(get_web_asset_url("BevyLogo-MetallicRoughness.png")),
61            decal_emissive_texture: asset_server.load(get_web_asset_url("BevyLogo-Emissive.png")),
62        }
63    }
64}
65
66/// A component that we place on our decals to track them for animation
67/// purposes.
68#[derive(Component)]
69struct ExampleDecal {
70    /// The width and height of the square decal in meters.
71    size: f32,
72    /// What state the decal is in (animating in, idling, or animating out).
73    state: ExampleDecalState,
74}
75
76/// The animation state of a decal.
77///
78/// When each [`Timer`] goes off, the decal advances to the next state.
79enum ExampleDecalState {
80    /// The decal has just been spawned and is animating in.
81    AnimatingIn(Timer),
82    /// The decal has animated in and is waiting to animate out.
83    Idling(Timer),
84    /// The decal is animating out.
85    ///
86    /// When this timer expires, the decal is despawned.
87    AnimatingOut(Timer),
88}
89
90/// All settings that the user can change.
91///
92/// This app only has one: whether newly-spawned decals are emissive.
93#[derive(Clone, Copy, PartialEq)]
94enum AppSetting {
95    /// True if newly-spawned decals have an emissive channel (i.e. they glow),
96    /// or false otherwise.
97    EmissiveDecals(bool),
98}
99
100/// The current values of the settings that the user can change.
101///
102/// This app only has one: whether newly-spawned decals are emissive.
103#[derive(Default, Resource)]
104struct AppStatus {
105    /// True if newly-spawned decals have an emissive channel (i.e. they glow),
106    /// or false otherwise.
107    emissive_decals: bool,
108}
109
110/// Half of the width and height of the plane onto which the decals are
111/// projected.
112const PLANE_HALF_SIZE: f32 = 2.0;
113/// The minimum width and height that a decal may have.
114///
115/// The actual size is determined randomly, using this value as a lower bound.
116const DECAL_MIN_SIZE: f32 = 0.5;
117/// The maximum width and height that a decal may have.
118///
119/// The actual size is determined randomly, using this value as an upper bound.
120const DECAL_MAX_SIZE: f32 = 1.5;
121
122/// How long it takes the decal to grow to its full size when animating in.
123const DECAL_ANIMATE_IN_DURATION: Duration = Duration::from_millis(300);
124/// How long a decal stays in the idle state before starting to animate out.
125const DECAL_IDLE_DURATION: Duration = Duration::from_secs(10);
126/// How long it takes the decal to shrink down to nothing when animating out.
127const DECAL_ANIMATE_OUT_DURATION: Duration = Duration::from_millis(300);
128
129/// The demo entry point.
130fn main() {
131    App::new()
132        .add_plugins(
133            DefaultPlugins
134                .set(WebAssetPlugin {
135                    silence_startup_warning: true,
136                })
137                .set(WindowPlugin {
138                    primary_window: Some(Window {
139                        title: "Bevy Clustered Decal Maps Example".into(),
140                        ..default()
141                    }),
142                    ..default()
143                }),
144        )
145        .add_message::<WidgetClickEvent<AppSetting>>()
146        .init_resource::<AppStatus>()
147        .init_resource::<AppTextures>()
148        .add_systems(Startup, setup)
149        .add_systems(Update, draw_gizmos)
150        .add_systems(Update, spawn_decal)
151        .add_systems(Update, animate_decals)
152        .add_systems(
153            Update,
154            (
155                widgets::handle_ui_interactions::<AppSetting>,
156                update_radio_buttons,
157            ),
158        )
159        .add_systems(
160            Update,
161            handle_emission_type_change.after(widgets::handle_ui_interactions::<AppSetting>),
162        )
163        .insert_resource(SeededRng(ChaCha8Rng::seed_from_u64(19878367467712)))
164        .run();
165}
166
167#[derive(Resource)]
168struct SeededRng(ChaCha8Rng);
169
170/// Spawns all the objects in the scene.
171fn setup(
172    mut commands: Commands,
173    asset_server: Res<AssetServer>,
174    mut meshes: ResMut<Assets<Mesh>>,
175    mut materials: ResMut<Assets<StandardMaterial>>,
176) {
177    spawn_plane_mesh(&mut commands, &asset_server, &mut meshes, &mut materials);
178    spawn_light(&mut commands);
179    spawn_camera(&mut commands);
180    spawn_buttons(&mut commands);
181}
182
183/// Spawns the plane onto which the decals are projected.
184fn spawn_plane_mesh(
185    commands: &mut Commands,
186    asset_server: &AssetServer,
187    meshes: &mut Assets<Mesh>,
188    materials: &mut Assets<StandardMaterial>,
189) {
190    // Create a plane onto which we project decals.
191    //
192    // As the plane has a normal map, we must generate tangents for the
193    // vertices.
194    let plane_mesh = meshes.add(
195        Plane3d {
196            normal: Dir3::NEG_Z,
197            half_size: Vec2::splat(PLANE_HALF_SIZE),
198        }
199        .mesh()
200        .build()
201        .with_duplicated_vertices()
202        .with_computed_flat_normals()
203        .with_generated_tangents()
204        .unwrap(),
205    );
206
207    // Give the plane some texture.
208    //
209    // Note that, as this is a normal map, we must disable sRGB when loading.
210    let normal_map_texture = asset_server
211        .load_builder()
212        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
213        .load("textures/ScratchedGold-Normal.png");
214
215    // Actually spawn the plane.
216    commands.spawn((
217        Mesh3d(plane_mesh),
218        MeshMaterial3d(materials.add(StandardMaterial {
219            base_color: Color::from(CRIMSON),
220            normal_map_texture: Some(normal_map_texture),
221            ..StandardMaterial::default()
222        })),
223        Transform::IDENTITY,
224    ));
225}
```

Hide additional examples

examples/3d/rotate\_environment\_map.rs ([lines 77-79](../../src/rotate_environment_map/rotate_environment_map.rs.html#77-79))

```rust
62fn spawn_sphere(
63    commands: &mut Commands,
64    materials: &mut Assets<StandardMaterial>,
65    asset_server: &AssetServer,
66    sphere_mesh: &Handle<Mesh>,
67) {
68    commands.spawn((
69        Mesh3d(sphere_mesh.clone()),
70        MeshMaterial3d(
71            materials.add(StandardMaterial {
72                clearcoat: 1.0,
73                clearcoat_perceptual_roughness: 0.3,
74                clearcoat_normal_texture: Some(
75                    asset_server
76                        .load_builder()
77                        .with_settings(|settings: &mut ImageLoaderSettings| {
78                            settings.is_srgb = false;
79                        })
80                        .load("textures/ScratchedGold-Normal.png"),
81                ),
82                metallic: 0.9,
83                perceptual_roughness: 0.1,
84                base_color: GOLD.into(),
85                ..default()
86            }),
87        ),
88        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.25)),
89    ));
90}
```

examples/3d/clearcoat.rs ([lines 110-112](../../src/clearcoat/clearcoat.rs.html#110-112))

```rust
94fn spawn_car_paint_sphere(
95    commands: &mut Commands,
96    materials: &mut Assets<StandardMaterial>,
97    asset_server: &AssetServer,
98    sphere: &Handle<Mesh>,
99) {
100    commands
101        .spawn((
102            Mesh3d(sphere.clone()),
103            MeshMaterial3d(
104                materials.add(StandardMaterial {
105                    clearcoat: 1.0,
106                    clearcoat_perceptual_roughness: 0.1,
107                    normal_map_texture: Some(
108                        asset_server
109                            .load_builder()
110                            .with_settings(|settings: &mut ImageLoaderSettings| {
111                                settings.is_srgb = false;
112                            })
113                            .load("textures/BlueNoise-Normal.png"),
114                    ),
115                    metallic: 0.9,
116                    perceptual_roughness: 0.5,
117                    base_color: BLUE.into(),
118                    ..default()
119                }),
120            ),
121            Transform::from_xyz(-1.0, 1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
122        ))
123        .insert(ExampleSphere);
124}
125
126/// Spawn a semitransparent object with a clearcoat layer.
127fn spawn_coated_glass_bubble_sphere(
128    commands: &mut Commands,
129    materials: &mut Assets<StandardMaterial>,
130    sphere: &Handle<Mesh>,
131) {
132    commands
133        .spawn((
134            Mesh3d(sphere.clone()),
135            MeshMaterial3d(materials.add(StandardMaterial {
136                clearcoat: 1.0,
137                clearcoat_perceptual_roughness: 0.1,
138                metallic: 0.5,
139                perceptual_roughness: 0.1,
140                base_color: Color::srgba(0.9, 0.9, 0.9, 0.3),
141                alpha_mode: AlphaMode::Blend,
142                ..default()
143            })),
144            Transform::from_xyz(-1.0, -1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
145        ))
146        .insert(ExampleSphere);
147}
148
149/// Spawns an object with both a clearcoat normal map (a scratched varnish) and
150/// a main layer normal map (the golf ball pattern).
151///
152/// This object is in glTF format, using the `KHR_materials_clearcoat`
153/// extension.
154fn spawn_golf_ball(commands: &mut Commands, asset_server: &AssetServer) {
155    commands.spawn((
156        WorldAssetRoot(
157            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/GolfBall/GolfBall.glb")),
158        ),
159        Transform::from_xyz(1.0, 1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
160        ExampleSphere,
161    ));
162}
163
164/// Spawns an object with only a clearcoat normal map (a scratch pattern) and no
165/// main layer normal map.
166fn spawn_scratched_gold_ball(
167    commands: &mut Commands,
168    materials: &mut Assets<StandardMaterial>,
169    asset_server: &AssetServer,
170    sphere: &Handle<Mesh>,
171) {
172    commands
173        .spawn((
174            Mesh3d(sphere.clone()),
175            MeshMaterial3d(
176                materials.add(StandardMaterial {
177                    clearcoat: 1.0,
178                    clearcoat_perceptual_roughness: 0.3,
179                    clearcoat_normal_texture: Some(
180                        asset_server
181                            .load_builder()
182                            .with_settings(|settings: &mut ImageLoaderSettings| {
183                                settings.is_srgb = false;
184                            })
185                            .load("textures/ScratchedGold-Normal.png"),
186                    ),
187                    metallic: 0.9,
188                    perceptual_roughness: 0.1,
189                    base_color: GOLD.into(),
190                    ..default()
191                }),
192            ),
193            Transform::from_xyz(1.0, -1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
194        ))
195        .insert(ExampleSphere);
196}
```

examples/shader/array\_texture.rs ([lines 41-45](../../src/array_texture/array_texture.rs.html#41-45))

```rust
32fn setup(
33    mut commands: Commands,
34    mut meshes: ResMut<Assets<Mesh>>,
35    mut materials: ResMut<Assets<ArrayTextureMaterial>>,
36    asset_server: Res<AssetServer>,
37) {
38    // Load the texture.
39    let array_texture = asset_server
40        .load_builder()
41        .with_settings(|settings: &mut ImageLoaderSettings| {
42            settings.array_layout = Some(ImageArrayLayout::RowCount {
43                rows: TEXTURE_COUNT,
44            });
45        })
46        .load("textures/array_texture.png");
47
48    // light
49    commands.spawn((
50        DirectionalLight::default(),
51        Transform::from_xyz(3.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
52    ));
53
54    // camera
55    commands.spawn((
56        Camera3d::default(),
57        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::new(1.5, 0.0, 0.0), Vec3::Y),
58    ));
59
60    // Spawn some cubes using the array texture.
61    let mesh_handle = meshes.add(Cuboid::default());
62    let material_handle = materials.add(ArrayTextureMaterial { array_texture });
63    for x in -5..=5 {
64        commands.spawn((
65            Mesh3d(mesh_handle.clone()),
66            MeshMaterial3d(material_handle.clone()),
67            // Pass a different mesh tag to allow selecting different layers of
68            // the array texture in the shader.
69            MeshTag(x as u32 % TEXTURE_COUNT),
70            Transform::from_xyz(x as f32 + 0.5, 0.0, 0.0),
71        ));
72    }
73}
```

examples/2d/tilemap\_chunk.rs ([lines 50-54](../../src/tilemap_chunk/tilemap_chunk.rs.html#50-54))

```rust
26fn setup(mut commands: Commands, assets: Res<AssetServer>) {
27    // We're seeding the PRNG here to make this example deterministic for testing purposes.
28    // This isn't strictly required in practical use unless you need your app to be deterministic.
29    let mut rng = ChaCha8Rng::seed_from_u64(42);
30
31    let chunk_size = UVec2::splat(64);
32    let tile_display_size = UVec2::splat(8);
33    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
34        .map(|_| rng.random_range(0..5))
35        .map(|i| {
36            if i == 0 {
37                None
38            } else {
39                Some(TileData::from_tileset_index(i - 1))
40            }
41        })
42        .collect();
43
44    commands.spawn((
45        TilemapChunk {
46            chunk_size,
47            tile_display_size,
48            tileset: assets
49                .load_builder()
50                .with_settings(|settings: &mut ImageLoaderSettings| {
51                    // The tileset texture is expected to be an array of tile textures, so we tell the
52                    // `ImageLoader` that our texture is composed of 4 stacked tile images.
53                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
54                })
55                .load("textures/array_texture.png"),
56            ..default()
57        },
58        TilemapChunkTileData(tile_data),
59        UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
60    ));
61
62    commands.spawn(Camera2d);
63
64    commands.insert_resource(SeededRng(rng));
65}
```

examples/testbed/3d.rs ([lines 468-473](../../src/testbed_3d/3d.rs.html#468-473))

```rust
429    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
430        commands.spawn((
431            Camera3d::default(),
432            Transform::from_xyz(-4.0, 4.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
433            DespawnOnExit(CURRENT_SCENE),
434        ));
435
436        commands.spawn((
437            DirectionalLight {
438                color: BLUE.into(),
439                ..default()
440            },
441            Transform::IDENTITY.looking_to(Dir3::Z, Dir3::Y),
442            DespawnOnExit(CURRENT_SCENE),
443        ));
444
445        commands.spawn((
446            DirectionalLight {
447                color: RED.into(),
448                ..default()
449            },
450            Transform::IDENTITY.looking_to(Dir3::X, Dir3::Y),
451            DespawnOnExit(CURRENT_SCENE),
452        ));
453
454        commands.spawn((
455            DirectionalLight {
456                color: GREEN.into(),
457                ..default()
458            },
459            Transform::IDENTITY.looking_to(Dir3::NEG_Y, Dir3::X),
460            DespawnOnExit(CURRENT_SCENE),
461        ));
462
463        commands
464            .spawn((
465                WorldAssetRoot(
466                    asset_server
467                        .load_builder()
468                        .with_settings(|s: &mut GltfLoaderSettings| {
469                            s.convert_coordinates = Some(GltfConvertCoordinates {
470                                rotate_scene_entity: true,
471                                rotate_meshes: true,
472                            });
473                        })
474                        .load(GltfAssetLabel::Scene(0).from_asset("models/Faces/faces.glb")),
475                ),
476                DespawnOnExit(CURRENT_SCENE),
477            ))
478            .observe(show_aabbs);
479    }
```

Additional examples can be found in:  

*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#223-227)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#55-71)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#278-287)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#382-391)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#66-70)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#20-23)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#94-107)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#44-54)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#109-111)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#67-69)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#37-47)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#60-76)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#211-215)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#248-253)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1925)

#### pub fn [override\_unapproved](#method.override_unapproved)(self) -> [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

Loads from unapproved paths are allowed, even if [`AssetPlugin::unapproved_path_mode`](../prelude/struct.AssetPlugin.html#structfield.unapproved_path_mode "field bevy::prelude::AssetPlugin::unapproved_path_mode") is [`Deny`](enum.UnapprovedPathMode.html#variant.Deny "variant bevy::asset::UnapprovedPathMode::Deny").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1938)

#### pub fn [with\_guard](#method.with_guard)(self, guard: impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static) -> [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

Sets the guard item that is held during the load.

The guard item is dropped when either the asset is loaded or loading has failed. This allows the [`Drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop") implementation of the guard item to notify the caller in some way. See the `multi_asset_sync` example for usage.

Only the last guard is kept. The previous guards are dropped before the load begins.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/asset/multi\_asset\_sync.rs ([line 147](../../src/multi_asset_sync/multi_asset_sync.rs.html#147))

```rust
144fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
145    let (barrier, guard) = AssetBarrier::new();
146    commands.insert_resource(OneHundredThings(std::array::from_fn(|i| {
147        let builder = asset_server.load_builder().with_guard(guard.clone());
148        match i % 5 {
149            0 => builder.load("models/GolfBall/GolfBall.glb"),
150            1 => builder.load("models/AlienCake/alien.glb"),
151            2 => builder.load("models/AlienCake/cakeBirthday.glb"),
152            3 => builder.load("models/FlightHelmet/FlightHelmet.gltf"),
153            4 => builder.load("models/torus/torus.gltf"),
154            _ => unreachable!(),
155        }
156    })));
157    let future = barrier.wait_async();
158    commands.insert_resource(barrier);
159
160    let loading_state = Arc::new(AtomicBool::new(false));
161    commands.insert_resource(AsyncLoadingState(loading_state.clone()));
162
163    // await the `AssetBarrierFuture`.
164    AsyncComputeTaskPool::get()
165        .spawn(async move {
166            future.await;
167            // Notify via `AsyncLoadingState`
168            loading_state.store(true, Ordering::Release);
169        })
170        .detach();
171}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1958)

#### pub fn [load](#method.load)<'b, A>(self, asset\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'b>>) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Begins loading an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") of type `A` stored at `path`. This will not block on the asset load. Instead, it returns a “strong” [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle"). When the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") is loaded (and enters [`LoadState::Loaded`](enum.LoadState.html#variant.Loaded "variant bevy::asset::LoadState::Loaded")), it will be added to the associated [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") resource.

Note that if the asset at this path is already loaded, this function will return the existing handle, and will not waste work spawning a new load task.

This matches the behavior of [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load"), but supporting all other features of the builder. See its docs for more details.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/3d/clustered\_decal\_maps.rs ([line 56](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#56))

```rust
48    fn from_world(world: &mut World) -> Self {
49        // Load all the decal textures.
50        let asset_server = world.resource::<AssetServer>();
51        AppTextures {
52            decal_base_color_texture: asset_server.load("branding/bevy_bird_dark.png"),
53            decal_normal_map_texture: asset_server
54                .load_builder()
55                .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
56                .load(get_web_asset_url("BevyLogo-Normal.png")),
57            decal_metallic_roughness_map_texture: asset_server
58                .load_builder()
59                .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
60                .load(get_web_asset_url("BevyLogo-MetallicRoughness.png")),
61            decal_emissive_texture: asset_server.load(get_web_asset_url("BevyLogo-Emissive.png")),
62        }
63    }
64}
65
66/// A component that we place on our decals to track them for animation
67/// purposes.
68#[derive(Component)]
69struct ExampleDecal {
70    /// The width and height of the square decal in meters.
71    size: f32,
72    /// What state the decal is in (animating in, idling, or animating out).
73    state: ExampleDecalState,
74}
75
76/// The animation state of a decal.
77///
78/// When each [`Timer`] goes off, the decal advances to the next state.
79enum ExampleDecalState {
80    /// The decal has just been spawned and is animating in.
81    AnimatingIn(Timer),
82    /// The decal has animated in and is waiting to animate out.
83    Idling(Timer),
84    /// The decal is animating out.
85    ///
86    /// When this timer expires, the decal is despawned.
87    AnimatingOut(Timer),
88}
89
90/// All settings that the user can change.
91///
92/// This app only has one: whether newly-spawned decals are emissive.
93#[derive(Clone, Copy, PartialEq)]
94enum AppSetting {
95    /// True if newly-spawned decals have an emissive channel (i.e. they glow),
96    /// or false otherwise.
97    EmissiveDecals(bool),
98}
99
100/// The current values of the settings that the user can change.
101///
102/// This app only has one: whether newly-spawned decals are emissive.
103#[derive(Default, Resource)]
104struct AppStatus {
105    /// True if newly-spawned decals have an emissive channel (i.e. they glow),
106    /// or false otherwise.
107    emissive_decals: bool,
108}
109
110/// Half of the width and height of the plane onto which the decals are
111/// projected.
112const PLANE_HALF_SIZE: f32 = 2.0;
113/// The minimum width and height that a decal may have.
114///
115/// The actual size is determined randomly, using this value as a lower bound.
116const DECAL_MIN_SIZE: f32 = 0.5;
117/// The maximum width and height that a decal may have.
118///
119/// The actual size is determined randomly, using this value as an upper bound.
120const DECAL_MAX_SIZE: f32 = 1.5;
121
122/// How long it takes the decal to grow to its full size when animating in.
123const DECAL_ANIMATE_IN_DURATION: Duration = Duration::from_millis(300);
124/// How long a decal stays in the idle state before starting to animate out.
125const DECAL_IDLE_DURATION: Duration = Duration::from_secs(10);
126/// How long it takes the decal to shrink down to nothing when animating out.
127const DECAL_ANIMATE_OUT_DURATION: Duration = Duration::from_millis(300);
128
129/// The demo entry point.
130fn main() {
131    App::new()
132        .add_plugins(
133            DefaultPlugins
134                .set(WebAssetPlugin {
135                    silence_startup_warning: true,
136                })
137                .set(WindowPlugin {
138                    primary_window: Some(Window {
139                        title: "Bevy Clustered Decal Maps Example".into(),
140                        ..default()
141                    }),
142                    ..default()
143                }),
144        )
145        .add_message::<WidgetClickEvent<AppSetting>>()
146        .init_resource::<AppStatus>()
147        .init_resource::<AppTextures>()
148        .add_systems(Startup, setup)
149        .add_systems(Update, draw_gizmos)
150        .add_systems(Update, spawn_decal)
151        .add_systems(Update, animate_decals)
152        .add_systems(
153            Update,
154            (
155                widgets::handle_ui_interactions::<AppSetting>,
156                update_radio_buttons,
157            ),
158        )
159        .add_systems(
160            Update,
161            handle_emission_type_change.after(widgets::handle_ui_interactions::<AppSetting>),
162        )
163        .insert_resource(SeededRng(ChaCha8Rng::seed_from_u64(19878367467712)))
164        .run();
165}
166
167#[derive(Resource)]
168struct SeededRng(ChaCha8Rng);
169
170/// Spawns all the objects in the scene.
171fn setup(
172    mut commands: Commands,
173    asset_server: Res<AssetServer>,
174    mut meshes: ResMut<Assets<Mesh>>,
175    mut materials: ResMut<Assets<StandardMaterial>>,
176) {
177    spawn_plane_mesh(&mut commands, &asset_server, &mut meshes, &mut materials);
178    spawn_light(&mut commands);
179    spawn_camera(&mut commands);
180    spawn_buttons(&mut commands);
181}
182
183/// Spawns the plane onto which the decals are projected.
184fn spawn_plane_mesh(
185    commands: &mut Commands,
186    asset_server: &AssetServer,
187    meshes: &mut Assets<Mesh>,
188    materials: &mut Assets<StandardMaterial>,
189) {
190    // Create a plane onto which we project decals.
191    //
192    // As the plane has a normal map, we must generate tangents for the
193    // vertices.
194    let plane_mesh = meshes.add(
195        Plane3d {
196            normal: Dir3::NEG_Z,
197            half_size: Vec2::splat(PLANE_HALF_SIZE),
198        }
199        .mesh()
200        .build()
201        .with_duplicated_vertices()
202        .with_computed_flat_normals()
203        .with_generated_tangents()
204        .unwrap(),
205    );
206
207    // Give the plane some texture.
208    //
209    // Note that, as this is a normal map, we must disable sRGB when loading.
210    let normal_map_texture = asset_server
211        .load_builder()
212        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
213        .load("textures/ScratchedGold-Normal.png");
214
215    // Actually spawn the plane.
216    commands.spawn((
217        Mesh3d(plane_mesh),
218        MeshMaterial3d(materials.add(StandardMaterial {
219            base_color: Color::from(CRIMSON),
220            normal_map_texture: Some(normal_map_texture),
221            ..StandardMaterial::default()
222        })),
223        Transform::IDENTITY,
224    ));
225}
```

Hide additional examples

examples/3d/rotate\_environment\_map.rs ([line 80](../../src/rotate_environment_map/rotate_environment_map.rs.html#80))

```rust
62fn spawn_sphere(
63    commands: &mut Commands,
64    materials: &mut Assets<StandardMaterial>,
65    asset_server: &AssetServer,
66    sphere_mesh: &Handle<Mesh>,
67) {
68    commands.spawn((
69        Mesh3d(sphere_mesh.clone()),
70        MeshMaterial3d(
71            materials.add(StandardMaterial {
72                clearcoat: 1.0,
73                clearcoat_perceptual_roughness: 0.3,
74                clearcoat_normal_texture: Some(
75                    asset_server
76                        .load_builder()
77                        .with_settings(|settings: &mut ImageLoaderSettings| {
78                            settings.is_srgb = false;
79                        })
80                        .load("textures/ScratchedGold-Normal.png"),
81                ),
82                metallic: 0.9,
83                perceptual_roughness: 0.1,
84                base_color: GOLD.into(),
85                ..default()
86            }),
87        ),
88        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.25)),
89    ));
90}
```

examples/3d/clearcoat.rs ([line 113](../../src/clearcoat/clearcoat.rs.html#113))

```rust
94fn spawn_car_paint_sphere(
95    commands: &mut Commands,
96    materials: &mut Assets<StandardMaterial>,
97    asset_server: &AssetServer,
98    sphere: &Handle<Mesh>,
99) {
100    commands
101        .spawn((
102            Mesh3d(sphere.clone()),
103            MeshMaterial3d(
104                materials.add(StandardMaterial {
105                    clearcoat: 1.0,
106                    clearcoat_perceptual_roughness: 0.1,
107                    normal_map_texture: Some(
108                        asset_server
109                            .load_builder()
110                            .with_settings(|settings: &mut ImageLoaderSettings| {
111                                settings.is_srgb = false;
112                            })
113                            .load("textures/BlueNoise-Normal.png"),
114                    ),
115                    metallic: 0.9,
116                    perceptual_roughness: 0.5,
117                    base_color: BLUE.into(),
118                    ..default()
119                }),
120            ),
121            Transform::from_xyz(-1.0, 1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
122        ))
123        .insert(ExampleSphere);
124}
125
126/// Spawn a semitransparent object with a clearcoat layer.
127fn spawn_coated_glass_bubble_sphere(
128    commands: &mut Commands,
129    materials: &mut Assets<StandardMaterial>,
130    sphere: &Handle<Mesh>,
131) {
132    commands
133        .spawn((
134            Mesh3d(sphere.clone()),
135            MeshMaterial3d(materials.add(StandardMaterial {
136                clearcoat: 1.0,
137                clearcoat_perceptual_roughness: 0.1,
138                metallic: 0.5,
139                perceptual_roughness: 0.1,
140                base_color: Color::srgba(0.9, 0.9, 0.9, 0.3),
141                alpha_mode: AlphaMode::Blend,
142                ..default()
143            })),
144            Transform::from_xyz(-1.0, -1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
145        ))
146        .insert(ExampleSphere);
147}
148
149/// Spawns an object with both a clearcoat normal map (a scratched varnish) and
150/// a main layer normal map (the golf ball pattern).
151///
152/// This object is in glTF format, using the `KHR_materials_clearcoat`
153/// extension.
154fn spawn_golf_ball(commands: &mut Commands, asset_server: &AssetServer) {
155    commands.spawn((
156        WorldAssetRoot(
157            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/GolfBall/GolfBall.glb")),
158        ),
159        Transform::from_xyz(1.0, 1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
160        ExampleSphere,
161    ));
162}
163
164/// Spawns an object with only a clearcoat normal map (a scratch pattern) and no
165/// main layer normal map.
166fn spawn_scratched_gold_ball(
167    commands: &mut Commands,
168    materials: &mut Assets<StandardMaterial>,
169    asset_server: &AssetServer,
170    sphere: &Handle<Mesh>,
171) {
172    commands
173        .spawn((
174            Mesh3d(sphere.clone()),
175            MeshMaterial3d(
176                materials.add(StandardMaterial {
177                    clearcoat: 1.0,
178                    clearcoat_perceptual_roughness: 0.3,
179                    clearcoat_normal_texture: Some(
180                        asset_server
181                            .load_builder()
182                            .with_settings(|settings: &mut ImageLoaderSettings| {
183                                settings.is_srgb = false;
184                            })
185                            .load("textures/ScratchedGold-Normal.png"),
186                    ),
187                    metallic: 0.9,
188                    perceptual_roughness: 0.1,
189                    base_color: GOLD.into(),
190                    ..default()
191                }),
192            ),
193            Transform::from_xyz(1.0, -1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
194        ))
195        .insert(ExampleSphere);
196}
```

examples/asset/multi\_asset\_sync.rs ([line 149](../../src/multi_asset_sync/multi_asset_sync.rs.html#149))

```rust
144fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
145    let (barrier, guard) = AssetBarrier::new();
146    commands.insert_resource(OneHundredThings(std::array::from_fn(|i| {
147        let builder = asset_server.load_builder().with_guard(guard.clone());
148        match i % 5 {
149            0 => builder.load("models/GolfBall/GolfBall.glb"),
150            1 => builder.load("models/AlienCake/alien.glb"),
151            2 => builder.load("models/AlienCake/cakeBirthday.glb"),
152            3 => builder.load("models/FlightHelmet/FlightHelmet.gltf"),
153            4 => builder.load("models/torus/torus.gltf"),
154            _ => unreachable!(),
155        }
156    })));
157    let future = barrier.wait_async();
158    commands.insert_resource(barrier);
159
160    let loading_state = Arc::new(AtomicBool::new(false));
161    commands.insert_resource(AsyncLoadingState(loading_state.clone()));
162
163    // await the `AssetBarrierFuture`.
164    AsyncComputeTaskPool::get()
165        .spawn(async move {
166            future.await;
167            // Notify via `AsyncLoadingState`
168            loading_state.store(true, Ordering::Release);
169        })
170        .detach();
171}
```

examples/shader/array\_texture.rs ([line 46](../../src/array_texture/array_texture.rs.html#46))

```rust
32fn setup(
33    mut commands: Commands,
34    mut meshes: ResMut<Assets<Mesh>>,
35    mut materials: ResMut<Assets<ArrayTextureMaterial>>,
36    asset_server: Res<AssetServer>,
37) {
38    // Load the texture.
39    let array_texture = asset_server
40        .load_builder()
41        .with_settings(|settings: &mut ImageLoaderSettings| {
42            settings.array_layout = Some(ImageArrayLayout::RowCount {
43                rows: TEXTURE_COUNT,
44            });
45        })
46        .load("textures/array_texture.png");
47
48    // light
49    commands.spawn((
50        DirectionalLight::default(),
51        Transform::from_xyz(3.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
52    ));
53
54    // camera
55    commands.spawn((
56        Camera3d::default(),
57        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::new(1.5, 0.0, 0.0), Vec3::Y),
58    ));
59
60    // Spawn some cubes using the array texture.
61    let mesh_handle = meshes.add(Cuboid::default());
62    let material_handle = materials.add(ArrayTextureMaterial { array_texture });
63    for x in -5..=5 {
64        commands.spawn((
65            Mesh3d(mesh_handle.clone()),
66            MeshMaterial3d(material_handle.clone()),
67            // Pass a different mesh tag to allow selecting different layers of
68            // the array texture in the shader.
69            MeshTag(x as u32 % TEXTURE_COUNT),
70            Transform::from_xyz(x as f32 + 0.5, 0.0, 0.0),
71        ));
72    }
73}
```

examples/2d/tilemap\_chunk.rs ([line 55](../../src/tilemap_chunk/tilemap_chunk.rs.html#55))

```rust
26fn setup(mut commands: Commands, assets: Res<AssetServer>) {
27    // We're seeding the PRNG here to make this example deterministic for testing purposes.
28    // This isn't strictly required in practical use unless you need your app to be deterministic.
29    let mut rng = ChaCha8Rng::seed_from_u64(42);
30
31    let chunk_size = UVec2::splat(64);
32    let tile_display_size = UVec2::splat(8);
33    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
34        .map(|_| rng.random_range(0..5))
35        .map(|i| {
36            if i == 0 {
37                None
38            } else {
39                Some(TileData::from_tileset_index(i - 1))
40            }
41        })
42        .collect();
43
44    commands.spawn((
45        TilemapChunk {
46            chunk_size,
47            tile_display_size,
48            tileset: assets
49                .load_builder()
50                .with_settings(|settings: &mut ImageLoaderSettings| {
51                    // The tileset texture is expected to be an array of tile textures, so we tell the
52                    // `ImageLoader` that our texture is composed of 4 stacked tile images.
53                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
54                })
55                .load("textures/array_texture.png"),
56            ..default()
57        },
58        TilemapChunkTileData(tile_data),
59        UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
60    ));
61
62    commands.spawn(Camera2d);
63
64    commands.insert_resource(SeededRng(rng));
65}
```

Additional examples can be found in:  

*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#474)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#228)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#72)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#288)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#392)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#71)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#24)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#108)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#55)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#112)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#70)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#48)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#77-89)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#216)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#254)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1966-1970)

#### pub fn [load\_erased](#method.load_erased)<'b>( self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), asset\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'b>>, ) -> [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

Same as [`load`](struct.LoadBuilder.html#method.load "method bevy::asset::LoadBuilder::load"), but the type of the asset to load is specified by the runtime `type_id`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1998-2001)

#### pub fn [load\_untyped](#method.load_untyped)<'b>( self, asset\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'b>>, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[LoadedUntypedAsset](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset")\>

Load an asset without knowing its type. The method returns a handle to a [`LoadedUntypedAsset`](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset").

Once the [`LoadedUntypedAsset`](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset") is loaded, an untyped handle for the requested path can be retrieved from it.

```rust
use bevy_asset::{Assets, Handle, LoadedUntypedAsset};
use bevy_ecs::system::Res;
use bevy_ecs::resource::Resource;

#[derive(Resource)]
struct LoadingUntypedHandle(Handle<LoadedUntypedAsset>);

fn resolve_loaded_untyped_handle(loading_handle: Res<LoadingUntypedHandle>, loaded_untyped_assets: Res<Assets<LoadedUntypedAsset>>) {
    if let Some(loaded_untyped_asset) = loaded_untyped_assets.get(&loading_handle.0) {
        let handle = loaded_untyped_asset.handle.clone();
        // continue working with `handle` which points to the asset at the originally requested path
    }
}
```

This indirection enables a non blocking load of an untyped asset, since I/O is required to figure out the asset type before a handle can be created.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#2019-2022)

#### pub async fn [load\_untyped\_async](#method.load_untyped_async)<'b>( self, asset\_path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'b>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle"), [AssetLoadError](enum.AssetLoadError.html "enum bevy::asset::AssetLoadError")\>

Asynchronously load an asset that you do not know the type of statically. If you _do_ know the type of the asset, you should use [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load"). If you don’t know the type of the asset, but you can’t use an async method, consider using [`AssetServer::load_untyped`](../prelude/struct.AssetServer.html#method.load_untyped "method bevy::prelude::AssetServer::load_untyped").

## Auto Trait Implementations

### impl<'a> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

### impl<'a> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

### impl<'a> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

### impl<'a> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

### impl<'a> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

### impl<'a> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

### impl<'a> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'a>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

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

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}