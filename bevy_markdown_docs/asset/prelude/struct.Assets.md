[bevy](../../index.html)::[asset](../index.html)::[prelude](index.html)

# Struct Assets 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#288)

```rust
pub struct Assets<A>where
    A: Asset,{ /* private fields */ }
```

Stores [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") values identified by their [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

Assets identified by [`AssetId::Index`](../../prelude/enum.AssetId.html#variant.Index "variant bevy::prelude::AssetId::Index") will be stored in a “dense” vec-like storage. This is more efficient, but it means that the assets can only be identified at runtime. This is the default behavior.

Assets identified by [`AssetId::Uuid`](../../prelude/enum.AssetId.html#variant.Uuid "variant bevy::prelude::AssetId::Uuid") will be stored in a hashmap. This is less efficient, but it means that the assets can be referenced at compile time.

This tracks (and queues) [`AssetEvent`](../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent") events whenever changes to the collection occur. To check whether the asset used by a given component has changed (due to a change in the handle or the underlying asset) use the [`AssetChanged`](../../prelude/struct.AssetChanged.html "struct bevy::prelude::AssetChanged") query filter.

## Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#313)

### impl<A> [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#316)

#### pub fn [get\_handle\_provider](#method.get_handle_provider)(&self) -> [AssetHandleProvider](../struct.AssetHandleProvider.html "struct bevy::asset::AssetHandleProvider")

Retrieves an [`AssetHandleProvider`](../struct.AssetHandleProvider.html "struct bevy::asset::AssetHandleProvider") capable of reserving new [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") values for assets that will be stored in this collection.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#321)

#### pub fn [reserve\_handle](#method.reserve_handle)(&self) -> [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

Reserves a new [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") for an asset that will be stored in this collection.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/asset/generated\_assets.rs ([line 41](../../../src/generated_assets/generated_assets.rs.html#41))

```rust
13fn setup(
14    mut commands: Commands,
15    asset_server: Res<AssetServer>,
16    mut materials: ResMut<Assets<StandardMaterial>>,
17    meshes: Res<Assets<Mesh>>,
18) {
19    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 5.0)));
20
21    commands.spawn((
22        DirectionalLight::default(),
23        Transform::default().looking_to(Dir3::new(Vec3::new(-1.0, -1.0, -1.0)).unwrap(), Dir3::Y),
24    ));
25
26    // The simplest way to generate an asset is to add it directly to the `Assets`.
27    let material_handle = materials.add(StandardMaterial::default());
28
29    commands.spawn((
30        Transform::from_xyz(-2.0, 0.0, 0.0),
31        MeshMaterial3d(material_handle.clone()),
32        // Alternatively, `add_async` creates a task that runs your async function. Once it
33        // completes, the asset is added to the `Assets`. This is "deferred" meaning that the asset
34        // may take a frame to be added after the task completes.
35        Mesh3d(asset_server.add_async(generate_mesh_async())),
36    ));
37
38    // The last way to generate assets is to reserve a handle, and then use `Assets::insert` to
39    // populate the asset later. In this example, the `generate_mesh_system` system runs to populate
40    // the mesh.
41    let mesh_handle = meshes.reserve_handle();
42    commands.insert_resource(HandleToGenerate(mesh_handle.clone()));
43    commands.spawn((
44        Transform::from_xyz(2.0, 0.0, 0.0)
45            .with_rotation(Quat::from_rotation_x(50.0f32.to_radians())),
46        Mesh3d(mesh_handle),
47        MeshMaterial3d(material_handle),
48    ));
49}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#329-333)

#### pub fn [insert](#method.insert)( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>, asset: A, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [InvalidGenerationError](../enum.InvalidGenerationError.html "enum bevy::asset::InvalidGenerationError")\>

Inserts the given `asset`, identified by the given `id`. If an asset already exists for `id`, it will be replaced.

Note: This will never return an error for UUID asset IDs.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/asset/generated\_assets.rs ([line 70](../../../src/generated_assets/generated_assets.rs.html#70))

```rust
65fn generate_mesh_system(
66    handle_to_generate: Res<HandleToGenerate>,
67    mut meshes: ResMut<Assets<Mesh>>,
68) {
69    let mesh = Mesh::from(Torus::new(0.8, 1.2));
70    meshes.insert(&handle_to_generate.0, mesh).unwrap();
71}
```

Hide additional examples

examples/3d/tonemapping.rs ([line 259](../../../src/tonemapping/tonemapping.rs.html#259))

```rust
227fn resize_image(
228    image_mesh: Query<(&MeshMaterial3d<StandardMaterial>, &Mesh3d), With<HDRViewer>>,
229    materials: Res<Assets<StandardMaterial>>,
230    mut meshes: ResMut<Assets<Mesh>>,
231    images: Res<Assets<Image>>,
232    mut image_event_reader: MessageReader<AssetEvent<Image>>,
233) {
234    for event in image_event_reader.read() {
235        let (AssetEvent::Added { id } | AssetEvent::Modified { id }) = event else {
236            continue;
237        };
238
239        for (mat_h, mesh_h) in &image_mesh {
240            let Some(mat) = materials.get(mat_h) else {
241                continue;
242            };
243
244            let Some(ref base_color_texture) = mat.base_color_texture else {
245                continue;
246            };
247
248            if *id != base_color_texture.id() {
249                continue;
250            };
251
252            let Some(image_changed) = images.get(*id) else {
253                continue;
254            };
255
256            let size = image_changed.size_f32().normalize_or_zero() * 1.4;
257            // Resize Mesh
258            let quad = Mesh::from(Rectangle::from_size(size));
259            meshes.insert(mesh_h, quad).unwrap();
260        }
261    }
262}
```

examples/asset/asset\_saving.rs ([lines 127-141](../../../src/asset_saving/asset_saving.rs.html#127-141))

```rust
85fn setup(
86    mut commands: Commands,
87    asset_server: Res<AssetServer>,
88    mut images: ResMut<Assets<Image>>,
89) {
90    commands.spawn((
91        Camera2d,
92        Projection::Orthographic(OrthographicProjection {
93            scaling_mode: ScalingMode::FixedVertical {
94                viewport_height: 125.0,
95            },
96            ..OrthographicProjection::default_2d()
97        }),
98    ));
99
100    commands.spawn(Text(
101        r"Select a color from the palette at the bottom
102LMB - Draw with selected color
103F5 - Save image"
104            .into(),
105    ));
106
107    let handle = asset_server
108        .load_builder()
109        .with_settings(|settings: &mut ImageLoaderSettings| {
110            settings.sampler = ImageSampler::nearest();
111        })
112        .load(ASSET_PATH);
113    commands.spawn((
114        Sprite {
115            image: handle.clone(),
116            ..Default::default()
117        },
118        SpriteToSave,
119        Pickable::default(),
120    ));
121
122    // We're doing something a little cursed here: we initiate a load, and then insert a default
123    // image into that handle. If the load succeeds, the image will be replaced with the loaded
124    // contents. If it fails, the default image will remain. In real code, you likely want to poll
125    // `AssetServer::load_state` and only insert this on load failure.
126    images
127        .insert(&handle, {
128            let mut image = Image::new_fill(
129                Extent3d {
130                    width: 100,
131                    height: 100,
132                    depth_or_array_layers: 1,
133                },
134                TextureDimension::D2,
135                &[0, 0, 0, 255],
136                TextureFormat::Rgba8Unorm,
137                RenderAssetUsages::all(),
138            );
139            image.sampler = ImageSampler::nearest();
140            image
141        })
142        .unwrap();
143
144    commands.insert_resource(ImageToSave(handle));
145
146    let container = commands
147        .spawn((
148            Node {
149                width: percent(100),
150                height: percent(100),
151                align_items: AlignItems::End,
152                justify_content: JustifyContent::Center,
153                ..Default::default()
154            },
155            Pickable::IGNORE,
156        ))
157        .id();
158
159    for color in [
160        Color::WHITE,
161        Color::Srgba(tailwind::RED_500),
162        Color::Srgba(tailwind::ORANGE_500),
163        Color::Srgba(tailwind::YELLOW_500),
164        Color::Srgba(tailwind::GREEN_500),
165        Color::Srgba(tailwind::BLUE_500),
166        Color::Srgba(tailwind::INDIGO_500),
167        Color::Srgba(tailwind::VIOLET_500),
168        Color::BLACK,
169    ] {
170        let mut entity = commands.spawn((
171            Node {
172                width: vw(5),
173                height: vh(5),
174                border: px(5).all(),
175                ..Default::default()
176            },
177            SelectableColor,
178            BackgroundColor(color),
179            BorderColor::all(NORMAL_COLOR),
180            ChildOf(container),
181        ));
182        if color == Color::WHITE {
183            entity.insert((Selected, BorderColor::all(SELECTED_COLOR)));
184        }
185    }
186}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#348-352)

#### pub fn [get\_or\_insert\_with](#method.get_or_insert_with)( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>, insert\_fn: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> A, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[AssetMut](../struct.AssetMut.html "struct bevy::asset::AssetMut")<'\_, A>, [InvalidGenerationError](../enum.InvalidGenerationError.html "enum bevy::asset::InvalidGenerationError")\>

Retrieves an [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") stored for the given `id` if it exists. If it does not exist, it will be inserted using `insert_fn`.

Note: This will never return an error for UUID asset IDs.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#365)

#### pub fn [contains](#method.contains)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the `id` exists in this collection. Otherwise it returns `false`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#401)

#### pub fn [add](#method.add)(&mut self, asset: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<A>) -> [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

Adds the given `asset` and allocates a new strong [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") for it.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/3d/light\_probe\_blending.rs ([lines 237-243](../../../src/light_probe_blending/light_probe_blending.rs.html#237-243))

```rust
234fn create_reflective_material(
235    materials: &mut Assets<StandardMaterial>,
236) -> Handle<StandardMaterial> {
237    materials.add(StandardMaterial {
238        base_color: WHITE.into(),
239        metallic: 1.0,
240        reflectance: 1.0,
241        perceptual_roughness: 0.0,
242        ..default()
243    })
244}
245
246/// Spawns the orbital pan/zoom camera.
247fn spawn_camera(commands: &mut Commands) {
248    commands.spawn((
249        Camera3d::default(),
250        Transform::IDENTITY,
251        Hdr,
252        OrbitCamera {
253            radius: 3.0,
254            inclination: 7.0 * FRAC_PI_4,
255            azimuth: FRAC_PI_4,
256        },
257    ));
258}
259
260/// Spawns the glTF scene that contains the two rooms.
261fn spawn_gltf_scene(commands: &mut Commands, asset_server: &AssetServer) {
262    commands.spawn(WorldAssetRoot(asset_server.load(
263        GltfAssetLabel::Scene(0).from_asset(get_web_asset_url("two_rooms.glb")),
264    )));
265}
266
267/// Spawns the reflective sphere, creating its mesh in the process.
268fn spawn_reflective_sphere(
269    commands: &mut Commands,
270    meshes: &mut Assets<Mesh>,
271    material: Handle<StandardMaterial>,
272) {
273    // Create a mesh.
274    let sphere = meshes.add(Sphere::default().mesh().uv(32, 18));
275
276    // Spawn the sphere.
277    commands.spawn((
278        Mesh3d(sphere),
279        MeshMaterial3d(material),
280        Transform::IDENTITY,
281        ReflectiveSphere,
282    ));
283}
284
285/// Spawns the reflective prism, creating its mesh in the process.
286///
287/// The reflective prism starts invisible, but the user can toggle it on and off
288/// as desired.
289fn spawn_reflective_prism(
290    commands: &mut Commands,
291    meshes: &mut Assets<Mesh>,
292    material: Handle<StandardMaterial>,
293) {
294    // Create a mesh.
295    let cube = meshes.add(
296        Cuboid {
297            half_size: vec3(2.0, 1.0, 10.0),
298        }
299        .mesh()
300        .build()
301        // We use flat normals so that the surface appears flat, not curved.
302        .with_duplicated_vertices()
303        .with_computed_flat_normals(),
304    );
305
306    // Spawn the cube.
307    commands.spawn((
308        Mesh3d(cube),
309        MeshMaterial3d(material),
310        Transform::from_xyz(0.0, -4.0, -5.5),
311        ReflectivePrism,
312        Visibility::Hidden,
313    ));
314}
```

Hide additional examples

examples/audio/decodable.rs ([lines 97-99](../../../src/decodable/decodable.rs.html#97-99))

```rust
95fn setup(mut assets: ResMut<Assets<SineAudio>>, mut commands: Commands) {
96    // add a `SineAudio` to the asset server so that it can be played
97    let audio_handle = assets.add(SineAudio {
98        frequency: 440., // this is the frequency of A4
99    });
100    commands.spawn(AudioPlayer(audio_handle));
101}
```

examples/2d/mesh2d.rs ([line 20](../../../src/mesh2d/mesh2d.rs.html#20))

```rust
12fn setup(
13    mut commands: Commands,
14    mut meshes: ResMut<Assets<Mesh>>,
15    mut materials: ResMut<Assets<ColorMaterial>>,
16) {
17    commands.spawn(Camera2d);
18
19    commands.spawn((
20        Mesh2d(meshes.add(Rectangle::default())),
21        MeshMaterial2d(materials.add(Color::from(PURPLE))),
22        Transform::default().with_scale(Vec3::splat(128.)),
23    ));
24}
```

examples/3d/clearcoat.rs ([line 90](../../../src/clearcoat/clearcoat.rs.html#90))

```rust
82fn create_sphere_mesh(meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
83    // We're going to use normal maps, so make sure we've generated tangents, or
84    // else the normal maps won't show up.
85
86    let mut sphere_mesh = Sphere::new(1.0).mesh().build();
87    sphere_mesh
88        .generate_tangents()
89        .expect("Failed to generate tangents");
90    meshes.add(sphere_mesh)
91}
92
93/// Spawn a regular object with a clearcoat layer. This looks like car paint.
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

examples/3d/rotate\_environment\_map.rs ([line 58](../../../src/rotate_environment_map/rotate_environment_map.rs.html#58))

```rust
50fn create_sphere_mesh(meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
51    // We're going to use normal maps, so make sure we've generated tangents, or
52    // else the normal maps won't show up.
53
54    let mut sphere_mesh = Sphere::new(1.0).mesh().build();
55    sphere_mesh
56        .generate_tangents()
57        .expect("Failed to generate tangents");
58    meshes.add(sphere_mesh)
59}
60
61/// Spawn a regular object with a clearcoat layer. This looks like car paint.
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

examples/2d/pixel\_grid\_snap.rs ([line 76](../../../src/pixel_grid_snap/pixel_grid_snap.rs.html#76))

```rust
70fn setup_mesh(
71    mut commands: Commands,
72    mut meshes: ResMut<Assets<Mesh>>,
73    mut materials: ResMut<Assets<ColorMaterial>>,
74) {
75    commands.spawn((
76        Mesh2d(meshes.add(Capsule2d::default())),
77        MeshMaterial2d(materials.add(Color::BLACK)),
78        Transform::from_xyz(25., 0., 2.).with_scale(Vec3::splat(32.)),
79        Rotate,
80        PIXEL_PERFECT_LAYERS,
81    ));
82}
83
84fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
85    let canvas_size = Extent3d {
86        width: RES_WIDTH,
87        height: RES_HEIGHT,
88        ..default()
89    };
90
91    // This Image serves as a canvas representing the low-resolution game screen
92    let mut canvas = Image {
93        texture_descriptor: TextureDescriptor {
94            label: None,
95            size: canvas_size,
96            dimension: TextureDimension::D2,
97            format: TextureFormat::Bgra8UnormSrgb,
98            mip_level_count: 1,
99            sample_count: 1,
100            usage: TextureUsages::TEXTURE_BINDING
101                | TextureUsages::COPY_DST
102                | TextureUsages::RENDER_ATTACHMENT,
103            view_formats: &[],
104        },
105        ..default()
106    };
107
108    // Fill image.data with zeroes
109    canvas.resize(canvas_size);
110
111    let image_handle = images.add(canvas);
112
113    // This camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
114    commands.spawn((
115        Camera2d,
116        Camera {
117            // Render before the "main pass" camera
118            order: -1,
119            clear_color: ClearColorConfig::Custom(GRAY.into()),
120            ..default()
121        },
122        RenderTarget::Image(image_handle.clone().into()),
123        Msaa::Off,
124        InGameCamera,
125        PIXEL_PERFECT_LAYERS,
126    ));
127
128    // Spawn the canvas
129    commands.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));
130
131    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
132    // here, the canvas and one of the sample sprites will be rendered by this camera
133    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
134}
```

Additional examples can be found in:  

*   [examples/3d/mirror.rs](../../../src/mirror/mirror.rs.html#226)
*   [examples/async\_tasks/async\_compute.rs](../../../src/async_compute/async_compute.rs.html#49)
*   [examples/shader/animate\_shader.rs](../../../src/animate_shader/animate_shader.rs.html#25)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../../src/async_channel_pattern/async_channel_pattern.rs.html#122)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#293)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#147)
*   [examples/audio/pitch.rs](../../../src/pitch/pitch.rs.html#34)
*   [examples/shader/shader\_material\_2d.rs](../../../src/shader_material_2d/shader_material_2d.rs.html#36)
*   [examples/shader/shader\_material\_wesl.rs](../../../src/shader_material_wesl/shader_material_wesl.rs.html#58)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#202)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#266)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#61)
*   [examples/camera/2d\_top\_down\_camera.rs](../../../src/2d_top_down_camera/2d_top_down_camera.rs.html#38)
*   [examples/shader/fallback\_image.rs](../../../src/fallback_image/fallback_image.rs.html#31)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../../src/fullscreen_material/fullscreen_material.rs.html#38)
*   [examples/3d/reflection\_probes.rs](../../../src/reflection_probes/reflection_probes.rs.html#140)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#49)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#188)
*   [examples/shader/shader\_material.rs](../../../src/shader_material/shader_material.rs.html#26)
*   [examples/shader/shader\_material\_glsl.rs](../../../src/shader_material_glsl/shader_material_glsl.rs.html#27)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#92)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../../src/texture_binding_array/texture_binding_array.rs.html#72)
*   [examples/3d/pccm.rs](../../../src/pccm/pccm.rs.html#109-117)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#210)
*   [examples/shader\_advanced/manual\_material.rs](../../../src/manual_material/manual_material.rs.html#221)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#180)
*   [examples/transforms/3d\_rotation.rs](../../../src/3d_rotation/3d_rotation.rs.html#28)
*   [examples/2d/tilemap\_chunk.rs](../../../src/tilemap_chunk/tilemap_chunk.rs.html#80)
*   [examples/transforms/scale.rs](../../../src/scale/scale.rs.html#44)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#45)
*   [examples/animation/morph\_targets.rs](../../../src/morph_targets/morph_targets.rs.html#40)
*   [tests/window/minimizing.rs](../../../src/minimizing/minimizing.rs.html#37)
*   [tests/window/resizing.rs](../../../src/resizing/resizing.rs.html#113)
*   [examples/transforms/translation.rs](../../../src/translation/translation.rs.html#42)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#90)
*   [examples/shader/shader\_defs.rs](../../../src/shader_defs/shader_defs.rs.html#32)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#171-176)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#59)
*   [examples/2d/sprite\_sheet.rs](../../../src/sprite_sheet/sprite_sheet.rs.html#49)
*   [examples/3d/parenting.rs](../../../src/parenting/parenting.rs.html#31)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#29)
*   [examples/camera/2d\_screen\_shake.rs](../../../src/2d_screen_shake/2d_screen_shake.rs.html#186)
*   [examples/camera/camera\_orbit.rs](../../../src/camera_orbit/camera_orbit.rs.html#59)
*   [examples/shader/shader\_material\_bindless.rs](../../../src/shader_material_bindless/shader_material_bindless.rs.html#59)
*   [examples/remote/server.rs](../../../src/server/server.rs.html#33)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#151)
*   [examples/window/screenshot.rs](../../../src/screenshot/screenshot.rs.html#57)
*   [examples/3d/animated\_material.rs](../../../src/animated_material/animated_material.rs.html#30)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../../src/custom_post_processing/custom_post_processing.rs.html#265)
*   [examples/ecs/entity\_disabling.rs](../../../src/entity_disabling/entity_disabling.rs.html#112)
*   [examples/camera/custom\_projection.rs](../../../src/custom_projection/custom_projection.rs.html#68)
*   [examples/picking/custom\_hit\_data.rs](../../../src/custom_hit_data/custom_hit_data.rs.html#79)
*   [examples/3d/two\_passes.rs](../../../src/two_passes/two_passes.rs.html#20)
*   [examples/stress\_tests/many\_materials.rs](../../../src/many_materials/many_materials.rs.html#77)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#102)
*   [examples/diagnostics/log\_diagnostics.rs](../../../src/log_diagnostics/log_diagnostics.rs.html#65)
*   [examples/animation/animated\_mesh.rs](../../../src/animated_mesh/animated_mesh.rs.html#46)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#193)
*   [examples/shader/extended\_material\_bindless.rs](../../../src/extended_material_bindless/extended_material_bindless.rs.html#116-122)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#194-205)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../../src/custom_render_phase/custom_render_phase.rs.html#82)
*   [examples/dev\_tools/infinite\_grid.rs](../../../src/infinite_grid/infinite_grid.rs.html#50)
*   [examples/3d/atmospheric\_fog.rs](../../../src/atmospheric_fog/atmospheric_fog.rs.html#75)
*   [examples/3d/lines.rs](../../../src/lines/lines.rs.html#29-34)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#129)
*   [examples/window/custom\_cursor\_image.rs](../../../src/custom_cursor_image/custom_cursor_image.rs.html#39)
*   [examples/shader/storage\_buffer.rs](../../../src/storage_buffer/storage_buffer.rs.html#36)
*   [examples/window/low\_power.rs](../../../src/low_power/low_power.rs.html#174)
*   [examples/2d/mesh2d\_vertex\_color\_texture.rs](../../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#33)
*   [examples/shader/array\_texture.rs](../../../src/array_texture/array_texture.rs.html#61)
*   [examples/3d/generate\_custom\_mesh.rs](../../../src/generate_custom_mesh/generate_custom_mesh.rs.html#35)
*   [examples/2d/wireframe\_2d.rs](../../../src/wireframe_2d/wireframe_2d.rs.html#58-62)
*   [examples/3d/mesh\_ray\_cast.rs](../../../src/mesh_ray_cast/mesh_ray_cast.rs.html#77)
*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#145)
*   [examples/audio/spatial\_audio\_2d.rs](../../../src/spatial_audio_2d/spatial_audio_2d.rs.html#37)
*   [examples/picking/simple\_picking.rs](../../../src/simple_picking/simple_picking.rs.html#41)
*   [examples/movement/smooth\_follow.rs](../../../src/smooth_follow/smooth_follow.rs.html#49)
*   [examples/shader/extended\_material.rs](../../../src/extended_material/extended_material.rs.html#33)
*   [examples/ui/ui\_material.rs](../../../src/ui_material/ui_material.rs.html#47-52)
*   [examples/3d/vertex\_colors.rs](../../../src/vertex_colors/vertex_colors.rs.html#20)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#59)
*   [examples/animation/animation\_events.rs](../../../src/animation_events/animation_events.rs.html#90)
*   [examples/asset/generated\_assets.rs](../../../src/generated_assets/generated_assets.rs.html#27)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../../src/custom_shader_instancing/custom_shader_instancing.rs.html#58)
*   [tests/window/desktop\_request\_redraw.rs](../../../src/desktop_request_redraw/desktop_request_redraw.rs.html#80)
*   [examples/3d/orthographic.rs](../../../src/orthographic/orthographic.rs.html#33)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#120)
*   [examples/picking/debug\_picking.rs](../../../src/debug_picking/debug_picking.rs.html#65)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#88)
*   [examples/3d/spherical\_area\_lights.rs](../../../src/spherical_area_lights/spherical_area_lights.rs.html#29)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#166)
*   [examples/usage/cooldown.rs](../../../src/cooldown/cooldown.rs.html#30)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#237)
*   [examples/transforms/transform.rs](../../../src/transform/transform.rs.html#48)
*   [examples/math/render\_primitives.rs](../../../src/render_primitives/render_primitives.rs.html#519)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#31)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#64)
*   [examples/2d/bloom\_2d.rs](../../../src/bloom_2d/bloom_2d.rs.html#44)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#66)
*   [examples/camera/first\_person\_view\_model.rs](../../../src/first_person_view_model/first_person_view_model.rs.html#104)
*   [examples/gizmos/axes.rs](../../../src/axes/axes.rs.html#66)
*   [examples/camera/free\_camera\_controller.rs](../../../src/free_camera_controller/free_camera_controller.rs.html#244)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#76)
*   [examples/app/headless\_renderer.rs](../../../src/headless_renderer/headless_renderer.rs.html#175)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#260-264)
*   [examples/3d/ssao.rs](../../../src/ssao/ssao.rs.html#39-44)
*   [examples/3d/atmosphere.rs](../../../src/atmosphere/atmosphere.rs.html#132)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#94)
*   [examples/app/render\_recovery.rs](../../../src/render_recovery/render_recovery.rs.html#41)
*   [examples/animation/eased\_motion.rs](../../../src/eased_motion/eased_motion.rs.html#41)
*   [examples/audio/spatial\_audio\_3d.rs](../../../src/spatial_audio_3d/spatial_audio_3d.rs.html#29)
*   [examples/shader/automatic\_instancing.rs](../../../src/automatic_instancing/automatic_instancing.rs.html#36)
*   [examples/ecs/error\_handling.rs](../../../src/error_handling/error_handling.rs.html#69)
*   [examples/camera/projection\_zoom.rs](../../../src/projection_zoom/projection_zoom.rs.html#69)
*   [examples/3d/fog.rs](../../../src/fog/fog.rs.html#61-65)
*   [examples/3d/rect\_light.rs](../../../src/rect_light/rect_light.rs.html#35-40)
*   [examples/3d/specular\_tint.rs](../../../src/specular_tint/specular_tint.rs.html#105)
*   [examples/ui/widgets/viewport\_node.rs](../../../src/viewport_node/viewport_node.rs.html#44)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#96)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#121)
*   [examples/2d/sprite\_animation.rs](../../../src/sprite_animation/sprite_animation.rs.html#108)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#205)
*   [examples/2d/cpu\_draw.rs](../../../src/cpu_draw/cpu_draw.rs.html#79)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#83)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../../src/many_cameras_lights/many_cameras_lights.rs.html#42)
*   [examples/3d/texture.rs](../../../src/texture/texture.rs.html#27)
*   [examples/3d/shadow\_caster\_receiver.rs](../../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#35-39)
*   [examples/3d/wireframe.rs](../../../src/wireframe/wireframe.rs.html#63)
*   [examples/3d/bloom\_3d.rs](../../../src/bloom_3d/bloom_3d.rs.html#38-41)
*   [examples/stress\_tests/many\_lights.rs](../../../src/many_lights/many_lights.rs.html#54)
*   [examples/3d/anti\_aliasing.rs](../../../src/anti_aliasing/anti_aliasing.rs.html#421)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#42)
*   [examples/picking/dragdrop\_picking.rs](../../../src/dragdrop_picking/dragdrop_picking.rs.html#85)
*   [examples/transforms/align.rs](../../../src/align/align.rs.html#63)
*   [examples/3d/decal.rs](../../../src/decal/decal.rs.html#31-39)
*   [examples/math/random\_sampling.rs](../../../src/random_sampling/random_sampling.rs.html#62)
*   [examples/testbed/3d.rs](../../../src/testbed_3d/3d.rs.html#135)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#55)
*   [examples/3d/irradiance\_volumes.rs](../../../src/irradiance_volumes/irradiance_volumes.rs.html#553-567)
*   [examples/2d/mesh2d\_alpha\_mode.rs](../../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#26)
*   [examples/2d/mesh2d\_arcs.rs](../../../src/mesh2d_arcs/mesh2d_arcs.rs.html#40)
*   [examples/3d/scrolling\_fog.rs](../../../src/scrolling_fog/scrolling_fog.rs.html#73)
*   [examples/ecs/iter\_combinations.rs](../../../src/iter_combinations/iter_combinations.rs.html#44)
*   [examples/shader\_advanced/compute\_mesh.rs](../../../src/compute_mesh/compute_mesh.rs.html#115)
*   [examples/3d/render\_to\_texture.rs](../../../src/render_to_texture/render_to_texture.rs.html#38)
*   [examples/2d/mesh2d\_manual.rs](../../../src/mesh2d_manual/mesh2d_manual.rs.html#118)
*   [examples/gizmos/transform\_gizmo.rs](../../../src/transform_gizmo/transform_gizmo.rs.html#52)
*   [examples/3d/transparency\_3d.rs](../../../src/transparency_3d/transparency_3d.rs.html#22)
*   [examples/asset/repeated\_texture.rs](../../../src/repeated_texture/repeated_texture.rs.html#28)
*   [examples/shader/gpu\_readback.rs](../../../src/gpu_readback/gpu_readback.rs.html#77)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#52)
*   [examples/3d/spotlight.rs](../../../src/spotlight/spotlight.rs.html#45)
*   [examples/asset/alter\_mesh.rs](../../../src/alter_mesh/alter_mesh.rs.html#101-104)
*   [examples/3d/meshlet.rs](../../../src/meshlet/meshlet.rs.html#73)
*   [examples/3d/pbr.rs](../../../src/pbr/pbr.rs.html#21)
*   [examples/math/custom\_primitives.rs](../../../src/custom_primitives/custom_primitives.rs.html#182)
*   [examples/3d/auto\_exposure.rs](../../../src/auto_exposure/auto_exposure.rs.html#56-64)
*   [examples/animation/animated\_ui.rs](../../../src/animated_ui/animated_ui.rs.html#104)
*   [examples/asset/asset\_loading.rs](../../../src/asset_loading/asset_loading.rs.html#70-73)
*   [examples/gizmos/light\_gizmos.rs](../../../src/light_gizmos/light_gizmos.rs.html#47)
*   [examples/stress\_tests/bevymark\_3d.rs](../../../src/bevymark_3d/bevymark_3d.rs.html#207)
*   [examples/shader/shader\_prepass.rs](../../../src/shader_prepass/shader_prepass.rs.html#63)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#244)
*   [examples/3d/shadow\_biases.rs](../../../src/shadow_biases/shadow_biases.rs.html#40-44)
*   [examples/3d/split\_screen.rs](../../../src/split_screen/split_screen.rs.html#26)
*   [examples/2d/2d\_shapes.rs](../../../src/2d_shapes/2d_shapes.rs.html#55)
*   [examples/showcase/breakout.rs](../../../src/breakout/breakout.rs.html#201)
*   [examples/picking/mesh\_picking.rs](../../../src/mesh_picking/mesh_picking.rs.html#49)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#59)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#123)
*   [examples/3d/parallax\_mapping.rs](../../../src/parallax_mapping/parallax_mapping.rs.html#226)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#68)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#134)
*   [examples/3d/deferred\_rendering.rs](../../../src/deferred_rendering/deferred_rendering.rs.html#87)
*   [examples/3d/3d\_shapes.rs](../../../src/3d_shapes/3d_shapes.rs.html#63-66)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#52-55)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#185-189)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#34)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#224)
*   [examples/animation/animated\_transform.rs](../../../src/animated_transform/animated_transform.rs.html#136)
*   [examples/2d/sprite\_scale.rs](../../../src/sprite_scale/sprite_scale.rs.html#143-149)
*   [examples/3d/lighting.rs](../../../src/lighting/lighting.rs.html#51)
*   [examples/3d/camera\_sub\_view.rs](../../../src/camera_sub_view/camera_sub_view.rs.html#35)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#67)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#236)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#412)

#### pub fn [get\_strong\_handle](#method.get_strong_handle)(&mut self, id: [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>>

Upgrade an `AssetId` into a strong `Handle` that will prevent asset drop.

Returns `None` if the provided `id` is not part of this `Assets` collection. For example, it may have been dropped earlier.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#430)

#### pub fn [get](#method.get)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&A](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Retrieves a reference to the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") with the given `id`, if it exists. Note that this supports anything that implements `Into<AssetId<A>>`, which includes [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") and [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/stress\_tests/many\_animated\_sprites.rs ([line 119](../../../src/many_animated_sprites/many_animated_sprites.rs.html#119))

```rust
108fn animate_sprite(
109    time: Res<Time>,
110    texture_atlases: Res<Assets<TextureAtlasLayout>>,
111    mut query: Query<(&mut AnimationTimer, &mut Sprite)>,
112) {
113    for (mut timer, mut sprite) in query.iter_mut() {
114        timer.tick(time.delta());
115        if timer.just_finished() {
116            let Some(atlas) = &mut sprite.texture_atlas else {
117                continue;
118            };
119            let texture_atlas = texture_atlases.get(&atlas.layout).unwrap();
120            atlas.index = (atlas.index + 1) % texture_atlas.textures.len();
121        }
122    }
123}
```

Hide additional examples

examples/animation/morph\_targets.rs ([line 88](../../../src/morph_targets/morph_targets.rs.html#88))

```rust
80fn name_morphs(
81    asset_server: Res<AssetServer>,
82    mut events: MessageReader<AssetEvent<Mesh>>,
83    meshes: Res<Assets<Mesh>>,
84) {
85    for event in events.read() {
86        if let AssetEvent::<Mesh>::Added { id } = event
87            && let Some(path) = asset_server.get_path(*id)
88            && let Some(mesh) = meshes.get(*id)
89            && let Some(names) = mesh.morph_target_names()
90        {
91            info!("Morph target names for {path:?}:");
92
93            for name in names {
94                info!("  {name}");
95            }
96        }
97    }
98}
```

examples/stress\_tests/many\_animated\_sprite\_meshes.rs ([line 121](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#121))

```rust
110fn animate_sprite(
111    time: Res<Time>,
112    texture_atlases: Res<Assets<TextureAtlasLayout>>,
113    mut query: Query<(&mut AnimationTimer, &mut SpriteMesh)>,
114) {
115    for (mut timer, mut sprite) in query.iter_mut() {
116        timer.tick(time.delta());
117        if timer.just_finished() {
118            let Some(atlas) = &mut sprite.texture_atlas else {
119                continue;
120            };
121            let texture_atlas = texture_atlases.get(&atlas.layout).unwrap();
122            atlas.index = (atlas.index + 1) % texture_atlas.textures.len();
123        }
124    }
125}
```

examples/asset/asset\_saving.rs ([line 43](../../../src/asset_saving/asset_saving.rs.html#43))

```rust
38fn perform_save(
39    image_to_save: Res<ImageToSave>,
40    images: Res<Assets<Image>>,
41    asset_server: Res<AssetServer>,
42) {
43    let image = images.get(&image_to_save.0).unwrap();
44
45    let image = image.clone();
46    let asset_server = asset_server.clone();
47    IoTaskPool::get()
48        .spawn(async move {
49            match save_using_saver(
50                asset_server.clone(),
51                &ImageSaver,
52                &ASSET_PATH.into(),
53                SavedAsset::from_asset(&image),
54                &ImageSaverSettings::default(),
55            )
56            .await
57            {
58                Ok(()) => info!("Completed save of {ASSET_PATH}"),
59                Err(err) => error!("Failed to save asset: {err}"),
60            }
61        })
62        .detach();
63}
```

examples/asset/processing/asset\_processing.rs ([line 258](../../../src/asset_processing/asset_processing.rs.html#258))

```rust
249fn print_text(
250    handles: Res<TextAssets>,
251    texts: Res<Assets<Text>>,
252    mut asset_events: MessageReader<AssetEvent<Text>>,
253) {
254    if !asset_events.is_empty() {
255        // This prints the current values of the assets
256        // Hot-reloading is supported, so try modifying the source assets (and their meta files)!
257        println!("Current Values:");
258        println!("  a: {:?}", texts.get(&handles.a));
259        println!("  b: {:?}", texts.get(&handles.b));
260        println!("  c: {:?}", texts.get(&handles.c));
261        println!("  d: {:?}", texts.get(&handles.d));
262        println!("  e: {:?}", texts.get(&handles.e));
263        println!("(You can modify source assets and their .meta files to hot-reload changes!)");
264        println!();
265        asset_events.clear();
266    }
267}
```

examples/ui/text/font\_atlas\_debug.rs ([line 51](../../../src/font_atlas_debug/font_atlas_debug.rs.html#51))

```rust
39fn atlas_render_system(
40    mut commands: Commands,
41    mut state: ResMut<State>,
42    font_atlas_set: Res<FontAtlasSet>,
43    images: Res<Assets<Image>>,
44) {
45    if let Some(font_atlases) = font_atlas_set.values().next() {
46        let x_offset = state.atlas_count as f32;
47        if state.atlas_count == font_atlases.len() as u32 {
48            return;
49        }
50        let font_atlas = &font_atlases[state.atlas_count as usize];
51        let image = images.get(&font_atlas.texture).unwrap();
52        state.atlas_count += 1;
53        commands.spawn((
54            ImageNode::new(font_atlas.texture.clone()),
55            Node {
56                position_type: PositionType::Absolute,
57                top: Val::ZERO,
58                left: px(image.width() as f32 * x_offset),
59                ..default()
60            },
61        ));
62    }
63}
```

Additional examples can be found in:  

*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#79)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#165)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#109)
*   [examples/asset/custom\_asset.rs](../../../src/custom_asset/custom_asset.rs.html#129)
*   [examples/asset/multi\_asset\_sync.rs](../../../src/multi_asset_sync/multi_asset_sync.rs.html#245)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#240)
*   [examples/animation/animated\_mesh\_control.rs](../../../src/animated_mesh_control/animated_mesh_control.rs.html#110)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#552)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#78)
*   [examples/animation/animated\_mesh\_events.rs](../../../src/animated_mesh_events/animated_mesh_events.rs.html#164)
*   [examples/3d/irradiance\_volumes.rs](../../../src/irradiance_volumes/irradiance_volumes.rs.html#547)
*   [examples/asset/asset\_loading.rs](../../../src/asset_loading/asset_loading.rs.html#39)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#58)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#440)

#### pub fn [get\_mut](#method.get_mut)(&mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[AssetMut](../struct.AssetMut.html "struct bevy::asset::AssetMut")<'\_, A>>

Retrieves a mutable reference to the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") with the given `id`, if it exists. Note that this supports anything that implements `Into<AssetId<A>>`, which includes [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") and [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/3d/reflection\_probes.rs ([line 376](../../../src/reflection_probes/reflection_probes.rs.html#376))

```rust
375fn setup_environment_map_usage(cubemaps: Res<Cubemaps>, mut images: ResMut<Assets<Image>>) {
376    if let Some(mut image) = images.get_mut(&cubemaps.specular_environment_map)
377        && !image
378            .texture_descriptor
379            .usage
380            .contains(TextureUsages::COPY_SRC)
381    {
382        image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
383    }
384}
385
386impl Default for AppStatus {
387    fn default() -> Self {
388        Self {
389            reflection_mode: ReflectionMode::ReflectionProbe,
390            rotating: false,
391            sphere_roughness: 0.2,
392        }
393    }
394}
395
396#[derive(Component)]
397struct SphereMaterial;
398
399#[derive(Component)]
400struct CubesScene;
401
402// A system that changes the sphere's roughness with up/down arrow keys
403fn change_sphere_roughness(
404    keyboard: Res<ButtonInput<KeyCode>>,
405    mut app_status: ResMut<AppStatus>,
406    mut materials: ResMut<Assets<StandardMaterial>>,
407    sphere_query: Query<&MeshMaterial3d<StandardMaterial>, With<SphereMaterial>>,
408) {
409    let roughness_delta = if keyboard.pressed(KeyCode::ArrowUp) {
410        0.01 // Decrease roughness
411    } else if keyboard.pressed(KeyCode::ArrowDown) {
412        -0.01 // Increase roughness
413    } else {
414        0.0 // No change
415    };
416
417    if roughness_delta != 0.0 {
418        // Update the app status
419        app_status.sphere_roughness =
420            (app_status.sphere_roughness + roughness_delta).clamp(0.0, 1.0);
421
422        // Update the sphere material
423        for material_handle in sphere_query.iter() {
424            if let Some(mut material) = materials.get_mut(&material_handle.0) {
425                material.perceptual_roughness = app_status.sphere_roughness;
426            }
427        }
428    }
429}
```

Hide additional examples

examples/animation/animated\_mesh\_events.rs ([line 155](../../../src/animated_mesh_events/animated_mesh_events.rs.html#155))

```rust
148    fn get_clip<'a>(
149        node: AnimationNodeIndex,
150        graph: &AnimationGraph,
151        clips: &'a mut Assets<AnimationClip>,
152    ) -> &'a mut AnimationClip {
153        let node = graph.get(node).unwrap();
154        let clip = match &node.node_type {
155            AnimationNodeType::Clip(handle) => clips.get_mut(handle),
156            _ => unreachable!(),
157        };
158        clip.unwrap().into_inner()
159    }
```

examples/3d/animated\_material.rs ([line 53](../../../src/animated_material/animated_material.rs.html#53))

```rust
47fn animate_materials(
48    material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
49    time: Res<Time>,
50    mut materials: ResMut<Assets<StandardMaterial>>,
51) {
52    for material_handle in material_handles.iter() {
53        if let Some(mut material) = materials.get_mut(material_handle)
54            && let Color::Hsla(ref mut hsla) = material.base_color
55        {
56            *hsla = hsla.rotate_hue(time.delta_secs() * 100.0);
57        }
58    }
59}
```

examples/stress\_tests/many\_materials.rs ([line 95](../../../src/many_materials/many_materials.rs.html#95))

```rust
89fn animate_materials(
90    material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
91    time: Res<Time>,
92    mut materials: ResMut<Assets<StandardMaterial>>,
93) {
94    for (i, material_handle) in material_handles.iter().enumerate() {
95        if let Some(mut material) = materials.get_mut(material_handle) {
96            let color = Color::hsl(
97                ((i as f32 * 2.345 + time.elapsed_secs()) * 100.0) % 360.0,
98                1.0,
99                0.5,
100            );
101            material.base_color = color;
102        }
103    }
104}
```

examples/3d/specular\_tint.rs ([line 156](../../../src/specular_tint/specular_tint.rs.html#156))

```rust
144fn shift_hue(
145    mut app_status: ResMut<AppStatus>,
146    objects_with_materials: Query<&MeshMaterial3d<StandardMaterial>>,
147    mut standard_materials: ResMut<Assets<StandardMaterial>>,
148) {
149    if app_status.tint_type != TintType::Solid {
150        return;
151    }
152
153    app_status.hue += HUE_SHIFT_SPEED;
154
155    for material_handle in objects_with_materials.iter() {
156        let Some(mut material) = standard_materials.get_mut(material_handle) else {
157            continue;
158        };
159        material.specular_tint = Color::hsva(app_status.hue, 1.0, 1.0, 1.0);
160    }
161}
162
163impl AppStatus {
164    /// Returns appropriate help text that reflects the current app status.
165    fn create_text(&self) -> Text {
166        let tint_map_help_text = match self.tint_type {
167            TintType::Solid => SWITCH_TO_MAP_HELP_TEXT,
168            TintType::Map => SWITCH_TO_SOLID_TINT_HELP_TEXT,
169        };
170
171        Text::new(tint_map_help_text)
172    }
173}
174
175/// Changes the specular tint to a solid color or map when the user presses
176/// Space.
177fn toggle_specular_map(
178    keyboard: Res<ButtonInput<KeyCode>>,
179    mut app_status: ResMut<AppStatus>,
180    app_assets: Res<AppAssets>,
181    objects_with_materials: Query<&MeshMaterial3d<StandardMaterial>>,
182    mut standard_materials: ResMut<Assets<StandardMaterial>>,
183) {
184    if !keyboard.just_pressed(KeyCode::Space) {
185        return;
186    }
187
188    // Swap tint type.
189    app_status.tint_type = match app_status.tint_type {
190        TintType::Solid => TintType::Map,
191        TintType::Map => TintType::Solid,
192    };
193
194    for material_handle in objects_with_materials.iter() {
195        let Some(mut material) = standard_materials.get_mut(material_handle) else {
196            continue;
197        };
198
199        // Adjust the tint type.
200        match app_status.tint_type {
201            TintType::Solid => {
202                material.reflectance = 1.0;
203                material.specular_tint_texture = None;
204            }
205            TintType::Map => {
206                // Set reflectance to 2.0 to spread out the map's reflectance
207                // range from the default [0.0, 0.5] to [0.0, 1.0].
208                material.reflectance = 2.0;
209                // As the tint map is multiplied by the tint color, we set the
210                // latter to white so that only the map has an effect.
211                material.specular_tint = WHITE.into();
212                material.specular_tint_texture = Some(app_assets.noise_texture.clone());
213            }
214        };
215    }
216}
```

examples/shader/shader\_material\_wesl.rs ([line 80](../../../src/shader_material_wesl/shader_material_wesl.rs.html#80))

```rust
73fn update(
74    time: Res<Time>,
75    mut query: Query<(&MeshMaterial3d<CustomMaterial>, &mut Transform)>,
76    mut materials: ResMut<Assets<CustomMaterial>>,
77    keys: Res<ButtonInput<KeyCode>>,
78) {
79    for (material, mut transform) in query.iter_mut() {
80        let mut material = materials.get_mut(material).unwrap();
81        material.time.x = time.elapsed_secs();
82        if keys.just_pressed(KeyCode::Space) {
83            material.party_mode = !material.party_mode;
84        }
85
86        if material.party_mode {
87            transform.rotate(Quat::from_rotation_y(0.005));
88        }
89    }
90}
```

Additional examples can be found in:  

*   [examples/asset/alter\_sprite.rs](../../../src/alter_sprite/alter_sprite.rs.html#128)
*   [examples/shader/storage\_buffer.rs](../../../src/storage_buffer/storage_buffer.rs.html#74)
*   [examples/ui/ui\_material.rs](../../../src/ui_material/ui_material.rs.html#97)
*   [examples/3d/rect\_light.rs](../../../src/rect_light/rect_light.rs.html#117)
*   [examples/3d/depth\_of\_field.rs](../../../src/depth_of_field/depth_of_field.rs.html#198)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#216)
*   [examples/shader/shader\_prepass.rs](../../../src/shader_prepass/shader_prepass.rs.html#236)
*   [examples/3d/generate\_custom\_mesh.rs](../../../src/generate_custom_mesh/generate_custom_mesh.rs.html#80)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#244)
*   [examples/3d/skybox.rs](../../../src/skybox/skybox.rs.html#156)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#237)
*   [examples/gltf/query\_gltf\_primitives.rs](../../../src/query_gltf_primitives/query_gltf_primitives.rs.html#29)
*   [examples/3d/lightmaps.rs](../../../src/lightmaps/lightmaps.rs.html#75)
*   [examples/2d/cpu\_draw.rs](../../../src/cpu_draw/cpu_draw.rs.html#110)
*   [examples/asset/alter\_mesh.rs](../../../src/alter_mesh/alter_mesh.rs.html#181)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#436)
*   [examples/3d/blend\_modes.rs](../../../src/blend_modes/blend_modes.rs.html#281)
*   [examples/3d/solari.rs](../../../src/solari/solari.rs.html#397)
*   [examples/3d/mixed\_lighting.rs](../../../src/mixed_lighting/mixed_lighting.rs.html#259)
*   [examples/app/headless\_renderer.rs](../../../src/headless_renderer/headless_renderer.rs.html#471)
*   [examples/3d/transmission.rs](../../../src/transmission/transmission.rs.html#434)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#460)

#### pub fn [get\_mut\_untracked](#method.get_mut_untracked)(&mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut A](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

Retrieves a mutable reference to the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") with the given `id`, if it exists.

This is the same as [`Assets::get_mut`](../../prelude/struct.Assets.html#method.get_mut "method bevy::prelude::Assets::get_mut") except it doesn’t emit [`AssetEvent::Modified`](../../prelude/enum.AssetEvent.html#variant.Modified "variant bevy::prelude::AssetEvent::Modified").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#470)

#### pub fn [remove](#method.remove)(&mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<A>

Removes (and returns) the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") with the given `id`, if it exists. Note that this supports anything that implements `Into<AssetId<A>>`, which includes [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") and [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/asset/asset\_decompression.rs ([line 122](../../../src/asset_decompression/asset_decompression.rs.html#122))

```rust
115fn decompress<T: Component + From<Handle<A>>, A: Asset>(
116    mut commands: Commands,
117    asset_server: Res<AssetServer>,
118    mut compressed_assets: ResMut<Assets<GzAsset>>,
119    query: Query<(Entity, &Compressed<A>)>,
120) {
121    for (entity, Compressed { compressed, .. }) in query.iter() {
122        let Some(GzAsset { uncompressed }) = compressed_assets.remove(compressed) else {
123            continue;
124        };
125
126        let uncompressed = uncompressed.take::<A>().unwrap();
127
128        commands
129            .entity(entity)
130            .remove::<Compressed<A>>()
131            .insert(T::from(asset_server.add(uncompressed)));
132    }
133}
```

Hide additional examples

examples/3d/mirror.rs ([line 409](../../../src/mirror/mirror.rs.html#409))

```rust
388fn handle_window_resize_messages(
389    windows_query: Query<&Window>,
390    mut mirror_cameras_query: Query<&mut RenderTarget, With<MirrorCamera>>,
391    mut images: ResMut<Assets<Image>>,
392    mut mirror_image: ResMut<MirrorImage>,
393    mut screen_space_texture_materials: ResMut<
394        Assets<ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>>,
395    >,
396    mut resize_messages: MessageReader<WindowResized>,
397) {
398    // We run at most once, regardless of the number of window resize messages
399    // there were this frame.
400    let Some(resize_message) = resize_messages.read().next() else {
401        return;
402    };
403    let Ok(window) = windows_query.get(resize_message.window) else {
404        return;
405    };
406
407    let window_size = uvec2(window.physical_width(), window.physical_height());
408    let image = create_mirror_texture_image(&mut images, window_size);
409    images.remove(mirror_image.0.id());
410
411    mirror_image.0 = image.clone();
412
413    for mut target in mirror_cameras_query.iter_mut() {
414        *target = image.clone().into();
415    }
416
417    for (_, material) in screen_space_texture_materials.iter_mut() {
418        material.base.emissive_texture = Some(image.clone());
419    }
420}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#483)

#### pub fn [remove\_untracked](#method.remove_untracked)(&mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<A>

Removes (and returns) the [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") with the given `id`, if it exists. This skips emitting [`AssetEvent::Removed`](../../prelude/enum.AssetEvent.html#variant.Removed "variant bevy::prelude::AssetEvent::Removed"). Note that this supports anything that implements `Into<AssetId<A>>`, which includes [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") and [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId").

This is the same as [`Assets::remove`](../../prelude/struct.Assets.html#method.remove "method bevy::prelude::Assets::remove") except it doesn’t emit [`AssetEvent::Removed`](../../prelude/enum.AssetEvent.html#variant.Removed "variant bevy::prelude::AssetEvent::Removed").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#518)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if there are no assets in this collection.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#523)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of assets currently stored in the collection.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/audio/pitch.rs ([line 37](../../../src/pitch/pitch.rs.html#37))

```rust
25fn play_pitch(
26    mut pitch_assets: ResMut<Assets<Pitch>>,
27    frequency: Res<PitchFrequency>,
28    mut play_pitch_reader: MessageReader<PlayPitch>,
29    mut commands: Commands,
30) {
31    for _ in play_pitch_reader.read() {
32        info!("playing pitch with frequency: {}", frequency.0);
33        commands.spawn((
34            AudioPlayer(pitch_assets.add(Pitch::new(frequency.0, Duration::new(1, 0)))),
35            PlaybackSettings::DESPAWN,
36        ));
37        info!("number of pitch assets: {}", pitch_assets.len());
38    }
39}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#528)

#### pub fn [ids](#method.ids)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>>

Returns an iterator over the [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId") of every [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") stored in this collection.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#536)

#### pub fn [iter](#method.iter)(&self) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = ([AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>, [&A](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

Returns an iterator over the [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId") and [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") ref of every asset in this collection.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#563)

#### pub fn [iter\_mut](#method.iter_mut)(&mut self) -> [AssetsMutIterator](../struct.AssetsMutIterator.html "struct bevy::asset::AssetsMutIterator")<'\_, A> [ⓘ](#)

Returns an iterator over the [`AssetId`](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId") and mutable [`Asset`](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") ref of every asset in this collection.

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/3d/transparency\_3d.rs ([line 112](../../../src/transparency_3d/transparency_3d.rs.html#112))

```rust
110pub fn fade_transparency(time: Res<Time>, mut materials: ResMut<Assets<StandardMaterial>>) {
111    let alpha = (ops::sin(time.elapsed_secs()) / 2.0) + 0.5;
112    for (_, material) in materials.iter_mut() {
113        material.base_color.set_alpha(alpha);
114    }
115}
```

Hide additional examples

examples/stress\_tests/many\_cubes.rs ([line 590](../../../src/many_cubes/many_cubes.rs.html#590))

```rust
588fn update_materials(mut materials: ResMut<Assets<StandardMaterial>>, time: Res<Time>) {
589    let elapsed = time.elapsed_secs();
590    for (i, (_, material)) in materials.iter_mut().enumerate() {
591        let hue = (elapsed + i as f32 * 0.005).rem_euclid(1.0);
592        // This is much faster than using base_color.set_hue(hue), and in a tight loop it shows.
593        let color = fast_hue_to_rgb(hue);
594        material.base_color = Color::linear_rgb(color.x, color.y, color.z);
595    }
596}
```

examples/3d/parallax\_mapping.rs ([line 99](../../../src/parallax_mapping/parallax_mapping.rs.html#99))

```rust
80fn update_parallax_depth_scale(
81    input: Res<ButtonInput<KeyCode>>,
82    mut materials: ResMut<Assets<StandardMaterial>>,
83    mut target_depth: Local<TargetDepth>,
84    mut depth_update: Local<bool>,
85    mut writer: TextUiWriter,
86    text: Single<Entity, With<Text>>,
87) {
88    if input.just_pressed(KeyCode::Digit1) {
89        target_depth.0 -= DEPTH_UPDATE_STEP;
90        target_depth.0 = target_depth.0.max(0.0);
91        *depth_update = true;
92    }
93    if input.just_pressed(KeyCode::Digit2) {
94        target_depth.0 += DEPTH_UPDATE_STEP;
95        target_depth.0 = target_depth.0.min(MAX_DEPTH);
96        *depth_update = true;
97    }
98    if *depth_update {
99        for (_, mat) in materials.iter_mut() {
100            let current_depth = mat.parallax_depth_scale;
101            let new_depth = current_depth.lerp(target_depth.0, DEPTH_CHANGE_RATE);
102            mat.parallax_depth_scale = new_depth;
103            *writer.text(*text, 1) = format!("Parallax depth scale: {new_depth:.5}\n");
104            if (new_depth - current_depth).abs() <= 0.000000001 {
105                *depth_update = false;
106            }
107        }
108    }
109}
110
111fn switch_method(
112    input: Res<ButtonInput<KeyCode>>,
113    mut materials: ResMut<Assets<StandardMaterial>>,
114    text: Single<Entity, With<Text>>,
115    mut writer: TextUiWriter,
116    mut current: Local<CurrentMethod>,
117) {
118    if input.just_pressed(KeyCode::Space) {
119        current.next_method();
120    } else {
121        return;
122    }
123    let text_entity = *text;
124    *writer.text(text_entity, 3) = format!("Method: {}\n", *current);
125
126    for (_, mat) in materials.iter_mut() {
127        mat.parallax_mapping_method = current.0;
128    }
129}
130
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
```

examples/3d/mirror.rs ([line 417](../../../src/mirror/mirror.rs.html#417))

```rust
388fn handle_window_resize_messages(
389    windows_query: Query<&Window>,
390    mut mirror_cameras_query: Query<&mut RenderTarget, With<MirrorCamera>>,
391    mut images: ResMut<Assets<Image>>,
392    mut mirror_image: ResMut<MirrorImage>,
393    mut screen_space_texture_materials: ResMut<
394        Assets<ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>>,
395    >,
396    mut resize_messages: MessageReader<WindowResized>,
397) {
398    // We run at most once, regardless of the number of window resize messages
399    // there were this frame.
400    let Some(resize_message) = resize_messages.read().next() else {
401        return;
402    };
403    let Ok(window) = windows_query.get(resize_message.window) else {
404        return;
405    };
406
407    let window_size = uvec2(window.physical_width(), window.physical_height());
408    let image = create_mirror_texture_image(&mut images, window_size);
409    images.remove(mirror_image.0.id());
410
411    mirror_image.0 = image.clone();
412
413    for mut target in mirror_cameras_query.iter_mut() {
414        *target = image.clone().into();
415    }
416
417    for (_, material) in screen_space_texture_materials.iter_mut() {
418        material.base.emissive_texture = Some(image.clone());
419    }
420}
```

examples/3d/deferred\_rendering.rs ([line 300](../../../src/deferred_rendering/deferred_rendering.rs.html#300))

```rust
279fn switch_mode(
280    mut text: Single<&mut Text>,
281    mut commands: Commands,
282    keys: Res<ButtonInput<KeyCode>>,
283    mut default_opaque_renderer_method: ResMut<DefaultOpaqueRendererMethod>,
284    mut materials: ResMut<Assets<StandardMaterial>>,
285    cameras: Query<Entity, With<Camera>>,
286    mut pause: ResMut<Pause>,
287    mut hide_ui: Local<bool>,
288    mut mode: Local<DefaultRenderMode>,
289) {
290    text.clear();
291
292    if keys.just_pressed(KeyCode::Space) {
293        pause.0 = !pause.0;
294    }
295
296    if keys.just_pressed(KeyCode::Digit1) {
297        *mode = DefaultRenderMode::Deferred;
298        default_opaque_renderer_method.set_to_deferred();
299        println!("DefaultOpaqueRendererMethod: Deferred");
300        for _ in materials.iter_mut() {}
301        for camera in &cameras {
302            commands.entity(camera).remove::<NormalPrepass>();
303            commands.entity(camera).insert(DepthPrepass);
304            commands.entity(camera).insert(MotionVectorPrepass);
305            commands.entity(camera).insert(DeferredPrepass);
306        }
307    }
308    if keys.just_pressed(KeyCode::Digit2) {
309        *mode = DefaultRenderMode::Forward;
310        default_opaque_renderer_method.set_to_forward();
311        println!("DefaultOpaqueRendererMethod: Forward");
312        for _ in materials.iter_mut() {}
313        for camera in &cameras {
314            commands.entity(camera).remove::<NormalPrepass>();
315            commands.entity(camera).remove::<DepthPrepass>();
316            commands.entity(camera).remove::<MotionVectorPrepass>();
317            commands.entity(camera).remove::<DeferredPrepass>();
318        }
319    }
320    if keys.just_pressed(KeyCode::Digit3) {
321        *mode = DefaultRenderMode::ForwardPrepass;
322        default_opaque_renderer_method.set_to_forward();
323        println!("DefaultOpaqueRendererMethod: Forward + Prepass");
324        for _ in materials.iter_mut() {}
325        for camera in &cameras {
326            commands.entity(camera).insert(NormalPrepass);
327            commands.entity(camera).insert(DepthPrepass);
328            commands.entity(camera).insert(MotionVectorPrepass);
329            commands.entity(camera).remove::<DeferredPrepass>();
330        }
331    }
332
333    if keys.just_pressed(KeyCode::KeyH) {
334        *hide_ui = !*hide_ui;
335    }
336
337    if !*hide_ui {
338        text.push_str("(H) Hide UI\n");
339        text.push_str("(Space) Play/Pause\n\n");
340        text.push_str("Rendering Method:\n");
341
342        text.push_str(&format!(
343            "(1) {} Deferred\n",
344            if let DefaultRenderMode::Deferred = *mode {
345                ">"
346            } else {
347                ""
348            }
349        ));
350        text.push_str(&format!(
351            "(2) {} Forward\n",
352            if let DefaultRenderMode::Forward = *mode {
353                ">"
354            } else {
355                ""
356            }
357        ));
358        text.push_str(&format!(
359            "(3) {} Forward + Prepass\n",
360            if let DefaultRenderMode::ForwardPrepass = *mode {
361                ">"
362            } else {
363                ""
364            }
365        ));
366    }
367}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#573)

#### pub fn [track\_assets](#method.track_assets)( assets: [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'\_, [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>>, asset\_server: [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'\_, [AssetServer](../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")\>, )

A system that synchronizes the state of assets in this collection with the [`AssetServer`](../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"). This manages [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") drop events.

## Trait Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

### impl<A> [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"), [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

#### const [STORAGE\_TYPE](../../prelude/trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::SparseSet

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

#### type [Mutability](../../prelude/trait.Component.html#associatedtype.Mutability) = [Mutable](../../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](../../prelude/trait.Component.html "trait bevy::prelude::Component"). [Read more](../../prelude/trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

#### fn [register\_required\_components](../../prelude/trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](../../prelude/trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

#### fn [clone\_behavior](../../prelude/trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](../../prelude/trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

#### fn [relationship\_accessor](../../prelude/trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>>>

Returns [`ComponentRelationshipAccessor`](../../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](../../prelude/trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](../../prelude/trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](../../prelude/trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](../../prelude/trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](../../prelude/trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](../../prelude/trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](../../prelude/trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](../../prelude/trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#298)

### impl<A> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#299)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

### impl<A> [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"), [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

## Auto Trait Implementations

### impl<A> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

### impl<A> [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"),

### impl<A> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

### impl<A> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

### impl<A> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

### impl<A> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

### impl<A> [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"),

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](../../prelude/trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](../../prelude/trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

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

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

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

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","AssetsMutIterator<'\_, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../struct.AssetsMutIterator.html\\" title=\\"struct bevy::asset::AssetsMutIterator\\">AssetsMutIterator</a>&lt;'a, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../struct.AssetsMutIterator.html\\" title=\\"struct bevy::asset::AssetsMutIterator\\">AssetsMutIterator</a>&lt;'a, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"../../prelude/trait.Asset.html\\" title=\\"trait bevy::prelude::Asset\\">Asset</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"enum\\" href=\\"../../prelude/enum.AssetId.html\\" title=\\"enum bevy::prelude::AssetId\\">AssetId</a>&lt;A&gt;, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut A</a>);</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}